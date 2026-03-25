//! JS AST analysis functions moved from `svelte_types`.
//!
//! These functions produce metadata (`ExpressionInfo`, `ScriptInfo`, etc.)
//! from OXC AST nodes. They are internal to the analyze crate.

use crate::types::script::{RuneKind, ScriptInfo};
use compact_str::CompactString;
use oxc_ast::ast::{AssignmentTarget, CallExpression, Expression, SimpleAssignmentTarget};
use oxc_ast_visit::walk::{
    walk_arrow_function_expression, walk_call_expression, walk_expression, walk_function,
};
use oxc_ast_visit::Visit;
use oxc_semantic::ScopeFlags;
use smallvec::SmallVec;

use svelte_ast::{Component, ConcatPart, NodeId};

use crate::types::data::{
    AnalysisData, AwaitBindingInfo, DestructureKind, ExpressionInfo, ExpressionKind, ParserResult,
    Reference, ReferenceFlags,
};

// ---------------------------------------------------------------------------
// Entry-point functions (called from analyze pipeline)
// ---------------------------------------------------------------------------

/// Enrich pre-extracted ScriptInfo with semantic data and build Scoping.
/// `script_info` comes from `JsParseResult` (extracted by parser).
/// Returns the OXC Scoping for the script block.
pub(crate) fn analyze_script(
    parsed: &ParserResult<'_>,
    data: &mut AnalysisData,
    mut script_info: ScriptInfo,
) -> Option<oxc_semantic::Scoping> {
    let Some(ref program) = parsed.program else {
        return None;
    };

    let sem = oxc_semantic::SemanticBuilder::new().build(program);
    crate::utils::script_info::enrich_from_unresolved(&sem.semantic.scoping(), &mut script_info);

    // Classify script body in a single pass: effects, class state fields,
    // store mutations, proxy state inits.
    let body = analyze_script_body(program, &script_info);
    let has_effects = body.has_effects;
    let has_class_state_fields = body.has_class_state_fields;
    data.has_store_member_mutations = body.has_store_member_mutations;
    data.proxy_state_inits = body.proxy_state_inits;

    data.exports = std::mem::take(&mut script_info.exports);
    data.needs_context = has_effects
        || has_class_state_fields
        || script_body_needs_context(program, sem.semantic.scoping(), &script_info);
    data.has_class_state_fields = has_class_state_fields;
    data.script = Some(script_info);
    Some(sem.semantic.into_scoping())
}

/// Unwrap ChainExpression → CallExpression for render tags and extract callee name.
/// Must run before `extract_all_expressions` because it mutates `parsed.exprs`.
pub(crate) fn classify_render_tags(
    parsed: &mut ParserResult<'_>,
    component: &Component,
    data: &mut AnalysisData,
) {
    let root = data.scoping.root_scope_id();
    let mut visitor = RenderTagClassifier { parsed };
    let mut ctx = crate::walker::VisitContext::new(root, data);
    crate::walker::walk_template(&component.fragment, &mut ctx, &mut [&mut visitor]);
}

struct RenderTagClassifier<'a, 'b> {
    parsed: &'b mut ParserResult<'a>,
}

impl crate::walker::TemplateVisitor for RenderTagClassifier<'_, '_> {
    fn visit_render_tag(
        &mut self,
        tag: &svelte_ast::RenderTag,
        ctx: &mut crate::walker::VisitContext<'_>,
    ) {
        let offset = tag.expression_span.start;
        if matches!(
            self.parsed.exprs.get(&offset),
            Some(Expression::ChainExpression(_))
        ) {
            ctx.data.render_tag_is_chain.insert(tag.id);
            if let Some(Expression::ChainExpression(chain)) = self.parsed.exprs.remove(&offset) {
                if let oxc_ast::ast::ChainElement::CallExpression(call) = chain.unbox().expression {
                    self.parsed
                        .exprs
                        .insert(offset, Expression::CallExpression(call));
                }
            }
        }
        if let Some(Expression::CallExpression(call)) = self.parsed.exprs.get(&offset) {
            if let Expression::Identifier(ident) = &call.callee {
                ctx.data.render_tag_callee_name
                    .insert(tag.id, ident.name.to_string());
            }
        }
    }
}

/// Extract binding metadata from AwaitBlock parsed expressions.
///
/// The parser stores binding patterns as Identifier (simple) or
/// `(PATTERN = 1)` AssignmentExpression (destructured). This pass extracts
/// `AwaitBindingInfo` and removes the binding expression from `parsed.exprs`.
///
/// ConstTag names are handled separately — they come from `JsParseResult.const_tag_names`
/// (extracted during OXC statement parsing to support TS type annotations).
pub(crate) fn prepare_template_bindings(
    parsed: &mut ParserResult<'_>,
    component: &Component,
    data: &mut AnalysisData,
) {
    let root = data.scoping.root_scope_id();
    let mut visitor = BindingPreparer { parsed };
    let mut ctx = crate::walker::VisitContext::new(root, data);
    crate::walker::walk_template(&component.fragment, &mut ctx, &mut [&mut visitor]);
}

struct BindingPreparer<'a, 'b> {
    parsed: &'b mut ParserResult<'a>,
}

impl crate::walker::TemplateVisitor for BindingPreparer<'_, '_> {
    fn visit_await_block(
        &mut self,
        block: &svelte_ast::AwaitBlock,
        ctx: &mut crate::walker::VisitContext<'_>,
    ) {
        if let Some(val_span) = block.value_span {
            if let Some(info) = extract_await_binding_info(self.parsed, val_span.start) {
                ctx.data.await_bindings.values.insert(block.id, info);
            }
        }
        if let Some(err_span) = block.error_span {
            if let Some(info) = extract_await_binding_info(self.parsed, err_span.start) {
                ctx.data.await_bindings.errors.insert(block.id, info);
            }
        }
    }
}

/// Extract binding names from an OXC AssignmentTarget (left side of assignment).
fn extract_names_from_assignment_target(target: &oxc_ast::ast::AssignmentTarget) -> Vec<String> {
    let mut names = Vec::new();
    collect_assignment_target_names(target, &mut names);
    names
}

fn collect_assignment_target_names(
    target: &oxc_ast::ast::AssignmentTarget,
    names: &mut Vec<String>,
) {
    use oxc_ast::ast::{AssignmentTarget, AssignmentTargetProperty};
    match target {
        AssignmentTarget::AssignmentTargetIdentifier(ident) => {
            names.push(ident.name.to_string());
        }
        AssignmentTarget::ObjectAssignmentTarget(obj) => {
            for prop in &obj.properties {
                match prop {
                    AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(p) => {
                        names.push(p.binding.name.to_string());
                    }
                    AssignmentTargetProperty::AssignmentTargetPropertyProperty(p) => {
                        collect_maybe_default_names(&p.binding, names);
                    }
                }
            }
            if let Some(rest) = &obj.rest {
                collect_assignment_target_names(&rest.target, names);
            }
        }
        AssignmentTarget::ArrayAssignmentTarget(arr) => {
            for elem in arr.elements.iter().flatten() {
                collect_maybe_default_names(elem, names);
            }
            if let Some(rest) = &arr.rest {
                collect_assignment_target_names(&rest.target, names);
            }
        }
        _ => {}
    }
}

fn collect_maybe_default_names(
    target: &oxc_ast::ast::AssignmentTargetMaybeDefault,
    names: &mut Vec<String>,
) {
    use oxc_ast::ast::AssignmentTargetMaybeDefault;
    match target {
        AssignmentTargetMaybeDefault::AssignmentTargetWithDefault(d) => {
            collect_assignment_target_names(&d.binding, names);
        }
        _ => {
            if let Some(inner) = target.as_assignment_target() {
                collect_assignment_target_names(inner, names);
            }
        }
    }
}

/// Extract AwaitBindingInfo from a parsed binding expression and remove it from `exprs`.
fn extract_await_binding_info(
    parsed: &mut ParserResult<'_>,
    offset: u32,
) -> Option<AwaitBindingInfo> {
    let expr = parsed.exprs.remove(&offset)?;
    // Unwrap ParenthesizedExpression from `(PATTERN = 1)` wrapping
    let inner = match &expr {
        Expression::ParenthesizedExpression(paren) => &paren.expression,
        other => other,
    };
    match inner {
        Expression::Identifier(ident) => Some(AwaitBindingInfo::Simple(ident.name.to_string())),
        Expression::AssignmentExpression(assign) => {
            use oxc_ast::ast::AssignmentTarget;
            match &assign.left {
                AssignmentTarget::ObjectAssignmentTarget(_) => {
                    let names = extract_names_from_assignment_target(&assign.left);
                    Some(AwaitBindingInfo::Destructured {
                        kind: DestructureKind::Object,
                        names,
                    })
                }
                AssignmentTarget::ArrayAssignmentTarget(_) => {
                    let names = extract_names_from_assignment_target(&assign.left);
                    Some(AwaitBindingInfo::Destructured {
                        kind: DestructureKind::Array,
                        names,
                    })
                }
                _ => None,
            }
        }
        _ => None,
    }
}

/// Extract ExpressionInfo for all parsed template and attribute expressions.
/// Also classifies: expression shorthand, needs_clsx, snippet_param_names,
/// render_tag_args, CE config.
pub(crate) fn extract_all_expressions(
    parsed: &ParserResult<'_>,
    component: &Component,
    data: &mut AnalysisData,
) {
    let root = data.scoping.root_scope_id();
    let mut visitor = ExpressionExtractor {
        pending_render_tag: None,
        pending_shorthand: None,
        pending_clsx: false,
    };
    let mut ctx = crate::walker::VisitContext::with_parsed(root, data, parsed);
    crate::walker::walk_template(&component.fragment, &mut ctx, &mut [&mut visitor]);

    // Extract CE config (not template-related)
    if let Some(svelte_ast::CustomElementConfig::Expression(span)) = component
        .options
        .as_ref()
        .and_then(|o| o.custom_element.as_ref())
    {
        if let Some(expr) = parsed.exprs.get(&span.start) {
            let config = crate::utils::ce_config::extract_ce_config_from_expr(expr, span.start);
            data.ce_config = Some(config);
        }
    }
}

struct ExpressionExtractor {
    pending_render_tag: Option<NodeId>,
    pending_shorthand: Option<(NodeId, String)>,
    pending_clsx: bool,
}

impl crate::walker::TemplateVisitor for ExpressionExtractor {
    // --- Offset storage ---

    fn visit_expression(
        &mut self,
        node_id: svelte_ast::NodeId,
        span: svelte_span::Span,
        ctx: &mut crate::walker::VisitContext<'_>,
    ) {
        if ctx.parent().map_or(false, |p| p.kind.is_attr()) {
            if !ctx.data.attr_expr_offsets.contains_key(node_id) {
                ctx.data.attr_expr_offsets.insert(node_id, span.start);
            }
        } else if !ctx.data.node_expr_offsets.contains_key(node_id) {
            ctx.data.node_expr_offsets.insert(node_id, span.start);
        }
    }

    // --- Parsed expression analysis ---

    fn visit_js_expression(
        &mut self,
        node_id: svelte_ast::NodeId,
        expr: &Expression<'_>,
        ctx: &mut crate::walker::VisitContext<'_>,
    ) {
        // Store ExpressionInfo (replaces insert_node_expr_info / insert_attr_expr_info)
        if ctx.parent().map_or(false, |p| p.kind.is_attr()) {
            if !ctx.data.attr_expressions.contains_key(node_id) {
                ctx.data.attr_expressions.insert(node_id, analyze_expression(expr));
            }
        } else if !ctx.data.expressions.contains_key(node_id) {
            ctx.data.expressions.insert(node_id, analyze_expression(expr));
        }

        // Render tag: classify arguments
        if self.pending_render_tag.take() == Some(node_id) {
            classify_render_tag_args(expr, ctx.data, node_id);
        }

        // Shorthand detection (set by visit_expression_attribute / visit_class_directive / visit_style_directive)
        if let Some((attr_id, name)) = self.pending_shorthand.take() {
            if let Expression::Identifier(ident) = expr {
                if ident.name.as_str() == name {
                    ctx.data.element_flags.expression_shorthand.insert(attr_id);
                }
            }
        }

        // Clsx detection for class={expr}
        if self.pending_clsx {
            self.pending_clsx = false;
            if !matches!(
                expr,
                Expression::StringLiteral(_)
                    | Expression::TemplateLiteral(_)
                    | Expression::BinaryExpression(_)
            ) {
                ctx.data.element_flags.needs_clsx.insert(node_id);
            }
        }
    }

    // --- Hooks that set pending state for visit_js_expression ---

    fn visit_render_tag(
        &mut self,
        tag: &svelte_ast::RenderTag,
        _ctx: &mut crate::walker::VisitContext<'_>,
    ) {
        self.pending_render_tag = Some(tag.id);
    }

    fn visit_const_tag(
        &mut self,
        tag: &svelte_ast::ConstTag,
        ctx: &mut crate::walker::VisitContext<'_>,
    ) {
        // ConstTag is a statement — visit_expression/visit_js_expression don't fire.
        // Store offset so codegen can look up the parsed statement.
        ctx.data.node_expr_offsets.insert(tag.id, tag.expression_span.start);
    }

    fn visit_expression_attribute(
        &mut self,
        attr: &svelte_ast::ExpressionAttribute,
        _ctx: &mut crate::walker::VisitContext<'_>,
    ) {
        self.pending_shorthand = Some((attr.id, attr.name.clone()));
        if attr.name == "class" {
            self.pending_clsx = true;
        }
    }

    fn visit_concatenation_attribute(
        &mut self,
        attr: &svelte_ast::ConcatenationAttribute,
        ctx: &mut crate::walker::VisitContext<'_>,
    ) {
        // Merge all dynamic parts into one ExpressionInfo before per-part visit_js_expression
        if let Some(parsed) = ctx.parsed() {
            insert_concat_expr_info(parsed, ctx.data, attr.id, &attr.parts);
        }
    }

    fn visit_class_directive(
        &mut self,
        dir: &svelte_ast::ClassDirective,
        _ctx: &mut crate::walker::VisitContext<'_>,
    ) {
        // Only non-shorthand directives have an expression to check
        if dir.expression_span.is_some() {
            self.pending_shorthand = Some((dir.id, dir.name.clone()));
        }
    }

    fn visit_style_directive(
        &mut self,
        dir: &svelte_ast::StyleDirective,
        ctx: &mut crate::walker::VisitContext<'_>,
    ) {
        use svelte_ast::StyleDirectiveValue;
        match &dir.value {
            StyleDirectiveValue::Expression(_) => {
                self.pending_shorthand = Some((dir.id, dir.name.clone()));
            }
            StyleDirectiveValue::Concatenation(parts) => {
                if let Some(parsed) = ctx.parsed() {
                    insert_concat_expr_info(parsed, ctx.data, dir.id, parts);
                }
            }
            StyleDirectiveValue::Shorthand | StyleDirectiveValue::String(_) => {}
        }
    }
}

/// Merge ExpressionInfo from all dynamic concatenation parts into a single entry.
fn insert_concat_expr_info(
    parsed: &ParserResult<'_>,
    data: &mut AnalysisData,
    attr_id: NodeId,
    parts: &[ConcatPart],
) {
    let mut all_refs = SmallVec::new();
    for part in parts {
        if let ConcatPart::Dynamic(span) = part {
            if let Some(expr) = parsed.exprs.get(&span.start) {
                let info = analyze_expression(expr);
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
        is_dynamic: false,
        has_state: false,
    };
    data.attr_expressions.insert(attr_id, merged);
}

/// Extract render tag argument metadata (has_call flags, ident names) from a parsed CallExpression.
fn classify_render_tag_args(
    expr: &Expression<'_>,
    data: &mut AnalysisData,
    tag_id: NodeId,
) {
    if let Expression::CallExpression(call) = expr {
        let flags: Vec<bool> = call
            .arguments
            .iter()
            .map(|arg| expression_has_call(arg.to_expression()))
            .collect();
        data.render_tag_arg_has_call.insert(tag_id, flags);

        let idents: Vec<Option<String>> = call
            .arguments
            .iter()
            .map(|arg| {
                if let Expression::Identifier(id) = arg.to_expression() {
                    Some(id.name.to_string())
                } else {
                    None
                }
            })
            .collect();
        data.render_tag_arg_idents.insert(tag_id, idents);
    }
}

// ---------------------------------------------------------------------------
// Script body analysis — single-pass OXC Visit over top-level statements
// ---------------------------------------------------------------------------

/// Analyze top-level script body for effects, class state fields, store
/// mutations, and proxyable state inits — all in a single walk.
fn analyze_script_body<'s>(
    program: &oxc_ast::ast::Program<'_>,
    script_info: &'s ScriptInfo,
) -> ScriptBodyAnalyzer<'s> {
    let mut analyzer = ScriptBodyAnalyzer {
        has_effects: false,
        has_class_state_fields: false,
        has_store_member_mutations: false,
        proxy_state_inits: rustc_hash::FxHashMap::default(),
        script_info,
    };
    analyzer.visit_program(program);
    analyzer
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
                if has_store_member_mutation(&es.expression) {
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
                if let Some(oxc_ast::ast::Declaration::VariableDeclaration(d)) = &export.declaration
                {
                    self.check_proxy_state_inits(&d.declarations);
                }
            }
            _ => {}
        }
        // No walk — top-level only
    }

    fn visit_class(&mut self, class: &oxc_ast::ast::Class<'a>) {
        for element in &class.body.body {
            self.visit_class_element(element);
        }
    }

    fn visit_property_definition(&mut self, prop: &oxc_ast::ast::PropertyDefinition<'a>) {
        if let Some(value) = &prop.value {
            if let Some(kind) = crate::utils::script_info::detect_rune(value) {
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
        let Some(body) = &method.value.body else {
            return;
        };
        for stmt in &body.statements {
            if let oxc_ast::ast::Statement::ExpressionStatement(es) = stmt {
                if let Expression::AssignmentExpression(assign) = &es.expression {
                    if let Some(kind) = crate::utils::script_info::detect_rune(&assign.right) {
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
    fn check_proxy_state_inits(
        &mut self,
        declarations: &oxc_allocator::Vec<'_, oxc_ast::ast::VariableDeclarator<'_>>,
    ) {
        for declarator in declarations.iter() {
            let oxc_ast::ast::BindingPattern::BindingIdentifier(ident) = &declarator.id else {
                continue;
            };
            let Some(init) = &declarator.init else {
                continue;
            };
            let rune = crate::utils::script_info::detect_rune(init);
            if !matches!(rune, Some(RuneKind::State | RuneKind::StateRaw)) {
                continue;
            }
            let name = ident.name.as_str();
            if self.script_info.declarations.iter().any(|d| {
                d.name == name && matches!(d.is_rune, Some(RuneKind::State | RuneKind::StateRaw))
            }) {
                if is_proxyable_state_init(init) {
                    self.proxy_state_inits
                        .insert(CompactString::from(name), true);
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
    let Expression::CallExpression(call) = expr else {
        return false;
    };
    let Some(arg) = call.arguments.first() else {
        return false;
    };
    let Some(e) = arg.as_expression() else {
        return false;
    };
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
        Expression::CallExpression(call) => !is_safe_identifier(&call.callee, scoping, prop_names),
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

    let Expression::Identifier(ident) = node else {
        return false;
    };
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
    for info in data
        .expressions
        .values_mut()
        .chain(data.attr_expressions.values_mut())
    {
        info.needs_context = match &info.kind {
            ExpressionKind::MemberExpression | ExpressionKind::CallExpression { .. } => {
                info.references.iter().any(|r| {
                    data.scoping.find_binding(root, &r.name).is_some_and(|sym| {
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

/// Classify `is_dynamic` and `has_state` for all expressions.
/// Must run after `resolve_references` and `precompute_dynamic_cache`.
pub(crate) fn classify_expression_dynamicity(data: &mut AnalysisData) {
    let root = data.scoping.root_scope_id();
    let has_class_state = data.has_class_state_fields;

    for info in data.expressions.values_mut() {
        info.is_dynamic = is_dynamic_template(info, &data.scoping, root, has_class_state);
        info.has_state = info.is_dynamic;
    }

    for info in data.attr_expressions.values_mut() {
        info.is_dynamic = is_dynamic_element_attr(info, &data.scoping);
        info.has_state = has_state_component_attr(info, &data.scoping, root);
    }
}

/// Template expression dynamicity: state runes, stores, dynamic bindings, class state fields.
fn is_dynamic_template(
    info: &ExpressionInfo,
    scoping: &crate::scope::ComponentScoping,
    root: oxc_semantic::ScopeId,
    has_class_state_fields: bool,
) -> bool {
    if info.has_state_rune || info.needs_context {
        return true;
    }

    // MemberExpressions: any resolved local binding → dynamic.
    if matches!(info.kind, ExpressionKind::MemberExpression) {
        return info.references.iter().any(|r| {
            scoping.is_store_ref(&r.name) || r.symbol_id.is_some()
        });
    }

    info.references.iter().any(|r| {
        if scoping.is_store_ref(&r.name) {
            return true;
        }
        if let Some(sym_id) = r.symbol_id {
            if scoping.is_dynamic_by_id(sym_id) {
                return true;
            }
            // When class state fields exist, member access on local bindings
            // is potentially reactive (getters call $.get internally).
            if has_class_state_fields
                && scoping.symbol_scope_id(sym_id) == root
                && !scoping.is_rune(sym_id)
            {
                return true;
            }
        }
        false
    })
}

/// Element attribute dynamicity: non-source props or mutated bindings.
fn is_dynamic_element_attr(
    info: &ExpressionInfo,
    scoping: &crate::scope::ComponentScoping,
) -> bool {
    info.references.iter().any(|r| {
        let Some(sym_id) = r.symbol_id else { return false };
        scoping.prop_non_source_name(sym_id).is_some() || scoping.is_dynamic_by_id(sym_id)
    })
}

/// Component/boundary attribute dynamicity (Svelte's `has_state` semantics):
/// any reference to a rune or non-root-scope binding.
fn has_state_component_attr(
    info: &ExpressionInfo,
    scoping: &crate::scope::ComponentScoping,
    root: oxc_semantic::ScopeId,
) -> bool {
    info.references.iter().any(|r| {
        if let Some(sym_id) = r.symbol_id {
            scoping.symbol_scope_id(sym_id) != root || scoping.is_rune(sym_id)
        } else {
            false
        }
    })
}

/// Unwrap a rune call to get its first argument expression.
/// E.g., `$derived(expr)` → `expr`, `$state(expr)` → `expr`.
/// Non-rune expressions pass through unchanged.
fn unwrap_rune_arg<'a>(expr: &'a Expression<'a>) -> &'a Expression<'a> {
    if let Expression::CallExpression(call) = expr {
        let is_rune = match &call.callee {
            Expression::Identifier(id) => crate::utils::script_info::is_rune_name(&id.name),
            Expression::StaticMemberExpression(m) => {
                if let Expression::Identifier(obj) = &m.object {
                    crate::utils::script_info::is_rune_name(&obj.name)
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
/// references, has_call, has_state_rune, has_store_member_mutation, has_side_effects.
struct ExpressionAnalyzer {
    kind: ExpressionKind,
    references: SmallVec<[Reference; 2]>,
    has_call: bool,
    has_state_rune: bool,
    has_store_member_mutation: bool,
    has_side_effects: bool,
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
                    flags: ReferenceFlags::Write,
                    symbol_id: None,
                });
            }
            AssignmentTarget::StaticMemberExpression(m) => {
                if member_root_is_store(&m.object) {
                    self.has_store_member_mutation = true;
                }
                self.visit_expression(&m.object);
            }
            AssignmentTarget::ComputedMemberExpression(m) => {
                if member_root_is_store(&m.object) {
                    self.has_store_member_mutation = true;
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
                    flags: ReferenceFlags::Write,
                    symbol_id: None,
                });
            }
            SimpleAssignmentTarget::StaticMemberExpression(m) => {
                if member_root_is_store(&m.object) {
                    self.has_store_member_mutation = true;
                }
                self.visit_expression(&m.object);
            }
            SimpleAssignmentTarget::ComputedMemberExpression(m) => {
                if member_root_is_store(&m.object) {
                    self.has_store_member_mutation = true;
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
            if let Some(rune) = crate::utils::script_info::detect_rune_from_call(call) {
                if matches!(rune, RuneKind::EffectPending | RuneKind::StateEager) {
                    self.has_state_rune = true;
                }
            }
        }
        walk_call_expression(self, call);
    }

    fn visit_arrow_function_expression(
        &mut self,
        arrow: &oxc_ast::ast::ArrowFunctionExpression<'a>,
    ) {
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
pub(crate) fn analyze_expression(expr: &Expression<'_>) -> ExpressionInfo {
    let mut analyzer = ExpressionAnalyzer {
        kind: ExpressionKind::Other,
        references: SmallVec::new(),
        has_call: false,
        has_state_rune: false,
        has_store_member_mutation: false,
        has_side_effects: false,
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
        has_store_member_mutation: analyzer.has_store_member_mutation,
        needs_context: false,
        is_dynamic: false,
        has_state: false,
    }
}

/// Lightweight check: does the expression contain a CallExpression?
/// Stops at function boundaries (arrow/function expressions are opaque).
fn expression_has_call(expr: &Expression<'_>) -> bool {
    struct HasCallCheck {
        found: bool,
        fn_depth: u32,
    }
    impl<'a> Visit<'a> for HasCallCheck {
        fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
            if self.fn_depth == 0 {
                self.found = true;
            }
            if !self.found {
                walk_call_expression(self, call);
            }
        }
        fn visit_arrow_function_expression(
            &mut self,
            arrow: &oxc_ast::ast::ArrowFunctionExpression<'a>,
        ) {
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
    let mut check = HasCallCheck {
        found: false,
        fn_depth: 0,
    };
    check.visit_expression(expr);
    check.found
}

/// Lightweight check for store member mutations (e.g. `$store.field = x`).
/// Uses a dedicated visitor instead of the full ExpressionAnalyzer.
fn has_store_member_mutation(expr: &Expression<'_>) -> bool {
    struct StoreMutationCheck(bool);
    impl<'a> Visit<'a> for StoreMutationCheck {
        fn visit_assignment_expression(&mut self, assign: &oxc_ast::ast::AssignmentExpression<'a>) {
            match &assign.left {
                AssignmentTarget::StaticMemberExpression(m) if member_root_is_store(&m.object) => {
                    self.0 = true
                }
                AssignmentTarget::ComputedMemberExpression(m)
                    if member_root_is_store(&m.object) =>
                {
                    self.0 = true
                }
                _ => {}
            }
            if !self.0 {
                self.visit_expression(&assign.right);
            }
        }
        fn visit_update_expression(&mut self, upd: &oxc_ast::ast::UpdateExpression<'a>) {
            match &upd.argument {
                SimpleAssignmentTarget::StaticMemberExpression(m)
                    if member_root_is_store(&m.object) =>
                {
                    self.0 = true
                }
                SimpleAssignmentTarget::ComputedMemberExpression(m)
                    if member_root_is_store(&m.object) =>
                {
                    self.0 = true
                }
                _ => {}
            }
        }
    }
    let mut check = StoreMutationCheck(false);
    check.visit_expression(expr);
    check.0
}

/// Check if the root of a member expression chain is a $-prefixed identifier.
fn member_root_is_store(expr: &Expression<'_>) -> bool {
    let mut node = expr;
    loop {
        match node {
            Expression::StaticMemberExpression(m) => node = &m.object,
            Expression::ComputedMemberExpression(m) => node = &m.object,
            _ => break,
        }
    }
    if let Expression::Identifier(id) = node {
        id.name.starts_with('$') && id.name.len() > 1
    } else {
        false
    }
}
