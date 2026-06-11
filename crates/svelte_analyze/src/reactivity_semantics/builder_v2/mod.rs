mod contextual;

mod import_subscribed;
mod legacy;
mod legacy_reactive;
mod references;
mod store;
mod util;

pub(crate) use util::expression_root_reference_id;

use util::{
    assignment_target_member_root_reference_id, property_key_atom,
    simple_assignment_target_member_root_reference_id,
};

use super::data::{
    BindingFacts, BindingSemantics, ClassFieldDerivedSemantics, ClassFieldStateSemantics,
    DeclaratorSemantics, DerivedDeclarationSemantics, DerivedEmit, DerivedKind,
    OptimizedRuneSemantics, PropBindingKind, PropBindingSemantics, PropDefaultKind, PropEmitMode,
    ReactivitySemantics, ReferenceFacts, RuntimeRuneKind, StateDeclarationSemantics, StateKind,
};
use crate::reactivity_semantics::script_info::detect_rune_from_call;
use crate::scope::{ComponentScoping, SymbolId};
use crate::types::data::{AnalysisData, JsAst};
use crate::types::script::RuneKind;
use crate::utils::expression_has_await;
use crate::utils::is_let_or_var;
use crate::value_evaluation::{Evaluation, ValueEvaluation};
use oxc_ast::ast::{
    AssignmentExpression, AssignmentTarget, BindingPattern, CallExpression, Class, ClassElement,
    ExportNamedDeclaration, Expression, IdentifierReference, MemberExpression,
    MethodDefinitionKind, NewExpression, Program, PropertyKey, Statement, StaticMemberExpression,
    TaggedTemplateExpression, UpdateExpression, VariableDeclaration, VariableDeclarationKind,
    VariableDeclarator,
};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{
    walk_assignment_expression, walk_call_expression, walk_class, walk_export_named_declaration,
    walk_member_expression, walk_new_expression, walk_program, walk_static_member_expression,
    walk_tagged_template_expression, walk_update_expression, walk_variable_declaration,
    walk_variable_declarator,
};
use std::mem;

use oxc_span::Ident;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use svelte_ast::Component;
use svelte_component_semantics::{ComponentSemantics, OxcNodeId, ReferenceId, sym_state};

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
    let runes_mode = super::mode_resolution::resolve(
        &data.scoping,
        parsed,
        inputs.inline_runes,
        inputs.compile_runes,
    );
    data.script.runes_mode = runes_mode;
    let runes = runes_mode.is_runes();
    data.script.immutable = runes || inputs.immutable;
    data.script.accessors = data.output.is_custom_element_target || (!runes && inputs.accessors);

    data.reactivity.set_uses_runes(runes);
    data.reactivity.set_runes_mode(runes_mode);
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

    let reference_count = data.scoping.references_len();
    data.reactivity.reserve_references(reference_count);
    references::collect_each_key_contextual_reads(component, parsed, data);
    references::collect_symbol_semantics(data);
    compute_const_tag_reactivity(component, parsed, data);

    legacy::register_legacy_synthetic_objects(data);
    legacy::finalize_legacy_aggregates(data);
    legacy_reactive::classify_mutated_import_references(data);
    import_subscribed::classify_import_subscribed_reads(data);
}

pub(crate) fn build_optimized_derived(
    reactivity: &mut ReactivitySemantics,
    value_evaluation: &ValueEvaluation,
    semantics: &ComponentSemantics<'_>,
) {
    let mut optimizable = Vec::new();

    for symbol in semantics.symbol_ids() {
        if !matches!(
            reactivity.binding_semantics(symbol),
            BindingSemantics::Derived(_)
        ) {
            continue;
        }
        if !matches!(value_evaluation.evaluation(symbol), Evaluation::Known(_)) {
            continue;
        }
        optimizable.push(symbol);
    }

    reactivity.optimize_derived_rune(&optimizable);
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
    use super::data::{BindingSemantics, ConstBindingSemantics, ReferenceSemantics};
    if matches!(
        reactivity.reference_semantics(ref_id),
        ReferenceSemantics::StoreRead { .. }
            | ReferenceSemantics::StoreWrite { .. }
            | ReferenceSemantics::StoreUpdate { .. }
    ) {
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
        BindingSemantics::Const(ConstBindingSemantics::ConstTag { reactive, .. }) => reactive,
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
            emit: derived_emit(call),
        }),
        RuneKind::DerivedBy => Some(DeclaratorSemantics::RuneDerived {
            kind: DerivedKind::DerivedBy,
            emit: derived_emit(call),
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

fn compute_const_tag_reactivity<'a>(
    component: &Component,
    parsed: &JsAst<'a>,
    data: &mut AnalysisData<'a>,
) {
    use super::data::ConstBindingSemantics;
    use svelte_component_semantics::walk_bindings;

    for node in component.store.iter_nodes() {
        let svelte_ast::Node::ConstTag(tag) = node else {
            continue;
        };
        let Some(stmt) = parsed.stmt(tag.decl.id()) else {
            continue;
        };

        let Statement::VariableDeclaration(decl) = stmt else {
            continue;
        };
        let Some(declarator) = decl.declarations.first() else {
            continue;
        };
        let mut syms: Vec<SymbolId> = Vec::new();
        walk_bindings(&declarator.id, |v| syms.push(v.symbol));
        if syms.is_empty() {
            continue;
        }

        let emit = match declarator.init.as_ref() {
            Some(init) if expression_has_await(init) => DerivedEmit::Async,
            _ => DerivedEmit::Sync,
        };
        data.reactivity
            .record_declarator_semantics(decl.node_id(), DeclaratorSemantics::ConstTag { emit });

        let mut refs: SmallVec<[ReferenceId; 4]> = SmallVec::new();
        let mut eager_rune = false;
        let mut collector = RefCollector {
            refs: &mut refs,
            reactive_rune_call: &mut eager_rune,
        };
        collector.visit_statement(stmt);

        let init_is_impure_member_or_call = declarator
            .init
            .as_ref()
            .is_some_and(init_expression_is_impure);

        let is_destructured =
            syms.len() > 1 || !matches!(&declarator.id, BindingPattern::BindingIdentifier(_));
        let legacy_destructured_signal_read = is_destructured && !data.script.runes();

        let reactive = legacy_destructured_signal_read
            || eager_rune
            || init_is_impure_member_or_call
            || refs.iter().any(|&ref_id| {
                reference_is_reactive(
                    &data.reactivity,
                    &data.scoping,
                    ref_id,
                    ReferenceReactivityMode::General,
                )
            });

        for sym in syms {
            if let Some(BindingFacts::Const(ConstBindingSemantics::ConstTag {
                reactive: r, ..
            })) = data.reactivity.binding_facts_mut(sym)
            {
                *r = reactive;
            }
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

fn init_expression_is_impure(expr: &Expression<'_>) -> bool {
    struct ImpureProbe {
        has: bool,
    }
    impl<'a> Visit<'a> for ImpureProbe {
        fn visit_member_expression(&mut self, expr: &MemberExpression<'a>) {
            self.has = true;
            walk_member_expression(self, expr);
        }
        fn visit_call_expression(&mut self, expr: &CallExpression<'a>) {
            self.has = true;
            walk_call_expression(self, expr);
        }
        fn visit_new_expression(&mut self, expr: &NewExpression<'a>) {
            self.has = true;
            walk_new_expression(self, expr);
        }
        fn visit_tagged_template_expression(&mut self, expr: &TaggedTemplateExpression<'a>) {
            self.has = true;
            walk_tagged_template_expression(self, expr);
        }
    }
    let mut p = ImpureProbe { has: false };
    p.visit_expression(expr);
    p.has
}

fn build_script_semantics_v2<'a>(
    component: &Component,
    parsed: &JsAst<'a>,
    data: &mut AnalysisData<'a>,
    prop_lowering_mode: PropEmitMode,
) -> LegacyReactiveCollected {
    let reactive_body_refs = parsed
        .program
        .as_ref()
        .map(|program| legacy_reactive::collect_reactive_body_symbol_refs(program, data))
        .unwrap_or_default();
    let mut collector = ScriptSemanticCollector::new(data, prop_lowering_mode, reactive_body_refs);
    collector.collect_bind_this_legacy_state_promotion_roots(component, parsed);
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
    collector.mark_bind_member_mutation_roots(component, parsed);
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
}

impl<'d, 'a> ScriptSemanticCollector<'d, 'a> {
    fn new(
        data: &'d mut AnalysisData<'a>,
        prop_lowering_mode: PropEmitMode,
        reactive_body_refs: FxHashSet<SymbolId>,
    ) -> Self {
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

    fn collect_bind_this_legacy_state_promotion_roots(
        &mut self,
        component: &Component,
        parsed: &JsAst<'a>,
    ) {
        if self.data.script.runes() {
            return;
        }
        for node in component.store.iter_nodes() {
            let attrs: &[svelte_ast::Attribute] = match node {
                svelte_ast::Node::Element(n) => &n.attributes,
                svelte_ast::Node::SvelteElement(n) => &n.attributes,
                svelte_ast::Node::ComponentNode(n) => &n.attributes,
                svelte_ast::Node::SvelteComponentLegacy(n) => &n.attributes,
                svelte_ast::Node::SvelteSelf(n) => &n.attributes,
                _ => continue,
            };
            for attr in attrs {
                let svelte_ast::Attribute::BindDirective(d) = attr else {
                    continue;
                };
                if d.name != "this" {
                    continue;
                }
                let Some(expr) = parsed.expr(d.expression.id()) else {
                    continue;
                };
                if !matches!(expr, Expression::Identifier(_)) {
                    continue;
                }
                let Some(ref_id) = expression_root_reference_id(expr) else {
                    continue;
                };
                let Some(sym) = self.data.scoping.symbol_for_reference(ref_id) else {
                    continue;
                };
                self.bind_this_legacy_state_root_syms.insert(sym);
            }
        }
    }

    fn mark_bind_member_mutation_roots(&mut self, component: &Component, parsed: &JsAst<'a>) {
        for node in component.store.iter_nodes() {
            let attrs: &[svelte_ast::Attribute] = match node {
                svelte_ast::Node::Element(n) => &n.attributes,
                svelte_ast::Node::SvelteElement(n) => &n.attributes,
                svelte_ast::Node::ComponentNode(n) => &n.attributes,
                svelte_ast::Node::SvelteComponentLegacy(n) => &n.attributes,
                svelte_ast::Node::SvelteSelf(n) => &n.attributes,
                svelte_ast::Node::SvelteWindow(n) => &n.attributes,
                svelte_ast::Node::SvelteDocument(n) => &n.attributes,
                svelte_ast::Node::SvelteBody(n) => &n.attributes,
                svelte_ast::Node::SlotElementLegacy(n) => &n.attributes,
                _ => continue,
            };
            for attr in attrs {
                let svelte_ast::Attribute::BindDirective(d) = attr else {
                    continue;
                };
                let Some(expr) = parsed.expr(d.expression.id()) else {
                    continue;
                };
                if !matches!(
                    expr,
                    Expression::StaticMemberExpression(_) | Expression::ComputedMemberExpression(_)
                ) {
                    continue;
                }
                if let Some(ref_id) = expression_root_reference_id(expr) {
                    self.prop_member_mutation_root_refs.insert(ref_id);
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

        {
            let imports = self.legacy_reactive_mutated_imports.clone();
            let lr = self.data.reactivity.legacy_reactive_mut();
            for sym in &imports {
                lr.add_mutated_import(*sym);
            }
        }

        store::collect_store_declarations(self.data);

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
                use super::data::ReferenceSemantics;
                if matches!(
                    self.data.reactivity.reference_semantics(r),
                    ReferenceSemantics::StoreRead { .. }
                        | ReferenceSemantics::StoreWrite { .. }
                        | ReferenceSemantics::StoreUpdate { .. }
                ) {
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

    fn record_rune_declarator(&mut self, declarator: &VariableDeclarator<'a>) {
        if !self.data.script.runes() {
            return;
        }
        let Some((call, rune_kind)) = rune_call(declarator) else {
            return;
        };
        let root_node = declarator.node_id();

        let var_declared = matches!(self.current_decl_kind, Some(VariableDeclarationKind::Var));
        let init_proxyable =
            matches!(rune_kind, RuneKind::State) && state_initializer_is_proxyable(call);

        let is_destructure = !matches!(&declarator.id, BindingPattern::BindingIdentifier(_));

        match rune_kind {
            RuneKind::State => {
                if is_destructure {
                    self.data.reactivity.record_declarator_semantics(
                        root_node,
                        DeclaratorSemantics::RuneState {
                            kind: StateKind::State,
                        },
                    );
                }
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
                if is_destructure {
                    self.data.reactivity.record_declarator_semantics(
                        root_node,
                        DeclaratorSemantics::RuneState {
                            kind: StateKind::StateRaw,
                        },
                    );
                }
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
                let emit = derived_emit(call);
                if is_destructure {
                    self.data.reactivity.record_declarator_semantics(
                        root_node,
                        DeclaratorSemantics::RuneDerived {
                            kind: DerivedKind::Derived,
                            emit,
                        },
                    );
                }
                self.record_derived_pattern(
                    &declarator.id,
                    DerivedDeclarationSemantics {
                        kind: DerivedKind::Derived,
                        emit,
                    },
                );
                self.collect_derived_init_refs(declarator, RuneKind::Derived);
            }
            RuneKind::DerivedBy => {
                let emit = derived_emit(call);
                if is_destructure {
                    self.data.reactivity.record_declarator_semantics(
                        root_node,
                        DeclaratorSemantics::RuneDerived {
                            kind: DerivedKind::DerivedBy,
                            emit,
                        },
                    );
                }
                self.record_derived_pattern(
                    &declarator.id,
                    DerivedDeclarationSemantics {
                        kind: DerivedKind::DerivedBy,
                        emit,
                    },
                );
                self.collect_derived_init_refs(declarator, RuneKind::DerivedBy);
            }
            RuneKind::Props => {
                self.record_props_pattern(&declarator.id, root_node);
            }
            RuneKind::PropsId => {
                self.record_runtime_rune_pattern(&declarator.id, RuntimeRuneKind::PropsId);
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
            if !self.data.scoping.is_member_mutated(sym)
                && !self.data.scoping.is_mutated_any(sym)
                && !self.bind_this_legacy_state_root_syms.contains(&sym)
            {
                continue;
            }
            self.data.reactivity.record_legacy_state_binding(
                sym,
                super::data::LegacyStateSemantics {
                    var_declared: false,
                    immutable,
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
                if !self.data.scoping.is_member_mutated(sym)
                    && !self.data.scoping.is_mutated_any(sym)
                    && !self.bind_this_legacy_state_root_syms.contains(&sym)
                {
                    continue;
                }
                self.data.reactivity.record_legacy_state_binding(
                    sym,
                    super::data::LegacyStateSemantics {
                        var_declared: false,
                        immutable,
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
        let mut visitor = RefCollector {
            refs: &mut refs,
            reactive_rune_call: &mut reactive_rune_call,
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
                        bindable: false,
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
                    },
                );
                Some(sym)
            }
            BindingPattern::AssignmentPattern(assign) => {
                let bindable = prop_default_is_bindable(&assign.right);
                let default_lowering = self.prop_default_emit(&assign.right);
                let default_needs_proxy = prop_default_needs_proxy(&assign.right, bindable);
                self.record_named_prop_assignment_left(
                    &assign.left,
                    bindable,
                    default_lowering,
                    default_needs_proxy,
                )
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
                    bindable,
                    updated: self.data.scoping.is_mutated(sym)
                        || self.data.scoping.is_reexported_specifier_local(sym),
                    default_lowering,
                    default_needs_proxy,
                },
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
        self.record_legacy_state_declarator(declarator);
        walk_variable_declarator(self, declarator);
    }

    fn visit_assignment_expression(&mut self, expr: &AssignmentExpression<'a>) {
        if let Some(ref_id) = assignment_target_member_root_reference_id(&expr.left) {
            self.prop_member_mutation_root_refs.insert(ref_id);
        }
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
                PropertyKey::StaticIdentifier(_) | PropertyKey::PrivateIdentifier(_)
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
                emit: derived_emit(call),
            },
        )),
        RuneKind::DerivedBy => Some(DeclaratorSemantics::ClassFieldDerived(
            ClassFieldDerivedSemantics {
                kind: DerivedKind::DerivedBy,
                emit: derived_emit(call),
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
        let default_expr = bindable_default_arg(expr).unwrap_or(expr);
        if bindable_default_arg(expr).is_none() && prop_default_is_bindable(expr) {
            return PropDefaultKind::None;
        }
        if !is_simple_expression(default_expr) {
            return PropDefaultKind::Lazy;
        }
        if let Expression::Identifier(id) = default_expr.get_inner_expression()
            && let Some(ref_id) = id.reference_id.get()
            && reference_is_reactive(
                &self.data.reactivity,
                &self.data.scoping,
                ref_id,
                ReferenceReactivityMode::PropDefault,
            )
        {
            return PropDefaultKind::Lazy;
        }
        PropDefaultKind::Eager
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
    matches!(
        expr.get_inner_expression(),
        Expression::Identifier(_)
            | Expression::NullLiteral(_)
            | Expression::BooleanLiteral(_)
            | Expression::StringLiteral(_)
            | Expression::NumericLiteral(_)
            | Expression::BigIntLiteral(_)
            | Expression::RegExpLiteral(_)
            | Expression::TemplateLiteral(_)
            | Expression::ArrowFunctionExpression(_)
            | Expression::FunctionExpression(_)
    )
}

fn derived_emit(call: &CallExpression<'_>) -> DerivedEmit {
    let has_await = call
        .arguments
        .first()
        .and_then(|arg| arg.as_expression())
        .is_some_and(expression_has_await);
    if has_await {
        DerivedEmit::Async
    } else {
        DerivedEmit::Sync
    }
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

struct RefCollector<'s> {
    refs: &'s mut SmallVec<[ReferenceId; 4]>,
    reactive_rune_call: &'s mut bool,
}

impl<'a> Visit<'a> for RefCollector<'_> {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        if let Some(ref_id) = ident.reference_id.get() {
            self.refs.push(ref_id);
        }
    }
    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
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
