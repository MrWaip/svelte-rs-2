//! AST walk — fills `ParserResult` by walking the Component tree.
//!
//! The top-level `parse_js` function walks the component AST, calls OXC parsing
//! utilities, and populates `ParserResult` with parsed expressions and statements.

use oxc_allocator::Allocator;
use svelte_ast::{AstStore, Attribute, Component, ConcatPart, Fragment, Node, ScriptLanguage};
use svelte_diagnostics::Diagnostic;

use crate::parse_js::{
    parse_const_declaration_with_alloc, parse_each_context_with_alloc, parse_each_index_with_alloc,
    parse_expression_with_alloc, parse_script_with_alloc, parse_slot_let_decl_with_alloc,
    parse_snippet_decl_with_alloc,
};
use crate::types::JsAst;

pub(crate) fn parse_js<'a>(
    alloc: &'a Allocator,
    component: &Component,
    result: &mut JsAst<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    // Template expressions use TS parsing if either script is TypeScript.
    let template_typescript = component
        .instance_script
        .as_ref()
        .or(component.module_script.as_ref())
        .is_some_and(|s| matches!(s.language, ScriptLanguage::TypeScript));

    if let Some(script) = &component.instance_script {
        let typescript = matches!(script.language, ScriptLanguage::TypeScript);
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

    if let Some(script) = &component.module_script {
        let typescript = matches!(script.language, ScriptLanguage::TypeScript);
        let source = component.source_text(script.content_span);
        let arena_source: &'a str = alloc.alloc_str(source);
        match parse_script_with_alloc(alloc, arena_source, script.content_span.start, typescript) {
            Ok(program) => {
                result.module_program = Some(program);
                result.module_script_content_span = Some(script.content_span);
            }
            Err(errs) => diags.extend(errs),
        }
    }

    walk_fragment(
        alloc,
        &component.fragment,
        &component.store,
        component,
        template_typescript,
        result,
        diags,
    );

    // Parse the custom element config expression itself. Property classification
    // belongs in analyze, where CE object semantics are already interpreted.
    if let Some(svelte_ast::CustomElementConfig::Expression(span)) = component
        .options
        .as_ref()
        .and_then(|o| o.custom_element.as_ref())
    {
        parse_span(alloc, component, *span, template_typescript, result, diags);
    }
}

/// Parse a `use:` directive name like `"actions.tooltip-extra"` into a valid JS expression.
///
/// Segments that are not valid JS identifiers (e.g. contain `-`) are emitted as computed
/// bracket access: `actions["tooltip-extra"]`. This matches the Svelte reference compiler's
/// `parse_directive_name` utility. Stores the result at `name_span.start`.
fn parse_directive_name_span<'a>(
    alloc: &'a Allocator,
    component: &Component,
    name_span: svelte_span::Span,
    typescript: bool,
    result: &mut JsAst<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    let name = component.source_text(name_span);
    let js = directive_name_to_js(name);
    let arena_js: &'a str = alloc.alloc_str(&js);
    match parse_expression_with_alloc(alloc, arena_js, name_span.start, typescript) {
        Ok(expr) => {
            result.alloc_expr(name_span.start, expr);
        }
        Err(diag) => diags.push(diag),
    }
}

/// Convert a Svelte directive name (e.g. `"actions.tooltip-extra"`) to a valid JS expression
/// string (e.g. `actions["tooltip-extra"]`), applying bracket notation for segments that are
/// not valid JS identifiers.
fn directive_name_to_js(name: &str) -> String {
    let mut parts = name.split('.');
    let mut result = parts.next().unwrap_or("").to_string();
    for part in parts {
        if is_valid_js_identifier(part) {
            result.push('.');
            result.push_str(part);
        } else {
            result.push('[');
            result.push('"');
            result.push_str(part);
            result.push('"');
            result.push(']');
        }
    }
    result
}

fn is_valid_js_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        None => false,
        Some(first) => {
            (first.is_alphabetic() || first == '_' || first == '$')
                && chars.all(|c| c.is_alphanumeric() || c == '_' || c == '$')
        }
    }
}

/// Parse an expression and store it by source offset.
fn parse_span<'a>(
    alloc: &'a Allocator,
    component: &Component,
    span: svelte_span::Span,
    typescript: bool,
    result: &mut JsAst<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    let source = component.source_text(span);
    let arena_source: &'a str = alloc.alloc_str(source);
    match parse_expression_with_alloc(alloc, arena_source, span.start, typescript) {
        Ok(expr) => {
            result.alloc_expr(span.start, expr);
        }
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
    result: &mut JsAst<'a>,
    _diags: &mut Vec<Diagnostic>,
) {
    let source = component.source_text(span);
    let arena_source: &'a str = alloc.alloc_str(source);
    if let Some(stmt) = parse_each_context_with_alloc(alloc, arena_source, typescript) {
        result.alloc_stmt(span.start, stmt);
    }
}

fn walk_fragment<'a>(
    alloc: &'a Allocator,
    fragment: &Fragment,
    store: &AstStore,
    component: &Component,
    typescript: bool,
    result: &mut JsAst<'a>,
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
    result: &mut JsAst<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    match node {
        Node::ExpressionTag(tag) => {
            parse_span(
                alloc,
                component,
                tag.expression.span,
                typescript,
                result,
                diags,
            );
        }
        Node::Element(el) => {
            walk_attrs(alloc, &el.attributes, component, typescript, result, diags);
            walk_fragment(
                alloc,
                &el.fragment,
                store,
                component,
                typescript,
                result,
                diags,
            );
        }
        Node::SlotElementLegacy(el) => {
            walk_attrs(alloc, &el.attributes, component, typescript, result, diags);
            walk_fragment(
                alloc,
                &el.fragment,
                store,
                component,
                typescript,
                result,
                diags,
            );
        }
        Node::ComponentNode(cn) => {
            walk_attrs(alloc, &cn.attributes, component, typescript, result, diags);
            walk_fragment(
                alloc,
                &cn.fragment,
                store,
                component,
                typescript,
                result,
                diags,
            );
            for slot in &cn.legacy_slots {
                walk_fragment(
                    alloc,
                    &slot.fragment,
                    store,
                    component,
                    typescript,
                    result,
                    diags,
                );
            }
        }
        Node::IfBlock(block) => {
            parse_span(alloc, component, block.test.span, typescript, result, diags);
            walk_fragment(
                alloc,
                &block.consequent,
                store,
                component,
                typescript,
                result,
                diags,
            );
            if let Some(alt) = &block.alternate {
                walk_fragment(alloc, alt, store, component, typescript, result, diags);
            }
        }
        Node::EachBlock(block) => {
            parse_span(
                alloc,
                component,
                block.expression.span,
                typescript,
                result,
                diags,
            );
            if let Some(r) = block.key.as_ref() {
                parse_span(alloc, component, r.span, typescript, result, diags);
            }

            if let Some(r) = block.context.as_ref() {
                let ctx_span = r.span;
                let ctx_text = component.source_text(ctx_span);
                let arena_ctx: &'a str = alloc.alloc_str(ctx_text);
                if let Some(stmt) = parse_each_context_with_alloc(alloc, arena_ctx, typescript) {
                    result.alloc_stmt(ctx_span.start, stmt);
                }
            }

            if let Some(r) = block.index.as_ref() {
                let idx_span = r.span;
                let idx_text = component.source_text(idx_span);
                let arena_idx: &'a str = alloc.alloc_str(idx_text);
                if let Some(stmt) = parse_each_index_with_alloc(alloc, arena_idx) {
                    result.alloc_stmt(idx_span.start, stmt);
                }
            }

            walk_fragment(
                alloc,
                &block.body,
                store,
                component,
                typescript,
                result,
                diags,
            );

            if let Some(fb) = &block.fallback {
                walk_fragment(alloc, fb, store, component, typescript, result, diags);
            }
        }
        Node::SnippetBlock(block) => {
            let expr_text = component.source_text(block.decl.span);
            let arena_text: &'a str = alloc.alloc_str(expr_text);
            if let Some(stmt) = parse_snippet_decl_with_alloc(alloc, arena_text, typescript) {
                result.alloc_stmt(block.decl.span.start, stmt);
            }
            walk_fragment(
                alloc,
                &block.body,
                store,
                component,
                typescript,
                result,
                diags,
            );
        }
        Node::RenderTag(tag) => {
            parse_span(
                alloc,
                component,
                tag.expression.span,
                typescript,
                result,
                diags,
            );
        }
        Node::HtmlTag(tag) => {
            parse_span(
                alloc,
                component,
                tag.expression.span,
                typescript,
                result,
                diags,
            );
        }
        Node::KeyBlock(block) => {
            parse_span(
                alloc,
                component,
                block.expression.span,
                typescript,
                result,
                diags,
            );
            walk_fragment(
                alloc,
                &block.fragment,
                store,
                component,
                typescript,
                result,
                diags,
            );
        }
        Node::AwaitBlock(block) => {
            parse_span(
                alloc,
                component,
                block.expression.span,
                typescript,
                result,
                diags,
            );

            for r in [block.value.as_ref(), block.error.as_ref()]
                .into_iter()
                .flatten()
            {
                parse_binding_pattern(alloc, component, r.span, typescript, result, diags);
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
            let source = component.source_text(tag.decl.span);
            let arena_source: &'a str = alloc.alloc_str(source);
            match parse_const_declaration_with_alloc(
                alloc,
                arena_source,
                tag.decl.span.start,
                typescript,
            ) {
                Ok(stmt) => {
                    result.alloc_stmt(tag.decl.span.start, stmt);
                }
                Err(diag) => diags.push(diag),
            }
        }
        Node::SvelteHead(head) => {
            walk_fragment(
                alloc,
                &head.fragment,
                store,
                component,
                typescript,
                result,
                diags,
            );
        }
        Node::SvelteFragmentLegacy(el) => {
            walk_attrs(alloc, &el.attributes, component, typescript, result, diags);
            walk_fragment(
                alloc,
                &el.fragment,
                store,
                component,
                typescript,
                result,
                diags,
            );
        }
        Node::SvelteElement(el) => {
            if !el.static_tag {
                parse_span(alloc, component, el.tag_span, typescript, result, diags);
            }
            walk_attrs(alloc, &el.attributes, component, typescript, result, diags);
            walk_fragment(
                alloc,
                &el.fragment,
                store,
                component,
                typescript,
                result,
                diags,
            );
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
            walk_fragment(
                alloc,
                &b.fragment,
                store,
                component,
                typescript,
                result,
                diags,
            );
        }
        Node::DebugTag(tag) => {
            for r in &tag.identifier_refs {
                parse_span(alloc, component, r.span, typescript, result, diags);
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
    result: &mut JsAst<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    for attr in attrs {
        match attr {
            Attribute::ExpressionAttribute(a) => {
                parse_span(
                    alloc,
                    component,
                    a.expression.span,
                    typescript,
                    result,
                    diags,
                );
            }
            Attribute::ConcatenationAttribute(a) => {
                for part in &a.parts {
                    if let ConcatPart::Dynamic { expr, .. } = part {
                        parse_span(alloc, component, expr.span, typescript, result, diags);
                    }
                }
            }
            Attribute::ClassDirective(a) => {
                parse_span(
                    alloc,
                    component,
                    a.expression.span,
                    typescript,
                    result,
                    diags,
                );
            }
            Attribute::StyleDirective(a) => {
                use svelte_ast::StyleDirectiveValue;
                match &a.value {
                    StyleDirectiveValue::Expression => {
                        parse_span(
                            alloc,
                            component,
                            a.expression.span,
                            typescript,
                            result,
                            diags,
                        );
                    }
                    StyleDirectiveValue::Concatenation(parts) => {
                        for part in parts {
                            if let ConcatPart::Dynamic { expr, .. } = part {
                                parse_span(alloc, component, expr.span, typescript, result, diags);
                            }
                        }
                    }
                    StyleDirectiveValue::String(_) => {}
                }
            }
            Attribute::BindDirective(a) => {
                parse_span(
                    alloc,
                    component,
                    a.expression.span,
                    typescript,
                    result,
                    diags,
                );
            }
            Attribute::LetDirectiveLegacy(a) => {
                let pattern_span = a.binding.as_ref().map(|r| r.span).unwrap_or(a.name_span);
                let pattern_source = component.source_text(pattern_span);
                match parse_slot_let_decl_with_alloc(
                    alloc,
                    alloc.alloc_str(pattern_source),
                    a.name.as_str(),
                    a.name_span.start,
                    typescript,
                ) {
                    Ok(stmt) => {
                        result.alloc_stmt(a.name_span.start, stmt);
                    }
                    Err(diag) => diags.push(diag),
                }
            }
            Attribute::SpreadAttribute(a) => {
                parse_span(
                    alloc,
                    component,
                    a.expression.span,
                    typescript,
                    result,
                    diags,
                );
            }
            Attribute::UseDirective(a) => {
                if let Some(r) = a.expression.as_ref() {
                    parse_span(alloc, component, r.span, typescript, result, diags);
                }
                parse_directive_name_span(
                    alloc,
                    component,
                    a.name_ref.span,
                    typescript,
                    result,
                    diags,
                );
            }
            Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_) => {}
            // LEGACY(svelte4): on:directive — parse expression if present
            Attribute::OnDirectiveLegacy(a) => {
                if let Some(r) = a.expression.as_ref() {
                    parse_span(alloc, component, r.span, typescript, result, diags);
                }
            }
            Attribute::TransitionDirective(a) => {
                if let Some(r) = a.expression.as_ref() {
                    parse_span(alloc, component, r.span, typescript, result, diags);
                }
                parse_span(alloc, component, a.name_ref.span, typescript, result, diags);
            }
            Attribute::AnimateDirective(a) => {
                if let Some(r) = a.expression.as_ref() {
                    parse_span(alloc, component, r.span, typescript, result, diags);
                }
                parse_span(alloc, component, a.name_ref.span, typescript, result, diags);
            }
            Attribute::AttachTag(a) => {
                parse_span(
                    alloc,
                    component,
                    a.expression.span,
                    typescript,
                    result,
                    diags,
                );
            }
        }
    }
}
