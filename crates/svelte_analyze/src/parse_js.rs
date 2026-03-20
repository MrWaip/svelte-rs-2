use oxc_allocator::Allocator;
use oxc_ast::ast::{ArrowFunctionExpression, BindingPattern, Expression, FormalParameters};
use oxc_ast_visit::Visit;
use svelte_ast::{Attribute, Component, ConcatPart, Fragment, Node, NodeId, ScriptLanguage};
use svelte_diagnostics::Diagnostic;
use svelte_js::{ExpressionInfo, ExpressionKind};

use crate::data::{AnalysisData, FragmentKey, ParsedExprs};
use crate::scope::{ComponentScoping, ScopeId};

pub fn parse_js<'a>(
    alloc: &'a Allocator,
    component: &Component,
    data: &mut AnalysisData,
    parsed: &mut ParsedExprs<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    let typescript = component.script.as_ref()
        .is_some_and(|s| matches!(s.language, ScriptLanguage::TypeScript));

    if let Some(script) = &component.script {
        let source = component.source_text(script.content_span);
        let arena_source: &'a str = alloc.alloc_str(source);
        match svelte_js::analyze_script_with_alloc(
            alloc,
            arena_source,
            script.content_span.start,
            typescript,
        ) {
            Ok((mut info, scoping, program)) => {
                data.exports = std::mem::take(&mut info.exports);
                data.needs_context = info.has_effects || info.has_class_state_fields;
                data.has_class_state_fields = info.has_class_state_fields;
                data.script = Some(info);
                data.scoping = ComponentScoping::from_scoping(scoping);
                parsed.script_program = Some(program);
            }
            Err(errs) => diags.extend(errs),
        }
    }

    // Pre-parse prop default expressions so codegen doesn't re-parse them
    if let Some(ref script_info) = data.script {
        if let Some(ref props_decl) = script_info.props_declaration {
            for prop in &props_decl.props {
                if let Some(span) = prop.default_span {
                    let src = component.source_text(span);
                    let arena_src: &'a str = alloc.alloc_str(src);
                    match svelte_js::analyze_expression_with_alloc(alloc, arena_src, span.start, typescript) {
                        Ok((_info, expr)) => parsed.prop_default_exprs.push(Some(expr)),
                        Err(diag) => { diags.push(diag); parsed.prop_default_exprs.push(None); }
                    }
                } else {
                    parsed.prop_default_exprs.push(None);
                }
            }
        }
    }

    walk_fragment(alloc, &component.fragment, component, typescript, data, parsed, diags);

    // Parse custom element config expression (if present)
    if let Some(svelte_ast::CustomElementConfig::Expression(span)) =
        component.options.as_ref().and_then(|o| o.custom_element.as_ref())
    {
        let ce_source = component.source_text(*span);
        let config = svelte_js::parse_ce_config(ce_source, span.start);

        if let Some(ext_span) = config.extend_span {
            let ext_src = component.source_text(ext_span);
            let arena_src: &'a str = alloc.alloc_str(ext_src);
            match svelte_js::analyze_expression_with_alloc(alloc, arena_src, ext_span.start, typescript) {
                Ok((_info, expr)) => { parsed.ce_extend_expr = Some(expr); }
                Err(diag) => diags.push(diag),
            }
        }

        data.ce_config = Some(config);
    }
}

/// Parse an expression into the shared allocator, storing both metadata and AST.
fn parse_expr<'a>(
    alloc: &'a Allocator,
    source: &str,
    offset: u32,
    node_id: NodeId,
    typescript: bool,
    data: &mut AnalysisData,
    parsed: &mut ParsedExprs<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    // Copy source into arena so Expression AST can reference it with lifetime 'a
    let arena_source: &'a str = alloc.alloc_str(source);
    match svelte_js::analyze_expression_with_alloc(alloc, arena_source, offset, typescript) {
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
    typescript: bool,
    data: &mut AnalysisData,
    parsed: &mut ParsedExprs<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    let arena_source: &'a str = alloc.alloc_str(source);
    match svelte_js::analyze_expression_with_alloc(alloc, arena_source, offset, typescript) {
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
    typescript: bool,
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
            match svelte_js::analyze_expression_with_alloc(alloc, arena_source, span.start, typescript) {
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
        has_call: false,
        has_state_rune: false,
        has_store_member_mutation: false,
    };
    data.attr_expressions.insert(attr_id, merged);
}

fn walk_fragment<'a>(
    alloc: &'a Allocator,
    fragment: &Fragment,
    component: &Component,
    typescript: bool,
    data: &mut AnalysisData,
    parsed: &mut ParsedExprs<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    for node in &fragment.nodes {
        walk_node(alloc, node, component, typescript, data, parsed, diags);
    }
}

fn walk_node<'a>(
    alloc: &'a Allocator,
    node: &Node,
    component: &Component,
    typescript: bool,
    data: &mut AnalysisData,
    parsed: &mut ParsedExprs<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    match node {
        Node::ExpressionTag(tag) => {
            let source = component.source_text(tag.expression_span);
            parse_expr(alloc, source, tag.expression_span.start, tag.id, typescript, data, parsed, diags);
        }
        Node::Element(el) => {
            walk_attrs(alloc, &el.attributes, component, typescript, data, parsed, diags);
            walk_fragment(alloc, &el.fragment, component, typescript, data, parsed, diags);
        }
        Node::ComponentNode(cn) => {
            walk_attrs(alloc, &cn.attributes, component, typescript, data, parsed, diags);
            walk_fragment(alloc, &cn.fragment, component, typescript, data, parsed, diags);
        }
        Node::IfBlock(block) => {
            let source = component.source_text(block.test_span);
            parse_expr(alloc, source, block.test_span.start, block.id, typescript, data, parsed, diags);
            walk_fragment(alloc, &block.consequent, component, typescript, data, parsed, diags);
            if let Some(alt) = &block.alternate {
                walk_fragment(alloc, alt, component, typescript, data, parsed, diags);
            }
        }
        Node::EachBlock(block) => {
            let source = component.source_text(block.expression_span);
            parse_expr(alloc, source, block.expression_span.start, block.id, typescript, data, parsed, diags);
            if let Some(key_span) = block.key_span {
                let key_source = component.source_text(key_span);
                let arena_source: &'a str = alloc.alloc_str(key_source);
                match svelte_js::analyze_expression_with_alloc(alloc, arena_source, key_span.start, typescript) {
                    Ok((info, expr)) => {
                        // Check if key expression references the index variable
                        if let Some(idx_span) = block.index_span {
                            let idx_name = component.source_text(idx_span);
                            if info.references.iter().any(|r| r.name.as_str() == idx_name) {
                                data.each_blocks.key_uses_index.insert(block.id);
                            }
                        }
                        parsed.key_exprs.insert(block.id, expr);
                    }
                    Err(diag) => diags.push(diag),
                }
            }

            // Track expression keys before walking body to detect body_uses_index
            let expr_keys_before: rustc_hash::FxHashSet<NodeId> =
                data.expressions.keys().copied().collect();
            let attr_keys_before: rustc_hash::FxHashSet<NodeId> =
                data.attr_expressions.keys().copied().collect();

            walk_fragment(alloc, &block.body, component, typescript, data, parsed, diags);

            // Check if any body expression references the index variable
            if let Some(idx_span) = block.index_span {
                let idx_name = component.source_text(idx_span);
                let body_uses_idx = data.expressions.iter()
                    .filter(|(k, _)| !expr_keys_before.contains(k))
                    .any(|(_, info)| info.references.iter().any(|r| r.name.as_str() == idx_name))
                || data.attr_expressions.iter()
                    .filter(|(k, _)| !attr_keys_before.contains(k))
                    .any(|(_, info)| info.references.iter().any(|r| r.name.as_str() == idx_name));
                if body_uses_idx {
                    data.each_blocks.body_uses_index.insert(block.id);
                }
            }

            if let Some(fb) = &block.fallback {
                walk_fragment(alloc, fb, component, typescript, data, parsed, diags);
            }
        }
        Node::SnippetBlock(block) => {
            walk_fragment(alloc, &block.body, component, typescript, data, parsed, diags);
        }
        Node::RenderTag(tag) => {
            let source = component.source_text(tag.expression_span);
            parse_expr(alloc, source, tag.expression_span.start, tag.id, typescript, data, parsed, diags);

            // Unwrap ChainExpression → CallExpression, recording the chain flag.
            // OXC parses `fn?.()` as ChainExpression { expression: CallExpression }.
            if matches!(parsed.exprs.get(&tag.id), Some(Expression::ChainExpression(_))) {
                data.render_tag_is_chain.insert(tag.id);
                if let Some(Expression::ChainExpression(chain)) = parsed.exprs.remove(&tag.id) {
                    if let oxc_ast::ast::ChainElement::CallExpression(call) = chain.unbox().expression {
                        parsed.exprs.insert(tag.id, Expression::CallExpression(call));
                    }
                }
            }

            // Store per-argument has_call flags, identifier names, and callee name
            if let Some(Expression::CallExpression(call)) = parsed.exprs.get(&tag.id) {
                // Extract callee identifier name for dynamic detection
                if let Expression::Identifier(ident) = &call.callee {
                    data.render_tag_callee_name.insert(tag.id, ident.name.to_string());
                }

                let flags: Vec<bool> = call.arguments.iter().map(|arg| {
                    svelte_js::expression_has_call(arg.to_expression())
                }).collect();
                data.render_tag_arg_has_call.insert(tag.id, flags);

                let idents: Vec<Option<String>> = call.arguments.iter().map(|arg| {
                    if let Expression::Identifier(id) = arg.to_expression() {
                        Some(id.name.to_string())
                    } else {
                        None
                    }
                }).collect();
                data.render_tag_arg_idents.insert(tag.id, idents);
            }
        }
        Node::HtmlTag(tag) => {
            let source = component.source_text(tag.expression_span);
            parse_expr(alloc, source, tag.expression_span.start, tag.id, typescript, data, parsed, diags);
        }
        Node::KeyBlock(block) => {
            let source = component.source_text(block.expression_span);
            parse_expr(alloc, source, block.expression_span.start, block.id, typescript, data, parsed, diags);
            walk_fragment(alloc, &block.fragment, component, typescript, data, parsed, diags);
        }
        Node::AwaitBlock(block) => {
            let source = component.source_text(block.expression_span);
            parse_expr(alloc, source, block.expression_span.start, block.id, typescript, data, parsed, diags);

            if let Some(val_span) = block.value_span {
                let binding_text = component.source_text(val_span);
                let info = svelte_js::parse_await_binding(binding_text);
                data.await_bindings.values.insert(block.id, info);
            }
            if let Some(err_span) = block.error_span {
                let binding_text = component.source_text(err_span);
                let info = svelte_js::parse_await_binding(binding_text);
                data.await_bindings.errors.insert(block.id, info);
            }

            if let Some(ref p) = block.pending {
                walk_fragment(alloc, p, component, typescript, data, parsed, diags);
            }
            if let Some(ref t) = block.then {
                walk_fragment(alloc, t, component, typescript, data, parsed, diags);
            }
            if let Some(ref c) = block.catch {
                walk_fragment(alloc, c, component, typescript, data, parsed, diags);
            }
        }
        Node::ConstTag(tag) => {
            let decl_text = component.source_text(tag.declaration_span);
            let arena_source: &'a str = alloc.alloc_str(decl_text);
            match svelte_js::parse_const_declaration_with_alloc(alloc, arena_source, tag.declaration_span.start, typescript) {
                Ok((names, references, init_expr)) => {
                    data.expressions.insert(tag.id, ExpressionInfo {
                        kind: ExpressionKind::Other,
                        references,
                        has_side_effects: false,
                        has_call: false,
                        has_state_rune: false,
                        has_store_member_mutation: false,
                    });
                    parsed.exprs.insert(tag.id, init_expr);
                    data.const_tags.names.insert(tag.id, names.iter().map(|n| n.to_string()).collect());
                }
                Err(diag) => diags.push(diag),
            }
        }
        Node::SvelteHead(head) => {
            walk_fragment(alloc, &head.fragment, component, typescript, data, parsed, diags);
        }
        Node::SvelteElement(el) => {
            // Parse the tag expression (skip for static string tags like this="div")
            if !el.static_tag {
                let tag_source = component.source_text(el.tag_span);
                parse_expr(alloc, tag_source, el.tag_span.start, el.id, typescript, data, parsed, diags);
            }
            walk_attrs(alloc, &el.attributes, component, typescript, data, parsed, diags);
            walk_fragment(alloc, &el.fragment, component, typescript, data, parsed, diags);
        }
        Node::SvelteWindow(w) => {
            walk_attrs(alloc, &w.attributes, component, typescript, data, parsed, diags);
        }
        Node::SvelteDocument(d) => {
            walk_attrs(alloc, &d.attributes, component, typescript, data, parsed, diags);
        }
        Node::SvelteBody(b) => {
            walk_attrs(alloc, &b.attributes, component, typescript, data, parsed, diags);
        }
        Node::SvelteBoundary(b) => {
            walk_attrs(alloc, &b.attributes, component, typescript, data, parsed, diags);
            walk_fragment(alloc, &b.fragment, component, typescript, data, parsed, diags);
        }
        Node::DebugTag(tag) => {
            for (i, span) in tag.identifiers.iter().enumerate() {
                let name = component.source_text(*span);
                let arena_name: &'a str = alloc.alloc_str(name);
                match svelte_js::analyze_expression_with_alloc(alloc, arena_name, span.start, typescript) {
                    Ok((_info, expr)) => {
                        parsed.debug_tag_exprs.insert((tag.id, i), expr);
                    }
                    Err(_) => {}
                }
            }
        }
        Node::Text(_) | Node::Comment(_) | Node::Error(_) => {}
    }
}

/// Parse and store attribute expressions, keyed by attribute NodeId.
fn walk_attrs<'a>(
    alloc: &'a Allocator,
    attrs: &[Attribute],
    component: &Component,
    typescript: bool,
    data: &mut AnalysisData,
    parsed: &mut ParsedExprs<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    for attr in attrs {
        let attr_id = attr.id();
        match attr {
            Attribute::ExpressionAttribute(a) => {
                let source = component.source_text(a.expression_span);
                parse_attr_expr(alloc, source, a.expression_span.start, attr_id, typescript, data, parsed, diags);
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
                parse_concat_parts(alloc, &a.parts, attr_id, component, typescript, data, parsed, diags);
            }
            Attribute::ClassDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, attr_id, typescript, data, parsed, diags);
                }
            }
            Attribute::StyleDirective(a) => {
                use svelte_ast::StyleDirectiveValue;
                match &a.value {
                    StyleDirectiveValue::Expression(span) => {
                        let source = component.source_text(*span);
                        parse_attr_expr(alloc, source, span.start, attr_id, typescript, data, parsed, diags);
                    }
                    StyleDirectiveValue::Concatenation(parts) => {
                        parse_concat_parts(alloc, parts, attr_id, component, typescript, data, parsed, diags);
                    }
                    StyleDirectiveValue::Shorthand | StyleDirectiveValue::String(_) => {}
                }
            }
            Attribute::BindDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, attr_id, typescript, data, parsed, diags);
                }
            }
            Attribute::SpreadAttribute(a) => {
                // Skip the "..." prefix
                debug_assert!(
                    a.expression_span.end >= a.expression_span.start + 3,
                    "spread expression span too short to contain '...'"
                );
                let span = svelte_span::Span::new(a.expression_span.start + 3, a.expression_span.end);
                let source = component.source_text(span);
                parse_attr_expr(alloc, source, span.start, attr_id, typescript, data, parsed, diags);
            }
            Attribute::Shorthand(a) => {
                let source = component.source_text(a.expression_span);
                parse_attr_expr(alloc, source, a.expression_span.start, attr_id, typescript, data, parsed, diags);
            }
            Attribute::UseDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, attr_id, typescript, data, parsed, diags);
                }
            }
            Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_) => {}
            // LEGACY(svelte4): on:directive — parse expression if present
            Attribute::OnDirectiveLegacy(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, attr_id, typescript, data, parsed, diags);
                }
            }
            Attribute::TransitionDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, attr_id, typescript, data, parsed, diags);
                }
            }
            Attribute::AnimateDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, attr_id, typescript, data, parsed, diags);
                }
            }
            Attribute::AttachTag(a) => {
                let span = a.expression_span;
                let source = component.source_text(span);
                parse_attr_expr(alloc, source, span.start, attr_id, typescript, data, parsed, diags);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Arrow scope registration (runs after build_scoping)
// ---------------------------------------------------------------------------

/// Walk all parsed OXC expressions and register arrow function parameter scopes.
/// Must be called after `build_scoping` so that template scopes exist as parents.
pub(crate) fn register_arrow_scopes(
    component: &Component,
    data: &mut AnalysisData,
    parsed: &ParsedExprs<'_>,
) {
    let root = data.scoping.root_scope_id();
    walk_fragment_arrows(&component.fragment, component, &mut data.scoping, parsed, root);
}

fn walk_fragment_arrows(
    fragment: &Fragment,
    component: &Component,
    scoping: &mut ComponentScoping,
    parsed: &ParsedExprs<'_>,
    scope: ScopeId,
) {
    for node in &fragment.nodes {
        walk_node_arrows(node, component, scoping, parsed, scope);
    }
}

fn walk_node_arrows(
    node: &Node,
    component: &Component,
    scoping: &mut ComponentScoping,
    parsed: &ParsedExprs<'_>,
    scope: ScopeId,
) {
    match node {
        Node::ExpressionTag(tag) => {
            scan_expr_arrows(parsed.exprs.get(&tag.id), scoping, scope);
        }
        Node::Element(el) => {
            scan_attrs_arrows(&el.attributes, scoping, parsed, scope);
            walk_fragment_arrows(&el.fragment, component, scoping, parsed, scope);
        }
        Node::ComponentNode(cn) => {
            scan_attrs_arrows(&cn.attributes, scoping, parsed, scope);
            walk_fragment_arrows(&cn.fragment, component, scoping, parsed, scope);
        }
        Node::IfBlock(block) => {
            scan_expr_arrows(parsed.exprs.get(&block.id), scoping, scope);
            let cons_scope = scoping.fragment_scope(&FragmentKey::IfConsequent(block.id)).unwrap_or(scope);
            walk_fragment_arrows(&block.consequent, component, scoping, parsed, cons_scope);
            if let Some(alt) = &block.alternate {
                let alt_scope = scoping.fragment_scope(&FragmentKey::IfAlternate(block.id)).unwrap_or(scope);
                walk_fragment_arrows(alt, component, scoping, parsed, alt_scope);
            }
        }
        Node::EachBlock(block) => {
            scan_expr_arrows(parsed.exprs.get(&block.id), scoping, scope);
            let body_scope = scoping.node_scope(block.id).unwrap_or(scope);
            walk_fragment_arrows(&block.body, component, scoping, parsed, body_scope);
            if let Some(fb) = &block.fallback {
                walk_fragment_arrows(fb, component, scoping, parsed, scope);
            }
        }
        Node::SnippetBlock(block) => {
            let snippet_scope = scoping.node_scope(block.id).unwrap_or(scope);
            walk_fragment_arrows(&block.body, component, scoping, parsed, snippet_scope);
        }
        Node::RenderTag(tag) => {
            scan_expr_arrows(parsed.exprs.get(&tag.id), scoping, scope);
        }
        Node::HtmlTag(tag) => {
            scan_expr_arrows(parsed.exprs.get(&tag.id), scoping, scope);
        }
        Node::ConstTag(tag) => {
            scan_expr_arrows(parsed.exprs.get(&tag.id), scoping, scope);
        }
        Node::KeyBlock(block) => {
            scan_expr_arrows(parsed.exprs.get(&block.id), scoping, scope);
            let child_scope = scoping.fragment_scope(&FragmentKey::KeyBlockBody(block.id)).unwrap_or(scope);
            walk_fragment_arrows(&block.fragment, component, scoping, parsed, child_scope);
        }
        Node::SvelteHead(head) => {
            let child_scope = scoping.fragment_scope(&FragmentKey::SvelteHeadBody(head.id)).unwrap_or(scope);
            walk_fragment_arrows(&head.fragment, component, scoping, parsed, child_scope);
        }
        Node::SvelteElement(el) => {
            if !el.static_tag {
                scan_expr_arrows(parsed.exprs.get(&el.id), scoping, scope);
            }
            scan_attrs_arrows(&el.attributes, scoping, parsed, scope);
            let child_scope = scoping.fragment_scope(&FragmentKey::SvelteElementBody(el.id)).unwrap_or(scope);
            walk_fragment_arrows(&el.fragment, component, scoping, parsed, child_scope);
        }
        Node::SvelteWindow(w) => {
            scan_attrs_arrows(&w.attributes, scoping, parsed, scope);
        }
        Node::SvelteDocument(d) => {
            scan_attrs_arrows(&d.attributes, scoping, parsed, scope);
        }
        Node::SvelteBody(b) => {
            scan_attrs_arrows(&b.attributes, scoping, parsed, scope);
        }
        Node::SvelteBoundary(b) => {
            scan_attrs_arrows(&b.attributes, scoping, parsed, scope);
            let child_scope = scoping.fragment_scope(&FragmentKey::SvelteBoundaryBody(b.id)).unwrap_or(scope);
            walk_fragment_arrows(&b.fragment, component, scoping, parsed, child_scope);
        }
        Node::AwaitBlock(block) => {
            scan_expr_arrows(parsed.exprs.get(&block.id), scoping, scope);
            if let Some(ref p) = block.pending {
                let s = scoping.fragment_scope(&FragmentKey::AwaitPending(block.id)).unwrap_or(scope);
                walk_fragment_arrows(p, component, scoping, parsed, s);
            }
            if let Some(ref t) = block.then {
                let s = scoping.node_scope(block.id).unwrap_or(scope);
                walk_fragment_arrows(t, component, scoping, parsed, s);
            }
            if let Some(ref c) = block.catch {
                let s = scoping.await_catch_scope(block.id).unwrap_or(scope);
                walk_fragment_arrows(c, component, scoping, parsed, s);
            }
        }
        Node::DebugTag(_) | Node::Text(_) | Node::Comment(_) | Node::Error(_) => {}
    }
}

fn scan_attrs_arrows(
    attrs: &[Attribute],
    scoping: &mut ComponentScoping,
    parsed: &ParsedExprs<'_>,
    scope: ScopeId,
) {
    for attr in attrs {
        let attr_id = attr.id();
        scan_expr_arrows(parsed.attr_exprs.get(&attr_id), scoping, scope);
        // Concat part expressions
        let concat_parts: Option<&[ConcatPart]> = match attr {
            Attribute::ConcatenationAttribute(a) => Some(&a.parts),
            Attribute::StyleDirective(a) => match &a.value {
                svelte_ast::StyleDirectiveValue::Concatenation(parts) => Some(parts),
                _ => None,
            },
            _ => None,
        };
        if let Some(parts) = concat_parts {
            let dyn_count = parts.iter().filter(|p| matches!(p, ConcatPart::Dynamic(_))).count();
            for dyn_idx in 0..dyn_count {
                scan_expr_arrows(parsed.concat_part_exprs.get(&(attr_id, dyn_idx)), scoping, scope);
            }
        }
    }
}

/// Scan a single OXC Expression for ArrowFunctionExpressions, registering scopes.
fn scan_expr_arrows<'a>(
    expr: Option<&Expression<'a>>,
    scoping: &mut ComponentScoping,
    scope: ScopeId,
) {
    let Some(expr) = expr else { return };
    let mut collector = ArrowScopeCollector { scoping, scope };
    collector.visit_expression(expr);
}

/// Visit-based collector: walks OXC expression AST, registers arrow param scopes.
struct ArrowScopeCollector<'s> {
    scoping: &'s mut ComponentScoping,
    scope: ScopeId,
}

impl<'a> Visit<'a> for ArrowScopeCollector<'_> {
    fn visit_arrow_function_expression(&mut self, arrow: &ArrowFunctionExpression<'a>) {
        let param_names = extract_arrow_param_names(&arrow.params);
        let arrow_scope = self.scoping.register_arrow_scope(arrow.span.start, self.scope, &param_names);
        let parent_scope = self.scope;
        self.scope = arrow_scope;
        for stmt in &arrow.body.statements {
            self.visit_statement(stmt);
        }
        self.scope = parent_scope;
    }
}

fn extract_arrow_param_names(params: &FormalParameters<'_>) -> Vec<String> {
    let mut names = Vec::new();
    for param in &params.items {
        collect_binding_names(&param.pattern, &mut names);
    }
    if let Some(rest) = &params.rest {
        collect_binding_names(&rest.rest.argument, &mut names);
    }
    names
}

fn collect_binding_names(pattern: &BindingPattern<'_>, names: &mut Vec<String>) {
    match pattern {
        BindingPattern::BindingIdentifier(id) => {
            names.push(id.name.as_str().to_string());
        }
        BindingPattern::ObjectPattern(obj) => {
            for prop in &obj.properties {
                collect_binding_names(&prop.value, names);
            }
            if let Some(rest) = &obj.rest {
                collect_binding_names(&rest.argument, names);
            }
        }
        BindingPattern::ArrayPattern(arr) => {
            for elem in arr.elements.iter().flatten() {
                collect_binding_names(elem, names);
            }
            if let Some(rest) = &arr.rest {
                collect_binding_names(&rest.argument, names);
            }
        }
        BindingPattern::AssignmentPattern(assign) => {
            collect_binding_names(&assign.left, names);
        }
    }
}
