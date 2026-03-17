use oxc_allocator::Allocator;
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
    attr_id: NodeId,
    data: &mut AnalysisData,
    parsed: &mut ParsedExprs<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    let arena_source: &'a str = alloc.alloc_str(source);
    match svelte_js::analyze_expression_with_alloc(alloc, arena_source, offset) {
        Ok((info, expr)) => {
            data.attr_expressions.insert(attr_id, info);
            parsed.attr_exprs.insert(attr_id, expr);
        }
        Err(diag) => diags.push(diag),
    }
}

/// Parse concatenation parts (shared by ConcatenationAttribute and StyleDirective::Concatenation).
fn parse_concat_parts<'a>(
    alloc: &'a Allocator,
    parts: &[ConcatPart],
    attr_id: NodeId,
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
                    parsed.concat_part_exprs.insert((attr_id, dyn_idx), expr);
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
    data.attr_expressions.insert(attr_id, merged);
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
            walk_attrs(alloc, &el.attributes, component, data, parsed, diags);
            walk_fragment(alloc, &el.fragment, component, data, parsed, diags);
        }
        Node::ComponentNode(cn) => {
            walk_attrs(alloc, &cn.attributes, component, data, parsed, diags);
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
            if let Some(key_span) = block.key_span {
                let key_source = component.source_text(key_span);
                let arena_source: &'a str = alloc.alloc_str(key_source);
                match svelte_js::analyze_expression_with_alloc(alloc, arena_source, key_span.start) {
                    Ok((_info, expr)) => {
                        parsed.key_exprs.insert(block.id, expr);
                    }
                    Err(diag) => diags.push(diag),
                }
            }
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
            let decl_text = component.source_text(tag.declaration_span);
            let arena_source: &'a str = alloc.alloc_str(decl_text);
            match svelte_js::parse_const_declaration_with_alloc(alloc, arena_source, tag.declaration_span.start) {
                Ok((names, references, init_expr)) => {
                    data.expressions.insert(tag.id, ExpressionInfo {
                        kind: ExpressionKind::Other,
                        references,
                        has_side_effects: false,
                    });
                    parsed.exprs.insert(tag.id, init_expr);
                    data.const_tags.names.insert(tag.id, names.iter().map(|n| n.to_string()).collect());
                }
                Err(diag) => diags.push(diag),
            }
        }
        Node::SvelteHead(head) => {
            walk_fragment(alloc, &head.fragment, component, data, parsed, diags);
        }
        Node::SvelteElement(el) => {
            // Parse the tag expression (skip for static string tags like this="div")
            if !el.static_tag {
                let tag_source = component.source_text(el.tag_span);
                parse_expr(alloc, tag_source, el.tag_span.start, el.id, data, parsed, diags);
            }
            walk_attrs(alloc, &el.attributes, component, data, parsed, diags);
            walk_fragment(alloc, &el.fragment, component, data, parsed, diags);
        }
        Node::Text(_) | Node::Comment(_) | Node::Error(_) => {}
    }
}

/// Parse and store attribute expressions, keyed by attribute NodeId.
fn walk_attrs<'a>(
    alloc: &'a Allocator,
    attrs: &[Attribute],
    component: &Component,
    data: &mut AnalysisData,
    parsed: &mut ParsedExprs<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    for attr in attrs {
        let attr_id = attr.id();
        match attr {
            Attribute::ExpressionAttribute(a) => {
                let source = component.source_text(a.expression_span);
                parse_attr_expr(alloc, source, a.expression_span.start, attr_id, data, parsed, diags);
                // class={[...]} or class={{...}} or class={x} need clsx to resolve
                if a.name == "class" {
                    if let Some(expr) = parsed.attr_exprs.get(&attr_id) {
                        let needs = !matches!(
                            expr,
                            oxc_ast::ast::Expression::StringLiteral(_)
                                | oxc_ast::ast::Expression::TemplateLiteral(_)
                                | oxc_ast::ast::Expression::BinaryExpression(_)
                        );
                        if needs {
                            data.element_flags.needs_clsx.insert(attr_id);
                        }
                    }
                }
            }
            Attribute::ConcatenationAttribute(a) => {
                parse_concat_parts(alloc, &a.parts, attr_id, component, data, parsed, diags);
            }
            Attribute::ClassDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, attr_id, data, parsed, diags);
                }
            }
            Attribute::StyleDirective(a) => {
                use svelte_ast::StyleDirectiveValue;
                match &a.value {
                    StyleDirectiveValue::Expression(span) => {
                        let source = component.source_text(*span);
                        parse_attr_expr(alloc, source, span.start, attr_id, data, parsed, diags);
                    }
                    StyleDirectiveValue::Concatenation(parts) => {
                        parse_concat_parts(alloc, parts, attr_id, component, data, parsed, diags);
                    }
                    StyleDirectiveValue::Shorthand | StyleDirectiveValue::String(_) => {}
                }
            }
            Attribute::BindDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, attr_id, data, parsed, diags);
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
                parse_attr_expr(alloc, source, span.start, attr_id, data, parsed, diags);
            }
            Attribute::UseDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, attr_id, data, parsed, diags);
                }
            }
            Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_) => {}
            // LEGACY(svelte4): on:directive — parse expression if present
            Attribute::OnDirectiveLegacy(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, attr_id, data, parsed, diags);
                }
            }
            Attribute::TransitionDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, attr_id, data, parsed, diags);
                }
            }
            Attribute::AnimateDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, attr_id, data, parsed, diags);
                }
            }
            Attribute::AttachTag(a) => {
                let span = a.expression_span;
                let source = component.source_text(span);
                parse_attr_expr(alloc, source, span.start, attr_id, data, parsed, diags);
            }
        }
    }
}
