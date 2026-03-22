//! JS AST analysis functions moved from `svelte_types`.
//!
//! These functions produce metadata (`ExpressionInfo`, `ScriptInfo`, etc.)
//! from OXC AST nodes. They are internal to the analyze crate.

use compact_str::CompactString;
use oxc_ast::ast::Expression;
use svelte_span::Span;
use svelte_parser::{RuneKind, ScriptInfo};

use svelte_ast::NodeId;

use crate::data::{
    AnalysisData, ExpressionInfo, ExpressionKind, ParsedExprs, Reference, ReferenceFlags,
};

// ---------------------------------------------------------------------------
// Entry-point functions (called from analyze pipeline)
// ---------------------------------------------------------------------------

/// Enrich pre-extracted ScriptInfo with semantic data and build Scoping.
/// `script_info` comes from `JsParseResult` (extracted by parser).
/// Returns the OXC Scoping for the script block.
pub(crate) fn analyze_script(
    parsed: &ParsedExprs<'_>,
    data: &mut AnalysisData,
    mut script_info: ScriptInfo,
) -> Option<oxc_semantic::Scoping> {
    let Some(ref program) = parsed.script_program else { return None };

    let sem = oxc_semantic::SemanticBuilder::new().build(program);
    svelte_parser::script_info::enrich_from_unresolved(&sem.semantic.scoping(), &mut script_info);

    // Detect deep store mutations in script body
    script_info.has_store_member_mutations = program.body.iter().any(|stmt| {
        if let oxc_ast::ast::Statement::ExpressionStatement(es) = stmt {
            has_deep_store_mutation(&es.expression)
        } else {
            false
        }
    });

    data.exports = std::mem::take(&mut script_info.exports);
    data.needs_context = script_info.has_effects
        || script_info.has_class_state_fields
        || script_body_needs_context(program, sem.semantic.scoping(), &script_info);
    data.has_class_state_fields = script_info.has_class_state_fields;
    data.script = Some(script_info);
    Some(sem.semantic.into_scoping())
}

/// Extract ExpressionInfo for all parsed template and attribute expressions.
pub(crate) fn extract_all_expressions(
    parsed: &ParsedExprs<'_>,
    data: &mut AnalysisData,
) {
    // Template expressions
    for (&node_id, expr) in &parsed.exprs {
        let offset = parsed.expr_offsets.get(&node_id).copied().unwrap_or(0);
        let info = extract_expression_info(expr, offset);
        data.expressions.insert(node_id, info);
    }
    // Attribute expressions
    for (&attr_id, expr) in &parsed.attr_exprs {
        let offset = parsed.attr_expr_offsets.get(&attr_id).copied().unwrap_or(0);
        let info = extract_expression_info(expr, offset);
        data.attr_expressions.insert(attr_id, info);
    }
    // Concatenation attributes: merge references from dynamic parts into a single ExpressionInfo
    let mut concat_attr_ids: rustc_hash::FxHashSet<NodeId> = rustc_hash::FxHashSet::default();
    for &(attr_id, _) in parsed.concat_part_exprs.keys() {
        concat_attr_ids.insert(attr_id);
    }
    for attr_id in concat_attr_ids {
        let mut all_refs = Vec::new();
        let mut dyn_idx = 0usize;
        while let Some(expr) = parsed.concat_part_exprs.get(&(attr_id, dyn_idx)) {
            let offset = parsed.concat_part_offsets.get(&(attr_id, dyn_idx)).copied().unwrap_or(0);
            let info = extract_expression_info(expr, offset);
            all_refs.extend(info.references);
            dyn_idx += 1;
        }
        let merged = ExpressionInfo {
            kind: ExpressionKind::Other,
            references: all_refs,
            has_side_effects: false,
            has_call: false,
            has_state_rune: false,
            has_store_member_mutation: false,
        };
        data.attr_expressions.insert(attr_id, merged);
    }
}

/// Compute render tag argument metadata from parsed CallExpressions.
pub(crate) fn compute_render_tag_args(
    parsed: &ParsedExprs<'_>,
    data: &mut AnalysisData,
) {
    // Collect render tag node IDs (they have callee names set by parser)
    let render_tag_ids: Vec<NodeId> = data.render_tag_callee_name.keys().copied().collect();
    // Also check tags without callee names (member expression callees)
    let all_chain_ids: Vec<NodeId> = data.render_tag_is_chain.iter().copied().collect();

    let mut all_ids_set: rustc_hash::FxHashSet<NodeId> = rustc_hash::FxHashSet::default();
    all_ids_set.extend(render_tag_ids);
    all_ids_set.extend(all_chain_ids);
    let all_ids: Vec<NodeId> = all_ids_set.into_iter().collect();

    for node_id in all_ids {
        if let Some(Expression::CallExpression(call)) = parsed.exprs.get(&node_id) {
            let flags: Vec<bool> = call.arguments.iter().map(|arg| {
                expression_has_call(arg.to_expression())
            }).collect();
            data.render_tag_arg_has_call.insert(node_id, flags);

            let idents: Vec<Option<String>> = call.arguments.iter().map(|arg| {
                if let Expression::Identifier(id) = arg.to_expression() {
                    Some(id.name.to_string())
                } else {
                    None
                }
            }).collect();
            data.render_tag_arg_idents.insert(node_id, idents);
        }
    }
}

// ---------------------------------------------------------------------------
// needs_context detection (matches Svelte reference 2-analyze visitors)
// ---------------------------------------------------------------------------

/// Walk top-level script body to detect expressions that require component context.
/// Checks for: NewExpression, CallExpression with non-safe callee,
/// MemberExpression with non-safe root.
fn script_body_needs_context(
    program: &oxc_ast::ast::Program<'_>,
    scoping: &oxc_semantic::Scoping,
    script_info: &ScriptInfo,
) -> bool {
    // Collect prop declaration names for is_safe_identifier check
    let prop_names: rustc_hash::FxHashSet<&str> = script_info
        .declarations
        .iter()
        .filter(|d| d.is_rune == Some(RuneKind::Props))
        .map(|d| d.name.as_str())
        .collect();

    for stmt in &program.body {
        if stmt_needs_context(stmt, scoping, &prop_names) {
            return true;
        }
    }
    false
}

fn stmt_needs_context(
    stmt: &oxc_ast::ast::Statement<'_>,
    scoping: &oxc_semantic::Scoping,
    prop_names: &rustc_hash::FxHashSet<&str>,
) -> bool {
    match stmt {
        oxc_ast::ast::Statement::VariableDeclaration(decl) => {
            for declarator in &decl.declarations {
                if let Some(init) = &declarator.init {
                    // Skip rune wrappers — check inner expression for $state/$derived/etc.
                    let inner = unwrap_rune_arg(init);
                    if expr_needs_context(inner, scoping, prop_names) {
                        return true;
                    }
                }
            }
            false
        }
        oxc_ast::ast::Statement::ExpressionStatement(es) => {
            expr_needs_context(&es.expression, scoping, prop_names)
        }
        _ => false,
    }
}

fn expr_needs_context(
    expr: &Expression<'_>,
    scoping: &oxc_semantic::Scoping,
    prop_names: &rustc_hash::FxHashSet<&str>,
) -> bool {
    match expr {
        Expression::NewExpression(_) => true,
        Expression::CallExpression(call) => {
            !is_safe_identifier(&call.callee, scoping, prop_names)
        }
        Expression::StaticMemberExpression(_) | Expression::ComputedMemberExpression(_) => {
            !is_safe_identifier(expr, scoping, prop_names)
        }
        _ => false,
    }
}

/// A 'safe' identifier means foo in foo.bar or foo() will not call functions
/// that require component context. Mirrors reference utils.js:is_safe_identifier.
fn is_safe_identifier(
    expr: &Expression<'_>,
    scoping: &oxc_semantic::Scoping,
    prop_names: &rustc_hash::FxHashSet<&str>,
) -> bool {
    // Walk member chain to root
    let mut node = expr;
    loop {
        match node {
            Expression::StaticMemberExpression(m) => node = &m.object,
            Expression::ComputedMemberExpression(m) => node = &m.object,
            _ => break,
        }
    }

    let Expression::Identifier(ident) = node else { return false };
    let name = ident.name.as_str();

    // Prop bindings are not safe (they come from parent context)
    if prop_names.contains(name) {
        return false;
    }

    // Check OXC scoping for the identifier
    let root_scope = scoping.root_scope_id();
    if let Some(sym_id) = scoping.find_binding(root_scope, name.into()) {
        let flags = scoping.symbol_flags(sym_id);
        // Imports are not safe — they may call functions needing context
        if flags.contains(oxc_semantic::SymbolFlags::Import) {
            return false;
        }
        // Local binding (not import, not prop) — safe
        true
    } else {
        // No binding = global (Map, console, etc.) — safe
        true
    }
}

/// Unwrap a rune call to get its first argument expression.
/// E.g., `$derived(expr)` → `expr`, `$state(expr)` → `expr`.
/// Non-rune expressions pass through unchanged.
fn unwrap_rune_arg<'a>(expr: &'a Expression<'a>) -> &'a Expression<'a> {
    if let Expression::CallExpression(call) = expr {
        let is_rune = match &call.callee {
            Expression::Identifier(id) => svelte_parser::script_info::is_rune_name(&id.name),
            Expression::StaticMemberExpression(m) => {
                if let Expression::Identifier(obj) = &m.object {
                    svelte_parser::script_info::is_rune_name(&obj.name)
                } else {
                    false
                }
            }
            _ => false,
        };
        if is_rune {
            if let Some(arg) = call.arguments.first() {
                if let Some(e) = arg.as_expression() {
                    return e;
                }
            }
        }
    }
    expr
}

// ---------------------------------------------------------------------------
// Expression analysis (helpers)
// ---------------------------------------------------------------------------

pub(crate) fn extract_expression_info(expr: &Expression<'_>, offset: u32) -> ExpressionInfo {
    let kind = match expr {
        Expression::Identifier(ident) => ExpressionKind::Identifier(CompactString::from(ident.name.as_str())),
        Expression::NumericLiteral(_)
        | Expression::StringLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::NullLiteral(_) => ExpressionKind::Literal,
        Expression::CallExpression(call) => {
            let callee = match &call.callee {
                Expression::Identifier(ident) => CompactString::from(ident.name.as_str()),
                _ => CompactString::default(),
            };
            ExpressionKind::CallExpression { callee }
        }
        Expression::StaticMemberExpression(_) | Expression::ComputedMemberExpression(_) => {
            ExpressionKind::MemberExpression
        }
        Expression::ArrowFunctionExpression(_) => ExpressionKind::ArrowFunction,
        Expression::AssignmentExpression(_) => ExpressionKind::Assignment,
        _ => ExpressionKind::Other,
    };

    let mut references = Vec::new();
    collect_references(expr, offset, &mut references);

    let has_side_effects = matches!(
        expr,
        Expression::CallExpression(_)
            | Expression::AssignmentExpression(_)
            | Expression::UpdateExpression(_)
    );

    let has_call = expression_has_call(expr);

    let has_state_rune = expression_has_rune(expr, RuneKind::EffectPending)
        || expression_has_rune(expr, RuneKind::StateEager);

    let has_store_member_mutation = has_deep_store_mutation(expr);

    ExpressionInfo {
        kind,
        references,
        has_side_effects,
        has_call,
        has_state_rune,
        has_store_member_mutation,
    }
}

/// Check if the expression (or any sub-expression) contains a call to a specific rune.
fn expression_has_rune(expr: &Expression<'_>, target: RuneKind) -> bool {
    match expr {
        Expression::CallExpression(_) => svelte_parser::script_info::detect_rune(expr) == Some(target),
        Expression::ConditionalExpression(c) => {
            expression_has_rune(&c.test, target)
                || expression_has_rune(&c.consequent, target)
                || expression_has_rune(&c.alternate, target)
        }
        Expression::BinaryExpression(b) => {
            expression_has_rune(&b.left, target) || expression_has_rune(&b.right, target)
        }
        Expression::LogicalExpression(l) => {
            expression_has_rune(&l.left, target) || expression_has_rune(&l.right, target)
        }
        Expression::SequenceExpression(s) => s.expressions.iter().any(|e| expression_has_rune(e, target)),
        _ => false,
    }
}

/// Check if expression contains a deep mutation on a $-prefixed identifier
/// (e.g., `$store.field = val` or `$store.count++`).
pub(crate) fn has_deep_store_mutation(expr: &Expression<'_>) -> bool {
    match expr {
        Expression::AssignmentExpression(assign) => {
            let has_store_member_lhs = match &assign.left {
                oxc_ast::ast::AssignmentTarget::StaticMemberExpression(m) => {
                    member_root_is_store(&m.object)
                }
                oxc_ast::ast::AssignmentTarget::ComputedMemberExpression(m) => {
                    member_root_is_store(&m.object)
                }
                _ => false,
            };
            has_store_member_lhs || has_deep_store_mutation(&assign.right)
        }
        Expression::UpdateExpression(upd) => {
            match &upd.argument {
                oxc_ast::ast::SimpleAssignmentTarget::StaticMemberExpression(m) => {
                    member_root_is_store(&m.object)
                }
                oxc_ast::ast::SimpleAssignmentTarget::ComputedMemberExpression(m) => {
                    member_root_is_store(&m.object)
                }
                _ => false,
            }
        }
        Expression::ArrowFunctionExpression(arrow) => {
            arrow.body.statements.iter().any(|stmt| {
                if let oxc_ast::ast::Statement::ExpressionStatement(es) = stmt {
                    has_deep_store_mutation(&es.expression)
                } else {
                    false
                }
            })
        }
        Expression::SequenceExpression(seq) => {
            seq.expressions.iter().any(|e| has_deep_store_mutation(e))
        }
        Expression::ConditionalExpression(c) => {
            has_deep_store_mutation(&c.test)
                || has_deep_store_mutation(&c.consequent)
                || has_deep_store_mutation(&c.alternate)
        }
        _ => false,
    }
}

/// Check if the root of a member expression chain is a $-prefixed identifier.
fn member_root_is_store(expr: &Expression<'_>) -> bool {
    match expr {
        Expression::Identifier(id) => id.name.starts_with('$') && id.name.len() > 1,
        Expression::StaticMemberExpression(m) => member_root_is_store(&m.object),
        Expression::ComputedMemberExpression(m) => member_root_is_store(&m.object),
        _ => false,
    }
}

pub(crate) fn collect_references(expr: &Expression<'_>, offset: u32, refs: &mut Vec<Reference>) {
    match expr {
        Expression::Identifier(ident) => {
            refs.push(Reference {
                name: CompactString::from(ident.name.as_str()),
                span: Span::new(
                    ident.span.start + offset,
                    ident.span.end + offset,
                ),
                flags: ReferenceFlags::Read,
                symbol_id: None,
            });
        }
        Expression::AssignmentExpression(assign) => {
            // LHS: collect write reference from identifier or read reference from member chain root
            match &assign.left {
                oxc_ast::ast::AssignmentTarget::AssignmentTargetIdentifier(ident) => {
                    refs.push(Reference {
                        name: CompactString::from(ident.name.as_str()),
                        span: Span::new(
                            ident.span.start + offset,
                            ident.span.end + offset,
                        ),
                        flags: ReferenceFlags::Write,
                        symbol_id: None,
                    });
                }
                oxc_ast::ast::AssignmentTarget::StaticMemberExpression(m) => {
                    collect_references(&m.object, offset, refs);
                }
                oxc_ast::ast::AssignmentTarget::ComputedMemberExpression(m) => {
                    collect_references(&m.object, offset, refs);
                    collect_references(&m.expression, offset, refs);
                }
                _ => {}
            }
            collect_references(&assign.right, offset, refs);
        }
        Expression::BinaryExpression(bin) => {
            collect_references(&bin.left, offset, refs);
            collect_references(&bin.right, offset, refs);
        }
        Expression::LogicalExpression(log) => {
            collect_references(&log.left, offset, refs);
            collect_references(&log.right, offset, refs);
        }
        Expression::UnaryExpression(un) => {
            collect_references(&un.argument, offset, refs);
        }
        Expression::UpdateExpression(upd) => {
            match &upd.argument {
                oxc_ast::ast::SimpleAssignmentTarget::AssignmentTargetIdentifier(ident) => {
                    refs.push(Reference {
                        name: CompactString::from(ident.name.as_str()),
                        span: Span::new(ident.span.start + offset, ident.span.end + offset),
                        flags: ReferenceFlags::Write,
                        symbol_id: None,
                    });
                }
                // Walk member chain to collect root identifier (e.g., $store in $store.count++)
                oxc_ast::ast::SimpleAssignmentTarget::StaticMemberExpression(m) => {
                    collect_references(&m.object, offset, refs);
                }
                oxc_ast::ast::SimpleAssignmentTarget::ComputedMemberExpression(m) => {
                    collect_references(&m.object, offset, refs);
                    collect_references(&m.expression, offset, refs);
                }
                _ => {}
            }
        }
        Expression::CallExpression(call) => {
            collect_references(&call.callee, offset, refs);
            for arg in &call.arguments {
                if let oxc_ast::ast::Argument::SpreadElement(spread) = arg {
                    collect_references(&spread.argument, offset, refs);
                } else if let Some(expr) = arg.as_expression() {
                    collect_references(expr, offset, refs);
                }
            }
        }
        Expression::ConditionalExpression(cond) => {
            collect_references(&cond.test, offset, refs);
            collect_references(&cond.consequent, offset, refs);
            collect_references(&cond.alternate, offset, refs);
        }
        Expression::StaticMemberExpression(mem) => {
            collect_references(&mem.object, offset, refs);
        }
        Expression::ComputedMemberExpression(mem) => {
            collect_references(&mem.object, offset, refs);
            collect_references(&mem.expression, offset, refs);
        }
        Expression::TemplateLiteral(tl) => {
            for expr in &tl.expressions {
                collect_references(expr, offset, refs);
            }
        }
        Expression::ParenthesizedExpression(paren) => {
            collect_references(&paren.expression, offset, refs);
        }
        Expression::ArrayExpression(arr) => {
            for elem in &arr.elements {
                match elem {
                    oxc_ast::ast::ArrayExpressionElement::SpreadElement(spread) => {
                        collect_references(&spread.argument, offset, refs);
                    }
                    _ => {
                        if let Some(expr) = elem.as_expression() {
                            collect_references(expr, offset, refs);
                        }
                    }
                }
            }
        }
        Expression::ObjectExpression(obj) => {
            for prop in &obj.properties {
                match prop {
                    oxc_ast::ast::ObjectPropertyKind::ObjectProperty(p) => {
                        collect_references(&p.value, offset, refs);
                    }
                    oxc_ast::ast::ObjectPropertyKind::SpreadProperty(spread) => {
                        collect_references(&spread.argument, offset, refs);
                    }
                }
            }
        }
        Expression::ArrowFunctionExpression(arrow) => {
            for stmt in &arrow.body.statements {
                collect_statement_references(stmt, offset, refs);
            }
        }
        Expression::SequenceExpression(seq) => {
            for expr in &seq.expressions {
                collect_references(expr, offset, refs);
            }
        }
        _ => {}
    }
}

fn collect_statement_references(stmt: &oxc_ast::ast::Statement<'_>, offset: u32, refs: &mut Vec<Reference>) {
    use oxc_ast::ast::Statement;
    match stmt {
        Statement::ExpressionStatement(es) => collect_references(&es.expression, offset, refs),
        Statement::ReturnStatement(ret) => {
            if let Some(arg) = &ret.argument {
                collect_references(arg, offset, refs);
            }
        }
        Statement::BlockStatement(block) => {
            for s in &block.body {
                collect_statement_references(s, offset, refs);
            }
        }
        Statement::IfStatement(if_stmt) => {
            collect_references(&if_stmt.test, offset, refs);
            collect_statement_references(&if_stmt.consequent, offset, refs);
            if let Some(alt) = &if_stmt.alternate {
                collect_statement_references(alt, offset, refs);
            }
        }
        Statement::VariableDeclaration(decl) => {
            for d in &decl.declarations {
                if let Some(init) = &d.init {
                    collect_references(init, offset, refs);
                }
            }
        }
        _ => {}
    }
}

pub(crate) fn expression_has_call(expr: &Expression<'_>) -> bool {
    match expr {
        Expression::CallExpression(_) => true,
        Expression::ConditionalExpression(c) => {
            expression_has_call(&c.test)
                || expression_has_call(&c.consequent)
                || expression_has_call(&c.alternate)
        }
        Expression::BinaryExpression(b) => {
            expression_has_call(&b.left) || expression_has_call(&b.right)
        }
        Expression::LogicalExpression(l) => {
            expression_has_call(&l.left) || expression_has_call(&l.right)
        }
        Expression::StaticMemberExpression(m) => expression_has_call(&m.object),
        Expression::ComputedMemberExpression(m) => {
            expression_has_call(&m.object) || expression_has_call(&m.expression)
        }
        Expression::UnaryExpression(u) => expression_has_call(&u.argument),
        Expression::SequenceExpression(s) => s.expressions.iter().any(|e| expression_has_call(e)),
        // Function boundaries are opaque
        Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_) => false,
        _ => false,
    }
}

