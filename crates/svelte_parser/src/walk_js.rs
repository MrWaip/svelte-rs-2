//! AST walk — fills `JsParseResult` by walking the Component tree.
//!
//! The top-level `parse_js` function walks the component AST, calls OXC parsing
//! utilities, and populates `JsParseResult` with parsed expressions and metadata.

use oxc_allocator::Allocator;

use svelte_ast::{Attribute, Component, ConcatPart, Fragment, Node, ScriptLanguage};
use svelte_diagnostics::Diagnostic;

use crate::parse_js::{
    parse_ce_config, parse_const_declaration_with_alloc, parse_expression_with_alloc,
    parse_script_with_alloc,
};
use crate::types::JsParseResult;

pub(crate) fn parse_js<'a>(
    alloc: &'a Allocator,
    component: &Component,
    result: &mut JsParseResult<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    let typescript = component.script.as_ref()
        .is_some_and(|s| matches!(s.language, ScriptLanguage::TypeScript));

    if let Some(script) = &component.script {
        let source = component.source_text(script.content_span);
        let arena_source: &'a str = alloc.alloc_str(source);
        match parse_script_with_alloc(alloc, arena_source, script.content_span.start, typescript) {
            Ok(program) => {
                result.parsed.program = Some(program);
                result.script_content_span = Some(script.content_span);
            }
            Err(errs) => diags.extend(errs),
        }
        result.typescript = typescript;
    }

    walk_fragment(alloc, &component.fragment, component, typescript, result, diags);

    // Parse custom element config expression and its extend sub-expression (if present)
    if let Some(svelte_ast::CustomElementConfig::Expression(span)) =
        component.options.as_ref().and_then(|o| o.custom_element.as_ref())
    {
        parse_span(alloc, component, *span, typescript, result, diags);

        // Discover extend_span via temporary parse so we can parse that expression separately
        let ce_source = component.source_text(*span);
        let config = parse_ce_config(ce_source, span.start);
        if let Some(ext_span) = config.extend_span {
            parse_span(alloc, component, ext_span, typescript, result, diags);
        }
    }
}

/// Parse an expression and store it by source offset.
fn parse_span<'a>(
    alloc: &'a Allocator,
    component: &Component,
    span: svelte_span::Span,
    typescript: bool,
    result: &mut JsParseResult<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    let source = component.source_text(span);
    let arena_source: &'a str = alloc.alloc_str(source);
    match parse_expression_with_alloc(alloc, arena_source, span.start, typescript) {
        Ok(expr) => { result.parsed.exprs.insert(span.start, expr); }
        Err(diag) => diags.push(diag),
    }
}

fn walk_fragment<'a>(
    alloc: &'a Allocator,
    fragment: &Fragment,
    component: &Component,
    typescript: bool,
    result: &mut JsParseResult<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    for node in &fragment.nodes {
        walk_node(alloc, node, component, typescript, result, diags);
    }
}

fn walk_node<'a>(
    alloc: &'a Allocator,
    node: &Node,
    component: &Component,
    typescript: bool,
    result: &mut JsParseResult<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    match node {
        Node::ExpressionTag(tag) => {
            parse_span(alloc, component, tag.expression_span, typescript, result, diags);
        }
        Node::Element(el) => {
            walk_attrs(alloc, &el.attributes, component, typescript, result, diags);
            walk_fragment(alloc, &el.fragment, component, typescript, result, diags);
        }
        Node::ComponentNode(cn) => {
            walk_attrs(alloc, &cn.attributes, component, typescript, result, diags);
            walk_fragment(alloc, &cn.fragment, component, typescript, result, diags);
        }
        Node::IfBlock(block) => {
            parse_span(alloc, component, block.test_span, typescript, result, diags);
            walk_fragment(alloc, &block.consequent, component, typescript, result, diags);
            if let Some(alt) = &block.alternate {
                walk_fragment(alloc, alt, component, typescript, result, diags);
            }
        }
        Node::EachBlock(block) => {
            parse_span(alloc, component, block.expression_span, typescript, result, diags);
            if let Some(key_span) = block.key_span {
                parse_span(alloc, component, key_span, typescript, result, diags);
            }

            walk_fragment(alloc, &block.body, component, typescript, result, diags);

            if let Some(fb) = &block.fallback {
                walk_fragment(alloc, fb, component, typescript, result, diags);
            }
        }
        Node::SnippetBlock(block) => {
            walk_fragment(alloc, &block.body, component, typescript, result, diags);
        }
        Node::RenderTag(tag) => {
            parse_span(alloc, component, tag.expression_span, typescript, result, diags);
        }
        Node::HtmlTag(tag) => {
            parse_span(alloc, component, tag.expression_span, typescript, result, diags);
        }
        Node::KeyBlock(block) => {
            parse_span(alloc, component, block.expression_span, typescript, result, diags);
            walk_fragment(alloc, &block.fragment, component, typescript, result, diags);
        }
        Node::AwaitBlock(block) => {
            parse_span(alloc, component, block.expression_span, typescript, result, diags);

            if let Some(ref p) = block.pending {
                walk_fragment(alloc, p, component, typescript, result, diags);
            }
            if let Some(ref t) = block.then {
                walk_fragment(alloc, t, component, typescript, result, diags);
            }
            if let Some(ref c) = block.catch {
                walk_fragment(alloc, c, component, typescript, result, diags);
            }
        }
        Node::ConstTag(tag) => {
            let decl_text = component.source_text(tag.declaration_span);
            let arena_source: &'a str = alloc.alloc_str(decl_text);
            match parse_const_declaration_with_alloc(alloc, arena_source, tag.declaration_span.start, typescript) {
                Ok((_names, init_expr)) => {
                    // Adjusted offset for the "const " prefix that was wrapped around the source
                    let ref_offset = tag.declaration_span.start.wrapping_sub(6);
                    result.parsed.exprs.insert(ref_offset, init_expr);
                }
                Err(diag) => diags.push(diag),
            }
        }
        Node::SvelteHead(head) => {
            walk_fragment(alloc, &head.fragment, component, typescript, result, diags);
        }
        Node::SvelteElement(el) => {
            if !el.static_tag {
                parse_span(alloc, component, el.tag_span, typescript, result, diags);
            }
            walk_attrs(alloc, &el.attributes, component, typescript, result, diags);
            walk_fragment(alloc, &el.fragment, component, typescript, result, diags);
        }
        Node::SvelteWindow(w) => {
            walk_attrs(alloc, &w.attributes, component, typescript, result, diags);
        }
        Node::SvelteDocument(d) => {
            walk_attrs(alloc, &d.attributes, component, typescript, result, diags);
        }
        Node::SvelteBody(b) => {
            walk_attrs(alloc, &b.attributes, component, typescript, result, diags);
        }
        Node::SvelteBoundary(b) => {
            walk_attrs(alloc, &b.attributes, component, typescript, result, diags);
            walk_fragment(alloc, &b.fragment, component, typescript, result, diags);
        }
        Node::DebugTag(tag) => {
            for span in &tag.identifiers {
                parse_span(alloc, component, *span, typescript, result, diags);
            }
        }
        Node::Text(_) | Node::Comment(_) | Node::Error(_) => {}
    }
}

/// Parse and store attribute expressions by source offset.
fn walk_attrs<'a>(
    alloc: &'a Allocator,
    attrs: &[Attribute],
    component: &Component,
    typescript: bool,
    result: &mut JsParseResult<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    for attr in attrs {
        match attr {
            Attribute::ExpressionAttribute(a) => {
                parse_span(alloc, component, a.expression_span, typescript, result, diags);
            }
            Attribute::ConcatenationAttribute(a) => {
                for part in &a.parts {
                    if let ConcatPart::Dynamic(span) = part {
                        parse_span(alloc, component, *span, typescript, result, diags);
                    }
                }
            }
            Attribute::ClassDirective(a) => {
                if let Some(span) = a.expression_span {
                    parse_span(alloc, component, span, typescript, result, diags);
                }
            }
            Attribute::StyleDirective(a) => {
                use svelte_ast::StyleDirectiveValue;
                match &a.value {
                    StyleDirectiveValue::Expression(span) => {
                        parse_span(alloc, component, *span, typescript, result, diags);
                    }
                    StyleDirectiveValue::Concatenation(parts) => {
                        for part in parts {
                            if let ConcatPart::Dynamic(span) = part {
                                parse_span(alloc, component, *span, typescript, result, diags);
                            }
                        }
                    }
                    StyleDirectiveValue::Shorthand | StyleDirectiveValue::String(_) => {}
                }
            }
            Attribute::BindDirective(a) => {
                if let Some(span) = a.expression_span {
                    parse_span(alloc, component, span, typescript, result, diags);
                }
            }
            Attribute::SpreadAttribute(a) => {
                debug_assert!(
                    a.expression_span.end >= a.expression_span.start + 3,
                    "spread expression span too short to contain '...'"
                );
                let span = svelte_span::Span::new(a.expression_span.start + 3, a.expression_span.end);
                parse_span(alloc, component, span, typescript, result, diags);
            }
            Attribute::Shorthand(a) => {
                parse_span(alloc, component, a.expression_span, typescript, result, diags);
            }
            Attribute::UseDirective(a) => {
                if let Some(span) = a.expression_span {
                    parse_span(alloc, component, span, typescript, result, diags);
                }
                parse_span(alloc, component, a.name, typescript, result, diags);
            }
            Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_) => {}
            // LEGACY(svelte4): on:directive — parse expression if present
            Attribute::OnDirectiveLegacy(a) => {
                if let Some(span) = a.expression_span {
                    parse_span(alloc, component, span, typescript, result, diags);
                }
            }
            Attribute::TransitionDirective(a) => {
                if let Some(span) = a.expression_span {
                    parse_span(alloc, component, span, typescript, result, diags);
                }
                parse_span(alloc, component, a.name, typescript, result, diags);
            }
            Attribute::AnimateDirective(a) => {
                if let Some(span) = a.expression_span {
                    parse_span(alloc, component, span, typescript, result, diags);
                }
                parse_span(alloc, component, a.name, typescript, result, diags);
            }
            Attribute::AttachTag(a) => {
                parse_span(alloc, component, a.expression_span, typescript, result, diags);
            }
        }
    }
}
