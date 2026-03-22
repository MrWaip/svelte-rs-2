//! JS AST analysis functions moved from `svelte_types`.
//!
//! These functions produce metadata (`ExpressionInfo`, `ScriptInfo`, etc.)
//! from OXC AST nodes. They are internal to the analyze crate.

use compact_str::CompactString;
use oxc_ast::ast::Expression;
use svelte_span::Span;
use svelte_parser::{RuneKind, ScriptInfo};

use svelte_ast::{Component, Fragment, Node, NodeId};

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
    data.needs_context = script_info.has_effects || script_info.has_class_state_fields;
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

