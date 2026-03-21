use oxc_allocator::Allocator;
use oxc_ast::ast::Expression;
use svelte_ast::{Attribute, Component, ConcatPart, Fragment, Node, NodeId, ScriptLanguage};
use svelte_diagnostics::Diagnostic;
use svelte_types::{ExpressionInfo, ExpressionKind, JsParseResult};

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
        match crate::js_parse::analyze_script_with_alloc(
            alloc,
            arena_source,
            script.content_span.start,
            typescript,
        ) {
            Ok((mut info, scoping, program)) => {
                result.exports = std::mem::take(&mut info.exports);
                result.needs_context = info.has_effects || info.has_class_state_fields;
                result.has_class_state_fields = info.has_class_state_fields;
                result.script = Some(info);
                result.scoping = Some(scoping);
                result.parsed.script_program = Some(program);
            }
            Err(errs) => diags.extend(errs),
        }
    }

    // Pre-parse prop default expressions so codegen doesn't re-parse them
    if let Some(ref script_info) = result.script {
        if let Some(ref props_decl) = script_info.props_declaration {
            for prop in &props_decl.props {
                if let Some(span) = prop.default_span {
                    let src = component.source_text(span);
                    let arena_src: &'a str = alloc.alloc_str(src);
                    match crate::js_parse::analyze_expression_with_alloc(alloc, arena_src, span.start, typescript) {
                        Ok((_info, expr)) => result.parsed.prop_default_exprs.push(Some(expr)),
                        Err(diag) => { diags.push(diag); result.parsed.prop_default_exprs.push(None); }
                    }
                } else {
                    result.parsed.prop_default_exprs.push(None);
                }
            }
        }
    }

    walk_fragment(alloc, &component.fragment, component, typescript, result, diags);

    // Parse custom element config expression (if present)
    if let Some(svelte_ast::CustomElementConfig::Expression(span)) =
        component.options.as_ref().and_then(|o| o.custom_element.as_ref())
    {
        let ce_source = component.source_text(*span);
        let config = crate::js_parse::parse_ce_config(ce_source, span.start);

        if let Some(ext_span) = config.extend_span {
            let ext_src = component.source_text(ext_span);
            let arena_src: &'a str = alloc.alloc_str(ext_src);
            match crate::js_parse::analyze_expression_with_alloc(alloc, arena_src, ext_span.start, typescript) {
                Ok((_info, expr)) => { result.parsed.ce_extend_expr = Some(expr); }
                Err(diag) => diags.push(diag),
            }
        }

        result.ce_config = Some(config);
    }
}

/// Parse an expression into the shared allocator, storing both metadata and AST.
fn parse_expr<'a>(
    alloc: &'a Allocator,
    source: &str,
    offset: u32,
    node_id: NodeId,
    typescript: bool,
    result: &mut JsParseResult<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    let arena_source: &'a str = alloc.alloc_str(source);
    match crate::js_parse::analyze_expression_with_alloc(alloc, arena_source, offset, typescript) {
        Ok((info, expr)) => {
            result.expressions.insert(node_id, info);
            result.parsed.exprs.insert(node_id, expr);
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
    result: &mut JsParseResult<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    let arena_source: &'a str = alloc.alloc_str(source);
    match crate::js_parse::analyze_expression_with_alloc(alloc, arena_source, offset, typescript) {
        Ok((info, expr)) => {
            result.attr_expressions.insert(attr_id, info);
            result.parsed.attr_exprs.insert(attr_id, expr);
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
    result: &mut JsParseResult<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    let mut all_refs = Vec::new();
    let mut dyn_idx = 0usize;
    for part in parts {
        if let ConcatPart::Dynamic(span) = part {
            let source = component.source_text(*span);
            let arena_source: &'a str = alloc.alloc_str(source);
            match crate::js_parse::analyze_expression_with_alloc(alloc, arena_source, span.start, typescript) {
                Ok((info, expr)) => {
                    all_refs.extend(info.references);
                    result.parsed.concat_part_exprs.insert((attr_id, dyn_idx), expr);
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
    result.attr_expressions.insert(attr_id, merged);
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
            let source = component.source_text(tag.expression_span);
            parse_expr(alloc, source, tag.expression_span.start, tag.id, typescript, result, diags);
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
            let source = component.source_text(block.test_span);
            parse_expr(alloc, source, block.test_span.start, block.id, typescript, result, diags);
            walk_fragment(alloc, &block.consequent, component, typescript, result, diags);
            if let Some(alt) = &block.alternate {
                walk_fragment(alloc, alt, component, typescript, result, diags);
            }
        }
        Node::EachBlock(block) => {
            let source = component.source_text(block.expression_span);
            parse_expr(alloc, source, block.expression_span.start, block.id, typescript, result, diags);
            if let Some(key_span) = block.key_span {
                let key_source = component.source_text(key_span);
                let arena_source: &'a str = alloc.alloc_str(key_source);
                match crate::js_parse::analyze_expression_with_alloc(alloc, arena_source, key_span.start, typescript) {
                    Ok((info, expr)) => {
                        if let Some(idx_span) = block.index_span {
                            let idx_name = component.source_text(idx_span);
                            if info.references.iter().any(|r| r.name.as_str() == idx_name) {
                                result.each_key_uses_index.insert(block.id);
                            }
                        }
                        result.parsed.key_exprs.insert(block.id, expr);
                    }
                    Err(diag) => diags.push(diag),
                }
            }

            // Pre-parse destructuring context via OXC so codegen doesn't re-parse
            let ctx_source = component.source_text(block.context_span);
            let ctx_trimmed = ctx_source.trim();
            if ctx_trimmed.starts_with('{') || ctx_trimmed.starts_with('[') {
                let arena_ctx: &'a str = alloc.alloc_str(ctx_source);
                if let Some(binding) = crate::js_parse::parse_each_context_with_alloc(alloc, arena_ctx, typescript) {
                    result.parsed.each_context_bindings.insert(block.id, binding);
                }
            }

            // Track expression keys before walking body to detect body_uses_index
            let expr_keys_before: rustc_hash::FxHashSet<NodeId> =
                result.expressions.keys().copied().collect();
            let attr_keys_before: rustc_hash::FxHashSet<NodeId> =
                result.attr_expressions.keys().copied().collect();

            walk_fragment(alloc, &block.body, component, typescript, result, diags);

            // Check if any body expression references the index variable
            if let Some(idx_span) = block.index_span {
                let idx_name = component.source_text(idx_span);
                let body_uses_idx = result.expressions.iter()
                    .filter(|(k, _)| !expr_keys_before.contains(k))
                    .any(|(_, info)| info.references.iter().any(|r| r.name.as_str() == idx_name))
                || result.attr_expressions.iter()
                    .filter(|(k, _)| !attr_keys_before.contains(k))
                    .any(|(_, info)| info.references.iter().any(|r| r.name.as_str() == idx_name));
                if body_uses_idx {
                    result.each_body_uses_index.insert(block.id);
                }
            }

            if let Some(fb) = &block.fallback {
                walk_fragment(alloc, fb, component, typescript, result, diags);
            }
        }
        Node::SnippetBlock(block) => {
            walk_fragment(alloc, &block.body, component, typescript, result, diags);
        }
        Node::RenderTag(tag) => {
            let source = component.source_text(tag.expression_span);
            parse_expr(alloc, source, tag.expression_span.start, tag.id, typescript, result, diags);

            // Unwrap ChainExpression → CallExpression, recording the chain flag.
            if matches!(result.parsed.exprs.get(&tag.id), Some(Expression::ChainExpression(_))) {
                result.render_tag_is_chain.insert(tag.id);
                if let Some(Expression::ChainExpression(chain)) = result.parsed.exprs.remove(&tag.id) {
                    if let oxc_ast::ast::ChainElement::CallExpression(call) = chain.unbox().expression {
                        result.parsed.exprs.insert(tag.id, Expression::CallExpression(call));
                    }
                }
            }

            // Store per-argument has_call flags, identifier names, and callee name
            if let Some(Expression::CallExpression(call)) = result.parsed.exprs.get(&tag.id) {
                if let Expression::Identifier(ident) = &call.callee {
                    result.render_tag_callee_name.insert(tag.id, ident.name.to_string());
                }

                let flags: Vec<bool> = call.arguments.iter().map(|arg| {
                    svelte_types::expression_has_call(arg.to_expression())
                }).collect();
                result.render_tag_arg_has_call.insert(tag.id, flags);

                let idents: Vec<Option<String>> = call.arguments.iter().map(|arg| {
                    if let Expression::Identifier(id) = arg.to_expression() {
                        Some(id.name.to_string())
                    } else {
                        None
                    }
                }).collect();
                result.render_tag_arg_idents.insert(tag.id, idents);
            }
        }
        Node::HtmlTag(tag) => {
            let source = component.source_text(tag.expression_span);
            parse_expr(alloc, source, tag.expression_span.start, tag.id, typescript, result, diags);
        }
        Node::KeyBlock(block) => {
            let source = component.source_text(block.expression_span);
            parse_expr(alloc, source, block.expression_span.start, block.id, typescript, result, diags);
            walk_fragment(alloc, &block.fragment, component, typescript, result, diags);
        }
        Node::AwaitBlock(block) => {
            let source = component.source_text(block.expression_span);
            parse_expr(alloc, source, block.expression_span.start, block.id, typescript, result, diags);

            if let Some(val_span) = block.value_span {
                let binding_text = component.source_text(val_span);
                let info = crate::js_parse::parse_await_binding(binding_text);
                result.await_values.insert(block.id, info);
            }
            if let Some(err_span) = block.error_span {
                let binding_text = component.source_text(err_span);
                let info = crate::js_parse::parse_await_binding(binding_text);
                result.await_errors.insert(block.id, info);
            }

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
            match crate::js_parse::parse_const_declaration_with_alloc(alloc, arena_source, tag.declaration_span.start, typescript) {
                Ok((names, references, init_expr)) => {
                    result.expressions.insert(tag.id, ExpressionInfo {
                        kind: ExpressionKind::Other,
                        references,
                        has_side_effects: false,
                        has_call: false,
                        has_state_rune: false,
                        has_store_member_mutation: false,
                    });
                    result.parsed.exprs.insert(tag.id, init_expr);
                    result.const_tag_names.insert(tag.id, names.iter().map(|n| n.to_string()).collect());
                }
                Err(diag) => diags.push(diag),
            }
        }
        Node::SvelteHead(head) => {
            walk_fragment(alloc, &head.fragment, component, typescript, result, diags);
        }
        Node::SvelteElement(el) => {
            if !el.static_tag {
                let tag_source = component.source_text(el.tag_span);
                parse_expr(alloc, tag_source, el.tag_span.start, el.id, typescript, result, diags);
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
            for (i, span) in tag.identifiers.iter().enumerate() {
                let name = component.source_text(*span);
                let arena_name: &'a str = alloc.alloc_str(name);
                match crate::js_parse::analyze_expression_with_alloc(alloc, arena_name, span.start, typescript) {
                    Ok((_info, expr)) => {
                        result.parsed.debug_tag_exprs.insert((tag.id, i), expr);
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
    result: &mut JsParseResult<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    for attr in attrs {
        let attr_id = attr.id();
        match attr {
            Attribute::ExpressionAttribute(a) => {
                let source = component.source_text(a.expression_span);
                parse_attr_expr(alloc, source, a.expression_span.start, attr_id, typescript, result, diags);
                // Detect semantic shorthand: expression is a simple identifier matching attr name
                if let Some(Expression::Identifier(ident)) = result.parsed.attr_exprs.get(&attr_id) {
                    if ident.name.as_str() == a.name {
                        result.expression_shorthand.insert(attr_id);
                    }
                }
                // class={[...]} or class={{...}} or class={x} need clsx to resolve
                if a.name == "class" {
                    if let Some(expr) = result.parsed.attr_exprs.get(&attr_id) {
                        let needs = !matches!(
                            expr,
                            oxc_ast::ast::Expression::StringLiteral(_)
                                | oxc_ast::ast::Expression::TemplateLiteral(_)
                                | oxc_ast::ast::Expression::BinaryExpression(_)
                        );
                        if needs {
                            result.needs_clsx.insert(attr_id);
                        }
                    }
                }
            }
            Attribute::ConcatenationAttribute(a) => {
                parse_concat_parts(alloc, &a.parts, attr_id, component, typescript, result, diags);
            }
            Attribute::ClassDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, attr_id, typescript, result, diags);
                    if let Some(Expression::Identifier(ident)) = result.parsed.attr_exprs.get(&attr_id) {
                        if ident.name.as_str() == a.name {
                            result.expression_shorthand.insert(attr_id);
                        }
                    }
                }
            }
            Attribute::StyleDirective(a) => {
                use svelte_ast::StyleDirectiveValue;
                match &a.value {
                    StyleDirectiveValue::Expression(span) => {
                        let source = component.source_text(*span);
                        parse_attr_expr(alloc, source, span.start, attr_id, typescript, result, diags);
                        if let Some(Expression::Identifier(ident)) = result.parsed.attr_exprs.get(&attr_id) {
                            if ident.name.as_str() == a.name {
                                result.expression_shorthand.insert(attr_id);
                            }
                        }
                    }
                    StyleDirectiveValue::Concatenation(parts) => {
                        parse_concat_parts(alloc, parts, attr_id, component, typescript, result, diags);
                    }
                    StyleDirectiveValue::Shorthand | StyleDirectiveValue::String(_) => {}
                }
            }
            Attribute::BindDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, attr_id, typescript, result, diags);
                }
            }
            Attribute::SpreadAttribute(a) => {
                debug_assert!(
                    a.expression_span.end >= a.expression_span.start + 3,
                    "spread expression span too short to contain '...'"
                );
                let span = svelte_span::Span::new(a.expression_span.start + 3, a.expression_span.end);
                let source = component.source_text(span);
                parse_attr_expr(alloc, source, span.start, attr_id, typescript, result, diags);
            }
            Attribute::Shorthand(a) => {
                let source = component.source_text(a.expression_span);
                parse_attr_expr(alloc, source, a.expression_span.start, attr_id, typescript, result, diags);
            }
            Attribute::UseDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, attr_id, typescript, result, diags);
                }
                let name_src = component.source_text(a.name);
                let arena_src: &'a str = alloc.alloc_str(name_src);
                if let Ok((_info, expr)) = crate::js_parse::analyze_expression_with_alloc(alloc, arena_src, a.name.start, typescript) {
                    result.parsed.directive_name_exprs.insert(a.id, expr);
                }
            }
            Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_) => {}
            // LEGACY(svelte4): on:directive — parse expression if present
            Attribute::OnDirectiveLegacy(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, attr_id, typescript, result, diags);
                }
            }
            Attribute::TransitionDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, attr_id, typescript, result, diags);
                }
                let name_src = component.source_text(a.name);
                let arena_src: &'a str = alloc.alloc_str(name_src);
                if let Ok((_info, expr)) = crate::js_parse::analyze_expression_with_alloc(alloc, arena_src, a.name.start, typescript) {
                    result.parsed.directive_name_exprs.insert(a.id, expr);
                }
            }
            Attribute::AnimateDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, attr_id, typescript, result, diags);
                }
                let name_src = component.source_text(a.name);
                let arena_src: &'a str = alloc.alloc_str(name_src);
                if let Ok((_info, expr)) = crate::js_parse::analyze_expression_with_alloc(alloc, arena_src, a.name.start, typescript) {
                    result.parsed.directive_name_exprs.insert(a.id, expr);
                }
            }
            Attribute::AttachTag(a) => {
                let span = a.expression_span;
                let source = component.source_text(span);
                parse_attr_expr(alloc, source, span.start, attr_id, typescript, result, diags);
            }
        }
    }
}
