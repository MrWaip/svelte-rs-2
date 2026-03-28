//! JS AST analysis functions moved from `svelte_types`.
//!
//! These functions produce metadata (`ExpressionInfo`, `ScriptInfo`, etc.)
//! from OXC AST nodes. They are internal to the analyze crate.

use crate::types::script::{RuneKind, ScriptInfo};
use compact_str::CompactString;
use oxc_ast::ast::{
    AssignmentTargetPropertyIdentifier, CallExpression, Expression, MemberExpression,
    SimpleAssignmentTarget,
};
use oxc_ast_visit::walk::{
    walk_arrow_function_expression, walk_assignment_expression, walk_call_expression,
    walk_expression, walk_function, walk_member_expression, walk_simple_assignment_target,
    walk_update_expression,
};
use oxc_ast_visit::Visit;
use oxc_semantic::ScopeFlags;
use smallvec::SmallVec;

use svelte_ast::{Component, NodeId};

use crate::types::data::{
    AnalysisData, AwaitBindingInfo, DestructureKind, ExpressionInfo, ExpressionKind, ParserResult,
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
        || NeedsContextVisitor::check(program, sem.semantic.scoping(), &script_info);
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
    let mut ctx = crate::walker::VisitContext::new(root, data, &component.store);
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
        // Callee SymbolId is resolved later in collect_symbols via reference_id
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
pub(crate) struct BindingPreparer;

impl crate::walker::TemplateVisitor for BindingPreparer {
    fn visit_await_block(
        &mut self,
        block: &svelte_ast::AwaitBlock,
        ctx: &mut crate::walker::VisitContext<'_>,
    ) {
        let Some(parsed) = ctx.parsed() else { return };
        if let Some(val_span) = block.value_span {
            if let Some(info) = extract_await_binding_info(parsed, val_span.start) {
                ctx.data.await_bindings.values.insert(block.id, info);
            }
        }
        if let Some(err_span) = block.error_span {
            if let Some(info) = extract_await_binding_info(parsed, err_span.start) {
                ctx.data.await_bindings.errors.insert(block.id, info);
            }
        }
    }
}

/// Extract AwaitBindingInfo from a parsed `let PATTERN = x;` statement.
fn extract_await_binding_info(
    parsed: &ParserResult<'_>,
    offset: u32,
) -> Option<AwaitBindingInfo> {
    use oxc_ast::ast::{BindingPattern, Statement};
    let stmt = parsed.stmts.get(&offset)?;
    let Statement::VariableDeclaration(decl) = stmt else { return None };
    let declarator = decl.declarations.first()?;
    match &declarator.id {
        BindingPattern::BindingIdentifier(ident) => {
            Some(AwaitBindingInfo::Simple(ident.name.to_string()))
        }
        BindingPattern::ObjectPattern(_) => {
            let mut names = Vec::new();
            crate::utils::binding_pattern::collect_binding_names(&declarator.id, &mut names);
            Some(AwaitBindingInfo::Destructured { kind: DestructureKind::Object, names })
        }
        BindingPattern::ArrayPattern(_) => {
            let mut names = Vec::new();
            crate::utils::binding_pattern::collect_binding_names(&declarator.id, &mut names);
            Some(AwaitBindingInfo::Destructured { kind: DestructureKind::Array, names })
        }
        _ => None,
    }
}

/// Extract render tag argument metadata (has_call flags, ident names) from a parsed CallExpression.
pub(crate) fn classify_render_tag_args(
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

        // Arg prop sources resolved later via reference_id in resolve_render_tag_prop_sources
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
                if analyze_expression(&es.expression).has_store_member_mutation {
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

/// OXC Visit that walks the entire script AST to detect expressions requiring
/// component context. Matches reference MemberExpression.js, CallExpression.js,
/// NewExpression.js + is_safe_identifier.
struct NeedsContextVisitor<'a> {
    scoping: &'a oxc_semantic::Scoping,
    /// SymbolIds of prop/rest-prop bindings — unsafe for context purposes
    unsafe_prop_syms: rustc_hash::FxHashSet<oxc_semantic::SymbolId>,
    needs_context: bool,
}

impl<'a> NeedsContextVisitor<'a> {
    /// Walk the entire program to determine if script body needs component context.
    fn check(
        program: &oxc_ast::ast::Program<'a>,
        scoping: &'a oxc_semantic::Scoping,
        script_info: &ScriptInfo,
    ) -> bool {
        let root = scoping.root_scope_id();
        let mut unsafe_prop_syms = rustc_hash::FxHashSet::default();

        // Resolve prop declaration names to SymbolIds
        for d in &script_info.declarations {
            if d.is_rune == Some(RuneKind::Props) {
                if let Some(sym) = scoping.find_binding(root, d.name.as_str().into()) {
                    unsafe_prop_syms.insert(sym);
                }
            }
        }
        // Rest prop bindings are also unsafe (they proxy $$props)
        if let Some(ref decl) = script_info.props_declaration {
            for p in &decl.props {
                if p.is_rest {
                    if let Some(sym) = scoping.find_binding(root, p.local_name.as_str().into()) {
                        unsafe_prop_syms.insert(sym);
                    }
                }
            }
        }

        let mut visitor = Self { scoping, unsafe_prop_syms, needs_context: false };
        visitor.visit_program(program);
        visitor.needs_context
    }

    /// Resolve an identifier reference to its SymbolId via OXC semantic.
    fn resolve_ref(&self, ident: &oxc_ast::ast::IdentifierReference<'_>) -> Option<oxc_semantic::SymbolId> {
        let ref_id = ident.reference_id.get()?;
        self.scoping.get_reference(ref_id).symbol_id()
    }

    /// Check if a root identifier is "safe" (won't trigger context-requiring behavior).
    /// Unsafe: imports, props, rest props. Safe: locals, globals.
    fn is_safe_sym(&self, ident: &oxc_ast::ast::IdentifierReference<'_>) -> bool {
        let Some(sym_id) = self.resolve_ref(ident) else {
            // Unresolved reference = global (Math, console, etc.) — safe
            return true;
        };
        !self.unsafe_prop_syms.contains(&sym_id)
            && !self.scoping.symbol_flags(sym_id).contains(oxc_semantic::SymbolFlags::Import)
    }

    /// Walk a member chain to its root and check if the root identifier is safe.
    fn is_safe_expression_root(&self, expr: &Expression<'_>) -> bool {
        let mut node = expr;
        loop {
            match node {
                Expression::StaticMemberExpression(m) => node = &m.object,
                Expression::ComputedMemberExpression(m) => node = &m.object,
                _ => break,
            }
        }
        match node {
            Expression::Identifier(ident) => self.is_safe_sym(ident),
            _ => false,
        }
    }
}

impl<'a> Visit<'a> for NeedsContextVisitor<'a> {
    fn visit_new_expression(&mut self, _it: &oxc_ast::ast::NewExpression<'a>) {
        self.needs_context = true;
    }

    fn visit_call_expression(&mut self, it: &oxc_ast::ast::CallExpression<'a>) {
        if !self.is_safe_expression_root(&it.callee) {
            self.needs_context = true;
        }
        if !self.needs_context {
            walk_call_expression(self, it);
        }
    }

    fn visit_member_expression(&mut self, it: &MemberExpression<'a>) {
        let obj = match it {
            MemberExpression::StaticMemberExpression(m) => &m.object,
            MemberExpression::ComputedMemberExpression(m) => &m.object,
            _ => {
                walk_member_expression(self, it);
                return;
            }
        };
        if !self.is_safe_expression_root(obj) {
            self.needs_context = true;
        }
        if !self.needs_context {
            walk_member_expression(self, it);
        }
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
                info.ref_symbols.iter().any(|&sym| {
                    data.scoping.is_import(sym)
                        || data.scoping.is_prop_source(sym)
                        || data.scoping.prop_non_source_name(sym).is_some()
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
        return info.has_store_ref || !info.ref_symbols.is_empty();
    }

    if info.has_store_ref {
        return true;
    }
    info.ref_symbols.iter().any(|&sym_id| {
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
        false
    })
}

/// Element attribute dynamicity: non-source props or mutated bindings.
fn is_dynamic_element_attr(
    info: &ExpressionInfo,
    scoping: &crate::scope::ComponentScoping,
) -> bool {
    info.ref_symbols.iter().any(|&sym_id| {
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
    info.ref_symbols.iter().any(|&sym_id| {
        scoping.symbol_scope_id(sym_id) != root || scoping.is_rune(sym_id)
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
/// Collects expression metadata in one walk: kind classification,
/// has_call, has_state_rune, has_store_member_mutation, has_store_ref, has_side_effects.
/// Does NOT collect references — JsMetadataVisitor handles bindings + OXC references.
struct ExpressionAnalyzer {
    kind: ExpressionKind,
    has_call: bool,
    has_await: bool,
    has_state_rune: bool,
    has_store_member_mutation: bool,
    has_store_ref: bool,
    has_side_effects: bool,
    /// Expression nesting depth. 0 = root expression (used for classification).
    depth: u32,
    /// Depth inside function boundaries. When >0, `has_call`, `has_await`, and
    /// `has_state_rune` are not updated (matching Svelte semantics: function
    /// bodies are opaque for call/rune/await detection).
    fn_depth: u32,
    /// Tracks write context for has_store_member_mutation detection.
    in_write_position: bool,
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
        let name = ident.name.as_str();
        if name.starts_with('$') && name.len() > 1 {
            self.has_store_ref = true;
        }
        self.in_write_position = false;
    }

    fn visit_assignment_expression(&mut self, assign: &oxc_ast::ast::AssignmentExpression<'a>) {
        walk_assignment_expression(self, assign);
    }

    fn visit_simple_assignment_target(&mut self, it: &SimpleAssignmentTarget<'a>) {
        self.in_write_position = true;
        walk_simple_assignment_target(self, it);
    }

    fn visit_assignment_target_property_identifier(
        &mut self,
        it: &AssignmentTargetPropertyIdentifier<'a>,
    ) {
        self.in_write_position = true;
        self.visit_identifier_reference(&it.binding);
        if let Some(init) = &it.init {
            self.visit_expression(init);
        }
    }

    fn visit_member_expression(&mut self, expr: &MemberExpression<'a>) {
        if self.in_write_position {
            let root_expr = match expr {
                MemberExpression::StaticMemberExpression(m) => Some(&m.object),
                MemberExpression::ComputedMemberExpression(m) => Some(&m.object),
                _ => None,
            };
            if root_expr.is_some_and(|e| member_root_is_store(e)) {
                self.has_store_member_mutation = true;
            }
        }
        self.in_write_position = false;
        walk_member_expression(self, expr);
    }

    fn visit_update_expression(&mut self, upd: &oxc_ast::ast::UpdateExpression<'a>) {
        self.in_write_position = true;
        walk_update_expression(self, upd);
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

    fn visit_await_expression(&mut self, expr: &oxc_ast::ast::AwaitExpression<'a>) {
        if self.fn_depth == 0 {
            self.has_await = true;
        }
        oxc_ast_visit::walk::walk_await_expression(self, expr);
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
        has_call: false,
        has_await: false,
        has_state_rune: false,
        has_store_member_mutation: false,
        has_store_ref: false,
        has_side_effects: false,
        depth: 0,
        fn_depth: 0,
        in_write_position: false,
    };
    analyzer.visit_expression(expr);
    ExpressionInfo {
        kind: analyzer.kind,
        ref_symbols: SmallVec::new(), // populated later by resolve_references
        has_store_ref: analyzer.has_store_ref,
        has_side_effects: analyzer.has_side_effects,
        has_call: analyzer.has_call,
        has_await: analyzer.has_await,
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

// ---------------------------------------------------------------------------
// Instance body blocker analysis (experimental.async)
// ---------------------------------------------------------------------------

/// Check if a statement contains `await` at the top level (not inside nested functions).
fn has_await_in_statement(stmt: &oxc_ast::ast::Statement<'_>) -> bool {
    struct AwaitCheck {
        found: bool,
        fn_depth: u32,
    }
    impl<'a> Visit<'a> for AwaitCheck {
        fn visit_await_expression(&mut self, _expr: &oxc_ast::ast::AwaitExpression<'a>) {
            if self.fn_depth == 0 {
                self.found = true;
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
    let mut check = AwaitCheck {
        found: false,
        fn_depth: 0,
    };
    check.visit_statement(stmt);
    check.found
}

/// Analyze instance body for async splitting: identify which bindings
/// are written by async statements and assign blocker indices.
///
/// Populates `data.blocker_data` with SymbolId → blocker index mappings.
/// Called from the main analysis pipeline after `collect_symbols`.
pub(crate) fn calculate_instance_blockers(
    parsed: &crate::types::data::ParserResult<'_>,
    data: &mut crate::types::data::AnalysisData,
) {
    let program = match parsed.program.as_ref() {
        Some(p) => p,
        None => return,
    };

    let mut awaited = false;
    let mut async_index: u32 = 0;
    let root = data.scoping.root_scope_id();
    // Non-import statement counter (1:1 with ScriptOutput.body in codegen)
    let mut non_import_idx: usize = 0;

    for stmt in &program.body {
        // Skip imports (not counted)
        if matches!(stmt, oxc_ast::ast::Statement::ImportDeclaration(_)) {
            continue;
        }

        // Unwrap export declarations
        let stmt_ref = if let oxc_ast::ast::Statement::ExportNamedDeclaration(export) = stmt {
            if export.declaration.is_some() {
                stmt // process the export statement itself (has_await looks inside)
            } else {
                non_import_idx += 1;
                continue;
            }
        } else {
            stmt
        };

        let has_await = has_await_in_statement(stmt_ref);
        awaited |= has_await;

        // Track first_await_index
        if awaited && data.blocker_data.first_await_index.is_none() {
            data.blocker_data.first_await_index = Some(non_import_idx);
        }

        // Function declarations are always sync (no blocker assignment, but still build metadata)
        let is_function = matches!(stmt_ref, oxc_ast::ast::Statement::FunctionDeclaration(_))
            || matches!(stmt_ref, oxc_ast::ast::Statement::ExportNamedDeclaration(export)
                if matches!(&export.declaration, Some(oxc_ast::ast::Declaration::FunctionDeclaration(_))));

        if is_function {
            if awaited {
                data.blocker_data.stmt_metas.push(crate::types::data::AsyncStmtMeta {
                    has_await: false,
                    hoist_names: Vec::new(),
                });
            }
            non_import_idx += 1;
            continue;
        }

        // Before first await: everything is sync
        if !awaited {
            non_import_idx += 1;
            continue;
        }

        // --- From here: statement is at/after first await, build metadata ---

        // Collect hoist_names for this statement
        let mut hoist_names = Vec::new();

        // Get the variable declaration (from statement or export)
        let var_decl = match stmt_ref {
            oxc_ast::ast::Statement::VariableDeclaration(v) => Some(&**v),
            oxc_ast::ast::Statement::ExportNamedDeclaration(export) => {
                if let Some(oxc_ast::ast::Declaration::VariableDeclaration(v)) = &export.declaration {
                    Some(&**v)
                } else {
                    None
                }
            }
            _ => None,
        };

        if let Some(var_decl) = var_decl {
            for declarator in &var_decl.declarations {
                // Function-valued initializers are sync — no hoisting, no blocker
                if matches!(
                    &declarator.init,
                    Some(Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_))
                ) {
                    continue;
                }

                let names = collect_binding_names(&declarator.id);
                for name in &names {
                    if let Some(sym) = data.scoping.find_binding(root, name) {
                        data.blocker_data.symbol_blockers.insert(sym, async_index);
                    }
                }
                hoist_names.extend(names);

                async_index += 1;
            }
        } else {
            // Non-variable: class declarations, expression statements, etc.
            if let oxc_ast::ast::Statement::ClassDeclaration(class) = stmt_ref {
                if let Some(ref id) = class.id {
                    let name = id.name.to_string();
                    if let Some(sym) = data.scoping.find_binding(root, &name) {
                        data.blocker_data.symbol_blockers.insert(sym, async_index);
                    }
                    hoist_names.push(name);
                }
            }
            async_index += 1;
        }

        data.blocker_data.stmt_metas.push(crate::types::data::AsyncStmtMeta {
            has_await,
            hoist_names,
        });

        non_import_idx += 1;
    }

    data.blocker_data.async_thunk_count = async_index;
    data.blocker_data.has_async = async_index > 0;
}

/// Collect all binding names from a pattern (for blocker tracking).
fn collect_binding_names(pattern: &oxc_ast::ast::BindingPattern<'_>) -> Vec<String> {
    use oxc_ast::ast::BindingPattern;
    let mut names = Vec::new();
    match pattern {
        BindingPattern::BindingIdentifier(id) => {
            names.push(id.name.to_string());
        }
        BindingPattern::ObjectPattern(obj) => {
            for prop in &obj.properties {
                names.extend(collect_binding_names(&prop.value));
            }
            if let Some(ref rest) = obj.rest {
                names.extend(collect_binding_names(&rest.argument));
            }
        }
        BindingPattern::ArrayPattern(arr) => {
            for elem in arr.elements.iter().flatten() {
                names.extend(collect_binding_names(elem));
            }
            if let Some(ref rest) = arr.rest {
                names.extend(collect_binding_names(&rest.argument));
            }
        }
        BindingPattern::AssignmentPattern(assign) => {
            names.extend(collect_binding_names(&assign.left));
        }
    }
    names
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
