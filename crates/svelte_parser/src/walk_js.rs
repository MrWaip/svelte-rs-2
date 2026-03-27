//! AST walk — fills `ParserResult` by walking the Component tree.
//!
//! The top-level `parse_js` function walks the component AST, calls OXC parsing
//! utilities, and populates `ParserResult` with parsed expressions and statements.

use oxc_allocator::Allocator;
use oxc_ast::ast::Expression;

use svelte_ast::{AstStore, Attribute, Component, ConcatPart, Fragment, Node, ScriptLanguage};
use svelte_diagnostics::Diagnostic;

use crate::parse_js::{
    parse_const_declaration_with_alloc, parse_each_context_with_alloc,
    parse_each_index_with_alloc, parse_expression_with_alloc, parse_script_with_alloc,
    parse_snippet_decl_with_alloc,
};
use crate::types::ParserResult;

pub(crate) fn parse_js<'a>(
    alloc: &'a Allocator,
    component: &Component,
    result: &mut ParserResult<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    let typescript = component.script.as_ref()
        .is_some_and(|s| matches!(s.language, ScriptLanguage::TypeScript));

    if let Some(script) = &component.script {
        let source = component.source_text(script.content_span);
        let arena_source: &'a str = alloc.alloc_str(source);
        match parse_script_with_alloc(alloc, arena_source, script.content_span.start, typescript) {
            Ok(program) => {
                result.program = Some(program);
                result.script_content_span = Some(script.content_span);
            }
            Err(errs) => diags.extend(errs),
        }
        result.typescript = typescript;
    }

    walk_fragment(alloc, &component.fragment, &component.store, component, typescript, result, diags);

    // Parse custom element config expression and its extend sub-expression (if present)
    if let Some(svelte_ast::CustomElementConfig::Expression(span)) =
        component.options.as_ref().and_then(|o| o.custom_element.as_ref())
    {
        parse_span(alloc, component, *span, typescript, result, diags);

        // Navigate the already-parsed ObjectExpression to find `extend` property span
        if let Some(ext_span) = find_ce_extend_span(result, span.start) {
            parse_span(alloc, component, ext_span, typescript, result, diags);
        }
    }
}

/// Find the `extend` property span inside an already-parsed CE config ObjectExpression.
/// OXC spans are relative to the expression start, so we adjust by `offset`.
fn find_ce_extend_span(result: &ParserResult<'_>, offset: u32) -> Option<svelte_span::Span> {
    use oxc_ast::ast::{ObjectPropertyKind, PropertyKey};
    use oxc_span::GetSpan as _;

    let expr = result.exprs.get(&offset)?;
    let Expression::ObjectExpression(obj) = expr else { return None };
    for prop_kind in &obj.properties {
        let ObjectPropertyKind::ObjectProperty(prop) = prop_kind else { continue };
        if let PropertyKey::StaticIdentifier(id) = &prop.key {
            if id.name.as_str() == "extend" {
                let s = prop.value.span();
                return Some(svelte_span::Span::new(s.start + offset, s.end + offset));
            }
        }
    }
    None
}

/// Parse an expression and store it by source offset.
fn parse_span<'a>(
    alloc: &'a Allocator,
    component: &Component,
    span: svelte_span::Span,
    typescript: bool,
    result: &mut ParserResult<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    let source = component.source_text(span);
    let arena_source: &'a str = alloc.alloc_str(source);
    match parse_expression_with_alloc(alloc, arena_source, span.start, typescript) {
        Ok(expr) => { result.exprs.insert(span.start, expr); }
        Err(diag) => diags.push(diag),
    }
}

/// Parse a binding pattern (e.g. `value`, `{name, age}`, `[a, b]`) as `let PATTERN = x;`.
///
/// Stored in `stmts` so dispatch_opt_stmt fires SemanticCollector, creating proper bindings.
fn parse_binding_pattern<'a>(
    alloc: &'a Allocator,
    component: &Component,
    span: svelte_span::Span,
    typescript: bool,
    result: &mut ParserResult<'a>,
    _diags: &mut Vec<Diagnostic>,
) {
    let source = component.source_text(span);
    let arena_source: &'a str = alloc.alloc_str(source);
    if let Some(stmt) = parse_each_context_with_alloc(alloc, arena_source, typescript) {
        result.stmts.insert(span.start, stmt);
    }
}

fn walk_fragment<'a>(
    alloc: &'a Allocator,
    fragment: &Fragment,
    store: &AstStore,
    component: &Component,
    typescript: bool,
    result: &mut ParserResult<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    for &id in &fragment.nodes {
        let node = store.get(id);
        walk_node(alloc, node, store, component, typescript, result, diags);
    }
}

fn walk_node<'a>(
    alloc: &'a Allocator,
    node: &Node,
    store: &AstStore,
    component: &Component,
    typescript: bool,
    result: &mut ParserResult<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    match node {
        Node::ExpressionTag(tag) => {
            parse_span(alloc, component, tag.expression_span, typescript, result, diags);
        }
        Node::Element(el) => {
            walk_attrs(alloc, &el.attributes, component, typescript, result, diags);
            walk_fragment(alloc, &el.fragment, store, component, typescript, result, diags);
        }
        Node::ComponentNode(cn) => {
            walk_attrs(alloc, &cn.attributes, component, typescript, result, diags);
            walk_fragment(alloc, &cn.fragment, store, component, typescript, result, diags);
        }
        Node::IfBlock(block) => {
            parse_span(alloc, component, block.test_span, typescript, result, diags);
            walk_fragment(alloc, &block.consequent, store, component, typescript, result, diags);
            if let Some(alt) = &block.alternate {
                walk_fragment(alloc, alt, store, component, typescript, result, diags);
            }
        }
        Node::EachBlock(block) => {
            parse_span(alloc, component, block.expression_span, typescript, result, diags);
            if let Some(key_span) = block.key_span {
                parse_span(alloc, component, key_span, typescript, result, diags);
            }

            // Parse context pattern (simple identifier or destructured)
            let ctx_text = component.source_text(block.context_span);
            let arena_ctx: &'a str = alloc.alloc_str(ctx_text);
            if let Some(stmt) = parse_each_context_with_alloc(alloc, arena_ctx, typescript) {
                result.stmts.insert(block.context_span.start, stmt);
            }

            // Parse index variable
            if let Some(idx_span) = block.index_span {
                let idx_text = component.source_text(idx_span);
                let arena_idx: &'a str = alloc.alloc_str(idx_text);
                if let Some(stmt) = parse_each_index_with_alloc(alloc, arena_idx) {
                    result.stmts.insert(idx_span.start, stmt);
                }
            }

            walk_fragment(alloc, &block.body, store, component, typescript, result, diags);

            if let Some(fb) = &block.fallback {
                walk_fragment(alloc, fb, store, component, typescript, result, diags);
            }
        }
        Node::SnippetBlock(block) => {
            let expr_text = component.source_text(block.expression_span);
            let arena_text: &'a str = alloc.alloc_str(expr_text);
            if let Some(stmt) = parse_snippet_decl_with_alloc(alloc, arena_text, typescript) {
                result.stmts.insert(block.expression_span.start, stmt);
            }
            walk_fragment(alloc, &block.body, store, component, typescript, result, diags);
        }
        Node::RenderTag(tag) => {
            parse_span(alloc, component, tag.expression_span, typescript, result, diags);
        }
        Node::HtmlTag(tag) => {
            parse_span(alloc, component, tag.expression_span, typescript, result, diags);
        }
        Node::KeyBlock(block) => {
            parse_span(alloc, component, block.expression_span, typescript, result, diags);
            walk_fragment(alloc, &block.fragment, store, component, typescript, result, diags);
        }
        Node::AwaitBlock(block) => {
            parse_span(alloc, component, block.expression_span, typescript, result, diags);

            for binding_span in [block.value_span, block.error_span].into_iter().flatten() {
                parse_binding_pattern(alloc, component, binding_span, typescript, result, diags);
            }

            if let Some(ref p) = block.pending {
                walk_fragment(alloc, p, store, component, typescript, result, diags);
            }
            if let Some(ref t) = block.then {
                walk_fragment(alloc, t, store, component, typescript, result, diags);
            }
            if let Some(ref c) = block.catch {
                walk_fragment(alloc, c, store, component, typescript, result, diags);
            }
        }
        Node::ConstTag(tag) => {
            // TS type annotations (e.g. `doubled: number = expr`) require statement-level
            // parsing. Wrap as `const SOURCE;` and store the full Statement.
            // Scope building and codegen extract names / init expression from it directly.
            let source = component.source_text(tag.expression_span);
            let arena_source: &'a str = alloc.alloc_str(source);
            match parse_const_declaration_with_alloc(alloc, arena_source, tag.expression_span.start, typescript) {
                Ok(stmt) => {
                    result.stmts.insert(tag.expression_span.start, stmt);
                }
                Err(diag) => diags.push(diag),
            }
        }
        Node::SvelteHead(head) => {
            walk_fragment(alloc, &head.fragment, store, component, typescript, result, diags);
        }
        Node::SvelteElement(el) => {
            if !el.static_tag {
                parse_span(alloc, component, el.tag_span, typescript, result, diags);
            }
            walk_attrs(alloc, &el.attributes, component, typescript, result, diags);
            walk_fragment(alloc, &el.fragment, store, component, typescript, result, diags);
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
            walk_fragment(alloc, &b.fragment, store, component, typescript, result, diags);
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
    result: &mut ParserResult<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    for attr in attrs {
        match attr {
            Attribute::ExpressionAttribute(a) => {
                parse_span(alloc, component, a.expression_span, typescript, result, diags);
            }
            Attribute::ConcatenationAttribute(a) => {
                for part in &a.parts {
                    if let ConcatPart::Dynamic { span, .. } = part {
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
                            if let ConcatPart::Dynamic { span, .. } = part {
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
                parse_span(alloc, component, a.expression_span, typescript, result, diags);
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
