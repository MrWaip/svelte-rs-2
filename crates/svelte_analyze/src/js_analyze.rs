//! JS AST analysis functions moved from `svelte_types`.
//!
//! These functions produce metadata (`ExpressionInfo`, `ScriptInfo`, etc.)
//! from OXC AST nodes. They are internal to the analyze crate.

use compact_str::CompactString;
use oxc_ast::ast::Expression;
use svelte_span::Span;
use crate::script_types::{RuneKind, ScriptInfo};

use svelte_ast::{Attribute, ConcatPart, Component, Fragment, Node, NodeId};

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
    let Some(ref program) = parsed.program else { return None };

    let sem = oxc_semantic::SemanticBuilder::new().build(program);
    crate::script_info::enrich_from_unresolved(&sem.semantic.scoping(), &mut script_info);

    // Classify script body: effects, class state fields, store mutations
    let (has_effects, has_class_state_fields) = detect_script_flags(program);
    data.has_store_member_mutations = program.body.iter().any(|stmt| {
        if let oxc_ast::ast::Statement::ExpressionStatement(es) = stmt {
            has_deep_store_mutation(&es.expression)
        } else {
            false
        }
    });

    data.exports = std::mem::take(&mut script_info.exports);
    data.needs_context = has_effects
        || has_class_state_fields
        || script_body_needs_context(program, sem.semantic.scoping(), &script_info);
    data.has_class_state_fields = has_class_state_fields;
    compute_proxy_state_inits(program, &script_info, data);
    data.script = Some(script_info);
    Some(sem.semantic.into_scoping())
}

/// Unwrap ChainExpression → CallExpression for render tags and extract callee name.
/// Must run before `extract_all_expressions` because it mutates `parsed.exprs`.
pub(crate) fn classify_render_tags(
    parsed: &mut ParsedExprs<'_>,
    component: &Component,
    data: &mut AnalysisData,
) {
    classify_render_tags_in_fragment(&component.fragment, parsed, data);
}

fn classify_render_tags_in_fragment(
    fragment: &Fragment,
    parsed: &mut ParsedExprs<'_>,
    data: &mut AnalysisData,
) {
    for node in &fragment.nodes {
        match node {
            Node::RenderTag(tag) => {
                let offset = tag.expression_span.start;
                if matches!(parsed.exprs.get(&offset), Some(Expression::ChainExpression(_))) {
                    data.render_tag_is_chain.insert(tag.id);
                    if let Some(Expression::ChainExpression(chain)) = parsed.exprs.remove(&offset) {
                        if let oxc_ast::ast::ChainElement::CallExpression(call) = chain.unbox().expression {
                            parsed.exprs.insert(offset, Expression::CallExpression(call));
                        }
                    }
                }
                if let Some(Expression::CallExpression(call)) = parsed.exprs.get(&offset) {
                    if let Expression::Identifier(ident) = &call.callee {
                        data.render_tag_callee_name.insert(tag.id, ident.name.to_string());
                    }
                }
            }
            Node::Element(el) => classify_render_tags_in_fragment(&el.fragment, parsed, data),
            Node::ComponentNode(cn) => classify_render_tags_in_fragment(&cn.fragment, parsed, data),
            Node::IfBlock(block) => {
                classify_render_tags_in_fragment(&block.consequent, parsed, data);
                if let Some(alt) = &block.alternate {
                    classify_render_tags_in_fragment(alt, parsed, data);
                }
            }
            Node::EachBlock(block) => {
                classify_render_tags_in_fragment(&block.body, parsed, data);
                if let Some(fb) = &block.fallback {
                    classify_render_tags_in_fragment(fb, parsed, data);
                }
            }
            Node::SnippetBlock(block) => classify_render_tags_in_fragment(&block.body, parsed, data),
            Node::KeyBlock(block) => classify_render_tags_in_fragment(&block.fragment, parsed, data),
            Node::AwaitBlock(block) => {
                if let Some(ref p) = block.pending {
                    classify_render_tags_in_fragment(p, parsed, data);
                }
                if let Some(ref t) = block.then {
                    classify_render_tags_in_fragment(t, parsed, data);
                }
                if let Some(ref c) = block.catch {
                    classify_render_tags_in_fragment(c, parsed, data);
                }
            }
            Node::SvelteHead(head) => classify_render_tags_in_fragment(&head.fragment, parsed, data),
            Node::SvelteElement(el) => classify_render_tags_in_fragment(&el.fragment, parsed, data),
            Node::SvelteBoundary(b) => classify_render_tags_in_fragment(&b.fragment, parsed, data),
            _ => {}
        }
    }
}

/// Extract ExpressionInfo for all parsed template and attribute expressions.
/// Also classifies: expression shorthand, needs_clsx, const_tag_names,
/// snippet_param_names, await_values/errors, CE config.
pub(crate) fn extract_all_expressions(
    parsed: &ParsedExprs<'_>,
    component: &Component,
    data: &mut AnalysisData,
    typescript: bool,
) {
    walk_fragment_for_exprs(&component.fragment, component, parsed, data, typescript);

    // Extract CE config from parsed ObjectExpression
    if let Some(svelte_ast::CustomElementConfig::Expression(span)) =
        component.options.as_ref().and_then(|o| o.custom_element.as_ref())
    {
        if let Some(expr) = parsed.exprs.get(&span.start) {
            let config = extract_ce_config_from_expr(expr, span.start);
            data.ce_config = Some(config);
        }
    }
}

fn walk_fragment_for_exprs(
    fragment: &Fragment,
    component: &Component,
    parsed: &ParsedExprs<'_>,
    data: &mut AnalysisData,
    typescript: bool,
) {
    for node in &fragment.nodes {
        walk_node_for_exprs(node, component, parsed, data, typescript);
    }
}

fn walk_node_for_exprs(
    node: &Node,
    component: &Component,
    parsed: &ParsedExprs<'_>,
    data: &mut AnalysisData,
    typescript: bool,
) {
    match node {
        Node::ExpressionTag(tag) => {
            insert_node_expr_info(parsed, data, tag.id, tag.expression_span.start);
        }
        Node::Element(el) => {
            walk_attrs_for_exprs(&el.attributes, component, parsed, data);
            walk_fragment_for_exprs(&el.fragment, component, parsed, data, typescript);
        }
        Node::ComponentNode(cn) => {
            walk_attrs_for_exprs(&cn.attributes, component, parsed, data);
            walk_fragment_for_exprs(&cn.fragment, component, parsed, data, typescript);
        }
        Node::IfBlock(block) => {
            insert_node_expr_info(parsed, data, block.id, block.test_span.start);
            walk_fragment_for_exprs(&block.consequent, component, parsed, data, typescript);
            if let Some(alt) = &block.alternate {
                walk_fragment_for_exprs(alt, component, parsed, data, typescript);
            }
        }
        Node::EachBlock(block) => {
            insert_node_expr_info(parsed, data, block.id, block.expression_span.start);
            walk_fragment_for_exprs(&block.body, component, parsed, data, typescript);
            if let Some(fb) = &block.fallback {
                walk_fragment_for_exprs(fb, component, parsed, data, typescript);
            }
        }
        Node::SnippetBlock(block) => {
            if let Some(span) = block.params_span {
                let params = svelte_parser::parse_snippet_params(component.source_text(span));
                data.snippets.params.insert(block.id, params);
            }
            walk_fragment_for_exprs(&block.body, component, parsed, data, typescript);
        }
        Node::RenderTag(tag) => {
            insert_node_expr_info(parsed, data, tag.id, tag.expression_span.start);
        }
        Node::HtmlTag(tag) => {
            insert_node_expr_info(parsed, data, tag.id, tag.expression_span.start);
        }
        Node::KeyBlock(block) => {
            insert_node_expr_info(parsed, data, block.id, block.expression_span.start);
            walk_fragment_for_exprs(&block.fragment, component, parsed, data, typescript);
        }
        Node::AwaitBlock(block) => {
            insert_node_expr_info(parsed, data, block.id, block.expression_span.start);

            if let Some(val_span) = block.value_span {
                let binding_text = component.source_text(val_span);
                let info = svelte_parser::parse_await_binding(binding_text);
                data.await_bindings.values.insert(block.id, info);
            }
            if let Some(err_span) = block.error_span {
                let binding_text = component.source_text(err_span);
                let info = svelte_parser::parse_await_binding(binding_text);
                data.await_bindings.errors.insert(block.id, info);
            }

            if let Some(ref p) = block.pending {
                walk_fragment_for_exprs(p, component, parsed, data, typescript);
            }
            if let Some(ref t) = block.then {
                walk_fragment_for_exprs(t, component, parsed, data, typescript);
            }
            if let Some(ref c) = block.catch {
                walk_fragment_for_exprs(c, component, parsed, data, typescript);
            }
        }
        Node::ConstTag(tag) => {
            let ref_offset = tag.declaration_span.start.wrapping_sub(6);
            insert_node_expr_info(parsed, data, tag.id, ref_offset);

            // Extract binding names from the const declaration
            let names = extract_const_tag_names(component.source_text(tag.declaration_span), typescript);
            data.const_tags.names.insert(tag.id, names);
        }
        Node::SvelteHead(head) => {
            walk_fragment_for_exprs(&head.fragment, component, parsed, data, typescript);
        }
        Node::SvelteElement(el) => {
            if !el.static_tag {
                insert_node_expr_info(parsed, data, el.id, el.tag_span.start);
            }
            walk_attrs_for_exprs(&el.attributes, component, parsed, data);
            walk_fragment_for_exprs(&el.fragment, component, parsed, data, typescript);
        }
        Node::SvelteWindow(w) => {
            walk_attrs_for_exprs(&w.attributes, component, parsed, data);
        }
        Node::SvelteDocument(d) => {
            walk_attrs_for_exprs(&d.attributes, component, parsed, data);
        }
        Node::SvelteBody(b) => {
            walk_attrs_for_exprs(&b.attributes, component, parsed, data);
        }
        Node::SvelteBoundary(b) => {
            walk_attrs_for_exprs(&b.attributes, component, parsed, data);
            walk_fragment_for_exprs(&b.fragment, component, parsed, data, typescript);
        }
        Node::DebugTag(_) | Node::Text(_) | Node::Comment(_) | Node::Error(_) => {}
    }
}

fn walk_attrs_for_exprs(
    attrs: &[Attribute],
    _component: &Component,
    parsed: &ParsedExprs<'_>,
    data: &mut AnalysisData,
) {
    for attr in attrs {
        let attr_id = attr.id();
        match attr {
            Attribute::ExpressionAttribute(a) => {
                insert_attr_expr_info(parsed, data, attr_id, a.expression_span.start);

                // Detect semantic shorthand: expression is a simple identifier matching attr name
                if let Some(Expression::Identifier(ident)) = parsed.exprs.get(&a.expression_span.start) {
                    if ident.name.as_str() == a.name {
                        data.element_flags.expression_shorthand.insert(attr_id);
                    }
                }
                // class={[...]} or class={{...}} or class={x} need clsx to resolve
                if a.name == "class" {
                    if let Some(expr) = parsed.exprs.get(&a.expression_span.start) {
                        let needs = !matches!(
                            expr,
                            Expression::StringLiteral(_)
                                | Expression::TemplateLiteral(_)
                                | Expression::BinaryExpression(_)
                        );
                        if needs {
                            data.element_flags.needs_clsx.insert(attr_id);
                        }
                    }
                }
            }
            Attribute::ConcatenationAttribute(a) => {
                insert_concat_expr_info(parsed, data, attr_id, &a.parts);
            }
            Attribute::ClassDirective(a) => {
                if let Some(span) = a.expression_span {
                    insert_attr_expr_info(parsed, data, attr_id, span.start);
                    if let Some(Expression::Identifier(ident)) = parsed.exprs.get(&span.start) {
                        if ident.name.as_str() == a.name {
                            data.element_flags.expression_shorthand.insert(attr_id);
                        }
                    }
                }
            }
            Attribute::StyleDirective(a) => {
                use svelte_ast::StyleDirectiveValue;
                match &a.value {
                    StyleDirectiveValue::Expression(span) => {
                        insert_attr_expr_info(parsed, data, attr_id, span.start);
                        if let Some(Expression::Identifier(ident)) = parsed.exprs.get(&span.start) {
                            if ident.name.as_str() == a.name {
                                data.element_flags.expression_shorthand.insert(attr_id);
                            }
                        }
                    }
                    StyleDirectiveValue::Concatenation(parts) => {
                        insert_concat_expr_info(parsed, data, attr_id, parts);
                    }
                    StyleDirectiveValue::Shorthand | StyleDirectiveValue::String(_) => {}
                }
            }
            Attribute::BindDirective(a) => {
                if let Some(span) = a.expression_span {
                    insert_attr_expr_info(parsed, data, attr_id, span.start);
                }
            }
            Attribute::SpreadAttribute(a) => {
                let offset = a.expression_span.start + 3;
                insert_attr_expr_info(parsed, data, attr_id, offset);
            }
            Attribute::Shorthand(a) => {
                insert_attr_expr_info(parsed, data, attr_id, a.expression_span.start);
            }
            Attribute::UseDirective(a) => {
                if let Some(span) = a.expression_span {
                    insert_attr_expr_info(parsed, data, attr_id, span.start);
                }
            }
            Attribute::OnDirectiveLegacy(a) => {
                if let Some(span) = a.expression_span {
                    insert_attr_expr_info(parsed, data, attr_id, span.start);
                }
            }
            Attribute::TransitionDirective(a) => {
                if let Some(span) = a.expression_span {
                    insert_attr_expr_info(parsed, data, attr_id, span.start);
                }
            }
            Attribute::AnimateDirective(a) => {
                if let Some(span) = a.expression_span {
                    insert_attr_expr_info(parsed, data, attr_id, span.start);
                }
            }
            Attribute::AttachTag(a) => {
                insert_attr_expr_info(parsed, data, attr_id, a.expression_span.start);
            }
            Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_) => {}
        }
    }
}

/// Look up a parsed expression by offset and store ExpressionInfo for a template node.
fn insert_node_expr_info(
    parsed: &ParsedExprs<'_>,
    data: &mut AnalysisData,
    node_id: NodeId,
    offset: u32,
) {
    if let Some(expr) = parsed.exprs.get(&offset) {
        let info = extract_expression_info(expr, offset);
        data.expressions.insert(node_id, info);
    }
}

/// Look up a parsed expression by offset and store ExpressionInfo for an attribute.
fn insert_attr_expr_info(
    parsed: &ParsedExprs<'_>,
    data: &mut AnalysisData,
    attr_id: NodeId,
    offset: u32,
) {
    if let Some(expr) = parsed.exprs.get(&offset) {
        let info = extract_expression_info(expr, offset);
        data.attr_expressions.insert(attr_id, info);
    }
}

/// Merge ExpressionInfo from all dynamic concatenation parts into a single entry.
fn insert_concat_expr_info(
    parsed: &ParsedExprs<'_>,
    data: &mut AnalysisData,
    attr_id: NodeId,
    parts: &[ConcatPart],
) {
    let mut all_refs = Vec::new();
    for part in parts {
        if let ConcatPart::Dynamic(span) = part {
            if let Some(expr) = parsed.exprs.get(&span.start) {
                let info = extract_expression_info(expr, span.start);
                all_refs.extend(info.references);
            }
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

/// Extract binding names from a const declaration using a temporary OXC allocator.
fn extract_const_tag_names(decl_text: &str, typescript: bool) -> Vec<String> {
    use oxc_parser::Parser as OxcParser;
    use oxc_span::SourceType;

    let alloc = oxc_allocator::Allocator::default();
    let wrapped = format!("const {};", decl_text);
    let wrapped_str: &str = alloc.alloc_str(&wrapped);

    let src_type = if typescript {
        SourceType::default().with_typescript(true).with_module(true)
    } else {
        SourceType::default()
    };
    let result = OxcParser::new(&alloc, wrapped_str, src_type).parse();

    if result.errors.is_empty() {
        if let Some(oxc_ast::ast::Statement::VariableDeclaration(decl)) = result.program.body.first() {
            if let Some(declarator) = decl.declarations.first() {
                let mut names = Vec::new();
                svelte_parser::extract_all_binding_names(&declarator.id, &mut names);
                return names.iter().map(|n| n.to_string()).collect();
            }
        }
    }
    Vec::new()
}

/// Extract a `ParsedCeConfig` from an already-parsed ObjectExpression AST node.
fn extract_ce_config_from_expr(
    expr: &Expression<'_>,
    offset: u32,
) -> svelte_parser::ParsedCeConfig {
    use oxc_ast::ast::{ObjectPropertyKind, PropertyKey};

    let mut config = svelte_parser::ParsedCeConfig {
        tag: None,
        shadow: svelte_parser::CeShadowMode::Open,
        props: Vec::new(),
        extend_span: None,
    };

    let Expression::ObjectExpression(obj) = expr else { return config };

    for prop_kind in &obj.properties {
        let ObjectPropertyKind::ObjectProperty(prop) = prop_kind else { continue };
        let key_name = match &prop.key {
            PropertyKey::StaticIdentifier(id) => id.name.as_str(),
            _ => continue,
        };

        match key_name {
            "tag" => {
                if let Expression::StringLiteral(lit) = &prop.value {
                    config.tag = Some(lit.value.to_string());
                }
            }
            "shadow" => {
                if let Expression::StringLiteral(lit) = &prop.value {
                    if lit.value.as_str() == "none" {
                        config.shadow = svelte_parser::CeShadowMode::None;
                    }
                }
            }
            "props" => {
                if let Expression::ObjectExpression(props_obj) = &prop.value {
                    for prop_entry in &props_obj.properties {
                        let ObjectPropertyKind::ObjectProperty(entry) = prop_entry else { continue };
                        let prop_name = match &entry.key {
                            PropertyKey::StaticIdentifier(id) => id.name.to_string(),
                            _ => continue,
                        };
                        let mut prop_cfg = svelte_parser::CePropConfig {
                            name: prop_name,
                            attribute: None,
                            reflect: false,
                            prop_type: None,
                        };
                        if let Expression::ObjectExpression(def_obj) = &entry.value {
                            for def_prop in &def_obj.properties {
                                let ObjectPropertyKind::ObjectProperty(dp) = def_prop else { continue };
                                let dk = match &dp.key {
                                    PropertyKey::StaticIdentifier(id) => id.name.as_str(),
                                    _ => continue,
                                };
                                match dk {
                                    "attribute" => {
                                        if let Expression::StringLiteral(lit) = &dp.value {
                                            prop_cfg.attribute = Some(lit.value.to_string());
                                        }
                                    }
                                    "reflect" => {
                                        if let Expression::BooleanLiteral(lit) = &dp.value {
                                            prop_cfg.reflect = lit.value;
                                        }
                                    }
                                    "type" => {
                                        if let Expression::StringLiteral(lit) = &dp.value {
                                            prop_cfg.prop_type = Some(lit.value.to_string());
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        config.props.push(prop_cfg);
                    }
                }
            }
            "extend" => {
                use oxc_span::GetSpan as _;
                let ext_span = prop.value.span();
                // The expression was parsed directly from source at `offset`, so
                // OXC spans are relative to the expression start. Adjust to absolute.
                config.extend_span = Some(Span::new(
                    ext_span.start + offset,
                    ext_span.end + offset,
                ));
            }
            _ => {}
        }
    }

    config
}

/// Compute render tag argument metadata from parsed CallExpressions.
pub(crate) fn compute_render_tag_args(
    parsed: &ParsedExprs<'_>,
    component: &Component,
    data: &mut AnalysisData,
) {
    walk_render_tags(&component.fragment, parsed, data);
}

fn walk_render_tags(
    fragment: &Fragment,
    parsed: &ParsedExprs<'_>,
    data: &mut AnalysisData,
) {
    for node in &fragment.nodes {
        match node {
            Node::RenderTag(tag) => {
                let offset = tag.expression_span.start;
                if let Some(Expression::CallExpression(call)) = parsed.exprs.get(&offset) {
                    let flags: Vec<bool> = call.arguments.iter().map(|arg| {
                        expression_has_call(arg.to_expression())
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
            Node::Element(el) => {
                walk_render_tags(&el.fragment, parsed, data);
            }
            Node::ComponentNode(cn) => {
                walk_render_tags(&cn.fragment, parsed, data);
            }
            Node::IfBlock(block) => {
                walk_render_tags(&block.consequent, parsed, data);
                if let Some(alt) = &block.alternate {
                    walk_render_tags(alt, parsed, data);
                }
            }
            Node::EachBlock(block) => {
                walk_render_tags(&block.body, parsed, data);
                if let Some(fb) = &block.fallback {
                    walk_render_tags(fb, parsed, data);
                }
            }
            Node::SnippetBlock(block) => {
                walk_render_tags(&block.body, parsed, data);
            }
            Node::KeyBlock(block) => {
                walk_render_tags(&block.fragment, parsed, data);
            }
            Node::AwaitBlock(block) => {
                if let Some(ref p) = block.pending {
                    walk_render_tags(p, parsed, data);
                }
                if let Some(ref t) = block.then {
                    walk_render_tags(t, parsed, data);
                }
                if let Some(ref c) = block.catch {
                    walk_render_tags(c, parsed, data);
                }
            }
            Node::SvelteHead(head) => {
                walk_render_tags(&head.fragment, parsed, data);
            }
            Node::SvelteElement(el) => {
                walk_render_tags(&el.fragment, parsed, data);
            }
            Node::SvelteBoundary(b) => {
                walk_render_tags(&b.fragment, parsed, data);
            }
            _ => {}
        }
    }
}

// ---------------------------------------------------------------------------
// Script-level classification (moved from parser — these are derived flags)
// ---------------------------------------------------------------------------

/// Detect `$effect()`/`$effect.pre()` calls and class fields with `$state()`/`$state.raw()`.
fn detect_script_flags(program: &oxc_ast::ast::Program<'_>) -> (bool, bool) {
    let mut has_effects = false;
    let mut has_class_state_fields = false;
    for stmt in &program.body {
        match stmt {
            oxc_ast::ast::Statement::ExpressionStatement(es) => {
                if is_effect_call(&es.expression) {
                    has_effects = true;
                }
            }
            oxc_ast::ast::Statement::ClassDeclaration(class) => {
                if has_class_state_runes(&class.body) {
                    has_class_state_fields = true;
                }
            }
            _ => {}
        }
    }
    (has_effects, has_class_state_fields)
}

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

fn has_class_state_runes(body: &oxc_ast::ast::ClassBody<'_>) -> bool {
    use crate::script_info::detect_rune;
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

// ---------------------------------------------------------------------------
// Proxy-candidate $state detection
// ---------------------------------------------------------------------------

/// Walk script body to find $state/$state.raw declarations with proxyable init.
/// Stores results in `data.proxy_state_inits` keyed by declaration name.
fn compute_proxy_state_inits(
    program: &oxc_ast::ast::Program<'_>,
    script_info: &ScriptInfo,
    data: &mut AnalysisData,
) {
    for stmt in &program.body {
        let decls = match stmt {
            oxc_ast::ast::Statement::VariableDeclaration(d) => &d.declarations,
            oxc_ast::ast::Statement::ExportNamedDeclaration(e) => {
                if let Some(oxc_ast::ast::Declaration::VariableDeclaration(d)) = &e.declaration {
                    &d.declarations
                } else {
                    continue;
                }
            }
            _ => continue,
        };
        for declarator in decls.iter() {
            let oxc_ast::ast::BindingPattern::BindingIdentifier(ident) = &declarator.id else { continue };
            let Some(init) = &declarator.init else { continue };
            let rune = crate::script_info::detect_rune(init);
            if !matches!(rune, Some(RuneKind::State | RuneKind::StateRaw)) {
                continue;
            }
            // Match declaration name against script_info to confirm it's tracked
            let name = ident.name.as_str();
            if script_info.declarations.iter().any(|d| d.name == name && matches!(d.is_rune, Some(RuneKind::State | RuneKind::StateRaw))) {
                if is_proxyable_state_init(init) {
                    data.proxy_state_inits.insert(CompactString::from(name), true);
                }
            }
        }
    }
}

/// Check if the first argument of a $state/$state.raw call is proxyable (non-primitive).
/// Arrays, objects, and expressions that might produce non-primitive values
/// require $.proxy() wrapping at runtime and remain reactive without reassignment.
fn is_proxyable_state_init(expr: &Expression<'_>) -> bool {
    let Expression::CallExpression(call) = expr else { return false };
    let Some(arg) = call.arguments.first() else { return false };
    let Some(e) = arg.as_expression() else { return false };
    if e.is_literal() {
        return false;
    }
    if matches!(
        e,
        Expression::TemplateLiteral(_)
            | Expression::ArrowFunctionExpression(_)
            | Expression::FunctionExpression(_)
            | Expression::UnaryExpression(_)
            | Expression::BinaryExpression(_)
    ) {
        return false;
    }
    if let Expression::Identifier(id) = e {
        if id.name == "undefined" {
            return false;
        }
    }
    true
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
            Expression::Identifier(id) => crate::script_info::is_rune_name(&id.name),
            Expression::StaticMemberExpression(m) => {
                if let Expression::Identifier(obj) = &m.object {
                    crate::script_info::is_rune_name(&obj.name)
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
        Expression::CallExpression(_) => crate::script_info::detect_rune(expr) == Some(target),
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
