//! JS AST analysis functions moved from `svelte_types`.
//!
//! These functions produce metadata (`ExpressionInfo`, `ScriptInfo`, etc.)
//! from OXC AST nodes. They are internal to the analyze crate.

use compact_str::CompactString;
use oxc_ast::ast::Expression;
use oxc_span::GetSpan as _;
use rustc_hash::FxHashSet;
use svelte_span::Span;
use svelte_types::{
    DeclarationInfo, DeclarationKind, ExportInfo, ExpressionInfo, ExpressionKind, PropInfo,
    PropsDeclaration, Reference, ReferenceFlags, RuneKind, ScriptInfo, expression_has_call,
    extract_all_binding_names, is_simple_expr,
};

// ---------------------------------------------------------------------------
// Expression analysis
// ---------------------------------------------------------------------------

use oxc_allocator::Allocator;
use svelte_ast::{Component, Fragment, Node, NodeId};
use svelte_diagnostics::Diagnostic;

use crate::data::{AnalysisData, ParsedExprs};
use crate::scope::ComponentScoping;

// ---------------------------------------------------------------------------
// Entry-point functions (called from analyze pipeline)
// ---------------------------------------------------------------------------

/// Extract ScriptInfo + build Scoping from the pre-parsed script Program.
pub(crate) fn analyze_script(
    parsed: &ParsedExprs<'_>,
    component: &Component,
    data: &mut AnalysisData,
    typescript: bool,
    script_content_span: Option<svelte_span::Span>,
) {
    let Some(ref program) = parsed.script_program else { return };
    let Some(span) = script_content_span else { return };

    let offset = span.start;
    let source = component.source_text(span);

    let mut info = extract_script_info(program, offset, source);

    let sem = oxc_semantic::SemanticBuilder::new().build(program);
    enrich_script_info_from_unresolved(&sem.semantic.scoping(), &mut info);

    // Detect deep store mutations in script body
    info.has_store_member_mutations = program.body.iter().any(|stmt| {
        if let oxc_ast::ast::Statement::ExpressionStatement(es) = stmt {
            has_deep_store_mutation(&es.expression)
        } else {
            false
        }
    });

    data.exports = std::mem::take(&mut info.exports);
    data.needs_context = info.has_effects || info.has_class_state_fields;
    data.has_class_state_fields = info.has_class_state_fields;
    data.scoping = ComponentScoping::from_scoping(sem.semantic.into_scoping());
    data.script = Some(info);
}

/// Parse prop default expressions into the shared allocator.
/// Must be called after `analyze_script` (needs `data.script`).
pub(crate) fn parse_prop_defaults<'a>(
    alloc: &'a Allocator,
    component: &Component,
    data: &AnalysisData,
    parsed: &mut ParsedExprs<'a>,
    typescript: bool,
    diags: &mut Vec<Diagnostic>,
) {
    let Some(ref script_info) = data.script else { return };
    let Some(ref props_decl) = script_info.props_declaration else { return };
    for prop in &props_decl.props {
        if let Some(span) = prop.default_span {
            let src = component.source_text(span);
            let arena_src: &'a str = alloc.alloc_str(src);
            match svelte_parser::js_parse::parse_expression_with_alloc(alloc, arena_src, span.start, typescript) {
                Ok(expr) => parsed.prop_default_exprs.push(Some(expr)),
                Err(diag) => { diags.push(diag); parsed.prop_default_exprs.push(None); }
            }
        } else {
            parsed.prop_default_exprs.push(None);
        }
    }
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

/// Compute each-block key/body index usage from expression references.
/// Must be called after `extract_all_expressions`.
pub(crate) fn compute_each_index_usage(
    parsed: &ParsedExprs<'_>,
    component: &Component,
    data: &mut AnalysisData,
) {
    walk_each_index(&component.fragment, component, parsed, data);
}

fn walk_each_index(
    fragment: &Fragment,
    component: &Component,
    parsed: &ParsedExprs<'_>,
    data: &mut AnalysisData,
) {
    for node in &fragment.nodes {
        match node {
            Node::EachBlock(block) => {
                if let Some(idx_span) = block.index_span {
                    let idx_name = component.source_text(idx_span);
                    // Check if key expression references the index
                    if let Some(key_expr) = parsed.key_exprs.get(&block.id) {
                        let offset = parsed.key_expr_offsets.get(&block.id).copied().unwrap_or(0);
                        let info = extract_expression_info(key_expr, offset);
                        if info.references.iter().any(|r| r.name.as_str() == idx_name) {
                            data.each_blocks.key_uses_index.insert(block.id);
                        }
                    }
                    // Check if body expressions reference the index
                    let body_uses_idx = check_fragment_uses_name(&block.body, idx_name, data);
                    if body_uses_idx {
                        data.each_blocks.body_uses_index.insert(block.id);
                    }
                }
                walk_each_index(&block.body, component, parsed, data);
                if let Some(ref fb) = block.fallback {
                    walk_each_index(fb, component, parsed, data);
                }
            }
            Node::Element(el) => walk_each_index(&el.fragment, component, parsed, data),
            Node::ComponentNode(cn) => walk_each_index(&cn.fragment, component, parsed, data),
            Node::IfBlock(b) => {
                walk_each_index(&b.consequent, component, parsed, data);
                if let Some(ref alt) = b.alternate {
                    walk_each_index(alt, component, parsed, data);
                }
            }
            Node::SnippetBlock(b) => walk_each_index(&b.body, component, parsed, data),
            Node::KeyBlock(b) => walk_each_index(&b.fragment, component, parsed, data),
            Node::SvelteHead(h) => walk_each_index(&h.fragment, component, parsed, data),
            Node::SvelteElement(e) => walk_each_index(&e.fragment, component, parsed, data),
            Node::SvelteBoundary(b) => walk_each_index(&b.fragment, component, parsed, data),
            Node::AwaitBlock(b) => {
                if let Some(ref p) = b.pending { walk_each_index(p, component, parsed, data); }
                if let Some(ref t) = b.then { walk_each_index(t, component, parsed, data); }
                if let Some(ref c) = b.catch { walk_each_index(c, component, parsed, data); }
            }
            _ => {}
        }
    }
}

/// Check if any expression in a fragment references a given name.
fn check_fragment_uses_name(fragment: &Fragment, name: &str, data: &AnalysisData) -> bool {
    for node in &fragment.nodes {
        let refs_match = |id: NodeId| -> bool {
            data.expressions.get(&id)
                .is_some_and(|info| info.references.iter().any(|r| r.name.as_str() == name))
        };
        let attr_refs_match = |attrs: &[svelte_ast::Attribute]| -> bool {
            attrs.iter().any(|a| {
                data.attr_expressions.get(&a.id())
                    .is_some_and(|info| info.references.iter().any(|r| r.name.as_str() == name))
            })
        };
        match node {
            Node::ExpressionTag(t) if refs_match(t.id) => return true,
            Node::Element(el) => {
                if attr_refs_match(&el.attributes) { return true; }
                if check_fragment_uses_name(&el.fragment, name, data) { return true; }
            }
            Node::ComponentNode(cn) => {
                if attr_refs_match(&cn.attributes) { return true; }
                if check_fragment_uses_name(&cn.fragment, name, data) { return true; }
            }
            Node::IfBlock(b) => {
                if refs_match(b.id) { return true; }
                if check_fragment_uses_name(&b.consequent, name, data) { return true; }
                if let Some(ref alt) = b.alternate {
                    if check_fragment_uses_name(alt, name, data) { return true; }
                }
            }
            Node::EachBlock(b) => {
                if refs_match(b.id) { return true; }
                if check_fragment_uses_name(&b.body, name, data) { return true; }
            }
            Node::RenderTag(t) if refs_match(t.id) => return true,
            Node::HtmlTag(t) if refs_match(t.id) => return true,
            Node::KeyBlock(b) => {
                if refs_match(b.id) { return true; }
                if check_fragment_uses_name(&b.fragment, name, data) { return true; }
            }
            Node::ConstTag(t) if refs_match(t.id) => return true,
            Node::SvelteElement(e) => {
                if !e.static_tag && refs_match(e.id) { return true; }
                if attr_refs_match(&e.attributes) { return true; }
                if check_fragment_uses_name(&e.fragment, name, data) { return true; }
            }
            _ => {}
        }
    }
    false
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
        Expression::CallExpression(_) => detect_rune(expr) == Some(target),
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

// ---------------------------------------------------------------------------
// Script analysis
// ---------------------------------------------------------------------------

pub(crate) fn extract_script_info(program: &oxc_ast::ast::Program<'_>, offset: u32, source: &str) -> ScriptInfo {
    let mut declarations = Vec::new();
    let mut props_declaration = None;
    let mut exports = Vec::new();
    let mut has_effects = false;
    let mut has_class_state_fields = false;

    for stmt in &program.body {
        use oxc_ast::ast::Statement;

        match stmt {
            Statement::ExportNamedDeclaration(export) => {
                // `export { x, y as z }` form
                for spec in &export.specifiers {
                    let local = CompactString::from(spec.local.name().as_str());
                    let exported = CompactString::from(spec.exported.name().as_str());
                    let alias = if local != exported { Some(exported) } else { None };
                    exports.push(ExportInfo { name: local, alias });
                }
                // `export const/function/class ...` form
                if let Some(decl) = &export.declaration {
                    collect_export_names_from_declaration(decl, &mut exports);
                    collect_declarations_from_declaration(decl, offset, source, &mut declarations, &mut props_declaration);
                }
            }
            Statement::VariableDeclaration(decl) => {
                collect_var_declarations(decl, offset, source, &mut declarations, &mut props_declaration);
            }
            Statement::FunctionDeclaration(func) => {
                collect_func_declaration(func, offset, &mut declarations);
            }
            Statement::ExpressionStatement(es) => {
                // $effect(fn) and $effect.pre(fn) need context (push/pop).
                // $effect.tracking() does NOT — it's a pure read.
                if is_effect_call(&es.expression) {
                    has_effects = true;
                }
            }
            Statement::ClassDeclaration(class) => {
                if has_class_state_runes(&class.body) {
                    has_class_state_fields = true;
                }
            }
            _ => {}
        }
    }

    ScriptInfo { declarations, props_declaration, exports, has_effects, has_class_state_fields, store_candidates: Vec::new(), has_store_member_mutations: false }
}

/// Enrich ScriptInfo from OXC's unresolved references in one pass.
/// Detects store candidates ($count etc) from unresolved `$`-prefixed references.
pub(crate) fn enrich_script_info_from_unresolved(scoping: &oxc_semantic::Scoping, info: &mut ScriptInfo) {
    for key in scoping.root_unresolved_references().keys() {
        let name = key.as_str();
        if name.starts_with('$') && name.len() > 1 && !name.starts_with("$$") && !is_rune_name(name) {
            info.store_candidates.push(CompactString::from(&name[1..]));
        }
    }
}

fn collect_export_names_from_declaration(
    decl: &oxc_ast::ast::Declaration<'_>,
    exports: &mut Vec<ExportInfo>,
) {
    match decl {
        oxc_ast::ast::Declaration::VariableDeclaration(var_decl) => {
            for declarator in &var_decl.declarations {
                if let oxc_ast::ast::BindingPattern::BindingIdentifier(ident) = &declarator.id {
                    exports.push(ExportInfo { name: CompactString::from(ident.name.as_str()), alias: None });
                }
            }
        }
        oxc_ast::ast::Declaration::FunctionDeclaration(func) => {
            if let Some(ident) = &func.id {
                exports.push(ExportInfo { name: CompactString::from(ident.name.as_str()), alias: None });
            }
        }
        oxc_ast::ast::Declaration::ClassDeclaration(cls) => {
            if let Some(ident) = &cls.id {
                exports.push(ExportInfo { name: CompactString::from(ident.name.as_str()), alias: None });
            }
        }
        _ => {}
    }
}

fn collect_declarations_from_declaration(
    decl: &oxc_ast::ast::Declaration<'_>,
    offset: u32,
    source: &str,
    declarations: &mut Vec<DeclarationInfo>,
    props_declaration: &mut Option<PropsDeclaration>,
) {
    match decl {
        oxc_ast::ast::Declaration::VariableDeclaration(var_decl) => {
            collect_var_declarations(var_decl, offset, source, declarations, props_declaration);
        }
        oxc_ast::ast::Declaration::FunctionDeclaration(func) => {
            collect_func_declaration(func, offset, declarations);
        }
        _ => {}
    }
}

fn collect_func_declaration(
    func: &oxc_ast::ast::Function<'_>,
    offset: u32,
    declarations: &mut Vec<DeclarationInfo>,
) {
    if let Some(ident) = &func.id {
        declarations.push(DeclarationInfo {
            name: CompactString::from(ident.name.as_str()),
            span: Span::new(ident.span.start + offset, ident.span.end + offset),
            kind: DeclarationKind::Function,
            init_span: None,
            is_rune: None,
            rune_init_refs: vec![],
        });
    }
}

fn collect_var_declarations(
    decl: &oxc_ast::ast::VariableDeclaration<'_>,
    offset: u32,
    source: &str,
    declarations: &mut Vec<DeclarationInfo>,
    props_declaration: &mut Option<PropsDeclaration>,
) {
    let kind = match decl.kind {
        oxc_ast::ast::VariableDeclarationKind::Let => DeclarationKind::Let,
        oxc_ast::ast::VariableDeclarationKind::Const => DeclarationKind::Const,
        oxc_ast::ast::VariableDeclarationKind::Var => DeclarationKind::Var,
        _ => DeclarationKind::Var,
    };

    for declarator in &decl.declarations {
        match &declarator.id {
            oxc_ast::ast::BindingPattern::BindingIdentifier(ident) => {
                let name = CompactString::from(ident.name.as_str());
                let decl_span = Span::new(
                    ident.span.start + offset,
                    ident.span.end + offset,
                );

                let (init_span, is_rune, rune_init_refs) = if let Some(init) = &declarator.init {
                    let init_sp = Span::new(
                        init.span().start + offset,
                        init.span().end + offset,
                    );
                    let rune = detect_rune(init);
                    let refs = if matches!(rune, Some(RuneKind::Derived | RuneKind::DerivedBy)) {
                        collect_derived_refs(init)
                    } else {
                        vec![]
                    };
                    (Some(init_sp), rune, refs)
                } else {
                    (None, None, vec![])
                };

                declarations.push(DeclarationInfo {
                    name,
                    span: decl_span,
                    kind,
                    init_span,
                    is_rune,
                    rune_init_refs,
                });
            }
            oxc_ast::ast::BindingPattern::ObjectPattern(obj_pat) => {
                let rune = declarator.init.as_ref().and_then(|init| detect_rune(init));

                if rune == Some(RuneKind::Props) {
                    let mut props = Vec::new();

                    for prop in &obj_pat.properties {
                        let key_name = extract_property_key_name(&prop.key);
                        let Some(key_name) = key_name else { continue };

                        let local_name = extract_binding_name(&prop.value);
                        let local_name = local_name.unwrap_or_else(|| key_name.clone());

                        let (default_span, default_text, is_bindable, is_simple_default) = extract_prop_default(&prop.value, offset, source);

                        let decl_span = Span::new(
                            prop.span.start + offset,
                            prop.span.end + offset,
                        );

                        declarations.push(DeclarationInfo {
                            name: local_name.clone(),
                            span: decl_span,
                            kind,
                            init_span: None,
                            is_rune: Some(RuneKind::Props),
                            rune_init_refs: vec![],
                        });

                        props.push(PropInfo {
                            local_name,
                            prop_name: key_name,
                            default_span,
                            default_text,
                            is_bindable,
                            is_rest: false,
                            is_simple_default,
                        });
                    }

                    if let Some(rest) = &obj_pat.rest {
                        if let oxc_ast::ast::BindingPattern::BindingIdentifier(ident) = &rest.argument {
                            let rest_name = CompactString::from(ident.name.as_str());
                            let decl_span = Span::new(
                                ident.span.start + offset,
                                ident.span.end + offset,
                            );
                            declarations.push(DeclarationInfo {
                                name: rest_name.clone(),
                                span: decl_span,
                                kind,
                                init_span: None,
                                is_rune: Some(RuneKind::Props),
                                rune_init_refs: vec![],
                            });
                            props.push(PropInfo {
                                local_name: rest_name.clone(),
                                prop_name: rest_name,
                                default_span: None,
                                default_text: None,
                                is_bindable: false,
                                is_rest: true,
                                is_simple_default: true,
                            });
                        }
                    }

                    *props_declaration = Some(PropsDeclaration { props });
                } else if matches!(rune, Some(RuneKind::State | RuneKind::StateRaw)) {
                    // Destructured $state/$state.raw: register each leaf binding.
                    // Use StateRaw for analysis so all bindings are considered dynamic
                    // (proxied values are always reactive, even if the binding isn't mutated).
                    let mut names = Vec::new();
                    extract_all_binding_names(&declarator.id, &mut names);
                    for name in names {
                        let decl_span = Span::new(
                            declarator.span.start + offset,
                            declarator.span.end + offset,
                        );
                        declarations.push(DeclarationInfo {
                            name,
                            span: decl_span,
                            kind,
                            init_span: None,
                            is_rune: Some(RuneKind::StateRaw),
                            rune_init_refs: vec![],
                        });
                    }
                }
            }
            oxc_ast::ast::BindingPattern::ArrayPattern(_) => {
                // Destructured $state/$state.raw: register each leaf binding.
                // Use StateRaw so all bindings are considered dynamic in analysis.
                let rune = declarator.init.as_ref().and_then(|init| detect_rune(init));
                if let Some(rune_kind) = rune {
                    if matches!(rune_kind, RuneKind::State | RuneKind::StateRaw) {
                        let mut names = Vec::new();
                        extract_all_binding_names(&declarator.id, &mut names);
                        for name in names {
                            let decl_span = Span::new(
                                declarator.span.start + offset,
                                declarator.span.end + offset,
                            );
                            declarations.push(DeclarationInfo {
                                name,
                                span: decl_span,
                                kind,
                                init_span: None,
                                is_rune: Some(RuneKind::StateRaw),
                                rune_init_refs: vec![],
                            });
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn extract_property_key_name(key: &oxc_ast::ast::PropertyKey<'_>) -> Option<CompactString> {
    match key {
        oxc_ast::ast::PropertyKey::StaticIdentifier(ident) => Some(CompactString::from(ident.name.as_str())),
        oxc_ast::ast::PropertyKey::StringLiteral(s) => Some(CompactString::from(s.value.as_str())),
        _ => None,
    }
}

fn extract_binding_name(pattern: &oxc_ast::ast::BindingPattern<'_>) -> Option<CompactString> {
    match pattern {
        oxc_ast::ast::BindingPattern::BindingIdentifier(ident) => Some(CompactString::from(ident.name.as_str())),
        oxc_ast::ast::BindingPattern::AssignmentPattern(assign) => {
            extract_binding_name(&assign.left)
        }
        _ => None,
    }
}

/// Extract default span, default text, bindable flag, and simplicity flag from a prop's binding pattern.
fn extract_prop_default(pattern: &oxc_ast::ast::BindingPattern<'_>, offset: u32, source: &str) -> (Option<Span>, Option<String>, bool, bool) {
    if let oxc_ast::ast::BindingPattern::AssignmentPattern(assign) = pattern {
        let right = &assign.right;
        // Check if default is $bindable(expr) or $bindable()
        if let Expression::CallExpression(call) = right {
            if let Expression::Identifier(ident) = &call.callee {
                if ident.name.as_str() == "$bindable" {
                    let (default_span, default_text, is_simple) = if let Some(arg) = call.arguments.first() {
                        let sp = arg.span();
                        let text = &source[sp.start as usize..sp.end as usize];
                        let expr = arg.as_expression().expect("argument should be expression");
                        (Some(Span::new(sp.start + offset, sp.end + offset)), Some(text.to_string()), is_simple_expr(expr))
                    } else {
                        (None, None, true)
                    };
                    return (default_span, default_text, true, is_simple);
                }
            }
        }
        let sp = right.span();
        let text = &source[sp.start as usize..sp.end as usize];
        let is_simple = is_simple_expr(right);
        (Some(Span::new(sp.start + offset, sp.end + offset)), Some(text.to_string()), false, is_simple)
    } else {
        (None, None, false, true)
    }
}

/// Returns true for `$effect(fn)` and `$effect.pre(fn)` calls — these need
/// `$.push`/`$.pop` context wrapping. Does NOT match `$effect.tracking()`.
fn is_effect_call(expr: &Expression<'_>) -> bool {
    if let Expression::CallExpression(call) = expr {
        match &call.callee {
            Expression::Identifier(id) if id.name.as_str() == "$effect" => return true,
            Expression::StaticMemberExpression(member) => {
                if let Expression::Identifier(obj) = &member.object {
                    if obj.name.as_str() == "$effect" && member.property.name.as_str() == "pre" {
                        return true;
                    }
                }
            }
            _ => {}
        }
    }
    false
}

/// Check if a class body contains any PropertyDefinition with $state/$state.raw initializer,
/// or constructor assignments like `this.x = $state(...)`.
fn has_class_state_runes(body: &oxc_ast::ast::ClassBody<'_>) -> bool {
    for element in &body.body {
        match element {
            oxc_ast::ast::ClassElement::PropertyDefinition(prop) => {
                if let Some(value) = &prop.value {
                    if let Some(kind) = detect_rune(value) {
                        if matches!(kind, RuneKind::State | RuneKind::StateRaw) {
                            return true;
                        }
                    }
                }
            }
            oxc_ast::ast::ClassElement::MethodDefinition(method) => {
                if method.kind == oxc_ast::ast::MethodDefinitionKind::Constructor {
                    if let Some(body) = &method.value.body {
                        for stmt in &body.statements {
                            if let oxc_ast::ast::Statement::ExpressionStatement(es) = stmt {
                                if let Expression::AssignmentExpression(assign) = &es.expression {
                                    if let Some(kind) = detect_rune(&assign.right) {
                                        if matches!(kind, RuneKind::State | RuneKind::StateRaw) {
                                            return true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    false
}

fn detect_rune(expr: &Expression<'_>) -> Option<RuneKind> {
    if let Expression::CallExpression(call) = expr {
        match &call.callee {
            Expression::Identifier(ident) => {
                return match ident.name.as_str() {
                    "$state" => Some(RuneKind::State),
                    "$derived" => Some(RuneKind::Derived),
                    "$effect" => Some(RuneKind::Effect),
                    "$props" => Some(RuneKind::Props),
                    "$bindable" => Some(RuneKind::Bindable),
                    "$inspect" => Some(RuneKind::Inspect),
                    "$host" => Some(RuneKind::Host),
                    _ => None,
                };
            }
            Expression::StaticMemberExpression(member) => {
                if let Expression::Identifier(obj) = &member.object {
                    let prop = member.property.name.as_str();
                    return match (obj.name.as_str(), prop) {
                        ("$derived", "by") => Some(RuneKind::DerivedBy),
                        ("$state", "raw") => Some(RuneKind::StateRaw),
                        ("$state", "eager") => Some(RuneKind::StateEager),
                        ("$effect", "tracking") => Some(RuneKind::EffectTracking),
                        ("$effect", "pending") => Some(RuneKind::EffectPending),
                        ("$props", "id") => Some(RuneKind::PropsId),
                        _ => None,
                    };
                }
            }
            _ => {}
        }
    }
    None
}

/// Collect identifier references from a $derived/$derived.by call's argument.
/// Returns deduplicated list — avoids redundant `is_dynamic_by_id` lookups.
fn collect_derived_refs(expr: &Expression<'_>) -> Vec<CompactString> {
    let Expression::CallExpression(call) = expr else {
        return vec![];
    };
    if call.arguments.is_empty() {
        return vec![];
    }
    let Some(arg_expr) = call.arguments[0].as_expression() else {
        return vec![];
    };
    let mut refs = Vec::new();
    collect_idents_recursive(arg_expr, &mut refs);
    let mut seen = FxHashSet::default();
    refs.retain(|r| seen.insert(r.clone()));
    refs
}

/// Check if a `$`-prefixed name is a known rune (not a store candidate).
fn is_rune_name(name: &str) -> bool {
    matches!(name, "$state" | "$derived" | "$effect" | "$props" | "$bindable" | "$inspect" | "$host")
}

fn collect_idents_recursive(expr: &Expression<'_>, refs: &mut Vec<CompactString>) {
    use oxc_ast::ast::Expression::*;
    match expr {
        Identifier(id) => {
            let name = id.name.as_str();
            if !name.starts_with('$') {
                refs.push(CompactString::from(name));
            }
        }
        BinaryExpression(bin) => {
            collect_idents_recursive(&bin.left, refs);
            collect_idents_recursive(&bin.right, refs);
        }
        CallExpression(call) => {
            collect_idents_recursive(&call.callee, refs);
            for arg in &call.arguments {
                if let Some(e) = arg.as_expression() {
                    collect_idents_recursive(e, refs);
                }
            }
        }
        ArrowFunctionExpression(arrow) => {
            // Collect refs from arrow body — skip params
            for stmt in &arrow.body.statements {
                match stmt {
                    oxc_ast::ast::Statement::ExpressionStatement(es) => {
                        collect_idents_recursive(&es.expression, refs);
                    }
                    oxc_ast::ast::Statement::ReturnStatement(ret) => {
                        if let Some(arg) = &ret.argument {
                            collect_idents_recursive(arg, refs);
                        }
                    }
                    _ => {}
                }
            }
        }
        UnaryExpression(unary) => {
            collect_idents_recursive(&unary.argument, refs);
        }
        ConditionalExpression(cond) => {
            collect_idents_recursive(&cond.test, refs);
            collect_idents_recursive(&cond.consequent, refs);
            collect_idents_recursive(&cond.alternate, refs);
        }
        LogicalExpression(log) => {
            collect_idents_recursive(&log.left, refs);
            collect_idents_recursive(&log.right, refs);
        }
        StaticMemberExpression(m) => {
            collect_idents_recursive(&m.object, refs);
        }
        ComputedMemberExpression(m) => {
            collect_idents_recursive(&m.object, refs);
            collect_idents_recursive(&m.expression, refs);
        }
        _ => {}
    }
}
