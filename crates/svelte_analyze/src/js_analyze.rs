//! JS AST analysis functions moved from `svelte_types`.
//!
//! These functions produce metadata (`ExpressionInfo`, `ScriptInfo`, etc.)
//! from OXC AST nodes. They are internal to the analyze crate.

use compact_str::CompactString;
use oxc_ast::ast::{AssignmentTarget, CallExpression, Expression, SimpleAssignmentTarget};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{
    walk_arrow_function_expression, walk_call_expression, walk_expression, walk_function,
};
use oxc_semantic::ScopeFlags;
use smallvec::SmallVec;
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

    // Classify script body in a single pass: effects, class state fields,
    // store mutations, proxy state inits.
    let body_flags = analyze_script_body(program, &script_info);
    data.has_store_member_mutations = body_flags.has_store_member_mutations;
    data.proxy_state_inits = body_flags.proxy_state_inits;

    data.exports = std::mem::take(&mut script_info.exports);
    data.needs_context = body_flags.has_effects
        || body_flags.has_class_state_fields
        || script_body_needs_context(program, sem.semantic.scoping(), &script_info);
    data.has_class_state_fields = body_flags.has_class_state_fields;
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
            let config = crate::ce_config::extract_ce_config_from_expr(expr, span.start);
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
            classify_render_tag_args(parsed, data, tag);
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
    data.node_expr_offsets.insert(node_id, offset);
    if let Some(expr) = parsed.exprs.get(&offset) {
        let info = analyze_expression(expr, offset);
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
    data.attr_expr_offsets.insert(attr_id, offset);
    if let Some(expr) = parsed.exprs.get(&offset) {
        let info = analyze_expression(expr, offset);
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
    let mut all_refs = SmallVec::new();
    for part in parts {
        if let ConcatPart::Dynamic(span) = part {
            if let Some(expr) = parsed.exprs.get(&span.start) {
                let info = analyze_expression(expr, span.start);
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
        needs_context: false,
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


/// Extract render tag argument metadata (has_call flags, ident names) from a parsed CallExpression.
fn classify_render_tag_args(
    parsed: &ParsedExprs<'_>,
    data: &mut AnalysisData,
    tag: &svelte_ast::RenderTag,
) {
    let offset = tag.expression_span.start;
    if let Some(Expression::CallExpression(call)) = parsed.exprs.get(&offset) {
        let flags: Vec<bool> = call.arguments.iter().map(|arg| {
            analyze_expression(arg.to_expression(), 0).has_call
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

// ---------------------------------------------------------------------------
// Script body analysis — single-pass OXC Visit over top-level statements
// ---------------------------------------------------------------------------

/// Results from analyzing the script Program body in a single pass.
struct ScriptBodyFlags {
    has_effects: bool,
    has_class_state_fields: bool,
    has_store_member_mutations: bool,
    proxy_state_inits: rustc_hash::FxHashMap<CompactString, bool>,
}

/// Analyze top-level script body for effects, class state fields, store
/// mutations, and proxyable state inits — all in a single walk.
fn analyze_script_body(
    program: &oxc_ast::ast::Program<'_>,
    script_info: &ScriptInfo,
) -> ScriptBodyFlags {
    let mut analyzer = ScriptBodyAnalyzer {
        has_effects: false,
        has_class_state_fields: false,
        has_store_member_mutations: false,
        proxy_state_inits: rustc_hash::FxHashMap::default(),
        script_info,
    };
    analyzer.visit_program(program);
    ScriptBodyFlags {
        has_effects: analyzer.has_effects,
        has_class_state_fields: analyzer.has_class_state_fields,
        has_store_member_mutations: analyzer.has_store_member_mutations,
        proxy_state_inits: analyzer.proxy_state_inits,
    }
}

struct ScriptBodyAnalyzer<'s> {
    has_effects: bool,
    has_class_state_fields: bool,
    has_store_member_mutations: bool,
    proxy_state_inits: rustc_hash::FxHashMap<CompactString, bool>,
    script_info: &'s ScriptInfo,
}

impl<'a> Visit<'a> for ScriptBodyAnalyzer<'_> {
    fn visit_program(&mut self, program: &oxc_ast::ast::Program<'a>) {
        // Intentionally iterate only top-level statements — these flags are
        // about the script module surface, not nested scopes.
        for stmt in &program.body {
            self.visit_statement(stmt);
        }
    }

    fn visit_statement(&mut self, stmt: &oxc_ast::ast::Statement<'a>) {
        use oxc_ast::ast::Statement;
        match stmt {
            Statement::ExpressionStatement(es) => {
                if is_effect_call(&es.expression) {
                    self.has_effects = true;
                }
                if analyze_expression(&es.expression, 0).has_store_member_mutation {
                    self.has_store_member_mutations = true;
                }
            }
            Statement::ClassDeclaration(class) => {
                self.visit_class(class);
            }
            Statement::VariableDeclaration(decl) => {
                self.check_proxy_state_inits(&decl.declarations);
            }
            Statement::ExportNamedDeclaration(export) => {
                if let Some(oxc_ast::ast::Declaration::VariableDeclaration(d)) = &export.declaration {
                    self.check_proxy_state_inits(&d.declarations);
                }
            }
            _ => {}
        }
        // No walk — top-level only
    }

    fn visit_class(&mut self, class: &oxc_ast::ast::Class<'a>) {
        // Walk class body elements using Visit dispatch
        for element in &class.body.body {
            self.visit_class_element(element);
        }
    }

    fn visit_property_definition(&mut self, prop: &oxc_ast::ast::PropertyDefinition<'a>) {
        if let Some(value) = &prop.value {
            if let Some(kind) = crate::script_info::detect_rune(value) {
                if matches!(kind, RuneKind::State | RuneKind::StateRaw) {
                    self.has_class_state_fields = true;
                }
            }
        }
    }

    fn visit_method_definition(&mut self, method: &oxc_ast::ast::MethodDefinition<'a>) {
        if method.kind != oxc_ast::ast::MethodDefinitionKind::Constructor {
            return;
        }
        let Some(body) = &method.value.body else { return };
        for stmt in &body.statements {
            if let oxc_ast::ast::Statement::ExpressionStatement(es) = stmt {
                if let Expression::AssignmentExpression(assign) = &es.expression {
                    if let Some(kind) = crate::script_info::detect_rune(&assign.right) {
                        if matches!(kind, RuneKind::State | RuneKind::StateRaw) {
                            self.has_class_state_fields = true;
                        }
                    }
                }
            }
        }
    }
}

impl ScriptBodyAnalyzer<'_> {
    fn check_proxy_state_inits(&mut self, declarations: &oxc_allocator::Vec<'_, oxc_ast::ast::VariableDeclarator<'_>>) {
        for declarator in declarations.iter() {
            let oxc_ast::ast::BindingPattern::BindingIdentifier(ident) = &declarator.id else { continue };
            let Some(init) = &declarator.init else { continue };
            let rune = crate::script_info::detect_rune(init);
            if !matches!(rune, Some(RuneKind::State | RuneKind::StateRaw)) {
                continue;
            }
            let name = ident.name.as_str();
            if self.script_info.declarations.iter().any(|d| d.name == name && matches!(d.is_rune, Some(RuneKind::State | RuneKind::StateRaw))) {
                if is_proxyable_state_init(init) {
                    self.proxy_state_inits.insert(CompactString::from(name), true);
                }
            }
        }
    }
}

/// Check if an expression is a `$effect()` or `$effect.pre()` call.
// TODO(oxc-visit): shallow callee check — allowed exception
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

/// Check if the first argument of a $state/$state.raw call is proxyable (non-primitive).
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

/// Classify per-expression `needs_context` using ComponentScoping.
/// MemberExpression/CallExpression on imports/props → needs_context.
/// Mirrors reference: MemberExpression.js + CallExpression.js + is_safe_identifier.
pub(crate) fn classify_expression_needs_context(data: &mut AnalysisData) {
    let root = data.scoping.root_scope_id();
    for info in data.expressions.values_mut()
        .chain(data.attr_expressions.values_mut())
    {
        info.needs_context = match &info.kind {
            ExpressionKind::MemberExpression | ExpressionKind::CallExpression { .. } => {
                info.references.iter().any(|r| {
                    data.scoping.find_binding(root, &r.name)
                        .is_some_and(|sym| {
                            data.scoping.is_import(sym)
                                || data.scoping.is_prop_source(sym)
                                || data.scoping.prop_non_source_name(sym).is_some()
                        })
                })
            }
            _ => false,
        };
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
// Expression analysis — OXC Visit-based unified analyzer
// ---------------------------------------------------------------------------

/// Single-pass expression analyzer using OXC Visit infrastructure.
/// Collects all expression metadata in one walk: kind classification,
/// references, has_call, has_state_rune, has_store_mutation, has_side_effects.
struct ExpressionAnalyzer {
    kind: ExpressionKind,
    references: SmallVec<[Reference; 2]>,
    has_call: bool,
    has_state_rune: bool,
    has_store_mutation: bool,
    has_side_effects: bool,
    offset: u32,
    /// Expression nesting depth. 0 = root expression (used for classification).
    depth: u32,
    /// Depth inside function boundaries. When >0, `has_call` and `has_state_rune`
    /// are not updated (matching Svelte semantics: function bodies are opaque
    /// for call/rune detection).
    fn_depth: u32,
}

impl<'a> Visit<'a> for ExpressionAnalyzer {
    fn visit_expression(&mut self, expr: &Expression<'a>) {
        if self.depth == 0 {
            self.kind = match expr {
                Expression::Identifier(ident) => {
                    ExpressionKind::Identifier(CompactString::from(ident.name.as_str()))
                }
                Expression::NumericLiteral(_)
                | Expression::StringLiteral(_)
                | Expression::BooleanLiteral(_)
                | Expression::NullLiteral(_) => ExpressionKind::Literal,
                Expression::CallExpression(call) => {
                    let callee = match &call.callee {
                        Expression::Identifier(ident) => {
                            CompactString::from(ident.name.as_str())
                        }
                        _ => CompactString::default(),
                    };
                    ExpressionKind::CallExpression { callee }
                }
                Expression::StaticMemberExpression(_)
                | Expression::ComputedMemberExpression(_) => ExpressionKind::MemberExpression,
                Expression::ArrowFunctionExpression(_) => ExpressionKind::ArrowFunction,
                Expression::AssignmentExpression(_) => ExpressionKind::Assignment,
                _ => ExpressionKind::Other,
            };
            self.has_side_effects = matches!(
                expr,
                Expression::CallExpression(_)
                    | Expression::AssignmentExpression(_)
                    | Expression::UpdateExpression(_)
            );
        }
        self.depth += 1;
        walk_expression(self, expr);
        self.depth -= 1;
    }

    fn visit_identifier_reference(&mut self, ident: &oxc_ast::ast::IdentifierReference<'a>) {
        self.references.push(Reference {
            name: CompactString::from(ident.name.as_str()),
            span: Span::new(ident.span.start + self.offset, ident.span.end + self.offset),
            flags: ReferenceFlags::Read,
            symbol_id: None,
        });
    }

    fn visit_assignment_expression(&mut self, assign: &oxc_ast::ast::AssignmentExpression<'a>) {
        // LHS: identifier → Write ref; member chain → Read root + store mutation check
        match &assign.left {
            AssignmentTarget::AssignmentTargetIdentifier(ident) => {
                self.references.push(Reference {
                    name: CompactString::from(ident.name.as_str()),
                    span: Span::new(ident.span.start + self.offset, ident.span.end + self.offset),
                    flags: ReferenceFlags::Write,
                    symbol_id: None,
                });
            }
            AssignmentTarget::StaticMemberExpression(m) => {
                if member_root_is_store(&m.object) {
                    self.has_store_mutation = true;
                }
                self.visit_expression(&m.object);
            }
            AssignmentTarget::ComputedMemberExpression(m) => {
                if member_root_is_store(&m.object) {
                    self.has_store_mutation = true;
                }
                self.visit_expression(&m.object);
                self.visit_expression(&m.expression);
            }
            _ => {}
        }
        // RHS: full walk
        self.visit_expression(&assign.right);
    }

    fn visit_update_expression(&mut self, upd: &oxc_ast::ast::UpdateExpression<'a>) {
        match &upd.argument {
            SimpleAssignmentTarget::AssignmentTargetIdentifier(ident) => {
                self.references.push(Reference {
                    name: CompactString::from(ident.name.as_str()),
                    span: Span::new(ident.span.start + self.offset, ident.span.end + self.offset),
                    flags: ReferenceFlags::Write,
                    symbol_id: None,
                });
            }
            SimpleAssignmentTarget::StaticMemberExpression(m) => {
                if member_root_is_store(&m.object) {
                    self.has_store_mutation = true;
                }
                self.visit_expression(&m.object);
            }
            SimpleAssignmentTarget::ComputedMemberExpression(m) => {
                if member_root_is_store(&m.object) {
                    self.has_store_mutation = true;
                }
                self.visit_expression(&m.object);
                self.visit_expression(&m.expression);
            }
            _ => {}
        }
    }

    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        if self.fn_depth == 0 {
            self.has_call = true;
            if let Some(rune) = crate::script_info::detect_rune_from_call(call) {
                if matches!(rune, RuneKind::EffectPending | RuneKind::StateEager) {
                    self.has_state_rune = true;
                }
            }
        }
        walk_call_expression(self, call);
    }

    fn visit_arrow_function_expression(&mut self, arrow: &oxc_ast::ast::ArrowFunctionExpression<'a>) {
        self.fn_depth += 1;
        walk_arrow_function_expression(self, arrow);
        self.fn_depth -= 1;
    }

    fn visit_function(&mut self, func: &oxc_ast::ast::Function<'a>, flags: ScopeFlags) {
        self.fn_depth += 1;
        walk_function(self, func, flags);
        self.fn_depth -= 1;
    }
}

/// Run the unified expression analyzer. Returns all metadata in a single pass.
pub(crate) fn analyze_expression(expr: &Expression<'_>, offset: u32) -> ExpressionInfo {
    let mut analyzer = ExpressionAnalyzer {
        kind: ExpressionKind::Other,
        references: SmallVec::new(),
        has_call: false,
        has_state_rune: false,
        has_store_mutation: false,
        has_side_effects: false,
        offset,
        depth: 0,
        fn_depth: 0,
    };
    analyzer.visit_expression(expr);
    ExpressionInfo {
        kind: analyzer.kind,
        references: analyzer.references,
        has_side_effects: analyzer.has_side_effects,
        has_call: analyzer.has_call,
        has_state_rune: analyzer.has_state_rune,
        has_store_member_mutation: analyzer.has_store_mutation,
        needs_context: false,
    }
}

/// Check if the root of a member expression chain is a $-prefixed identifier.
// TODO(oxc-visit): shallow member-chain walk — allowed exception per CLAUDE.md
fn member_root_is_store(expr: &Expression<'_>) -> bool {
    match expr {
        Expression::Identifier(id) => id.name.starts_with('$') && id.name.len() > 1,
        Expression::StaticMemberExpression(m) => member_root_is_store(&m.object),
        Expression::ComputedMemberExpression(m) => member_root_is_store(&m.object),
        _ => false,
    }
}
