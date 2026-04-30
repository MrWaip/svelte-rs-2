mod contextual;

mod legacy;
mod legacy_reactive;
mod references;
mod store;
mod util;

use util::{
    assignment_target_member_root_reference_id, property_key_atom,
    simple_assignment_target_member_root_reference_id,
};

use super::data::{
    BindingFacts, DeclaratorSemantics, DerivedDeclarationSemantics, DerivedKind, DerivedLowering,
    OptimizedRuneSemantics, PropBindingKind, PropBindingSemantics, PropDefaultLowering,
    PropLoweringMode, ReferenceFacts, RuntimeRuneKind, StateBindingSemantics,
    StateDeclarationSemantics, StateKind,
};
use crate::scope::{ComponentScoping, SymbolId};
use crate::types::data::{AnalysisData, JsAst};
use crate::types::script::RuneKind;
use crate::utils::script_info::detect_rune_from_call;
use oxc_ast::ast::{
    AssignmentExpression, BindingPattern, CallExpression, Expression, Statement,
    StaticMemberExpression, UpdateExpression, VariableDeclaration, VariableDeclarationKind,
    VariableDeclarator,
};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{
    walk_static_member_expression, walk_variable_declaration, walk_variable_declarator,
};
use oxc_span::Ident;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use svelte_ast::Component;
use svelte_component_semantics::{OxcNodeId, ReferenceId};

const JS_UNDEFINED_NAME: &str = "undefined";

pub(crate) fn build_v2<'a>(component: &Component, parsed: &JsAst<'a>, data: &mut AnalysisData<'a>) {
    data.reactivity.set_uses_runes(data.script.runes);
    let lr_collected =
        build_script_semantics_v2(parsed, data, component_prop_lowering_mode(component));
    contextual::collect_template_declarations(component, parsed, data);

    legacy_reactive::build_from_collected(
        data,
        lr_collected.labeled_nodes,
        lr_collected.implicit_names,
        lr_collected.mutated_imports,
    );

    let reference_count = data.scoping.references_len();
    data.reactivity.reserve_references(reference_count);
    references::collect_symbol_semantics(data);
    compute_const_tag_reactivity(component, parsed, data);

    legacy::classify_unresolved_legacy_identifiers(data);
    legacy::finalize_legacy_aggregates(data);
    legacy_reactive::classify_mutated_import_references(data);
}

pub(super) struct LegacyReactiveCollected {
    pub labeled_nodes: Vec<OxcNodeId>,
    pub implicit_names: Vec<compact_str::CompactString>,
    pub mutated_imports: SmallVec<[SymbolId; 2]>,
}

fn compute_const_tag_reactivity<'a>(
    component: &Component,
    parsed: &JsAst<'a>,
    data: &mut AnalysisData<'a>,
) {
    use super::data::{BindingSemantics, ConstBindingSemantics};
    use svelte_component_semantics::walk_bindings;

    let tag_ids: Vec<svelte_ast::NodeId> = data
        .template
        .const_tags
        .by_fragment
        .values()
        .flatten()
        .copied()
        .collect();
    if tag_ids.is_empty() {
        return;
    }
    for tag_id in tag_ids {
        let svelte_ast::Node::ConstTag(tag) = component.store.get(tag_id) else {
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

        let mut refs: SmallVec<[ReferenceId; 4]> = SmallVec::new();
        let mut eager_rune = false;
        let mut collector = RefCollector {
            refs: &mut refs,
            reactive_rune_call: &mut eager_rune,
        };
        collector.visit_statement(stmt);

        let reactive = eager_rune
            || refs.iter().any(|&ref_id| {
                let Some(sym) = data.scoping.symbol_for_reference(ref_id) else {
                    return false;
                };
                let decl = data.reactivity.binding_semantics(sym);
                match decl {
                    BindingSemantics::State(_)
                    | BindingSemantics::Prop(_)
                    | BindingSemantics::LegacyBindableProp(_)
                    | BindingSemantics::LegacyState(_)
                    | BindingSemantics::Store(_)
                    | BindingSemantics::Contextual(_)
                    | BindingSemantics::RuntimeRune { .. } => true,
                    BindingSemantics::Derived(d) => d.reactive,
                    BindingSemantics::Const(ConstBindingSemantics::ConstTag {
                        reactive, ..
                    }) => reactive,
                    BindingSemantics::OptimizedRune(opt) if opt.proxy_init => true,
                    BindingSemantics::NonReactive
                    | BindingSemantics::Unresolved
                    | BindingSemantics::OptimizedRune(_) => {
                        !data.scoping.is_component_top_level_symbol(sym)
                    }
                }
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

fn build_script_semantics_v2<'a>(
    parsed: &JsAst<'a>,
    data: &mut AnalysisData<'a>,
    prop_lowering_mode: PropLoweringMode,
) -> LegacyReactiveCollected {
    let mut collector = ScriptSemanticCollector::new(data, prop_lowering_mode);
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
    let labeled_nodes = std::mem::take(&mut collector.legacy_reactive_labeled_nodes);
    let implicit_names = std::mem::take(&mut collector.legacy_reactive_implicit_names);
    let mutated_imports = std::mem::take(&mut collector.legacy_reactive_mutated_imports);
    collector.finish();
    LegacyReactiveCollected {
        labeled_nodes,
        implicit_names,
        mutated_imports,
    }
}

struct ScriptSemanticCollector<'d, 'a> {
    data: &'d mut AnalysisData<'a>,
    current_decl_kind: Option<VariableDeclarationKind>,
    prop_lowering_mode: PropLoweringMode,

    prop_member_mutation_root_refs: FxHashSet<ReferenceId>,

    rest_prop_excluded: FxHashMap<SymbolId, FxHashSet<Ident<'a>>>,

    derived_init_refs: FxHashMap<SymbolId, SmallVec<[ReferenceId; 4]>>,

    eager_reactive_derived: FxHashSet<SymbolId>,

    is_instance_program: bool,
    legacy_reactive_labeled_nodes: Vec<OxcNodeId>,
    legacy_reactive_implicit_names: Vec<compact_str::CompactString>,
    legacy_reactive_mutated_imports: SmallVec<[SymbolId; 2]>,
}

impl<'d, 'a> ScriptSemanticCollector<'d, 'a> {
    fn new(data: &'d mut AnalysisData<'a>, prop_lowering_mode: PropLoweringMode) -> Self {
        Self {
            data,
            current_decl_kind: None,
            prop_lowering_mode,
            prop_member_mutation_root_refs: FxHashSet::default(),
            rest_prop_excluded: FxHashMap::default(),
            derived_init_refs: FxHashMap::default(),
            eager_reactive_derived: FxHashSet::default(),
            is_instance_program: false,
            legacy_reactive_labeled_nodes: Vec::new(),
            legacy_reactive_implicit_names: Vec::new(),
            legacy_reactive_mutated_imports: SmallVec::new(),
        }
    }

    fn visit_instance_program(&mut self, program: &oxc_ast::ast::Program<'a>) {
        self.is_instance_program = true;
        self.visit_program(program);
        self.is_instance_program = false;
    }

    fn visit_module_program(&mut self, program: &oxc_ast::ast::Program<'a>) {
        debug_assert!(!self.is_instance_program);
        self.visit_program(program);
    }

    fn finish(mut self) {
        let member_mutated_syms: Vec<SymbolId> = self
            .data
            .scoping
            .semantics()
            .symbols_with_state(svelte_component_semantics::sym_state::MEMBER_MUTATED)
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
            .record_prop_member_mutation_root_refs(std::mem::take(
                &mut self.prop_member_mutation_root_refs,
            ));
        store::collect_store_declarations(self.data);
        self.compute_derived_reactivity();
    }

    fn compute_derived_reactivity(&mut self) {
        if self.derived_init_refs.is_empty() && self.eager_reactive_derived.is_empty() {
            return;
        }
        let entries: Vec<(SymbolId, SmallVec<[ReferenceId; 4]>)> =
            self.derived_init_refs.drain().collect();
        let eager = std::mem::take(&mut self.eager_reactive_derived);

        loop {
            let mut changed = false;
            for (sym, refs) in &entries {
                let current_reactive = match self.data.reactivity.binding_facts(*sym) {
                    Some(BindingFacts::Derived(d)) => d.reactive,
                    _ => continue,
                };
                let new_reactive =
                    eager.contains(sym) || refs.iter().any(|&r| self.is_reference_reactive(r));
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

    fn is_reference_reactive(&self, ref_id: ReferenceId) -> bool {
        use super::data::{BindingSemantics, ConstBindingSemantics};
        let Some(sym) = self.data.scoping.symbol_for_reference(ref_id) else {
            return false;
        };
        let decl = self.data.reactivity.binding_semantics(sym);
        match decl {
            BindingSemantics::State(_)
            | BindingSemantics::Prop(_)
            | BindingSemantics::LegacyBindableProp(_)
            | BindingSemantics::LegacyState(_)
            | BindingSemantics::Store(_)
            | BindingSemantics::Contextual(_)
            | BindingSemantics::RuntimeRune { .. } => true,
            BindingSemantics::Derived(d) => d.reactive,
            BindingSemantics::Const(ConstBindingSemantics::ConstTag { reactive, .. }) => reactive,

            BindingSemantics::OptimizedRune(opt) if opt.proxy_init => true,
            BindingSemantics::NonReactive
            | BindingSemantics::Unresolved
            | BindingSemantics::OptimizedRune(_) => {
                !self.data.scoping.is_component_top_level_symbol(sym)
            }
        }
    }

    fn record_rune_declarator(&mut self, declarator: &VariableDeclarator<'a>) {
        let Some((call, rune_kind)) = rune_call(declarator) else {
            return;
        };
        let root_node = declarator.node_id();

        let var_declared = matches!(self.current_decl_kind, Some(VariableDeclarationKind::Var));
        let init_proxyable =
            matches!(rune_kind, RuneKind::State) && state_initializer_is_proxyable(call);

        match rune_kind {
            RuneKind::State => {
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
                        binding_semantics: collect_state_binding_semantics(
                            &self.data.scoping,
                            &declarator.id,
                            StateKind::State,
                            init_proxyable,
                        ),
                    },
                    true,
                );
            }
            RuneKind::StateRaw => {
                self.record_state_root_declaration(
                    &declarator.id,
                    root_node,
                    StateDeclarationSemantics {
                        kind: StateKind::StateRaw,
                        proxied: false,
                        var_declared,
                        binding_semantics: collect_state_binding_semantics(
                            &self.data.scoping,
                            &declarator.id,
                            StateKind::StateRaw,
                            false,
                        ),
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
                        binding_semantics: collect_state_binding_semantics(
                            &self.data.scoping,
                            &declarator.id,
                            StateKind::StateEager,
                            false,
                        ),
                    },
                    false,
                );
            }
            RuneKind::Derived => {
                let lowering = derived_lowering(call, rune_kind);
                self.record_derived_pattern(
                    &declarator.id,
                    DerivedDeclarationSemantics {
                        kind: DerivedKind::Derived,
                        lowering,

                        reactive: true,
                    },
                );
                self.collect_derived_init_refs(declarator);
            }
            RuneKind::DerivedBy => {
                let lowering = derived_lowering(call, rune_kind);
                self.record_derived_pattern(
                    &declarator.id,
                    DerivedDeclarationSemantics {
                        kind: DerivedKind::DerivedBy,
                        lowering,
                        reactive: true,
                    },
                );
                self.collect_derived_init_refs(declarator);
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
        if self.data.script.runes {
            return;
        }
        let Some(kind) = self.current_decl_kind else {
            return;
        };
        if !crate::utils::is_let_or_var(kind) {
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
            if !self.data.scoping.is_mutated_any(sym) {
                continue;
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
                DeclaratorSemantics::LegacyStateDestructure {
                    leaves: promoted_leaves,
                },
            );
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

    fn collect_derived_init_refs(&mut self, declarator: &VariableDeclarator<'a>) {
        let Some(Expression::CallExpression(call)) = declarator.init.as_ref() else {
            return;
        };
        let mut refs: SmallVec<[ReferenceId; 4]> = SmallVec::new();
        let mut reactive_rune_call = false;
        let mut visitor = RefCollector {
            refs: &mut refs,
            reactive_rune_call: &mut reactive_rune_call,
        };
        visitor.visit_call_expression(call);
        if reactive_rune_call {
            self.eager_reactive_derived
                .extend(leaf_decl_syms(&declarator.id));
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
        let root_is_mutated = match pattern {
            BindingPattern::BindingIdentifier(ident) => ident
                .symbol_id
                .get()
                .is_some_and(|sym| self.data.scoping.is_mutated_any(sym)),
            _ => true,
        };

        let optimize = require_mutation && !root_is_mutated;
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
                self.data
                    .reactivity
                    .record_state_binding(sym, semantics.clone());
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
                        lowering_mode: self.prop_lowering_mode,
                        kind: PropBindingKind::Rest,
                    },
                );
                self.data.reactivity.record_declarator_semantics(
                    root_node,
                    DeclaratorSemantics::PropsIdentifier { sym },
                );

                self.rest_prop_excluded.insert(sym, FxHashSet::default());
            }
            BindingPattern::ObjectPattern(obj) => {
                let mut leaves: SmallVec<[SymbolId; 4]> = SmallVec::new();
                let mut sibling_keys: FxHashSet<Ident<'a>> = FxHashSet::default();
                for prop in &obj.properties {
                    if let Some(key) = property_key_atom(&prop.key) {
                        sibling_keys.insert(key);
                    }
                    let Some(sym) = self.record_object_prop_pattern(&prop.value) else {
                        return;
                    };
                    leaves.push(sym);
                }

                let has_rest = obj.rest.is_some();
                if let Some(rest) = &obj.rest {
                    match self.record_rest_prop_pattern(&rest.argument) {
                        Some(rest_sym) => {
                            self.rest_prop_excluded.insert(rest_sym, sibling_keys);
                            leaves.push(rest_sym);
                        }
                        None => return,
                    }
                }
                self.data.reactivity.record_declarator_semantics(
                    root_node,
                    DeclaratorSemantics::PropsObject { leaves, has_rest },
                );
            }
            _ => {}
        }
    }

    fn record_object_prop_pattern(&mut self, pattern: &BindingPattern<'_>) -> Option<SymbolId> {
        match pattern {
            BindingPattern::BindingIdentifier(ident) => {
                let sym = ident.symbol_id.get()?;
                let is_source = matches!(self.prop_lowering_mode, PropLoweringMode::CustomElement)
                    || self.data.scoping.is_mutated_any(sym);
                let kind = if is_source {
                    PropBindingKind::Source {
                        bindable: false,
                        updated: self.data.scoping.is_mutated(sym),
                        default_lowering: PropDefaultLowering::None,
                        default_needs_proxy: false,
                    }
                } else {
                    PropBindingKind::NonSource
                };
                self.data.reactivity.record_prop_binding(
                    sym,
                    PropBindingSemantics {
                        lowering_mode: self.prop_lowering_mode,
                        kind,
                    },
                );
                Some(sym)
            }
            BindingPattern::AssignmentPattern(assign) => {
                let bindable = prop_default_is_bindable(&assign.right);
                let default_lowering = prop_default_lowering(&assign.right);
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
        default_lowering: PropDefaultLowering,
        default_needs_proxy: bool,
    ) -> Option<SymbolId> {
        let BindingPattern::BindingIdentifier(ident) = pattern else {
            return None;
        };
        let sym = ident.symbol_id.get()?;
        self.data.reactivity.record_prop_binding(
            sym,
            PropBindingSemantics {
                lowering_mode: self.prop_lowering_mode,
                kind: PropBindingKind::Source {
                    bindable,
                    updated: self.data.scoping.is_mutated(sym),
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
                lowering_mode: self.prop_lowering_mode,
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

fn component_prop_lowering_mode(component: &Component) -> PropLoweringMode {
    component
        .options
        .as_ref()
        .and_then(|options| options.custom_element.as_ref())
        .map(|_| PropLoweringMode::CustomElement)
        .unwrap_or(PropLoweringMode::Standard)
}

impl<'a> Visit<'a> for ScriptSemanticCollector<'_, 'a> {
    fn visit_program(&mut self, program: &oxc_ast::ast::Program<'a>) {
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
        oxc_ast_visit::walk::walk_program(self, program);
    }

    fn visit_variable_declaration(&mut self, decl: &VariableDeclaration<'a>) {
        let previous = self.current_decl_kind.replace(decl.kind);
        walk_variable_declaration(self, decl);
        self.current_decl_kind = previous;
    }

    fn visit_export_named_declaration(
        &mut self,
        export: &oxc_ast::ast::ExportNamedDeclaration<'a>,
    ) {
        legacy::classify_export_named_declaration(self.data, export);
        oxc_ast_visit::walk::walk_export_named_declaration(self, export);
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
        oxc_ast_visit::walk::walk_assignment_expression(self, expr);
    }

    fn visit_update_expression(&mut self, expr: &UpdateExpression<'a>) {
        if let Some(ref_id) = simple_assignment_target_member_root_reference_id(&expr.argument) {
            self.prop_member_mutation_root_refs.insert(ref_id);
        }
        oxc_ast_visit::walk::walk_update_expression(self, expr);
    }

    fn visit_static_member_expression(&mut self, member: &StaticMemberExpression<'a>) {
        self.classify_rest_prop_member_rewrite(member);
        walk_static_member_expression(self, member);
    }
}

fn rune_call<'a>(
    declarator: &'a VariableDeclarator<'a>,
) -> Option<(&'a CallExpression<'a>, RuneKind)> {
    let Expression::CallExpression(call) = declarator.init.as_ref()? else {
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

fn collect_state_binding_semantics(
    scoping: &ComponentScoping<'_>,
    pattern: &BindingPattern<'_>,
    rune_kind: StateKind,
    init_proxyable: bool,
) -> SmallVec<[StateBindingSemantics; 4]> {
    let mut semantics = SmallVec::new();
    collect_state_binding_semantics_inner(
        scoping,
        pattern,
        rune_kind,
        init_proxyable,
        &mut semantics,
    );
    semantics
}

fn collect_state_binding_semantics_inner(
    scoping: &ComponentScoping<'_>,
    pattern: &BindingPattern<'_>,
    rune_kind: StateKind,
    init_proxyable: bool,
    semantics: &mut SmallVec<[StateBindingSemantics; 4]>,
) {
    match pattern {
        BindingPattern::BindingIdentifier(ident) => {
            let mutated = ident
                .symbol_id
                .get()
                .is_some_and(|sym| scoping.is_mutated(sym));
            semantics.push(state_binding_semantic(rune_kind, mutated, init_proxyable));
        }
        BindingPattern::ObjectPattern(obj) => {
            for prop in &obj.properties {
                collect_state_binding_semantics_inner(
                    scoping,
                    &prop.value,
                    rune_kind,
                    init_proxyable,
                    semantics,
                );
            }
            if let Some(rest) = &obj.rest {
                collect_state_binding_semantics_inner(
                    scoping,
                    &rest.argument,
                    rune_kind,
                    init_proxyable,
                    semantics,
                );
            }
        }
        BindingPattern::ArrayPattern(arr) => {
            for elem in arr.elements.iter().flatten() {
                collect_state_binding_semantics_inner(
                    scoping,
                    elem,
                    rune_kind,
                    init_proxyable,
                    semantics,
                );
            }
            if let Some(rest) = &arr.rest {
                collect_state_binding_semantics_inner(
                    scoping,
                    &rest.argument,
                    rune_kind,
                    init_proxyable,
                    semantics,
                );
            }
        }
        BindingPattern::AssignmentPattern(assign) => {
            collect_state_binding_semantics_inner(
                scoping,
                &assign.left,
                rune_kind,
                init_proxyable,
                semantics,
            );
        }
    }
}

fn state_binding_semantic(
    rune_kind: StateKind,
    mutated: bool,
    init_proxyable: bool,
) -> StateBindingSemantics {
    match (rune_kind, mutated) {
        (StateKind::State, true) => StateBindingSemantics::StateSignal {
            proxied: init_proxyable,
        },
        (StateKind::State, false) => StateBindingSemantics::NonReactive {
            proxied: init_proxyable,
        },
        (StateKind::StateRaw, true) => StateBindingSemantics::StateRawSignal,
        (StateKind::StateRaw, false) => StateBindingSemantics::NonReactive { proxied: false },
        (StateKind::StateEager, _) => StateBindingSemantics::NonReactive { proxied: false },
    }
}

fn prop_default_is_bindable(expr: &Expression<'_>) -> bool {
    let Expression::CallExpression(call) = expr else {
        return false;
    };
    detect_rune_from_call(call) == Some(RuneKind::Bindable)
}

fn prop_default_lowering(expr: &Expression<'_>) -> PropDefaultLowering {
    let default_expr = bindable_default_arg(expr).unwrap_or(expr);
    if bindable_default_arg(expr).is_none() && prop_default_is_bindable(expr) {
        return PropDefaultLowering::None;
    }
    if is_simple_expression(default_expr) {
        PropDefaultLowering::Eager
    } else {
        PropDefaultLowering::Lazy
    }
}

fn prop_default_needs_proxy(expr: &Expression<'_>, bindable: bool) -> bool {
    bindable && state_expression_is_proxyable(bindable_default_arg(expr).unwrap_or(expr))
}

fn bindable_default_arg<'a>(expr: &'a Expression<'a>) -> Option<&'a Expression<'a>> {
    let Expression::CallExpression(call) = expr else {
        return None;
    };
    if detect_rune_from_call(call) != Some(RuneKind::Bindable) {
        return None;
    }
    call.arguments.first().and_then(|arg| arg.as_expression())
}

fn is_simple_expression(expr: &Expression<'_>) -> bool {
    matches!(
        expr,
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

fn derived_lowering(call: &CallExpression<'_>, rune_kind: RuneKind) -> DerivedLowering {
    if matches!(rune_kind, RuneKind::Derived)
        && call
            .arguments
            .first()
            .and_then(|arg| arg.as_expression())
            .is_some_and(|expr| matches!(expr, Expression::AwaitExpression(_)))
    {
        DerivedLowering::Async
    } else {
        DerivedLowering::Sync
    }
}

fn leaf_decl_syms(pattern: &BindingPattern<'_>) -> Vec<SymbolId> {
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
    fn visit_identifier_reference(&mut self, ident: &oxc_ast::ast::IdentifierReference<'a>) {
        if let Some(ref_id) = ident.reference_id.get() {
            self.refs.push(ref_id);
        }
    }
    fn visit_call_expression(&mut self, call: &oxc_ast::ast::CallExpression<'a>) {
        if let Some(rune) = crate::utils::script_info::detect_rune_from_call(call)
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
        oxc_ast_visit::walk::walk_call_expression(self, call);
    }
}
