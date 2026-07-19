mod const_tag_order_legacy;
mod contextual;

mod import_subscribed;
mod legacy;
mod legacy_reactive;
mod references;
mod store;
mod util;

pub(crate) use util::expression_root_reference_id;

use util::{property_key_atom, simple_assignment_target_member_root_reference_id};

use super::data::{
    BindingFacts, ClassFieldDerivedSemantics, ClassFieldSemantics, ClassFieldStateSemantics,
    DeclaratorSemantics, DerivedAsyncKind, DerivedDeclarationSemantics, DerivedKind, DerivedSource,
    OptimizedRuneSemantics, PropBindingKind, PropBindingSemantics, PropDefaultKind, PropEmitMode,
    ReactivitySemantics, ReferenceFacts, RuntimeRuneKind, StateDeclarationSemantics, StateKind,
};
use super::legacy_reactive::LegacyReactiveDep;
use crate::scope::{ComponentScoping, SymbolId};
use crate::types::data::{AnalysisData, JsAst};
use crate::utils::expression_has_await;
use crate::utils::is_let_or_var;
use crate::value_evaluation::{Evaluation, ValueEvaluation};
use oxc_ast::ast::{
    ArrowFunctionExpression, AssignmentExpression, AssignmentOperator, AssignmentPattern,
    AssignmentTarget, BindingPattern, CallExpression, Class, ClassElement, ExportNamedDeclaration,
    Expression, Function, IdentifierReference, ImportDeclarationSpecifier, MemberExpression,
    MethodDefinition, MethodDefinitionKind, NewExpression, PrivateFieldExpression, Program,
    PropertyDefinition, PropertyKey, Statement, StaticMemberExpression, TaggedTemplateExpression,
    UpdateExpression, VariableDeclaration, VariableDeclarationKind, VariableDeclarator,
};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{
    walk_arrow_function_expression, walk_assignment_expression, walk_assignment_pattern,
    walk_call_expression, walk_class, walk_export_named_declaration, walk_function,
    walk_member_expression, walk_method_definition, walk_new_expression,
    walk_private_field_expression, walk_program, walk_property_definition,
    walk_static_member_expression, walk_tagged_template_expression, walk_update_expression,
    walk_variable_declaration, walk_variable_declarator,
};
use oxc_syntax::operator::BinaryOperator;
use oxc_syntax::scope::ScopeFlags;
use std::mem;

use oxc_span::GetSpan as _;
use oxc_span::Ident;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use svelte_ast::Component;
use svelte_component_semantics::{
    ComponentSemantics, OxcNodeId, ReferenceId, WriteTarget, sym_state, walk_assignment_targets,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RuneKind {
    State,
    StateRaw,
    Derived,
    DerivedBy,
    Effect,
    EffectPre,
    EffectRoot,
    EffectTracking,
    Props,
    Bindable,
    StateEager,
    EffectPending,
    Inspect,
    InspectWith,
    InspectTrace,
    Host,
    PropsId,
    StateSnapshot,
}

impl RuneKind {
    pub(crate) fn is_bare_value_rune(self) -> bool {
        match self {
            RuneKind::State | RuneKind::Derived | RuneKind::Props => true,
            RuneKind::StateRaw
            | RuneKind::DerivedBy
            | RuneKind::Effect
            | RuneKind::EffectPre
            | RuneKind::EffectRoot
            | RuneKind::EffectTracking
            | RuneKind::EffectPending
            | RuneKind::Bindable
            | RuneKind::StateEager
            | RuneKind::Inspect
            | RuneKind::InspectWith
            | RuneKind::InspectTrace
            | RuneKind::Host
            | RuneKind::PropsId
            | RuneKind::StateSnapshot => false,
        }
    }
}

const STATE_RUNE_NAME: &str = "$state";
const DERIVED_RUNE_NAME: &str = "$derived";
const EFFECT_RUNE_NAME: &str = "$effect";
const PROPS_RUNE_NAME: &str = "$props";
const BINDABLE_RUNE_NAME: &str = "$bindable";
const INSPECT_RUNE_NAME: &str = "$inspect";
const HOST_RUNE_NAME: &str = "$host";

fn assignment_target_field_access(target: &AssignmentTarget<'_>) -> Option<OxcNodeId> {
    match target {
        AssignmentTarget::PrivateFieldExpression(pfe) => Some(pfe.node_id()),
        AssignmentTarget::StaticMemberExpression(member) => Some(member.node_id()),
        _ => None,
    }
}

pub(crate) fn detect_rune_from_call(call: &CallExpression<'_>) -> Option<RuneKind> {
    match &call.callee {
        Expression::Identifier(ident) => match ident.name.as_str() {
            STATE_RUNE_NAME => Some(RuneKind::State),
            DERIVED_RUNE_NAME => Some(RuneKind::Derived),
            EFFECT_RUNE_NAME => Some(RuneKind::Effect),
            PROPS_RUNE_NAME => Some(RuneKind::Props),
            BINDABLE_RUNE_NAME => Some(RuneKind::Bindable),
            INSPECT_RUNE_NAME => Some(RuneKind::Inspect),
            HOST_RUNE_NAME => Some(RuneKind::Host),
            _ => None,
        },
        Expression::StaticMemberExpression(member) => {
            if let Expression::Identifier(obj) = &member.object {
                let prop = member.property.name.as_str();
                match (obj.name.as_str(), prop) {
                    (DERIVED_RUNE_NAME, "by") => Some(RuneKind::DerivedBy),
                    (STATE_RUNE_NAME, "raw") => Some(RuneKind::StateRaw),
                    (STATE_RUNE_NAME, "eager") => Some(RuneKind::StateEager),
                    (STATE_RUNE_NAME, "snapshot") => Some(RuneKind::StateSnapshot),
                    (EFFECT_RUNE_NAME, "pre") => Some(RuneKind::EffectPre),
                    (EFFECT_RUNE_NAME, "root") => Some(RuneKind::EffectRoot),
                    (EFFECT_RUNE_NAME, "tracking") => Some(RuneKind::EffectTracking),
                    (EFFECT_RUNE_NAME, "pending") => Some(RuneKind::EffectPending),
                    (PROPS_RUNE_NAME, "id") => Some(RuneKind::PropsId),
                    (INSPECT_RUNE_NAME, "trace") => Some(RuneKind::InspectTrace),
                    _ => None,
                }
            } else if member.property.name == "with" {
                if let Expression::CallExpression(inner) = &member.object
                    && let Expression::Identifier(id) = &inner.callee
                    && id.name == INSPECT_RUNE_NAME
                {
                    return Some(RuneKind::InspectWith);
                }
                None
            } else {
                None
            }
        }
        _ => None,
    }
}

fn find_svelte_store_rune_import(
    parsed: &JsAst<'_>,
    scoping: &ComponentScoping,
) -> Option<SymbolId> {
    let base = &DERIVED_RUNE_NAME[1..];
    for program in [parsed.module_program.as_ref(), parsed.program.as_ref()]
        .into_iter()
        .flatten()
    {
        for stmt in &program.body {
            let Statement::ImportDeclaration(import) = stmt else {
                continue;
            };
            if import.import_kind.is_type()
                || import.source.value.as_str() != super::SVELTE_STORE_MODULE
            {
                continue;
            }
            for specifier in import.specifiers.iter().flatten() {
                let ImportDeclarationSpecifier::ImportSpecifier(named) = specifier else {
                    continue;
                };
                if named.local.name.as_str() == base {
                    return scoping.find_binding(scoping.root_scope_id(), base);
                }
            }
        }
    }
    None
}

const JS_UNDEFINED_NAME: &str = "undefined";

pub(crate) struct ReactivityInputs {
    pub inline_runes: Option<bool>,
    pub compile_runes: svelte_ast::RunesOption,
    pub immutable: bool,
    pub accessors: bool,
}

pub(crate) fn build_v2<'a>(
    component: &Component,
    parsed: &JsAst<'a>,
    data: &mut AnalysisData<'a>,
    inputs: ReactivityInputs,
) {
    let svelte_store_rune_import = find_svelte_store_rune_import(parsed, &data.scoping);
    let runes_mode = super::mode_resolution::resolve(
        &data.scoping,
        parsed,
        inputs.inline_runes,
        inputs.compile_runes,
        svelte_store_rune_import,
    );
    data.script.runes_mode = runes_mode;
    let runes = runes_mode.is_runes();
    data.script.immutable = runes || inputs.immutable;
    data.script.accessors = data.output.is_custom_element_target || (!runes && inputs.accessors);

    data.reactivity.set_uses_runes(runes);
    data.reactivity.set_runes_mode(runes_mode);
    data.reactivity
        .set_svelte_store_rune_import(svelte_store_rune_import);
    record_maybe_reactive_imports(data);
    let lr_collected = build_script_semantics_v2(
        component,
        parsed,
        data,
        component_prop_emit_mode(data.output.is_custom_element_target),
    );
    contextual::collect_template_declarations(component, parsed, data);
    contextual::promote_each_sources_to_legacy_state(component, parsed, data);

    legacy_reactive::build_from_collected(
        data,
        lr_collected.labeled_nodes,
        lr_collected.implicit_names,
    );
    promote_each_sources_transitive_legacy(
        &mut data.reactivity,
        &data.scoping,
        data.script.immutable,
    );
    promote_legacy_exported_const_state(
        &mut data.reactivity,
        &data.scoping,
        data.scoping.semantics(),
        data.script.immutable,
    );

    finalize_legacy_signal_sources(data);

    data.reactivity.consolidate_legacy_state_declarators();

    let reference_count = data.scoping.references_len();
    data.reactivity.reserve_references(reference_count);
    let bind_this_proxy_targets = references::collect_raw_param_reads(component, parsed, data);
    references::collect_symbol_semantics(data);
    references::apply_bind_this_proxy_targets(data, &bind_this_proxy_targets);
    data.reactivity.classify_derived_sources();
    const_tag_order_legacy::build(component, parsed, data);
    record_const_tag_declarators(component, parsed, data);
    optimize_const_and_declaration_tags(component, parsed, data);

    legacy::register_legacy_synthetic_objects(data);
    legacy::finalize_legacy_aggregates(data);
    legacy_reactive::classify_mutated_import_references(data);
    import_subscribed::classify_import_subscribed_reads(data);
}

pub(crate) fn finalize_reactivity(
    parsed: &JsAst<'_>,
    reactivity: &mut ReactivitySemantics,
    value_evaluation: &ValueEvaluation,
    semantics: &ComponentSemantics<'_>,
    dev: bool,
) {
    optimize_derived(reactivity, value_evaluation, semantics);
    finalize_proxy(parsed, reactivity, semantics, dev);
}

fn promote_legacy_exported_const_state(
    reactivity: &mut ReactivitySemantics,
    scoping: &ComponentScoping<'_>,
    semantics: &ComponentSemantics<'_>,
    immutable: bool,
) {
    use super::data::LegacyStateSemantics;

    if reactivity.uses_runes() {
        return;
    }

    let mut promotable: Vec<SymbolId> = Vec::new();
    for sym in semantics.symbol_ids() {
        if !reactivity.binding_semantics(sym).is_legacy_api_export() {
            continue;
        }
        if reactivity.store_shadow_of_internal(sym).is_some() {
            continue;
        }
        if !scoping.is_member_mutated(sym) && !scoping.is_mutated_any(sym) {
            continue;
        }
        if !legacy_export_has_template_reference(scoping, sym) {
            continue;
        }
        promotable.push(sym);
    }
    if promotable.is_empty() {
        return;
    }

    reactivity.promote_legacy_api_export_to_state(
        &promotable,
        LegacyStateSemantics {
            var_declared: false,
            immutable,
            is_signal_source: false,
        },
    );
}

fn finalize_legacy_signal_sources(data: &mut AnalysisData<'_>) {
    use super::data::BindingSemantics;

    let symbols: Vec<SymbolId> = data.scoping.semantics().symbol_ids().collect();
    for sym in symbols {
        if !matches!(
            data.reactivity.binding_semantics(sym),
            BindingSemantics::LegacyState(_)
        ) {
            continue;
        }
        let is_signal_source = data.script.is_state_source(
            data.scoping.is_mutated(sym) || data.scoping.is_reexported_specifier_local(sym),
        );
        data.reactivity
            .set_legacy_state_signal_source(sym, is_signal_source);
    }
}

fn legacy_export_has_template_reference(scoping: &ComponentScoping<'_>, sym: SymbolId) -> bool {
    for &ref_id in scoping.get_resolved_reference_ids(sym) {
        if scoping.is_template_reference(ref_id) {
            return true;
        }
    }
    false
}

fn promote_each_sources_transitive_legacy(
    reactivity: &mut ReactivitySemantics,
    scoping: &ComponentScoping<'_>,
    immutable: bool,
) {
    use super::data::LegacyStateSemantics;

    if reactivity.uses_runes() {
        return;
    }

    let mut reads_by_target: FxHashMap<SymbolId, SmallVec<[SymbolId; 8]>> = FxHashMap::default();
    let mut statements: Vec<(OxcNodeId, SmallVec<[LegacyReactiveDep; 8]>)> = Vec::new();
    for statement in reactivity.legacy_reactive().iter_statements_topo() {
        statements.push((statement.stmt_node, statement.structural_reads.clone()));
        for &target in &statement.assignments {
            let reads = reads_by_target.entry(target).or_default();
            for read in &statement.structural_reads {
                let LegacyReactiveDep::Binding(read) = read else {
                    continue;
                };
                if !reads.contains(read) {
                    reads.push(*read);
                }
            }
        }
    }
    if reads_by_target.is_empty() {
        return;
    }

    let each_sources: Vec<(SymbolId, Vec<SymbolId>)> = reactivity
        .iter_each_item_indirect_sources()
        .map(|(item_sym, sources)| (item_sym, sources.to_vec()))
        .collect();
    if each_sources.is_empty() {
        return;
    }

    let mut additions: Vec<(SymbolId, Vec<SymbolId>)> = Vec::new();
    let mut promotions: Vec<SymbolId> = Vec::new();
    let mut seen_promotion: FxHashSet<SymbolId> = FxHashSet::default();
    for (item_sym, direct_sources) in &each_sources {
        let mut reached: Vec<SymbolId> = Vec::new();
        let mut seen: FxHashSet<SymbolId> = FxHashSet::default();
        for &source in direct_sources {
            collect_transitive_legacy_reads(source, &reads_by_target, &mut seen, &mut reached);
        }
        for &sym in &reached {
            if !is_promotable_legacy_let(reactivity, scoping, sym) {
                continue;
            }
            if seen_promotion.insert(sym) {
                promotions.push(sym);
            }
        }
        additions.push((*item_sym, reached));
    }

    for &sym in &promotions {
        reactivity.record_legacy_state_binding(
            sym,
            LegacyStateSemantics {
                var_declared: false,
                immutable,
                is_signal_source: false,
            },
        );
    }
    for (item_sym, reached) in &additions {
        for &sym in reached {
            if legacy_reactive::is_reactive_legacy_dep(reactivity.binding_semantics(sym)) {
                reactivity.add_each_item_indirect_source(*item_sym, sym);
            }
        }
    }

    if promotions.is_empty() {
        return;
    }

    let mut rederived: Vec<(OxcNodeId, SmallVec<[LegacyReactiveDep; 8]>)> =
        Vec::with_capacity(statements.len());
    for (stmt_node, structural_reads) in &statements {
        let mut dependencies: SmallVec<[LegacyReactiveDep; 8]> = SmallVec::new();
        for read in structural_reads {
            match read {
                LegacyReactiveDep::Binding(sym) => {
                    if legacy_reactive::is_reactive_legacy_dep(reactivity.binding_semantics(*sym)) {
                        dependencies.push(*read);
                    }
                }
                LegacyReactiveDep::PropsObject | LegacyReactiveDep::RestPropsObject => {
                    dependencies.push(*read);
                }
            }
        }
        rederived.push((*stmt_node, dependencies));
    }
    for (stmt_node, dependencies) in rederived {
        reactivity
            .legacy_reactive_mut()
            .set_statement_dependencies(stmt_node, dependencies);
    }
}

fn collect_transitive_legacy_reads(
    sym: SymbolId,
    reads_by_target: &FxHashMap<SymbolId, SmallVec<[SymbolId; 8]>>,
    seen: &mut FxHashSet<SymbolId>,
    out: &mut Vec<SymbolId>,
) {
    if !seen.insert(sym) {
        return;
    }
    out.push(sym);
    let Some(reads) = reads_by_target.get(&sym) else {
        return;
    };
    for &read in reads {
        collect_transitive_legacy_reads(read, reads_by_target, seen, out);
    }
}

fn is_promotable_legacy_let(
    reactivity: &ReactivitySemantics,
    scoping: &ComponentScoping<'_>,
    sym: SymbolId,
) -> bool {
    use super::data::BindingSemantics;
    let is_plain = match reactivity.binding_semantics(sym) {
        BindingSemantics::NonReactive => true,
        BindingSemantics::MaybeReactive
        | BindingSemantics::State(_)
        | BindingSemantics::Derived(_)
        | BindingSemantics::OptimizedDerived(_)
        | BindingSemantics::OptimizedRune(_)
        | BindingSemantics::Prop(_)
        | BindingSemantics::LegacyBindableProp(_)
        | BindingSemantics::LegacyState(_)
        | BindingSemantics::Store(_)
        | BindingSemantics::Const(_)
        | BindingSemantics::OptimizedConst(_)
        | BindingSemantics::DeclarationTag
        | BindingSemantics::OptimizedDeclarationTag
        | BindingSemantics::Contextual(_)
        | BindingSemantics::RuntimeRune { .. }
        | BindingSemantics::Unresolved
        | BindingSemantics::LegacyApiExport => false,
    };
    if !is_plain {
        return false;
    }
    if !scoping.is_component_top_level_symbol(sym) {
        return false;
    }
    if !scoping.is_reassignable_declaration(sym) {
        return false;
    }
    if symbol_is_function_declaration(scoping, sym) {
        return false;
    }
    reactivity.store_shadow_of_internal(sym).is_none()
}

pub(super) fn symbol_is_function_declaration(
    scoping: &ComponentScoping<'_>,
    sym: SymbolId,
) -> bool {
    use oxc_ast::AstKind;
    let Some(parent) = scoping.js_parent_id(scoping.symbol_declaration(sym)) else {
        return false;
    };
    matches!(scoping.js_kind(parent), Some(AstKind::Function(_)))
}

fn optimize_derived(
    reactivity: &mut ReactivitySemantics,
    value_evaluation: &ValueEvaluation,
    semantics: &ComponentSemantics<'_>,
) {
    let mut optimizable = Vec::new();

    for symbol in semantics.symbol_ids() {
        if !reactivity.binding_semantics(symbol).is_derived() {
            continue;
        }
        if !matches!(value_evaluation.evaluation(symbol), Evaluation::Known(_)) {
            continue;
        }
        optimizable.push(symbol);
    }

    reactivity.optimize_derived_rune(&optimizable);
}

fn is_non_coercive(operator: AssignmentOperator) -> bool {
    match operator {
        AssignmentOperator::Assign
        | AssignmentOperator::LogicalAnd
        | AssignmentOperator::LogicalOr
        | AssignmentOperator::LogicalNullish => true,
        AssignmentOperator::Addition
        | AssignmentOperator::Subtraction
        | AssignmentOperator::Multiplication
        | AssignmentOperator::Division
        | AssignmentOperator::Remainder
        | AssignmentOperator::Exponential
        | AssignmentOperator::ShiftLeft
        | AssignmentOperator::ShiftRight
        | AssignmentOperator::ShiftRightZeroFill
        | AssignmentOperator::BitwiseOR
        | AssignmentOperator::BitwiseXOR
        | AssignmentOperator::BitwiseAnd => false,
    }
}

pub(crate) fn finalize_proxy(
    parsed: &JsAst<'_>,
    reactivity: &mut ReactivitySemantics,
    semantics: &ComponentSemantics<'_>,
    dev: bool,
) {
    if !reactivity.uses_runes() {
        return;
    }

    let init_proxyable = reactivity.take_init_proxyable();
    let (binding_inits, field_inits, signal_writes, class_fields) = {
        let mut collector = ProxyCollector {
            reactivity,
            semantics,
            init_proxyable: &init_proxyable,
            dev,
            binding_inits: Vec::new(),
            field_inits: Vec::new(),
            signal_writes: Vec::new(),
            class_fields: FxHashMap::default(),
            in_constructor: false,
            next_is_constructor: false,
        };
        for program in [parsed.program.as_ref(), parsed.module_program.as_ref()]
            .into_iter()
            .flatten()
        {
            collector.visit_program(program);
        }
        for expression in parsed.iter_exprs() {
            collector.visit_expression(expression);
        }
        for statement in parsed.iter_stmts() {
            collector.visit_statement(statement);
        }
        (
            collector.binding_inits,
            collector.field_inits,
            collector.signal_writes,
            collector.class_fields,
        )
    };

    for (sym, proxied) in binding_inits {
        reactivity.set_state_proxied(sym, proxied);
    }
    for (decl_node, proxied) in field_inits {
        reactivity.set_class_field_proxied(decl_node, proxied);
    }
    for ref_id in signal_writes {
        reactivity.set_signal_write_proxy(ref_id, true);
    }
    for (access_node, field) in class_fields {
        reactivity.record_class_field_semantics(access_node, field);
    }
}

struct ProxyCollector<'c, 'a> {
    reactivity: &'c ReactivitySemantics,
    semantics: &'c ComponentSemantics<'a>,
    init_proxyable: &'c FxHashMap<SymbolId, bool>,
    dev: bool,
    binding_inits: Vec<(SymbolId, bool)>,
    field_inits: Vec<(OxcNodeId, bool)>,
    signal_writes: Vec<ReferenceId>,
    class_fields: FxHashMap<OxcNodeId, ClassFieldSemantics>,
    in_constructor: bool,
    next_is_constructor: bool,
}

impl<'c, 'a> Visit<'a> for ProxyCollector<'c, 'a> {
    fn visit_variable_declarator(&mut self, declarator: &VariableDeclarator<'a>) {
        self.classify_binding_init(declarator);
        walk_variable_declarator(self, declarator);
    }

    fn visit_property_definition(&mut self, prop: &PropertyDefinition<'a>) {
        self.classify_field_init(prop.node_id(), prop.value.as_ref());
        walk_property_definition(self, prop);
    }

    fn visit_assignment_expression(&mut self, expr: &AssignmentExpression<'a>) {
        self.classify_assignment(expr);
        walk_assignment_expression(self, expr);
    }

    fn visit_private_field_expression(&mut self, expr: &PrivateFieldExpression<'a>) {
        self.classify_field_read(expr.node_id());
        walk_private_field_expression(self, expr);
    }

    fn visit_static_member_expression(&mut self, expr: &StaticMemberExpression<'a>) {
        if matches!(&expr.object, Expression::ThisExpression(_)) {
            self.classify_field_read(expr.node_id());
        }
        walk_static_member_expression(self, expr);
    }

    fn visit_method_definition(&mut self, method: &MethodDefinition<'a>) {
        if method.kind == MethodDefinitionKind::Constructor {
            self.next_is_constructor = true;
        }
        walk_method_definition(self, method);
    }

    fn visit_function(&mut self, func: &Function<'a>, flags: ScopeFlags) {
        let prev = self.in_constructor;
        self.in_constructor = mem::take(&mut self.next_is_constructor);
        walk_function(self, func, flags);
        self.in_constructor = prev;
    }

    fn visit_arrow_function_expression(&mut self, arrow: &ArrowFunctionExpression<'a>) {
        self.next_is_constructor = false;
        let prev = self.in_constructor;
        self.in_constructor = false;
        walk_arrow_function_expression(self, arrow);
        self.in_constructor = prev;
    }

    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        if matches!(
            detect_rune_from_call(call),
            Some(RuneKind::Derived | RuneKind::DerivedBy)
        ) {
            let prev = self.in_constructor;
            self.in_constructor = false;
            walk_call_expression(self, call);
            self.in_constructor = prev;
        } else {
            walk_call_expression(self, call);
        }
    }
}

impl<'c, 'a> ProxyCollector<'c, 'a> {
    fn classify_binding_init(&mut self, declarator: &VariableDeclarator<'a>) {
        let BindingPattern::BindingIdentifier(id) = &declarator.id else {
            return;
        };
        let Some(sym) = id.symbol_id.get() else {
            return;
        };
        let Some((call, rune_kind)) = rune_call(declarator) else {
            return;
        };
        if rune_kind != RuneKind::State {
            return;
        }
        let proxied = self.state_arg_proxies(call);
        self.binding_inits.push((sym, proxied));
    }

    fn classify_field_init(&mut self, decl_node: OxcNodeId, value: Option<&Expression<'a>>) {
        let DeclaratorSemantics::ClassFieldState(state) =
            self.reactivity.declarator_semantics(decl_node)
        else {
            return;
        };
        if state.kind != StateKind::State {
            return;
        }
        let Some(Expression::CallExpression(call)) = value.map(|v| v.get_inner_expression()) else {
            return;
        };
        let proxied = self.state_arg_proxies(call);
        self.field_inits.push((decl_node, proxied));
    }

    fn classify_assignment(&mut self, expr: &AssignmentExpression<'a>) {
        let decl = self.reactivity.declarator_semantics(expr.node_id());
        if matches!(
            &decl,
            DeclaratorSemantics::ClassFieldState(_) | DeclaratorSemantics::ClassFieldDerived(_)
        ) {
            if let DeclaratorSemantics::ClassFieldState(state) = &decl
                && state.kind == StateKind::State
                && let Expression::CallExpression(call) = expr.right.get_inner_expression()
            {
                let proxied = self.state_arg_proxies(call);
                self.field_inits.push((expr.node_id(), proxied));
            }
            if let Some(access) = assignment_target_field_access(&expr.left) {
                self.class_fields.insert(access, ClassFieldSemantics::None);
            }
            return;
        }
        match &expr.left {
            AssignmentTarget::AssignmentTargetIdentifier(id) => {
                self.classify_identifier_write(id, expr.operator, &expr.right);
            }
            AssignmentTarget::PrivateFieldExpression(field) => {
                self.classify_field_write(field.node_id(), expr.operator, &expr.right);
            }
            AssignmentTarget::StaticMemberExpression(member)
                if matches!(&member.object, Expression::ThisExpression(_)) =>
            {
                self.classify_field_write(member.node_id(), expr.operator, &expr.right);
            }
            AssignmentTarget::ArrayAssignmentTarget(_)
            | AssignmentTarget::ObjectAssignmentTarget(_) => {
                self.classify_destructure_write(&expr.left);
            }
            _ => {}
        }
    }

    fn classify_identifier_write(
        &mut self,
        id: &IdentifierReference<'a>,
        operator: AssignmentOperator,
        right: &Expression<'a>,
    ) {
        let Some(ref_id) = id.reference_id.get() else {
            return;
        };
        if self.signal_write_state_kind(ref_id) != Some(StateKind::State) {
            return;
        }
        if is_non_coercive(operator) && self.should_proxy(right, false) {
            self.signal_writes.push(ref_id);
        }
    }

    fn classify_destructure_write(&mut self, target: &AssignmentTarget<'a>) {
        let mut targets: Vec<ReferenceId> = Vec::new();
        walk_assignment_targets(target, |visit| {
            let WriteTarget::Identifier(id) = visit.target else {
                return;
            };
            let Some(ref_id) = id.reference_id.get() else {
                return;
            };
            targets.push(ref_id);
        });
        for ref_id in targets {
            if self.signal_write_state_kind(ref_id) == Some(StateKind::State) {
                self.signal_writes.push(ref_id);
            }
        }
    }

    fn classify_field_write(
        &mut self,
        access_node: OxcNodeId,
        operator: AssignmentOperator,
        right: &Expression<'a>,
    ) {
        let Some(base) = self.resolved_field(access_node) else {
            return;
        };
        let field = match base {
            ClassFieldSemantics::State { kind, .. } => {
                let proxy = kind == StateKind::State
                    && is_non_coercive(operator)
                    && self.write_value_proxies(operator, right);
                ClassFieldSemantics::State {
                    kind,
                    proxy,
                    tracked: self.read_tracked(kind),
                }
            }
            ClassFieldSemantics::Derived { .. } | ClassFieldSemantics::None => base,
        };
        self.class_fields.insert(access_node, field);
    }

    fn classify_field_read(&mut self, access_node: OxcNodeId) {
        if self.class_fields.contains_key(&access_node) {
            return;
        }
        let Some(base) = self.resolved_field(access_node) else {
            return;
        };
        let field = match base {
            ClassFieldSemantics::State { kind, proxy, .. } => ClassFieldSemantics::State {
                kind,
                proxy,
                tracked: self.read_tracked(kind),
            },
            ClassFieldSemantics::Derived { .. } | ClassFieldSemantics::None => base,
        };
        self.class_fields.insert(access_node, field);
    }

    fn read_tracked(&self, kind: StateKind) -> bool {
        !(self.in_constructor && matches!(kind, StateKind::State | StateKind::StateRaw))
    }

    fn resolved_field(&self, access_node: OxcNodeId) -> Option<ClassFieldSemantics> {
        let decl_node = self.semantics.field_access_target(access_node)?;
        match self.reactivity.declarator_semantics(decl_node) {
            DeclaratorSemantics::ClassFieldState(state) => Some(ClassFieldSemantics::State {
                kind: state.kind,
                proxy: false,
                tracked: true,
            }),
            DeclaratorSemantics::ClassFieldDerived(derived) => {
                Some(ClassFieldSemantics::Derived { kind: derived.kind })
            }
            DeclaratorSemantics::None
            | DeclaratorSemantics::RuntimeRuneCall { .. }
            | DeclaratorSemantics::RuneProps
            | DeclaratorSemantics::LegacyProps
            | DeclaratorSemantics::LegacyState
            | DeclaratorSemantics::RuneState { .. }
            | DeclaratorSemantics::RuneDerived { .. }
            | DeclaratorSemantics::ConstTag { .. }
            | DeclaratorSemantics::LetCarrier { .. }
            | DeclaratorSemantics::EachItem
            | DeclaratorSemantics::AwaitValue
            | DeclaratorSemantics::SnippetParam => None,
        }
    }

    fn write_value_proxies(&self, operator: AssignmentOperator, right: &Expression<'a>) -> bool {
        match operator {
            AssignmentOperator::Assign => self.should_proxy(right, false),
            AssignmentOperator::LogicalAnd
            | AssignmentOperator::LogicalOr
            | AssignmentOperator::LogicalNullish => true,
            AssignmentOperator::Addition
            | AssignmentOperator::Subtraction
            | AssignmentOperator::Multiplication
            | AssignmentOperator::Division
            | AssignmentOperator::Remainder
            | AssignmentOperator::Exponential
            | AssignmentOperator::ShiftLeft
            | AssignmentOperator::ShiftRight
            | AssignmentOperator::ShiftRightZeroFill
            | AssignmentOperator::BitwiseOR
            | AssignmentOperator::BitwiseXOR
            | AssignmentOperator::BitwiseAnd => false,
        }
    }

    fn state_arg_proxies(&self, call: &CallExpression<'a>) -> bool {
        let Some(arg) = call.arguments.first().and_then(|a| a.as_expression()) else {
            return false;
        };
        if self.dev && is_dev_rewritten_equality(arg.get_inner_expression()) {
            return true;
        }
        self.should_proxy(arg, true)
    }

    fn signal_write_state_kind(&self, ref_id: ReferenceId) -> Option<StateKind> {
        self.reactivity
            .reference_semantics(ref_id)
            .signal_write_kind()
    }

    fn should_proxy(&self, expr: &Expression<'a>, props_opaque: bool) -> bool {
        let expr = expr.get_inner_expression();
        let Expression::Identifier(id) = expr else {
            return should_proxy_node_only(expr);
        };
        if id.name == "undefined" {
            return false;
        }
        let Some(symbol) = self.semantics.symbol_for_identifier_reference(id) else {
            return true;
        };
        if self.semantics.is_mutated(symbol) {
            return true;
        }
        if props_opaque && self.reactivity.binding_semantics(symbol).is_props() {
            return true;
        }
        self.init_proxyable.get(&symbol).copied().unwrap_or(true)
    }
}

fn is_dev_rewritten_equality(expr: &Expression) -> bool {
    let Expression::BinaryExpression(binary) = expr else {
        return false;
    };
    matches!(
        binary.operator,
        BinaryOperator::Equality
            | BinaryOperator::Inequality
            | BinaryOperator::StrictEquality
            | BinaryOperator::StrictInequality
    )
}

fn should_proxy_node_only(expr: &Expression) -> bool {
    let expr = expr.get_inner_expression();
    if expr.is_literal() {
        return false;
    }
    if matches!(
        expr,
        Expression::TemplateLiteral(_)
            | Expression::ArrowFunctionExpression(_)
            | Expression::FunctionExpression(_)
            | Expression::UnaryExpression(_)
            | Expression::BinaryExpression(_)
    ) {
        return false;
    }
    if let Expression::Identifier(id) = expr
        && id.name == "undefined"
    {
        return false;
    }
    true
}

#[derive(Clone, Copy)]
enum ReferenceReactivityMode {
    General,
    PropDefault,
}

fn reference_is_reactive(
    reactivity: &ReactivitySemantics,
    scoping: &ComponentScoping<'_>,
    ref_id: ReferenceId,
    mode: ReferenceReactivityMode,
) -> bool {
    use super::data::BindingSemantics;
    if reactivity
        .reference_semantics(ref_id)
        .is_store_subscription()
    {
        return true;
    }
    let Some(sym) = scoping.symbol_for_reference(ref_id) else {
        return false;
    };
    match reactivity.binding_semantics(sym) {
        BindingSemantics::State(_)
        | BindingSemantics::Prop(_)
        | BindingSemantics::LegacyBindableProp(_)
        | BindingSemantics::LegacyState(_)
        | BindingSemantics::Store(_)
        | BindingSemantics::Contextual(_)
        | BindingSemantics::RuntimeRune { .. } => true,
        BindingSemantics::Derived(_) => reactivity.derived_reactive(sym),
        BindingSemantics::OptimizedDerived(_) => false,
        BindingSemantics::Const(_) | BindingSemantics::DeclarationTag => true,
        BindingSemantics::OptimizedConst(_) | BindingSemantics::OptimizedDeclarationTag => false,
        BindingSemantics::OptimizedRune(opt) if opt.proxy_init => true,
        BindingSemantics::MaybeReactive => match mode {
            ReferenceReactivityMode::General => true,
            ReferenceReactivityMode::PropDefault => false,
        },
        BindingSemantics::NonReactive
        | BindingSemantics::Unresolved
        | BindingSemantics::OptimizedRune(_) => match mode {
            ReferenceReactivityMode::General => !scoping.is_component_top_level_symbol(sym),
            ReferenceReactivityMode::PropDefault => false,
        },
        BindingSemantics::LegacyApiExport => false,
    }
}

fn runtime_rune_call_kind(kind: RuneKind) -> Option<RuntimeRuneKind> {
    match kind {
        RuneKind::Effect => Some(RuntimeRuneKind::Effect),
        RuneKind::EffectPre => Some(RuntimeRuneKind::EffectPre),
        RuneKind::EffectRoot => Some(RuntimeRuneKind::EffectRoot),
        RuneKind::EffectTracking => Some(RuntimeRuneKind::EffectTracking),
        RuneKind::EffectPending => Some(RuntimeRuneKind::EffectPending),
        RuneKind::Inspect => Some(RuntimeRuneKind::Inspect),
        RuneKind::InspectWith => Some(RuntimeRuneKind::InspectWith),
        RuneKind::InspectTrace => Some(RuntimeRuneKind::InspectTrace),
        RuneKind::Host => Some(RuntimeRuneKind::Host),
        RuneKind::PropsId => Some(RuntimeRuneKind::PropsId),
        RuneKind::StateSnapshot => Some(RuntimeRuneKind::StateSnapshot),
        RuneKind::StateEager => Some(RuntimeRuneKind::StateEager),
        RuneKind::Bindable => Some(RuntimeRuneKind::Bindable),
        RuneKind::State
        | RuneKind::StateRaw
        | RuneKind::Derived
        | RuneKind::DerivedBy
        | RuneKind::Props => None,
    }
}

fn rune_call_fact(kind: RuneKind, call: &CallExpression<'_>) -> Option<DeclaratorSemantics> {
    if let Some(runtime) = runtime_rune_call_kind(kind) {
        return Some(DeclaratorSemantics::RuntimeRuneCall { kind: runtime });
    }
    match kind {
        RuneKind::State => Some(DeclaratorSemantics::RuneState {
            kind: StateKind::State,
        }),
        RuneKind::StateRaw => Some(DeclaratorSemantics::RuneState {
            kind: StateKind::StateRaw,
        }),
        RuneKind::Derived => Some(DeclaratorSemantics::RuneDerived {
            kind: DerivedKind::Derived,
            async_kind: derived_async_kind(call),
            source: DerivedSource::Computed,
        }),
        RuneKind::DerivedBy => Some(DeclaratorSemantics::RuneDerived {
            kind: DerivedKind::DerivedBy,
            async_kind: derived_async_kind(call),
            source: DerivedSource::Computed,
        }),
        RuneKind::Props => Some(DeclaratorSemantics::RuneProps),
        RuneKind::Bindable
        | RuneKind::StateEager
        | RuneKind::StateSnapshot
        | RuneKind::Effect
        | RuneKind::EffectPre
        | RuneKind::EffectRoot
        | RuneKind::EffectTracking
        | RuneKind::EffectPending
        | RuneKind::Inspect
        | RuneKind::InspectWith
        | RuneKind::InspectTrace
        | RuneKind::Host
        | RuneKind::PropsId => None,
    }
}

fn record_maybe_reactive_imports(data: &mut AnalysisData<'_>) {
    let imports: Vec<SymbolId> = data
        .scoping
        .semantics()
        .symbol_ids()
        .filter(|&sym| data.scoping.is_import(sym))
        .collect();
    for sym in imports {
        data.reactivity.record_maybe_reactive_symbol(sym);
    }
}

pub(super) struct LegacyReactiveCollected {
    pub labeled_nodes: Vec<OxcNodeId>,
    pub implicit_names: Vec<compact_str::CompactString>,
}

fn record_const_tag_declarators<'a>(
    component: &Component,
    parsed: &JsAst<'a>,
    data: &mut AnalysisData<'a>,
) {
    for node in component.store.iter_nodes() {
        let svelte_ast::Node::ConstTag(tag) = node else {
            continue;
        };
        let Some(Statement::VariableDeclaration(decl)) = parsed.stmt(tag.decl.id()) else {
            continue;
        };
        let Some(declarator) = decl.declarations.first() else {
            continue;
        };
        let async_kind = match declarator.init.as_ref() {
            Some(init) if expression_has_await(init) => DerivedAsyncKind::Async,
            _ => DerivedAsyncKind::Sync,
        };
        data.reactivity.record_declarator_semantics(
            decl.node_id(),
            DeclaratorSemantics::ConstTag { async_kind },
        );
    }
}

fn optimize_const_and_declaration_tags<'a>(
    component: &Component,
    parsed: &JsAst<'a>,
    data: &mut AnalysisData<'a>,
) {
    let mut const_order: Vec<svelte_ast::NodeId> = Vec::new();
    for fragment in component.store.iter_fragments() {
        let ordered = data.reactivity.const_tags_in_order_legacy(fragment.id);
        if ordered.is_empty() {
            for &node_id in &fragment.nodes {
                if matches!(component.store.get(node_id), svelte_ast::Node::ConstTag(_)) {
                    const_order.push(node_id);
                }
            }
        } else {
            const_order.extend_from_slice(ordered);
        }
    }
    for node_id in const_order {
        if let svelte_ast::Node::ConstTag(tag) = component.store.get(node_id) {
            optimize_tag_declarators(tag.decl.id(), false, parsed, data);
        }
    }
    for node in component.store.iter_nodes() {
        if let svelte_ast::Node::DeclarationTag(tag) = node {
            optimize_tag_declarators(tag.declaration.id(), true, parsed, data);
        }
    }
}

fn optimize_tag_declarators<'a>(
    stmt_id: OxcNodeId,
    reassignable: bool,
    parsed: &JsAst<'a>,
    data: &mut AnalysisData<'a>,
) {
    use svelte_component_semantics::walk_bindings;

    let Some(Statement::VariableDeclaration(decl)) = parsed.stmt(stmt_id) else {
        return;
    };
    for declarator in &decl.declarations {
        let mut syms: Vec<SymbolId> = Vec::new();
        walk_bindings(&declarator.id, |v| syms.push(v.symbol));
        if syms.is_empty() {
            continue;
        }
        let destructured =
            syms.len() > 1 || !matches!(&declarator.id, BindingPattern::BindingIdentifier(_));

        let mut refs: SmallVec<[ReferenceId; 4]> = SmallVec::new();
        let mut eager_rune = false;
        let mut has_impure = false;
        let mut collector = RefCollector {
            refs: &mut refs,
            reactive_rune_call: &mut eager_rune,
            has_impure: &mut has_impure,
        };
        if let Some(init) = declarator.init.as_ref() {
            collector.visit_expression(init);
        }

        let refs_reactive = refs.iter().any(|&ref_id| {
            reference_is_reactive(
                &data.reactivity,
                &data.scoping,
                ref_id,
                ReferenceReactivityMode::General,
            )
        });
        let mutated = reassignable && syms.iter().any(|&s| data.scoping.is_mutated(s));

        if destructured || eager_rune || has_impure || refs_reactive || mutated {
            continue;
        }
        for sym in syms {
            data.reactivity.optimize_const_binding(sym);
        }
    }
}

fn has_non_store_mutation_legacy(data: &AnalysisData<'_>, sym: SymbolId) -> bool {
    let store_refs: rustc_hash::FxHashSet<ReferenceId> = data
        .scoping
        .store_candidate_refs()
        .iter()
        .map(|(_, r)| *r)
        .collect();
    let any_non_store_write = data
        .scoping
        .get_resolved_reference_ids(sym)
        .iter()
        .any(|&r| !store_refs.contains(&r) && data.scoping.get_reference(r).is_write());
    any_non_store_write || data.scoping.is_member_mutated(sym)
}

fn has_reactive_consumer_reference_legacy(
    data: &AnalysisData<'_>,
    sym: SymbolId,
    reactive_body_refs: &FxHashSet<SymbolId>,
) -> bool {
    if reactive_body_refs.contains(&sym) {
        return true;
    }
    for &ref_id in data.scoping.get_resolved_reference_ids(sym) {
        if data.scoping.is_template_reference(ref_id) {
            return true;
        }
        if reference_in_template_scope(data, ref_id) {
            return true;
        }
    }
    false
}

fn reference_in_template_scope(data: &AnalysisData<'_>, ref_id: ReferenceId) -> bool {
    let reference = data.scoping.get_reference(ref_id);
    let mut scope_opt = Some(reference.scope_id());
    while let Some(scope) = scope_opt {
        if data.scoping.is_template_scope(scope) {
            return true;
        }
        scope_opt = data.scoping.scope_parent_id(scope);
    }
    false
}

fn build_script_semantics_v2<'a>(
    component: &Component,
    parsed: &JsAst<'a>,
    data: &mut AnalysisData<'a>,
    prop_lowering_mode: PropEmitMode,
) -> LegacyReactiveCollected {
    let reactive_body_refs = if data.script.runes() {
        FxHashSet::default()
    } else {
        parsed
            .program
            .as_ref()
            .map(|program| legacy_reactive::collect_reactive_body_symbol_refs(program, data))
            .unwrap_or_default()
    };
    let mut collector = ScriptSemanticCollector::new(data, prop_lowering_mode, reactive_body_refs);
    collector.collect_bind_directive_roots(component, parsed);
    if let Some(program) = parsed.program.as_ref() {
        collector.visit_instance_program(program);
    }
    if let Some(program) = parsed.module_program.as_ref() {
        collector.visit_module_program(program);
    }

    for expr in parsed.iter_exprs() {
        collector.visit_expression(expr);
    }
    for stmt in parsed.iter_stmts() {
        collector.visit_statement(stmt);
    }
    let labeled_nodes = mem::take(&mut collector.legacy_reactive_labeled_nodes);
    let implicit_names = mem::take(&mut collector.legacy_reactive_implicit_names);
    collector.finish();
    LegacyReactiveCollected {
        labeled_nodes,
        implicit_names,
    }
}

struct ScriptSemanticCollector<'d, 'a> {
    data: &'d mut AnalysisData<'a>,
    current_decl_kind: Option<VariableDeclarationKind>,
    prop_lowering_mode: PropEmitMode,

    prop_member_mutation_root_refs: FxHashSet<ReferenceId>,

    bind_this_legacy_state_root_syms: FxHashSet<SymbolId>,

    standard_prop_source_symbols: SmallVec<[SymbolId; 4]>,

    rest_prop_excluded: FxHashMap<SymbolId, FxHashSet<Ident<'a>>>,

    derived_init_refs: FxHashMap<SymbolId, SmallVec<[ReferenceId; 4]>>,

    eager_reactive_derived: FxHashSet<SymbolId>,

    is_instance_program: bool,
    legacy_reactive_labeled_nodes: Vec<OxcNodeId>,
    legacy_reactive_implicit_names: Vec<compact_str::CompactString>,
    legacy_reactive_mutated_imports: SmallVec<[SymbolId; 2]>,
    deferred_const_legacy_state_syms: Vec<SymbolId>,
    deferred_const_destructured_legacy_decls: Vec<(OxcNodeId, SmallVec<[SymbolId; 4]>)>,
    reactive_body_refs: FxHashSet<SymbolId>,
    store_shadow_rune_candidates: Vec<(OxcNodeId, SymbolId, ReferenceId, bool)>,
    collect_init_proxyable: bool,
    init_proxyable: FxHashMap<SymbolId, bool>,
}

impl<'d, 'a> ScriptSemanticCollector<'d, 'a> {
    fn new(
        data: &'d mut AnalysisData<'a>,
        prop_lowering_mode: PropEmitMode,
        reactive_body_refs: FxHashSet<SymbolId>,
    ) -> Self {
        let collect_init_proxyable = data.script.runes();
        Self {
            data,
            current_decl_kind: None,
            prop_lowering_mode,
            prop_member_mutation_root_refs: FxHashSet::default(),
            bind_this_legacy_state_root_syms: FxHashSet::default(),
            standard_prop_source_symbols: SmallVec::new(),
            rest_prop_excluded: FxHashMap::default(),
            derived_init_refs: FxHashMap::default(),
            eager_reactive_derived: FxHashSet::default(),
            is_instance_program: false,
            legacy_reactive_labeled_nodes: Vec::new(),
            legacy_reactive_implicit_names: Vec::new(),
            legacy_reactive_mutated_imports: SmallVec::new(),
            deferred_const_legacy_state_syms: Vec::new(),
            deferred_const_destructured_legacy_decls: Vec::new(),
            reactive_body_refs,
            store_shadow_rune_candidates: Vec::new(),
            collect_init_proxyable,
            init_proxyable: FxHashMap::default(),
        }
    }

    fn visit_instance_program(&mut self, program: &Program<'a>) {
        self.is_instance_program = true;
        self.visit_program(program);
        self.is_instance_program = false;
    }

    fn visit_module_program(&mut self, program: &Program<'a>) {
        debug_assert!(!self.is_instance_program);
        self.visit_program(program);
    }

    fn collect_bind_directive_roots(&mut self, component: &Component, parsed: &JsAst<'a>) {
        let collect_bind_this = !self.data.script.runes();
        for node in component.store.iter_nodes() {
            let (attrs, node_takes_bind_this): (&[svelte_ast::Attribute], bool) = match node {
                svelte_ast::Node::Element(n) => (&n.attributes, true),
                svelte_ast::Node::SvelteElement(n) => (&n.attributes, true),
                svelte_ast::Node::ComponentNode(n) => (&n.attributes, true),
                svelte_ast::Node::SvelteComponentLegacy(n) => (&n.attributes, true),
                svelte_ast::Node::SvelteSelf(n) => (&n.attributes, true),
                svelte_ast::Node::SvelteWindow(n) => (&n.attributes, false),
                svelte_ast::Node::SvelteDocument(n) => (&n.attributes, false),
                svelte_ast::Node::SvelteBody(n) => (&n.attributes, false),
                svelte_ast::Node::SlotElementLegacy(n) => (&n.attributes, false),
                _ => continue,
            };
            for attr in attrs {
                let svelte_ast::Attribute::BindDirective(d) = attr else {
                    continue;
                };
                let Some(expr) = parsed.expr(d.expression.id()) else {
                    continue;
                };
                match expr {
                    Expression::Identifier(_)
                        if collect_bind_this && node_takes_bind_this && d.name == "this" =>
                    {
                        if let Some(ref_id) = expression_root_reference_id(expr)
                            && let Some(sym) = self.data.scoping.symbol_for_reference(ref_id)
                        {
                            self.bind_this_legacy_state_root_syms.insert(sym);
                        }
                    }
                    Expression::StaticMemberExpression(_)
                    | Expression::ComputedMemberExpression(_) => {
                        if let Some(ref_id) = expression_root_reference_id(expr) {
                            self.prop_member_mutation_root_refs.insert(ref_id);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn finish(mut self) {
        let member_mutated_syms: Vec<SymbolId> = self
            .data
            .scoping
            .semantics()
            .symbols_with_state(sym_state::MEMBER_MUTATED)
            .collect();

        for sym in member_mutated_syms {
            if let Some(BindingFacts::Prop(PropBindingSemantics {
                kind: PropBindingKind::Source { updated, .. },
                ..
            })) = self.data.reactivity.binding_facts_mut(sym)
            {
                *updated = true;
            }
        }

        self.data
            .reactivity
            .record_prop_member_mutation_root_refs(mem::take(
                &mut self.prop_member_mutation_root_refs,
            ));

        if !self.data.script.runes() {
            let imports = self.legacy_reactive_mutated_imports.clone();
            let lr = self.data.reactivity.legacy_reactive_mut();
            for sym in &imports {
                lr.add_mutated_import(*sym);
            }
        }

        store::collect_store_declarations(self.data);
        self.demote_store_shadowed_runes();

        self.finalize_deferred_const_legacy_state();
        self.finalize_deferred_const_destructured_legacy_state();

        for sym in mem::take(&mut self.standard_prop_source_symbols) {
            if self.data.scoping.is_member_mutated(sym) {
                continue;
            }
            let refs = self.data.scoping.get_resolved_reference_ids(sym).to_vec();
            if refs.is_empty() {
                continue;
            }
            let downgrade_ok = refs.iter().all(|&r| {
                if self
                    .data
                    .reactivity
                    .reference_semantics(r)
                    .is_store_subscription()
                {
                    return true;
                }
                !self.data.scoping.get_reference(r).is_write()
            });
            if !downgrade_ok {
                continue;
            }
            if let Some(BindingFacts::Prop(prop)) = self.data.reactivity.binding_facts_mut(sym) {
                prop.kind = PropBindingKind::NonSource;
            }
        }

        self.compute_derived_reactivity();

        if self.collect_init_proxyable {
            self.data
                .reactivity
                .set_init_proxyable(mem::take(&mut self.init_proxyable));
        }
    }

    fn compute_derived_reactivity(&mut self) {
        if self.derived_init_refs.is_empty() && self.eager_reactive_derived.is_empty() {
            return;
        }
        let entries: Vec<(SymbolId, SmallVec<[ReferenceId; 4]>)> =
            self.derived_init_refs.drain().collect();
        let eager = mem::take(&mut self.eager_reactive_derived);

        loop {
            let mut changed = false;
            for (sym, refs) in &entries {
                let current_reactive = match self.data.reactivity.binding_facts(*sym) {
                    Some(BindingFacts::Derived(d)) => d.reactive,
                    _ => continue,
                };
                let new_reactive = eager.contains(sym)
                    || refs.iter().any(|&r| {
                        reference_is_reactive(
                            &self.data.reactivity,
                            &self.data.scoping,
                            r,
                            ReferenceReactivityMode::General,
                        )
                    });
                if new_reactive != current_reactive {
                    self.data
                        .reactivity
                        .set_derived_reactive(*sym, new_reactive);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
    }

    fn demote_store_shadowed_runes(&mut self) {
        for (decl_node, symbol, callee_ref, is_module) in
            mem::take(&mut self.store_shadow_rune_candidates)
        {
            if !self
                .data
                .reactivity
                .reference_semantics(callee_ref)
                .is_store_subscription()
            {
                continue;
            }
            if is_module {
                self.data.reactivity.clear_reference_semantics(callee_ref);
            } else {
                self.data
                    .reactivity
                    .demote_store_shadowed_rune(decl_node, symbol);
            }
        }
    }

    fn record_rune_declarator(&mut self, declarator: &VariableDeclarator<'a>) {
        if !self.data.script.runes() {
            return;
        }
        let Some((call, rune_kind)) = rune_call(declarator) else {
            return;
        };
        if rune_callee_resolves_to_binding(self.data, call) {
            return;
        }
        let root_node = declarator.node_id();

        if let BindingPattern::BindingIdentifier(id) = &declarator.id
            && let Some(symbol) = id.symbol_id.get()
            && let Some(callee_ref) = expression_root_reference_id(&call.callee)
        {
            self.store_shadow_rune_candidates.push((
                root_node,
                symbol,
                callee_ref,
                !self.is_instance_program,
            ));
        }

        let var_declared = matches!(self.current_decl_kind, Some(VariableDeclarationKind::Var));
        let init_proxyable =
            matches!(rune_kind, RuneKind::State) && state_initializer_is_proxyable(call);

        match rune_kind {
            RuneKind::State => {
                self.data.reactivity.record_declarator_semantics(
                    root_node,
                    DeclaratorSemantics::RuneState {
                        kind: StateKind::State,
                    },
                );
                let root_proxied = if matches!(&declarator.id, BindingPattern::BindingIdentifier(_))
                {
                    init_proxyable
                } else {
                    true
                };
                self.record_state_root_declaration(
                    &declarator.id,
                    root_node,
                    StateDeclarationSemantics {
                        kind: StateKind::State,
                        proxied: root_proxied,
                        var_declared,
                        is_signal_source: false,
                    },
                    true,
                );
            }
            RuneKind::StateRaw => {
                self.data.reactivity.record_declarator_semantics(
                    root_node,
                    DeclaratorSemantics::RuneState {
                        kind: StateKind::StateRaw,
                    },
                );
                self.record_state_root_declaration(
                    &declarator.id,
                    root_node,
                    StateDeclarationSemantics {
                        kind: StateKind::StateRaw,
                        proxied: false,
                        var_declared,
                        is_signal_source: false,
                    },
                    true,
                );
            }
            RuneKind::StateEager => {
                self.data.reactivity.record_declarator_semantics(
                    root_node,
                    DeclaratorSemantics::RuneState {
                        kind: StateKind::StateEager,
                    },
                );
                self.record_state_root_declaration(
                    &declarator.id,
                    root_node,
                    StateDeclarationSemantics {
                        kind: StateKind::StateEager,
                        proxied: false,
                        var_declared,
                        is_signal_source: false,
                    },
                    false,
                );
            }
            RuneKind::Derived => {
                let async_kind = derived_async_kind(call);
                self.data.reactivity.record_declarator_semantics(
                    root_node,
                    DeclaratorSemantics::RuneDerived {
                        kind: DerivedKind::Derived,
                        async_kind,
                        source: DerivedSource::Computed,
                    },
                );
                if matches!(async_kind, DerivedAsyncKind::Sync)
                    && matches!(&declarator.id, BindingPattern::BindingIdentifier(_))
                    && let Some(ref_id) = derived_source_reference(call)
                {
                    self.data
                        .reactivity
                        .record_deferred_derived_source(root_node, ref_id);
                }
                self.record_derived_pattern(
                    &declarator.id,
                    DerivedDeclarationSemantics {
                        kind: DerivedKind::Derived,
                        async_kind,
                        var_declared,
                    },
                );
                self.collect_derived_init_refs(declarator, RuneKind::Derived);
            }
            RuneKind::DerivedBy => {
                let async_kind = derived_async_kind(call);
                self.data.reactivity.record_declarator_semantics(
                    root_node,
                    DeclaratorSemantics::RuneDerived {
                        kind: DerivedKind::DerivedBy,
                        async_kind,
                        source: DerivedSource::Computed,
                    },
                );
                self.record_derived_pattern(
                    &declarator.id,
                    DerivedDeclarationSemantics {
                        kind: DerivedKind::DerivedBy,
                        async_kind,
                        var_declared,
                    },
                );
                self.collect_derived_init_refs(declarator, RuneKind::DerivedBy);
            }
            RuneKind::Props => {
                self.record_props_pattern(&declarator.id, root_node);
            }
            RuneKind::PropsId => {
                self.record_runtime_rune_pattern(&declarator.id, RuntimeRuneKind::PropsId);
                self.record_props_id_symbol(&declarator.id);
            }
            RuneKind::EffectTracking => {
                self.record_runtime_rune_pattern(&declarator.id, RuntimeRuneKind::EffectTracking);
            }
            RuneKind::EffectPending => {
                self.record_runtime_rune_pattern(&declarator.id, RuntimeRuneKind::EffectPending);
            }
            RuneKind::Host => {
                self.record_runtime_rune_pattern(&declarator.id, RuntimeRuneKind::Host);
            }
            RuneKind::InspectTrace => {
                self.record_runtime_rune_pattern(&declarator.id, RuntimeRuneKind::InspectTrace);
            }
            _ => {}
        }
    }

    fn record_legacy_state_declarator(&mut self, declarator: &VariableDeclarator<'a>) {
        if self.data.script.runes() {
            return;
        }
        let Some(kind) = self.current_decl_kind else {
            return;
        };
        if matches!(kind, VariableDeclarationKind::Const) {
            if let BindingPattern::BindingIdentifier(ident) = &declarator.id {
                if let Some(sym) = ident.symbol_id.get()
                    && self.data.scoping.is_component_top_level_symbol(sym)
                {
                    self.deferred_const_legacy_state_syms.push(sym);
                }
                return;
            }
            let mut leaf_syms: SmallVec<[SymbolId; 4]> = SmallVec::new();
            svelte_component_semantics::walk_bindings(&declarator.id, |v| {
                if self.data.scoping.is_component_top_level_symbol(v.symbol) {
                    leaf_syms.push(v.symbol);
                }
            });
            if !leaf_syms.is_empty() {
                self.deferred_const_destructured_legacy_decls
                    .push((declarator.node_id(), leaf_syms));
            }
            return;
        }
        if !is_let_or_var(kind) {
            return;
        }
        let var_declared = matches!(kind, VariableDeclarationKind::Var);
        let immutable = self.data.script.immutable;

        let mut leaf_syms: Vec<SymbolId> = Vec::new();
        svelte_component_semantics::walk_bindings(&declarator.id, |v| leaf_syms.push(v.symbol));
        let is_destructured =
            leaf_syms.len() > 1 || !matches!(&declarator.id, BindingPattern::BindingIdentifier(_));
        let mut promoted_leaves: SmallVec<[SymbolId; 4]> = SmallVec::new();
        for sym in leaf_syms {
            if !self.data.scoping.is_component_top_level_symbol(sym) {
                continue;
            }
            if self.data.reactivity.binding_facts(sym).is_some() {
                continue;
            }
            let is_bind_this_root = self.bind_this_legacy_state_root_syms.contains(&sym);
            if !is_bind_this_root {
                if !has_non_store_mutation_legacy(self.data, sym) {
                    continue;
                }
                if !has_reactive_consumer_reference_legacy(self.data, sym, &self.reactive_body_refs)
                {
                    continue;
                }
            }
            self.data.reactivity.record_legacy_state_binding(
                sym,
                super::data::LegacyStateSemantics {
                    var_declared,
                    immutable,
                    is_signal_source: false,
                },
            );
            promoted_leaves.push(sym);
        }
        if is_destructured && !promoted_leaves.is_empty() {
            self.data.reactivity.record_declarator_semantics(
                declarator.node_id(),
                DeclaratorSemantics::LegacyState,
            );
        }
    }

    fn finalize_deferred_const_legacy_state(&mut self) {
        if self.data.script.runes() {
            return;
        }
        let immutable = self.data.script.immutable;
        let syms = mem::take(&mut self.deferred_const_legacy_state_syms);
        for sym in syms {
            if self.data.reactivity.binding_facts(sym).is_some() {
                continue;
            }
            if self.data.reactivity.store_shadow_of_internal(sym).is_some() {
                continue;
            }
            if !self.bind_this_legacy_state_root_syms.contains(&sym) {
                if !self.data.scoping.is_member_mutated(sym)
                    && !self.data.scoping.is_mutated_any(sym)
                {
                    continue;
                }
                if !has_reactive_consumer_reference_legacy(self.data, sym, &self.reactive_body_refs)
                {
                    continue;
                }
            }
            self.data.reactivity.record_legacy_state_binding(
                sym,
                super::data::LegacyStateSemantics {
                    var_declared: false,
                    immutable,
                    is_signal_source: false,
                },
            );
        }
    }

    fn finalize_deferred_const_destructured_legacy_state(&mut self) {
        if self.data.script.runes() {
            return;
        }
        let immutable = self.data.script.immutable;
        let decls = mem::take(&mut self.deferred_const_destructured_legacy_decls);
        for (decl_node_id, leaf_syms) in decls {
            let mut promoted: SmallVec<[SymbolId; 4]> = SmallVec::new();
            for sym in leaf_syms {
                if self.data.reactivity.binding_facts(sym).is_some() {
                    continue;
                }
                if self.data.reactivity.store_shadow_of_internal(sym).is_some() {
                    continue;
                }
                if !self.bind_this_legacy_state_root_syms.contains(&sym) {
                    if !self.data.scoping.is_member_mutated(sym)
                        && !self.data.scoping.is_mutated_any(sym)
                    {
                        continue;
                    }
                    if !has_reactive_consumer_reference_legacy(
                        self.data,
                        sym,
                        &self.reactive_body_refs,
                    ) {
                        continue;
                    }
                }
                self.data.reactivity.record_legacy_state_binding(
                    sym,
                    super::data::LegacyStateSemantics {
                        var_declared: false,
                        immutable,
                        is_signal_source: false,
                    },
                );
                promoted.push(sym);
            }
            if !promoted.is_empty() {
                self.data
                    .reactivity
                    .record_declarator_semantics(decl_node_id, DeclaratorSemantics::LegacyState);
            }
        }
    }

    fn record_props_id_symbol(&mut self, pattern: &BindingPattern<'_>) {
        if self.data.script.props_id.is_some() {
            return;
        }
        let mut root_symbol = None;
        svelte_component_semantics::walk_bindings(pattern, |v| {
            if v.path.is_empty() && !v.is_rest && root_symbol.is_none() {
                root_symbol = Some(v.symbol);
            }
        });
        let Some(symbol) = root_symbol else {
            return;
        };
        if !self.data.scoping.is_component_top_level_symbol(symbol) {
            return;
        }
        self.data.script.props_id = Some(symbol);
    }

    fn record_runtime_rune_pattern(&mut self, pattern: &BindingPattern<'_>, kind: RuntimeRuneKind) {
        match pattern {
            BindingPattern::BindingIdentifier(ident) => {
                let Some(sym) = ident.symbol_id.get() else {
                    return;
                };
                self.data.reactivity.record_runtime_rune_binding(sym, kind);
            }
            BindingPattern::ObjectPattern(obj) => {
                for prop in &obj.properties {
                    self.record_runtime_rune_pattern(&prop.value, kind);
                }
                if let Some(rest) = &obj.rest {
                    self.record_runtime_rune_pattern(&rest.argument, kind);
                }
            }
            BindingPattern::ArrayPattern(arr) => {
                for elem in arr.elements.iter().flatten() {
                    self.record_runtime_rune_pattern(elem, kind);
                }
                if let Some(rest) = &arr.rest {
                    self.record_runtime_rune_pattern(&rest.argument, kind);
                }
            }
            BindingPattern::AssignmentPattern(assign) => {
                self.record_runtime_rune_pattern(&assign.left, kind);
            }
        }
    }

    fn collect_derived_init_refs(
        &mut self,
        declarator: &VariableDeclarator<'a>,
        rune_kind: RuneKind,
    ) {
        let Some(init) = declarator.init.as_ref() else {
            return;
        };
        let Expression::CallExpression(call) = init.get_inner_expression() else {
            return;
        };
        let mut refs: SmallVec<[ReferenceId; 4]> = SmallVec::new();
        let mut reactive_rune_call = false;
        let mut ignored_impure = false;
        let mut visitor = RefCollector {
            refs: &mut refs,
            reactive_rune_call: &mut reactive_rune_call,
            has_impure: &mut ignored_impure,
        };
        match rune_kind {
            RuneKind::DerivedBy => {
                let Some(arg) = call.arguments.first().and_then(|a| a.as_expression()) else {
                    return;
                };
                let Expression::ArrowFunctionExpression(arrow) = arg else {
                    return;
                };
                if !arrow.expression {
                    return;
                }
                visitor.visit_function_body(&arrow.body);
            }
            _ => {
                visitor.visit_call_expression(call);
            }
        }
        if reactive_rune_call {
            self.eager_reactive_derived
                .extend(pattern_binding_symbols(&declarator.id));
        }
        if refs.is_empty() && !reactive_rune_call {
            return;
        }
        self.record_init_refs_for_pattern(&declarator.id, &refs);
    }

    fn record_init_refs_for_pattern(
        &mut self,
        pattern: &BindingPattern<'_>,
        refs: &SmallVec<[ReferenceId; 4]>,
    ) {
        match pattern {
            BindingPattern::BindingIdentifier(ident) => {
                if let Some(sym) = ident.symbol_id.get() {
                    self.derived_init_refs.insert(sym, refs.clone());
                }
            }
            BindingPattern::ObjectPattern(obj) => {
                for prop in &obj.properties {
                    self.record_init_refs_for_pattern(&prop.value, refs);
                }
                if let Some(rest) = &obj.rest {
                    self.record_init_refs_for_pattern(&rest.argument, refs);
                }
            }
            BindingPattern::ArrayPattern(arr) => {
                for elem in arr.elements.iter().flatten() {
                    self.record_init_refs_for_pattern(elem, refs);
                }
                if let Some(rest) = &arr.rest {
                    self.record_init_refs_for_pattern(&rest.argument, refs);
                }
            }
            BindingPattern::AssignmentPattern(assign) => {
                self.record_init_refs_for_pattern(&assign.left, refs);
            }
        }
    }

    fn record_state_root_declaration(
        &mut self,
        pattern: &BindingPattern<'_>,
        _root_node: OxcNodeId,
        semantics: StateDeclarationSemantics,
        require_mutation: bool,
    ) {
        let optimize = require_mutation
            && match pattern {
                BindingPattern::BindingIdentifier(ident) => {
                    let reassigned = ident.symbol_id.get().is_some_and(|sym| {
                        self.data.scoping.is_mutated_any(sym)
                            || self.data.scoping.is_reexported_specifier_local(sym)
                    });
                    !self.data.script.is_state_source(reassigned)
                }
                _ => false,
            };
        if optimize {
            let optimized = OptimizedRuneSemantics {
                kind: semantics.kind,
                proxy_init: semantics.proxied,
                var_declared: semantics.var_declared,
            };
            self.record_optimized_rune_leaves(pattern, optimized);
        } else {
            self.record_state_leaves(pattern, &semantics);
        }
    }

    fn record_optimized_rune_leaves(
        &mut self,
        pattern: &BindingPattern<'_>,
        semantics: OptimizedRuneSemantics,
    ) {
        match pattern {
            BindingPattern::BindingIdentifier(ident) => {
                let Some(sym) = ident.symbol_id.get() else {
                    return;
                };
                self.data
                    .reactivity
                    .record_optimized_rune_binding(sym, semantics);
            }
            BindingPattern::ObjectPattern(obj) => {
                for prop in &obj.properties {
                    self.record_optimized_rune_leaves(&prop.value, semantics);
                }
                if let Some(rest) = &obj.rest {
                    self.record_optimized_rune_leaves(&rest.argument, semantics);
                }
            }
            BindingPattern::ArrayPattern(arr) => {
                for elem in arr.elements.iter().flatten() {
                    self.record_optimized_rune_leaves(elem, semantics);
                }
                if let Some(rest) = &arr.rest {
                    self.record_optimized_rune_leaves(&rest.argument, semantics);
                }
            }
            BindingPattern::AssignmentPattern(assign) => {
                self.record_optimized_rune_leaves(&assign.left, semantics);
            }
        }
    }

    fn record_state_leaves(
        &mut self,
        pattern: &BindingPattern<'_>,
        semantics: &StateDeclarationSemantics,
    ) {
        match pattern {
            BindingPattern::BindingIdentifier(ident) => {
                let Some(sym) = ident.symbol_id.get() else {
                    return;
                };
                let binding = StateDeclarationSemantics {
                    is_signal_source: binding_is_signal_source(
                        &self.data.scoping,
                        &self.data.script,
                        sym,
                        semantics.kind,
                    ),
                    ..*semantics
                };
                self.data.reactivity.record_state_binding(sym, binding);
            }
            BindingPattern::ObjectPattern(obj) => {
                for prop in &obj.properties {
                    self.record_state_leaves(&prop.value, semantics);
                }
                if let Some(rest) = &obj.rest {
                    self.record_state_leaves(&rest.argument, semantics);
                }
            }
            BindingPattern::ArrayPattern(arr) => {
                for elem in arr.elements.iter().flatten() {
                    self.record_state_leaves(elem, semantics);
                }
                if let Some(rest) = &arr.rest {
                    self.record_state_leaves(&rest.argument, semantics);
                }
            }
            BindingPattern::AssignmentPattern(assign) => {
                self.record_state_leaves(&assign.left, semantics);
            }
        }
    }

    fn record_derived_pattern(
        &mut self,
        pattern: &BindingPattern<'_>,
        semantics: DerivedDeclarationSemantics,
    ) {
        match pattern {
            BindingPattern::BindingIdentifier(ident) => {
                let Some(sym) = ident.symbol_id.get() else {
                    return;
                };
                self.data.reactivity.record_derived_binding(sym, semantics);
            }
            BindingPattern::ObjectPattern(obj) => {
                for prop in &obj.properties {
                    self.record_derived_pattern(&prop.value, semantics);
                }
                if let Some(rest) = &obj.rest {
                    self.record_derived_pattern(&rest.argument, semantics);
                }
            }
            BindingPattern::ArrayPattern(arr) => {
                for elem in arr.elements.iter().flatten() {
                    self.record_derived_pattern(elem, semantics);
                }
                if let Some(rest) = &arr.rest {
                    self.record_derived_pattern(&rest.argument, semantics);
                }
            }
            BindingPattern::AssignmentPattern(assign) => {
                self.record_derived_pattern(&assign.left, semantics);
            }
        }
    }

    fn record_props_pattern(&mut self, pattern: &BindingPattern<'a>, root_node: OxcNodeId) {
        match pattern {
            BindingPattern::BindingIdentifier(ident) => {
                let Some(sym) = ident.symbol_id.get() else {
                    return;
                };
                self.data.reactivity.record_prop_binding(
                    sym,
                    PropBindingSemantics {
                        emit_mode: self.prop_lowering_mode,
                        kind: PropBindingKind::Rest,
                        bindable: false,
                    },
                );
                self.data
                    .reactivity
                    .record_declarator_semantics(root_node, DeclaratorSemantics::RuneProps);

                self.rest_prop_excluded.insert(sym, FxHashSet::default());
            }
            BindingPattern::ObjectPattern(obj) => {
                let mut sibling_keys: FxHashSet<Ident<'a>> = FxHashSet::default();
                for prop in &obj.properties {
                    if let Some(key) = property_key_atom(&prop.key) {
                        sibling_keys.insert(key);
                    }
                    if self.record_object_prop_pattern(&prop.value).is_none() {
                        return;
                    }
                }

                if let Some(rest) = &obj.rest {
                    match self.record_rest_prop_pattern(&rest.argument) {
                        Some(rest_sym) => {
                            self.rest_prop_excluded.insert(rest_sym, sibling_keys);
                        }
                        None => return,
                    }
                }
                self.data
                    .reactivity
                    .record_declarator_semantics(root_node, DeclaratorSemantics::RuneProps);
            }
            _ => {}
        }
    }

    fn record_object_prop_pattern(&mut self, pattern: &BindingPattern<'_>) -> Option<SymbolId> {
        match pattern {
            BindingPattern::BindingIdentifier(ident) => {
                let sym = ident.symbol_id.get()?;
                let is_source = matches!(self.prop_lowering_mode, PropEmitMode::CustomElement)
                    || self.data.scoping.is_mutated_any(sym);
                let kind = if is_source {
                    if matches!(self.prop_lowering_mode, PropEmitMode::Standard) {
                        self.standard_prop_source_symbols.push(sym);
                    }
                    PropBindingKind::Source {
                        updated: self.data.scoping.is_mutated(sym)
                            || self.data.scoping.is_reexported_specifier_local(sym),
                        default_lowering: PropDefaultKind::None,
                        default_needs_proxy: false,
                    }
                } else {
                    PropBindingKind::NonSource
                };
                self.data.reactivity.record_prop_binding(
                    sym,
                    PropBindingSemantics {
                        emit_mode: self.prop_lowering_mode,
                        kind,
                        bindable: false,
                    },
                );
                Some(sym)
            }
            BindingPattern::AssignmentPattern(assign) => {
                let bindable = prop_default_is_bindable(&assign.right);
                let default_lowering = self.prop_default_emit(&assign.right);
                let default_needs_proxy = prop_default_needs_proxy(&assign.right, bindable);
                let sym = self.record_named_prop_assignment_left(
                    &assign.left,
                    bindable,
                    default_lowering,
                    default_needs_proxy,
                )?;
                let default_span = assign.right.span();
                self.data.reactivity.record_prop_default_span(
                    sym,
                    svelte_span::Span::new(default_span.start, default_span.end),
                );
                Some(sym)
            }
            _ => None,
        }
    }

    fn record_named_prop_assignment_left(
        &mut self,
        pattern: &BindingPattern<'_>,
        bindable: bool,
        default_lowering: PropDefaultKind,
        default_needs_proxy: bool,
    ) -> Option<SymbolId> {
        let BindingPattern::BindingIdentifier(ident) = pattern else {
            return None;
        };
        let sym = ident.symbol_id.get()?;
        if bindable
            && matches!(self.prop_lowering_mode, PropEmitMode::Standard)
            && matches!(default_lowering, PropDefaultKind::None)
        {
            self.standard_prop_source_symbols.push(sym);
        }
        self.data.reactivity.record_prop_binding(
            sym,
            PropBindingSemantics {
                emit_mode: self.prop_lowering_mode,
                kind: PropBindingKind::Source {
                    updated: self.data.scoping.is_mutated(sym)
                        || self.data.scoping.is_reexported_specifier_local(sym),
                    default_lowering,
                    default_needs_proxy,
                },
                bindable,
            },
        );
        Some(sym)
    }

    fn record_rest_prop_pattern(&mut self, pattern: &BindingPattern<'_>) -> Option<SymbolId> {
        let BindingPattern::BindingIdentifier(ident) = pattern else {
            return None;
        };
        let sym = ident.symbol_id.get()?;
        self.data.reactivity.record_prop_binding(
            sym,
            PropBindingSemantics {
                emit_mode: self.prop_lowering_mode,
                kind: PropBindingKind::Rest,
                bindable: false,
            },
        );
        Some(sym)
    }

    fn classify_rest_prop_member_rewrite(&mut self, member: &StaticMemberExpression<'a>) {
        let Expression::Identifier(id) = &member.object else {
            return;
        };
        let Some(ref_id) = id.reference_id.get() else {
            return;
        };
        let Some(sym) = self.data.scoping.get_reference(ref_id).symbol_id() else {
            return;
        };
        let Some(excluded) = self.rest_prop_excluded.get(&sym) else {
            return;
        };
        if excluded.contains(&member.property.name) {
            return;
        }
        self.data
            .reactivity
            .record_reference_semantics(ref_id, ReferenceFacts::RestPropMemberRewrite);
    }
}

fn component_prop_emit_mode(is_custom_element: bool) -> PropEmitMode {
    if is_custom_element {
        PropEmitMode::CustomElement
    } else {
        PropEmitMode::Standard
    }
}

impl<'a> Visit<'a> for ScriptSemanticCollector<'_, 'a> {
    fn visit_program(&mut self, program: &Program<'a>) {
        if self.is_instance_program {
            for stmt in &program.body {
                legacy_reactive::collect_top_level_meta(
                    stmt,
                    self.data,
                    &mut self.legacy_reactive_labeled_nodes,
                    &mut self.legacy_reactive_implicit_names,
                    &mut self.legacy_reactive_mutated_imports,
                );
            }
        }
        walk_program(self, program);
    }

    fn visit_variable_declaration(&mut self, decl: &VariableDeclaration<'a>) {
        let previous = self.current_decl_kind.replace(decl.kind);
        walk_variable_declaration(self, decl);
        self.current_decl_kind = previous;
    }

    fn visit_export_named_declaration(&mut self, export: &ExportNamedDeclaration<'a>) {
        if self.is_instance_program {
            legacy::classify_export_named_declaration(self.data, export);
        }
        walk_export_named_declaration(self, export);
    }

    fn visit_variable_declarator(&mut self, declarator: &VariableDeclarator<'a>) {
        self.record_rune_declarator(declarator);
        if self.collect_init_proxyable
            && let BindingPattern::BindingIdentifier(id) = &declarator.id
            && let Some(init) = &declarator.init
            && let Some(symbol) = id.symbol_id.get()
        {
            let proxyable = should_proxy_node_only(init.get_inner_expression());
            self.init_proxyable.insert(symbol, proxyable);
        }
        if self.is_instance_program {
            self.record_legacy_state_declarator(declarator);
            let node = declarator.node_id();
            svelte_component_semantics::walk_bindings(&declarator.id, |v| {
                self.data
                    .reactivity
                    .record_declarator_node_for_symbol(v.symbol, node);
            });
        }
        walk_variable_declarator(self, declarator);
    }

    fn visit_assignment_pattern(&mut self, pattern: &AssignmentPattern<'a>) {
        if self.collect_init_proxyable
            && let BindingPattern::BindingIdentifier(id) = &pattern.left
            && let Some(symbol) = id.symbol_id.get()
        {
            let proxyable = should_proxy_node_only(pattern.right.get_inner_expression());
            self.init_proxyable.insert(symbol, proxyable);
        }
        walk_assignment_pattern(self, pattern);
    }

    fn visit_assignment_expression(&mut self, expr: &AssignmentExpression<'a>) {
        self.record_member_mutation_root_refs(&expr.left);
        walk_assignment_expression(self, expr);
    }

    fn visit_update_expression(&mut self, expr: &UpdateExpression<'a>) {
        if let Some(ref_id) = simple_assignment_target_member_root_reference_id(&expr.argument) {
            self.prop_member_mutation_root_refs.insert(ref_id);
        }
        walk_update_expression(self, expr);
    }

    fn visit_static_member_expression(&mut self, member: &StaticMemberExpression<'a>) {
        self.classify_rest_prop_member_rewrite(member);
        walk_static_member_expression(self, member);
    }

    fn visit_class(&mut self, class: &Class<'a>) {
        self.classify_class_fields(class);
        walk_class(self, class);
    }

    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        if self.data.reactivity.uses_runes()
            && !rune_callee_resolves_to_binding(self.data, call)
            && let Some(fact) =
                detect_rune_from_call(call).and_then(|kind| rune_call_fact(kind, call))
        {
            self.data
                .reactivity
                .record_declarator_semantics(call.node_id(), fact);
        }
        walk_call_expression(self, call);
    }
}

impl<'a> ScriptSemanticCollector<'_, 'a> {
    fn record_member_mutation_root_refs(&mut self, target: &AssignmentTarget<'a>) {
        walk_assignment_targets(target, |visit| {
            if let WriteTarget::Member(member) = visit.target
                && let Some(member_expr) = member.as_member_expression()
                && let Some(ref_id) = expression_root_reference_id(member_expr.object())
            {
                self.prop_member_mutation_root_refs.insert(ref_id);
            }
        });
    }

    fn classify_class_fields(&mut self, class: &Class<'a>) {
        if !self.data.reactivity.uses_runes() {
            return;
        }
        for element in &class.body.body {
            let ClassElement::PropertyDefinition(prop) = element else {
                continue;
            };
            if prop.r#static || prop.computed {
                continue;
            }
            if !matches!(
                &prop.key,
                PropertyKey::StaticIdentifier(_)
                    | PropertyKey::PrivateIdentifier(_)
                    | PropertyKey::StringLiteral(_)
                    | PropertyKey::NumericLiteral(_)
            ) {
                continue;
            }
            let Some(value) = prop.value.as_ref() else {
                continue;
            };
            let Expression::CallExpression(call) = value.get_inner_expression() else {
                continue;
            };
            let Some(rune_kind) = detect_rune_from_call(call) else {
                continue;
            };
            if let Some(semantics) = class_field_rune_semantics(rune_kind, call) {
                self.data
                    .reactivity
                    .record_declarator_semantics(prop.node_id(), semantics);
            }
        }

        let Some(constructor) = class.body.body.iter().find_map(|el| match el {
            ClassElement::MethodDefinition(m) if m.kind == MethodDefinitionKind::Constructor => {
                Some(m)
            }
            _ => None,
        }) else {
            return;
        };
        let Some(body) = &constructor.value.body else {
            return;
        };
        for stmt in &body.statements {
            let Statement::ExpressionStatement(es) = stmt else {
                continue;
            };
            let Expression::AssignmentExpression(assign) = es.expression.get_inner_expression()
            else {
                continue;
            };
            let this_object = match &assign.left {
                AssignmentTarget::StaticMemberExpression(member) => {
                    member.object.get_inner_expression()
                }
                AssignmentTarget::PrivateFieldExpression(member) => {
                    member.object.get_inner_expression()
                }
                AssignmentTarget::ComputedMemberExpression(member) => {
                    if !matches!(
                        member.expression.get_inner_expression(),
                        Expression::StringLiteral(_) | Expression::NumericLiteral(_)
                    ) {
                        continue;
                    }
                    member.object.get_inner_expression()
                }
                _ => continue,
            };
            if !matches!(this_object, Expression::ThisExpression(_)) {
                continue;
            }
            let Expression::CallExpression(call) = assign.right.get_inner_expression() else {
                continue;
            };
            let Some(rune_kind) = detect_rune_from_call(call) else {
                continue;
            };
            let Some(semantics) = class_field_rune_semantics(rune_kind, call) else {
                continue;
            };
            self.data
                .reactivity
                .record_declarator_semantics(assign.node_id(), semantics);
        }
    }
}

fn class_field_rune_semantics(
    rune_kind: RuneKind,
    call: &CallExpression<'_>,
) -> Option<DeclaratorSemantics> {
    match rune_kind {
        RuneKind::State => Some(DeclaratorSemantics::ClassFieldState(
            ClassFieldStateSemantics {
                kind: StateKind::State,
                proxied: state_initializer_is_proxyable(call),
            },
        )),
        RuneKind::StateRaw => Some(DeclaratorSemantics::ClassFieldState(
            ClassFieldStateSemantics {
                kind: StateKind::StateRaw,
                proxied: false,
            },
        )),
        RuneKind::Derived => Some(DeclaratorSemantics::ClassFieldDerived(
            ClassFieldDerivedSemantics {
                kind: DerivedKind::Derived,
                async_kind: derived_async_kind(call),
            },
        )),
        RuneKind::DerivedBy => Some(DeclaratorSemantics::ClassFieldDerived(
            ClassFieldDerivedSemantics {
                kind: DerivedKind::DerivedBy,
                async_kind: derived_async_kind(call),
            },
        )),
        _ => None,
    }
}

fn rune_call<'a>(
    declarator: &'a VariableDeclarator<'a>,
) -> Option<(&'a CallExpression<'a>, RuneKind)> {
    let Expression::CallExpression(call) = declarator.init.as_ref()?.get_inner_expression() else {
        return None;
    };
    let rune_kind = detect_rune_from_call(call)?;
    matches!(
        rune_kind,
        RuneKind::State
            | RuneKind::StateRaw
            | RuneKind::StateEager
            | RuneKind::Derived
            | RuneKind::DerivedBy
            | RuneKind::Props
            | RuneKind::PropsId
            | RuneKind::EffectTracking
            | RuneKind::EffectPending
            | RuneKind::Host
            | RuneKind::InspectTrace
    )
    .then_some((call, rune_kind))
}

fn rune_callee_resolves_to_binding(data: &AnalysisData<'_>, call: &CallExpression<'_>) -> bool {
    let Some(ref_id) = expression_root_reference_id(&call.callee) else {
        return false;
    };
    data.scoping
        .semantics()
        .symbol_for_reference(ref_id)
        .is_some()
}

fn state_initializer_is_proxyable(call: &CallExpression<'_>) -> bool {
    call.arguments
        .first()
        .and_then(|arg| arg.as_expression())
        .is_some_and(state_expression_is_proxyable)
}

fn state_expression_is_proxyable(expr: &Expression<'_>) -> bool {
    let expr = expr.get_inner_expression();
    if expr.is_literal() {
        return false;
    }

    if matches!(
        expr,
        Expression::TemplateLiteral(_)
            | Expression::ArrowFunctionExpression(_)
            | Expression::FunctionExpression(_)
            | Expression::UnaryExpression(_)
            | Expression::BinaryExpression(_)
    ) {
        return false;
    }

    if let Expression::Identifier(id) = expr
        && id.name == JS_UNDEFINED_NAME
    {
        return false;
    }

    true
}

fn binding_is_signal_source(
    scoping: &ComponentScoping<'_>,
    script: &crate::ScriptAnalysis,
    sym: SymbolId,
    kind: StateKind,
) -> bool {
    if matches!(kind, StateKind::StateEager) {
        return false;
    }
    let reassigned = scoping.is_mutated(sym) || scoping.is_reexported_specifier_local(sym);
    script.is_state_source(reassigned)
}

fn prop_default_is_bindable(expr: &Expression<'_>) -> bool {
    let Expression::CallExpression(call) = expr.get_inner_expression() else {
        return false;
    };
    detect_rune_from_call(call) == Some(RuneKind::Bindable)
}

impl<'d, 'a> ScriptSemanticCollector<'d, 'a> {
    fn prop_default_emit(&self, expr: &Expression<'_>) -> PropDefaultKind {
        let bindable_arg = bindable_default_arg(expr);
        let default_expr = bindable_arg.unwrap_or(expr);
        if bindable_arg.is_none() && prop_default_is_bindable(expr) {
            return PropDefaultKind::None;
        }
        if bindable_arg.is_some() && state_expression_is_proxyable(default_expr) {
            return PropDefaultKind::Lazy;
        }
        if !is_simple_expression(default_expr) {
            return PropDefaultKind::Lazy;
        }
        if self.prop_default_has_reactive_ref(default_expr) {
            return PropDefaultKind::Lazy;
        }
        PropDefaultKind::Eager
    }

    fn prop_default_has_reactive_ref(&self, expr: &Expression<'_>) -> bool {
        match expr.get_inner_expression() {
            Expression::Identifier(id) => id.reference_id.get().is_some_and(|ref_id| {
                reference_is_reactive(
                    &self.data.reactivity,
                    &self.data.scoping,
                    ref_id,
                    ReferenceReactivityMode::PropDefault,
                )
            }),
            Expression::BinaryExpression(b) => {
                self.prop_default_has_reactive_ref(&b.left)
                    || self.prop_default_has_reactive_ref(&b.right)
            }
            Expression::LogicalExpression(l) => {
                self.prop_default_has_reactive_ref(&l.left)
                    || self.prop_default_has_reactive_ref(&l.right)
            }
            Expression::ConditionalExpression(c) => {
                self.prop_default_has_reactive_ref(&c.test)
                    || self.prop_default_has_reactive_ref(&c.consequent)
                    || self.prop_default_has_reactive_ref(&c.alternate)
            }
            _ => false,
        }
    }
}

fn prop_default_needs_proxy(expr: &Expression<'_>, bindable: bool) -> bool {
    bindable && state_expression_is_proxyable(bindable_default_arg(expr).unwrap_or(expr))
}

fn bindable_default_arg<'a>(expr: &'a Expression<'a>) -> Option<&'a Expression<'a>> {
    let Expression::CallExpression(call) = expr.get_inner_expression() else {
        return None;
    };
    if detect_rune_from_call(call) != Some(RuneKind::Bindable) {
        return None;
    }
    call.arguments.first().and_then(|arg| arg.as_expression())
}

fn is_simple_expression(expr: &Expression<'_>) -> bool {
    match expr.get_inner_expression() {
        Expression::Identifier(_)
        | Expression::NullLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::StringLiteral(_)
        | Expression::NumericLiteral(_)
        | Expression::BigIntLiteral(_)
        | Expression::RegExpLiteral(_)
        | Expression::ArrowFunctionExpression(_)
        | Expression::FunctionExpression(_) => true,
        Expression::ConditionalExpression(c) => {
            is_simple_expression(&c.test)
                && is_simple_expression(&c.consequent)
                && is_simple_expression(&c.alternate)
        }
        Expression::BinaryExpression(b) => {
            is_simple_expression(&b.left) && is_simple_expression(&b.right)
        }
        Expression::LogicalExpression(l) => {
            is_simple_expression(&l.left) && is_simple_expression(&l.right)
        }
        _ => false,
    }
}

fn derived_async_kind(call: &CallExpression<'_>) -> DerivedAsyncKind {
    let has_await = call
        .arguments
        .first()
        .and_then(|arg| arg.as_expression())
        .is_some_and(expression_has_await);
    if has_await {
        DerivedAsyncKind::Async
    } else {
        DerivedAsyncKind::Sync
    }
}

fn derived_source_reference(call: &CallExpression<'_>) -> Option<ReferenceId> {
    let arg = call.arguments.first()?.as_expression()?;
    let Expression::Identifier(id) = arg.get_inner_expression() else {
        return None;
    };
    id.reference_id.get()
}

fn pattern_binding_symbols(pattern: &BindingPattern<'_>) -> Vec<SymbolId> {
    let mut out = Vec::new();
    fn recur(pattern: &BindingPattern<'_>, out: &mut Vec<SymbolId>) {
        match pattern {
            BindingPattern::BindingIdentifier(ident) => {
                if let Some(sym) = ident.symbol_id.get() {
                    out.push(sym);
                }
            }
            BindingPattern::ObjectPattern(obj) => {
                for prop in &obj.properties {
                    recur(&prop.value, out);
                }
                if let Some(rest) = &obj.rest {
                    recur(&rest.argument, out);
                }
            }
            BindingPattern::ArrayPattern(arr) => {
                for elem in arr.elements.iter().flatten() {
                    recur(elem, out);
                }
                if let Some(rest) = &arr.rest {
                    recur(&rest.argument, out);
                }
            }
            BindingPattern::AssignmentPattern(assign) => {
                recur(&assign.left, out);
            }
        }
    }
    recur(pattern, &mut out);
    out
}

pub(super) struct RefCollector<'s> {
    pub(super) refs: &'s mut SmallVec<[ReferenceId; 4]>,
    pub(super) reactive_rune_call: &'s mut bool,
    pub(super) has_impure: &'s mut bool,
}

impl<'a> Visit<'a> for RefCollector<'_> {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        if let Some(ref_id) = ident.reference_id.get() {
            self.refs.push(ref_id);
        }
    }
    fn visit_member_expression(&mut self, expr: &MemberExpression<'a>) {
        *self.has_impure = true;
        walk_member_expression(self, expr);
    }
    fn visit_new_expression(&mut self, expr: &NewExpression<'a>) {
        *self.has_impure = true;
        walk_new_expression(self, expr);
    }
    fn visit_tagged_template_expression(&mut self, expr: &TaggedTemplateExpression<'a>) {
        *self.has_impure = true;
        walk_tagged_template_expression(self, expr);
    }
    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        *self.has_impure = true;
        if let Some(rune) = detect_rune_from_call(call)
            && matches!(
                rune,
                RuneKind::EffectPending
                    | RuneKind::EffectTracking
                    | RuneKind::PropsId
                    | RuneKind::Host
                    | RuneKind::InspectTrace
            )
        {
            *self.reactive_rune_call = true;
        }
        walk_call_expression(self, call);
    }
}
