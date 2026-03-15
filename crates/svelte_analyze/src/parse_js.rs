use oxc_allocator::Allocator;
use oxc_ast::ast::{AssignmentTarget, AssignmentTargetMaybeDefault, Expression};
use oxc_parser::Parser as OxcParser;
use oxc_span::{GetSpan, SourceType};
use svelte_ast::{Attribute, Component, ConcatPart, Fragment, Node, NodeId, ScriptLanguage};
use svelte_diagnostics::Diagnostic;
use svelte_js::{ExpressionInfo, ExpressionKind};

use crate::data::{AnalysisData, ParsedExprs};
use crate::scope::ComponentScoping;

pub fn parse_js<'a>(
    alloc: &'a Allocator,
    component: &Component,
    data: &mut AnalysisData,
    parsed: &mut ParsedExprs<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    if let Some(script) = &component.script {
        let source = component.source_text(script.content_span);
        let typescript = matches!(script.language, ScriptLanguage::TypeScript);
        let arena_source: &'a str = alloc.alloc_str(source);
        match svelte_js::analyze_script_with_alloc(
            alloc,
            arena_source,
            script.content_span.start,
            typescript,
        ) {
            Ok((mut info, scoping, program)) => {
                data.exports = std::mem::take(&mut info.exports);
                data.needs_context = info.has_effects;
                data.script = Some(info);
                data.scoping = ComponentScoping::from_scoping(scoping);
                parsed.script_program = Some(program);
            }
            Err(errs) => diags.extend(errs),
        }
    }

    walk_fragment(alloc, &component.fragment, component, data, parsed, diags);
}

/// Parse an expression into the shared allocator, storing both metadata and AST.
fn parse_expr<'a>(
    alloc: &'a Allocator,
    source: &str,
    offset: u32,
    node_id: NodeId,
    data: &mut AnalysisData,
    parsed: &mut ParsedExprs<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    // Copy source into arena so Expression AST can reference it with lifetime 'a
    let arena_source: &'a str = alloc.alloc_str(source);
    match svelte_js::analyze_expression_with_alloc(alloc, arena_source, offset) {
        Ok((info, expr)) => {
            data.expressions.insert(node_id, info);
            parsed.exprs.insert(node_id, expr);
        }
        Err(diag) => diags.push(diag),
    }
}

/// Parse an attribute expression into the shared allocator.
fn parse_attr_expr<'a>(
    alloc: &'a Allocator,
    source: &str,
    offset: u32,
    key: (NodeId, usize),
    data: &mut AnalysisData,
    parsed: &mut ParsedExprs<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    let arena_source: &'a str = alloc.alloc_str(source);
    match svelte_js::analyze_expression_with_alloc(alloc, arena_source, offset) {
        Ok((info, expr)) => {
            data.attr_expressions.insert(key, info);
            parsed.attr_exprs.insert(key, expr);
        }
        Err(diag) => diags.push(diag),
    }
}

/// Parse concatenation parts (shared by ConcatenationAttribute and StyleDirective::Concatenation).
fn parse_concat_parts<'a>(
    alloc: &'a Allocator,
    parts: &[ConcatPart],
    owner_id: NodeId,
    attr_idx: usize,
    component: &Component,
    data: &mut AnalysisData,
    parsed: &mut ParsedExprs<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    let mut all_refs = Vec::new();
    let mut dyn_idx = 0usize;
    for part in parts {
        if let ConcatPart::Dynamic(span) = part {
            let source = component.source_text(*span);
            let arena_source: &'a str = alloc.alloc_str(source);
            match svelte_js::analyze_expression_with_alloc(alloc, arena_source, span.start) {
                Ok((info, expr)) => {
                    all_refs.extend(info.references);
                    parsed.concat_part_exprs.insert((owner_id, attr_idx, dyn_idx), expr);
                }
                Err(diag) => diags.push(diag),
            }
            dyn_idx += 1;
        }
    }
    let merged = ExpressionInfo {
        kind: ExpressionKind::Other,
        references: all_refs,
        has_side_effects: false,
    };
    data.attr_expressions.insert((owner_id, attr_idx), merged);
}

fn walk_fragment<'a>(
    alloc: &'a Allocator,
    fragment: &Fragment,
    component: &Component,
    data: &mut AnalysisData,
    parsed: &mut ParsedExprs<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    for node in &fragment.nodes {
        walk_node(alloc, node, component, data, parsed, diags);
    }
}

fn walk_node<'a>(
    alloc: &'a Allocator,
    node: &Node,
    component: &Component,
    data: &mut AnalysisData,
    parsed: &mut ParsedExprs<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    match node {
        Node::ExpressionTag(tag) => {
            let source = component.source_text(tag.expression_span);
            parse_expr(alloc, source, tag.expression_span.start, tag.id, data, parsed, diags);
        }
        Node::Element(el) => {
            walk_attrs(alloc, el.id, &el.attributes, component, data, parsed, diags);
            walk_fragment(alloc, &el.fragment, component, data, parsed, diags);
        }
        Node::ComponentNode(cn) => {
            walk_attrs(alloc, cn.id, &cn.attributes, component, data, parsed, diags);
            walk_fragment(alloc, &cn.fragment, component, data, parsed, diags);
        }
        Node::IfBlock(block) => {
            let source = component.source_text(block.test_span);
            parse_expr(alloc, source, block.test_span.start, block.id, data, parsed, diags);
            walk_fragment(alloc, &block.consequent, component, data, parsed, diags);
            if let Some(alt) = &block.alternate {
                walk_fragment(alloc, alt, component, data, parsed, diags);
            }
        }
        Node::EachBlock(block) => {
            let source = component.source_text(block.expression_span);
            parse_expr(alloc, source, block.expression_span.start, block.id, data, parsed, diags);
            walk_fragment(alloc, &block.body, component, data, parsed, diags);
            if let Some(fb) = &block.fallback {
                walk_fragment(alloc, fb, component, data, parsed, diags);
            }
        }
        Node::SnippetBlock(block) => {
            walk_fragment(alloc, &block.body, component, data, parsed, diags);
        }
        Node::RenderTag(tag) => {
            let source = component.source_text(tag.expression_span);
            parse_expr(alloc, source, tag.expression_span.start, tag.id, data, parsed, diags);
        }
        Node::HtmlTag(tag) => {
            let source = component.source_text(tag.expression_span);
            parse_expr(alloc, source, tag.expression_span.start, tag.id, data, parsed, diags);
        }
        Node::KeyBlock(block) => {
            let source = component.source_text(block.expression_span);
            parse_expr(alloc, source, block.expression_span.start, block.id, data, parsed, diags);
            walk_fragment(alloc, &block.fragment, component, data, parsed, diags);
        }
        Node::ConstTag(tag) => {
            parse_const_tag(alloc, tag, component, data, parsed, diags);
        }
        Node::Text(_) | Node::Comment(_) | Node::Error(_) => {}
    }
}

/// Parse and store attribute expressions, keyed by (owner_id, attr_index).
fn walk_attrs<'a>(
    alloc: &'a Allocator,
    owner_id: NodeId,
    attrs: &[Attribute],
    component: &Component,
    data: &mut AnalysisData,
    parsed: &mut ParsedExprs<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    for (attr_idx, attr) in attrs.iter().enumerate() {
        let key = (owner_id, attr_idx);
        match attr {
            Attribute::ExpressionAttribute(a) => {
                let source = component.source_text(a.expression_span);
                parse_attr_expr(alloc, source, a.expression_span.start, key, data, parsed, diags);
            }
            Attribute::ConcatenationAttribute(a) => {
                parse_concat_parts(alloc, &a.parts, owner_id, attr_idx, component, data, parsed, diags);
            }
            Attribute::ClassDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, key, data, parsed, diags);
                }
            }
            Attribute::StyleDirective(a) => {
                use svelte_ast::StyleDirectiveValue;
                match &a.value {
                    StyleDirectiveValue::Expression(span) => {
                        let source = component.source_text(*span);
                        parse_attr_expr(alloc, source, span.start, key, data, parsed, diags);
                    }
                    StyleDirectiveValue::Concatenation(parts) => {
                        parse_concat_parts(alloc, parts, owner_id, attr_idx, component, data, parsed, diags);
                    }
                    StyleDirectiveValue::Shorthand | StyleDirectiveValue::String(_) => {}
                }
            }
            Attribute::BindDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, key, data, parsed, diags);
                }
            }
            Attribute::ShorthandOrSpread(a) => {
                // For spread attrs, skip the "..." prefix
                let span = if a.is_spread {
                    debug_assert!(
                        a.expression_span.end >= a.expression_span.start + 3,
                        "spread expression span too short to contain '...'"
                    );
                    svelte_span::Span::new(a.expression_span.start + 3, a.expression_span.end)
                } else {
                    a.expression_span
                };
                let source = component.source_text(span);
                parse_attr_expr(alloc, source, span.start, key, data, parsed, diags);
            }
            Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_) => {}
            // LEGACY(svelte4): on:directive — parse expression if present
            Attribute::OnDirectiveLegacy(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, key, data, parsed, diags);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// ConstTag parsing — handles both simple identifiers and destructuring
// ---------------------------------------------------------------------------

fn parse_const_tag<'a>(
    alloc: &'a Allocator,
    tag: &svelte_ast::ConstTag,
    component: &Component,
    data: &mut AnalysisData,
    parsed: &mut ParsedExprs<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    let decl_text = component.source_text(tag.declaration_span);

    // Parse the full declaration text as an expression using a temp allocator.
    // "x = expr" or "{ x, y } = expr" are valid AssignmentExpressions.
    let temp_alloc = Allocator::default();
    let temp_src: &str = temp_alloc.alloc_str(decl_text);
    let Ok(full_expr) = OxcParser::new(&temp_alloc, temp_src, SourceType::default())
        .parse_expression()
    else {
        return;
    };

    let Expression::AssignmentExpression(assign) = &full_expr else {
        return;
    };

    // Extract names from LHS
    let mut names = Vec::new();
    let is_destructured = extract_target_names(&assign.left, &mut names);

    // Get init expression boundaries from RHS span
    let init_start = assign.right.span().start as usize;
    let init_text = decl_text[init_start..].trim();
    let init_offset = tag.declaration_span.start
        + (decl_text.len() - decl_text[init_start..].trim_start().len()) as u32;

    // Parse init into main allocator (existing pipeline)
    parse_expr(alloc, init_text, init_offset, tag.id, data, parsed, diags);

    data.const_tags.names.insert(tag.id, names);
    if is_destructured {
        let pattern_end = assign.left.span().end as usize;
        let pattern_text = decl_text[..pattern_end].trim().to_string();
        // temp_name is populated later in build_scoping (needs scope access)
        data.const_tags.destructured.insert(tag.id, crate::data::ConstDestructure {
            pattern_text,
            temp_name: String::new(),
        });
    }
}

/// Extract identifier names from an assignment target.
/// Returns true if the target is a destructuring pattern.
fn extract_target_names(target: &AssignmentTarget, names: &mut Vec<String>) -> bool {
    match target {
        AssignmentTarget::AssignmentTargetIdentifier(id) => {
            names.push(id.name.as_str().to_string());
            false
        }
        AssignmentTarget::ObjectAssignmentTarget(obj) => {
            for prop in &obj.properties {
                extract_assignment_property_names(prop, names);
            }
            if let Some(rest) = &obj.rest {
                extract_target_name(&rest.target, names);
            }
            true
        }
        AssignmentTarget::ArrayAssignmentTarget(arr) => {
            for elem in arr.elements.iter().flatten() {
                extract_maybe_default_name(elem, names);
            }
            if let Some(rest) = &arr.rest {
                extract_target_name(&rest.target, names);
            }
            true
        }
        _ => false,
    }
}

fn extract_assignment_property_names(
    prop: &oxc_ast::ast::AssignmentTargetProperty,
    names: &mut Vec<String>,
) {
    match prop {
        oxc_ast::ast::AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(id) => {
            names.push(id.binding.name.as_str().to_string());
        }
        oxc_ast::ast::AssignmentTargetProperty::AssignmentTargetPropertyProperty(p) => {
            extract_maybe_default_name(&p.binding, names);
        }
    }
}

fn extract_maybe_default_name(target: &AssignmentTargetMaybeDefault, names: &mut Vec<String>) {
    match target {
        AssignmentTargetMaybeDefault::AssignmentTargetWithDefault(d) => {
            extract_target_name(&d.binding, names);
        }
        _ => {
            // AssignmentTargetMaybeDefault also implements Into<AssignmentTarget>
            // but for simple targets, try direct match
            if let AssignmentTargetMaybeDefault::AssignmentTargetIdentifier(id) = target {
                names.push(id.name.as_str().to_string());
            }
        }
    }
}

fn extract_target_name(target: &AssignmentTarget, names: &mut Vec<String>) {
    match target {
        AssignmentTarget::AssignmentTargetIdentifier(id) => {
            names.push(id.name.as_str().to_string());
        }
        AssignmentTarget::ObjectAssignmentTarget(obj) => {
            for prop in &obj.properties {
                extract_assignment_property_names(prop, names);
            }
            if let Some(rest) = &obj.rest {
                extract_target_name(&rest.target, names);
            }
        }
        AssignmentTarget::ArrayAssignmentTarget(arr) => {
            for elem in arr.elements.iter().flatten() {
                extract_maybe_default_name(elem, names);
            }
            if let Some(rest) = &arr.rest {
                extract_target_name(&rest.target, names);
            }
        }
        _ => {}
    }
}
