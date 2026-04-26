mod contextual;
/// LEGACY(svelte4): see `legacy.rs` header. Removable as a unit.
mod legacy;
mod references;
mod store;
mod util;

use util::{
    assignment_target_member_root_reference_id, property_key_atom,
    simple_assignment_target_member_root_reference_id,
};

use super::data::{
    DerivedDeclarationSemantics, DerivedKind, DerivedLowering, OptimizedRuneSemantics,
    PropBindingFacts, PropDeclarationKind, PropDeclarationSemantics, PropDefaultLowering,
    PropLoweringMode, PropsObjectPropertySemantics, RuntimeRuneKind, StateBindingSemantics,
    StateDeclarationSemantics, StateKind, V2DeclarationFacts, V2ReferenceFacts,
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
use oxc_ast_visit::walk::{
    walk_static_member_expression, walk_variable_declaration, walk_variable_declarator,
};
use oxc_ast_visit::Visit;
use oxc_span::Ident;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use svelte_ast::Component;
use svelte_component_semantics::{OxcNodeId, ReferenceId};

const JS_UNDEFINED_NAME: &str = "undefined";

/// Transitional v2 reactivity builder entrypoint.
///
/// This path owns declaration-side semantics that can already be derived
/// without the old getter surface. The deprecated v1 builder still runs
/// afterwards for symbol-centric read/write queries until reference semantics
/// migration is complete.
pub(crate) fn build_v2<'a>(component: &Component, parsed: &JsAst<'a>, data: &mut AnalysisData<'a>) {
    data.reactivity.set_uses_runes(data.script.runes);
    build_script_semantics_v2(parsed, data, component_prop_lowering_mode(component));
    contextual::collect_template_declarations(component, parsed, data);
    // Reserve the dense reference-facts table once `ReferenceTable` is final
    // (after script + template walks). Avoids per-insert resize chains while
    // `references::collect_symbol_semantics` fills the table.
    let reference_count = data.scoping.references_len();
    data.reactivity.reserve_references(reference_count);
    references::collect_symbol_semantics(data);
    compute_const_tag_reactivity(component, parsed, data);
    // LEGACY(svelte4): classify $$props / $$restProps identifier reads from
    // ComponentSemantics.root_unresolved_references. Runes mode skipped inside.
    legacy::classify_unresolved_legacy_identifiers(data);
}

/// Fix-point-style refinement of `ConstDeclarationSemantics::ConstTag::reactive`.
///
/// `{@const}` declarations start with `reactive: true` (conservative seed from
/// `record_const_declaration_v2`). Here we relax them to `false` when the init
/// expression only references non-reactive symbols. Runs after all script-side
/// Derived `reactive` flags are computed, so transitive chains (`{@const x = y}`
/// where `y = $derived(inert)`) fold correctly.
fn compute_const_tag_reactivity<'a>(
    component: &Component,
    parsed: &JsAst<'a>,
    data: &mut AnalysisData<'a>,
) {
    use super::data::{ConstDeclarationSemantics, DeclarationSemantics};
    use svelte_component_semantics::walk_bindings;
    // Snapshot `(scope, tag_id)` pairs up front: the fragment scope for
    // each tag is carried by `by_fragment` and looked up once, so we
    // can traverse without re-reading the side-table during the
    // fix-point work below.
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
        // Collect ReferenceIds from the const-tag init expression via OXC Visit.
        let svelte_ast::Node::ConstTag(tag) = component.store.get(tag_id) else {
            continue;
        };
        let Some(stmt) = parsed.stmt(tag.decl.id()) else {
            continue;
        };

        // Resolve the binding leaves locally from the pattern — avoids
        // depending on `const_tags.syms` (removed in this slice).
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

        // Resolve reactivity for each ref via already-classified declaration facts.
        let reactive = eager_rune
            || refs.iter().any(|&ref_id| {
                let Some(sym) = data.scoping.symbol_for_reference(ref_id) else {
                    return false;
                };
                let decl = data
                    .reactivity
                    .declaration_semantics(data.scoping.symbol_declaration(sym));
                match decl {
                    DeclarationSemantics::State(_)
                    | DeclarationSemantics::Prop(_)
                    | DeclarationSemantics::LegacyBindableProp(_)
                    | DeclarationSemantics::Store(_)
                    | DeclarationSemantics::Contextual(_)
                    | DeclarationSemantics::RuntimeRune { .. } => true,
                    DeclarationSemantics::Derived(d) => d.reactive,
                    DeclarationSemantics::Const(ConstDeclarationSemantics::ConstTag {
                        reactive,
                        ..
                    }) => reactive,
                    DeclarationSemantics::OptimizedRune(opt) if opt.proxy_init => true,
                    DeclarationSemantics::NonReactive
                    | DeclarationSemantics::Unresolved
                    | DeclarationSemantics::OptimizedRune(_)
                    | DeclarationSemantics::LetCarrier { .. } => {
                        !data.scoping.is_component_top_level_symbol(sym)
                    }
                }
            });

        for sym in syms {
            let node_id = data.scoping.symbol_declaration(sym);
            if let Some(V2DeclarationFacts::Const(ConstDeclarationSemantics::ConstTag {
                reactive: r,
                ..
            })) = data.reactivity.declaration_facts_v2_mut(node_id)
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
) {
    let mut collector = ScriptSemanticCollector::new(data, prop_lowering_mode);
    if let Some(program) = parsed.program.as_ref() {
        collector.visit_program(program);
    }
    if let Some(program) = parsed.module_program.as_ref() {
        collector.visit_program(program);
    }
    // Template & attribute expressions and `{@const}` statements reuse the same
    // collector so reference-level reactivity (e.g. `RestPropMemberRewrite`) is
    // classified uniformly across script and template.
    for expr in parsed.iter_exprs() {
        collector.visit_expression(expr);
    }
    for stmt in parsed.iter_stmts() {
        collector.visit_statement(stmt);
    }
    collector.finish();
}

struct ScriptSemanticCollector<'d, 'a> {
    data: &'d mut AnalysisData<'a>,
    current_decl_kind: Option<VariableDeclarationKind>,
    prop_lowering_mode: PropLoweringMode,
    /// Set of `ReferenceId`s that are the **root identifier** of a
    /// MemberExpression LHS on an assignment or UpdateExpression argument.
    /// For `foo.x = val` or `foo.x++`, the `ReferenceId` of the `foo`
    /// identifier goes here. Consumed by `classify_reference_semantics`
    /// to emit `Prop*MemberMutationRoot` instead of `PropRead` for these
    /// references — so downstream consumers don't need to AST-reconstruct
    /// "is this a mutation target" from surrounding syntax.
    prop_member_mutation_root_refs: FxHashSet<ReferenceId>,
    pending_prop_objects: Vec<PendingPropObjectDeclaration>,
    /// Per-rest-prop-symbol sibling key set from the same `$props()` destructuring.
    /// Used by `visit_static_member_expression` to answer
    /// "does this `<rest>.<key>` require `$$props.<key>` rewrite?".
    rest_prop_excluded: FxHashMap<SymbolId, FxHashSet<Ident<'a>>>,
    /// Temp map from Derived/Const-tag declarator node id → list of
    /// `ReferenceId`s inside the init expression. Populated during the first
    /// walk; consumed by the `compute_derived_reactivity` fix-point pass to
    /// set `DerivedDeclarationSemantics::reactive` / `ConstDeclarationSemantics
    /// ::ConstTag::reactive`. Dropped when `finish()` completes.
    derived_init_refs: FxHashMap<OxcNodeId, SmallVec<[ReferenceId; 4]>>,
    /// Leaf declaration nodes that must be kept `reactive = true` regardless
    /// of what their ref-based reactivity check returns — because their init
    /// embeds a runtime-reactive rune call (`$effect.pending()` etc.) that
    /// doesn't resolve through local `ReferenceId`s.
    eager_reactive_derived: FxHashSet<OxcNodeId>,
    /// LEGACY(svelte4): ExportNamedDeclaration node ids classified at end-of-walk
    /// in `finish()`, so member-mutation tracking (`prop_member_updated`) is full
    /// before flags are computed.
    legacy_export_node_ids: Vec<OxcNodeId>,
}

struct PendingPropObjectDeclaration {
    root_node: OxcNodeId,
    property_syms: Vec<SymbolId>,
    has_rest: bool,
}

impl<'d, 'a> ScriptSemanticCollector<'d, 'a> {
    fn new(data: &'d mut AnalysisData<'a>, prop_lowering_mode: PropLoweringMode) -> Self {
        Self {
            data,
            current_decl_kind: None,
            prop_lowering_mode,
            prop_member_mutation_root_refs: FxHashSet::default(),
            pending_prop_objects: Vec::new(),
            rest_prop_excluded: FxHashMap::default(),
            derived_init_refs: FxHashMap::default(),
            eager_reactive_derived: FxHashSet::default(),
            legacy_export_node_ids: Vec::new(),
        }
    }

    fn finish(mut self) {
        // LEGACY(svelte4): classify legacy export props after the main walk —
        // js_visitor has already populated MEMBER_MUTATED on ComponentSemantics
        // for every script/template assignment, so legacy classify can read it
        // via `is_mutated_any` directly.
        if !self.legacy_export_node_ids.is_empty() {
            let nodes = std::mem::take(&mut self.legacy_export_node_ids);
            for node_id in nodes {
                legacy::classify_export_node(self.data, node_id);
            }
        }

        let member_mutated_syms: Vec<SymbolId> = self
            .data
            .scoping
            .semantics()
            .symbols_with_state(svelte_component_semantics::sym_state::MEMBER_MUTATED)
            .collect();
        for sym in member_mutated_syms {
            let Some(mut prop) = self.data.reactivity.prop_facts(sym) else {
                continue;
            };
            if !prop.is_source {
                continue;
            }
            prop.updated = true;
            self.data.reactivity.record_prop_facts(sym, prop.clone());
            let binding_node = self.data.scoping.symbol_declaration(sym);
            self.data.reactivity.record_prop_declaration_v2(
                binding_node,
                PropDeclarationSemantics {
                    lowering_mode: prop.lowering_mode,
                    kind: prop_binding_kind(&prop),
                },
            );
        }

        for pending in self.pending_prop_objects.drain(..) {
            let Some(properties) = pending
                .property_syms
                .iter()
                .map(|&sym| self.data.reactivity.prop_facts(sym))
                .map(|meta| {
                    let meta = meta?;
                    Some(match prop_binding_kind(&meta) {
                        PropDeclarationKind::Source {
                            bindable,
                            updated,
                            default_lowering,
                            default_needs_proxy,
                        } => PropsObjectPropertySemantics::Source {
                            bindable,
                            updated,
                            default_lowering,
                            default_needs_proxy,
                        },
                        PropDeclarationKind::NonSource => PropsObjectPropertySemantics::NonSource,
                        _ => return None,
                    })
                })
                .collect::<Option<Vec<_>>>()
            else {
                continue;
            };

            self.data.reactivity.record_prop_declaration_v2(
                pending.root_node,
                PropDeclarationSemantics {
                    lowering_mode: self.prop_lowering_mode,
                    kind: PropDeclarationKind::Object {
                        properties,
                        has_rest: pending.has_rest,
                    },
                },
            );
        }
        self.data
            .reactivity
            .record_prop_member_mutation_root_refs(std::mem::take(
                &mut self.prop_member_mutation_root_refs,
            ));
        store::collect_store_declarations(self.data);
        self.compute_derived_reactivity();
    }

    /// Fix-point pass over `derived_init_refs`: sets the `reactive` flag on
    /// each Derived declaration to `true` iff at least one `ReferenceId`
    /// inside its init expression resolves to a symbol whose declaration
    /// itself contributes to reactivity (State mutated, Prop, Store,
    /// Contextual, RuntimeRune, or another reactive Derived — transitively).
    ///
    /// We cannot do this inline with `record_rune_declarator` because a
    /// Derived may reference another Derived declared later in source order,
    /// and because reference semantics (used to classify a ref's target) are
    /// only available after `collect_symbol_semantics` runs. Runs last in
    /// `finish()` when all declaration + reference facts are recorded.
    fn compute_derived_reactivity(&mut self) {
        if self.derived_init_refs.is_empty() && self.eager_reactive_derived.is_empty() {
            return;
        }
        let entries: Vec<(OxcNodeId, SmallVec<[ReferenceId; 4]>)> =
            self.derived_init_refs.drain().collect();
        let eager = std::mem::take(&mut self.eager_reactive_derived);

        loop {
            let mut changed = false;
            for (decl_node, refs) in &entries {
                let current_reactive = match self.data.reactivity.declaration_facts_v2(*decl_node) {
                    Some(V2DeclarationFacts::Derived(d)) => d.reactive,
                    _ => continue,
                };
                // Eager-marked derived stays reactive no matter what refs resolve to.
                let new_reactive = eager.contains(decl_node)
                    || refs.iter().any(|&r| self.is_reference_reactive(r));
                if new_reactive != current_reactive {
                    self.data
                        .reactivity
                        .set_derived_reactive(*decl_node, new_reactive);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
    }

    /// Does reading this reference observe a value that can change at runtime?
    /// Answers via the target symbol's declaration semantics — with Derived
    /// resolved transitively via its own already-computed `reactive` flag.
    fn is_reference_reactive(&self, ref_id: ReferenceId) -> bool {
        use super::data::{ConstDeclarationSemantics, DeclarationSemantics};
        let Some(sym) = self.data.scoping.symbol_for_reference(ref_id) else {
            return false;
        };
        let decl = self
            .data
            .reactivity
            .declaration_semantics(self.data.scoping.symbol_declaration(sym));
        match decl {
            DeclarationSemantics::State(_)
            | DeclarationSemantics::Prop(_)
            | DeclarationSemantics::LegacyBindableProp(_)
            | DeclarationSemantics::Store(_)
            | DeclarationSemantics::Contextual(_)
            | DeclarationSemantics::RuntimeRune { .. } => true,
            DeclarationSemantics::Derived(d) => d.reactive,
            DeclarationSemantics::Const(ConstDeclarationSemantics::ConstTag {
                reactive, ..
            }) => reactive,
            // A non-mutated `$state(proxyable)` is lowered as an `OptimizedRune`
            // but its underlying value is still proxy-wrapped, so mutations to
            // its fields (or reassignment from the outside via bind/prop) remain
            // observable. Treat it as reactive.
            DeclarationSemantics::OptimizedRune(opt) if opt.proxy_init => true,
            DeclarationSemantics::NonReactive
            | DeclarationSemantics::Unresolved
            | DeclarationSemantics::OptimizedRune(_)
            | DeclarationSemantics::LetCarrier { .. } => {
                !self.data.scoping.is_component_top_level_symbol(sym)
            }
        }
    }

    fn record_rune_declarator(&mut self, declarator: &VariableDeclarator<'a>) {
        let Some((call, rune_kind)) = rune_call(declarator) else {
            return;
        };
        let root_node = declarator.node_id();
        if !matches!(rune_kind, RuneKind::Props) {
            self.record_pattern_declaration_root(&declarator.id, root_node);
        }

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
                self.record_derived_pattern(
                    &declarator.id,
                    root_node,
                    DerivedDeclarationSemantics {
                        kind: DerivedKind::Derived,
                        lowering: derived_lowering(call, rune_kind),
                        // Seeded to `true`; `compute_derived_reactivity` may
                        // lower it to `false` if all init-refs are inert.
                        reactive: true,
                    },
                );
                self.collect_derived_init_refs(declarator, root_node);
            }
            RuneKind::DerivedBy => {
                self.record_derived_pattern(
                    &declarator.id,
                    root_node,
                    DerivedDeclarationSemantics {
                        kind: DerivedKind::DerivedBy,
                        lowering: derived_lowering(call, rune_kind),
                        reactive: true,
                    },
                );
                self.collect_derived_init_refs(declarator, root_node);
            }
            RuneKind::Props => {
                self.record_props_pattern(&declarator.id, root_node);
            }
            RuneKind::PropsId => {
                self.record_runtime_rune_pattern(
                    &declarator.id,
                    root_node,
                    RuntimeRuneKind::PropsId,
                );
            }
            RuneKind::EffectTracking => {
                self.record_runtime_rune_pattern(
                    &declarator.id,
                    root_node,
                    RuntimeRuneKind::EffectTracking,
                );
            }
            RuneKind::EffectPending => {
                self.record_runtime_rune_pattern(
                    &declarator.id,
                    root_node,
                    RuntimeRuneKind::EffectPending,
                );
            }
            RuneKind::Host => {
                self.record_runtime_rune_pattern(&declarator.id, root_node, RuntimeRuneKind::Host);
            }
            RuneKind::InspectTrace => {
                self.record_runtime_rune_pattern(
                    &declarator.id,
                    root_node,
                    RuntimeRuneKind::InspectTrace,
                );
            }
            _ => {}
        }
    }

    /// Walk every leaf binding under `pattern` and record a `RuntimeRune`
    /// declaration keyed by the declarator root node. These runes are only
    /// expected in single-identifier form in practice, but recursion keeps
    /// behaviour well-defined if a user writes e.g. `const [a] = $props.id()`.
    fn record_runtime_rune_pattern(
        &mut self,
        pattern: &BindingPattern<'_>,
        root_node: OxcNodeId,
        kind: RuntimeRuneKind,
    ) {
        match pattern {
            BindingPattern::BindingIdentifier(ident) => {
                let node_id = ident
                    .symbol_id
                    .get()
                    .map(|sym| self.data.scoping.symbol_declaration(sym))
                    .unwrap_or(root_node);
                self.data
                    .reactivity
                    .record_runtime_rune_declaration_v2(node_id, kind);
            }
            BindingPattern::ObjectPattern(obj) => {
                for prop in &obj.properties {
                    self.record_runtime_rune_pattern(&prop.value, root_node, kind);
                }
                if let Some(rest) = &obj.rest {
                    self.record_runtime_rune_pattern(&rest.argument, root_node, kind);
                }
            }
            BindingPattern::ArrayPattern(arr) => {
                for elem in arr.elements.iter().flatten() {
                    self.record_runtime_rune_pattern(elem, root_node, kind);
                }
                if let Some(rest) = &arr.rest {
                    self.record_runtime_rune_pattern(&rest.argument, root_node, kind);
                }
            }
            BindingPattern::AssignmentPattern(assign) => {
                self.record_runtime_rune_pattern(&assign.left, root_node, kind);
            }
        }
    }

    /// Collect every resolved `ReferenceId` inside the `$derived(...)` /
    /// `$derived.by(...)` init expression and stash it against every leaf
    /// declaration node recorded for this declarator. The fix-point pass in
    /// `finish()` reads this map to compute each leaf's `reactive` flag.
    ///
    /// Keys must match the node ids used by `record_derived_declaration_v2`
    /// (i.e. `symbol_declaration(sym)` for each `BindingIdentifier` leaf) so
    /// that `declaration_facts_v2` lookups in the fix-point resolve.
    fn collect_derived_init_refs(
        &mut self,
        declarator: &VariableDeclarator<'a>,
        _root_node: OxcNodeId,
    ) {
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
            // An embedded `$effect.pending()` / `$props.id()` / etc. makes
            // the outer derived reactive regardless of other refs. Mark
            // eagerly so the fix-point doesn't lower it back to `false`.
            self.eager_reactive_derived
                .extend(leaf_decl_nodes(&self.data.scoping, &declarator.id));
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
                    let node_id = self.data.scoping.symbol_declaration(sym);
                    self.derived_init_refs.insert(node_id, refs.clone());
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

    fn record_pattern_declaration_root(
        &mut self,
        pattern: &BindingPattern<'_>,
        root_node: OxcNodeId,
    ) {
        match pattern {
            BindingPattern::BindingIdentifier(ident) => {
                let Some(sym) = ident.symbol_id.get() else {
                    return;
                };
                self.data
                    .reactivity
                    .record_symbol_declaration_root(sym, root_node);
            }
            BindingPattern::ObjectPattern(obj) => {
                for prop in &obj.properties {
                    self.record_pattern_declaration_root(&prop.value, root_node);
                }
                if let Some(rest) = &obj.rest {
                    self.record_pattern_declaration_root(&rest.argument, root_node);
                }
            }
            BindingPattern::ArrayPattern(arr) => {
                for elem in arr.elements.iter().flatten() {
                    self.record_pattern_declaration_root(elem, root_node);
                }
                if let Some(rest) = &arr.rest {
                    self.record_pattern_declaration_root(&rest.argument, root_node);
                }
            }
            BindingPattern::AssignmentPattern(assign) => {
                self.record_pattern_declaration_root(&assign.left, root_node);
            }
        }
    }

    fn record_state_root_declaration(
        &mut self,
        pattern: &BindingPattern<'_>,
        root_node: OxcNodeId,
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
        // Unmutated `$state` / `$state.raw` with a plain-identifier pattern
        // lowers as a plain `let`, but the binding stays reassignable from
        // the outside (bind, prop passing). Record `OptimizedRune` so
        // child-passing consumers still see a rune-classified declaration.
        // `$state.eager` (`require_mutation=false`) always stays as `State`.
        let optimize = require_mutation && !root_is_mutated;
        if optimize {
            let optimized = OptimizedRuneSemantics {
                kind: semantics.kind,
                proxy_init: semantics.proxied,
                var_declared: semantics.var_declared,
            };
            self.record_optimized_rune_leaves(pattern, root_node, optimized);
        } else {
            self.record_state_leaves(pattern, root_node, &semantics);
        }
    }

    fn record_optimized_rune_leaves(
        &mut self,
        pattern: &BindingPattern<'_>,
        root_node: OxcNodeId,
        semantics: OptimizedRuneSemantics,
    ) {
        match pattern {
            BindingPattern::BindingIdentifier(ident) => {
                let node_id = ident
                    .symbol_id
                    .get()
                    .map(|sym| self.data.scoping.symbol_declaration(sym))
                    .unwrap_or(root_node);
                self.data
                    .reactivity
                    .record_optimized_rune_declaration_v2(node_id, semantics);
            }
            BindingPattern::ObjectPattern(obj) => {
                for prop in &obj.properties {
                    self.record_optimized_rune_leaves(&prop.value, root_node, semantics);
                }
                if let Some(rest) = &obj.rest {
                    self.record_optimized_rune_leaves(&rest.argument, root_node, semantics);
                }
            }
            BindingPattern::ArrayPattern(arr) => {
                for elem in arr.elements.iter().flatten() {
                    self.record_optimized_rune_leaves(elem, root_node, semantics);
                }
                if let Some(rest) = &arr.rest {
                    self.record_optimized_rune_leaves(&rest.argument, root_node, semantics);
                }
            }
            BindingPattern::AssignmentPattern(assign) => {
                self.record_optimized_rune_leaves(&assign.left, root_node, semantics);
            }
        }
    }

    // Write declaration facts keyed by per-leaf `BindingIdentifier` NodeId so
    // consumers that read `declaration_semantics(scoping.symbol_declaration(sym))`
    // hit a consistent identity. `root_node` is the declarator id, kept as
    // fallback when the leaf symbol isn't resolved.
    fn record_state_leaves(
        &mut self,
        pattern: &BindingPattern<'_>,
        root_node: OxcNodeId,
        semantics: &StateDeclarationSemantics,
    ) {
        match pattern {
            BindingPattern::BindingIdentifier(ident) => {
                let node_id = ident
                    .symbol_id
                    .get()
                    .map(|sym| self.data.scoping.symbol_declaration(sym))
                    .unwrap_or(root_node);
                self.data
                    .reactivity
                    .record_state_declaration_v2(node_id, semantics.clone());
            }
            BindingPattern::ObjectPattern(obj) => {
                for prop in &obj.properties {
                    self.record_state_leaves(&prop.value, root_node, semantics);
                }
                if let Some(rest) = &obj.rest {
                    self.record_state_leaves(&rest.argument, root_node, semantics);
                }
            }
            BindingPattern::ArrayPattern(arr) => {
                for elem in arr.elements.iter().flatten() {
                    self.record_state_leaves(elem, root_node, semantics);
                }
                if let Some(rest) = &arr.rest {
                    self.record_state_leaves(&rest.argument, root_node, semantics);
                }
            }
            BindingPattern::AssignmentPattern(assign) => {
                self.record_state_leaves(&assign.left, root_node, semantics);
            }
        }
    }

    fn record_derived_pattern(
        &mut self,
        pattern: &BindingPattern<'_>,
        root_node: OxcNodeId,
        semantics: DerivedDeclarationSemantics,
    ) {
        match pattern {
            BindingPattern::BindingIdentifier(ident) => {
                let node_id = ident
                    .symbol_id
                    .get()
                    .map(|sym| self.data.scoping.symbol_declaration(sym))
                    .unwrap_or(root_node);
                self.data
                    .reactivity
                    .record_derived_declaration_v2(node_id, semantics);
            }
            BindingPattern::ObjectPattern(obj) => {
                for prop in &obj.properties {
                    self.record_derived_pattern(&prop.value, root_node, semantics);
                }
                if let Some(rest) = &obj.rest {
                    self.record_derived_pattern(&rest.argument, root_node, semantics);
                }
            }
            BindingPattern::ArrayPattern(arr) => {
                for elem in arr.elements.iter().flatten() {
                    self.record_derived_pattern(elem, root_node, semantics);
                }
                if let Some(rest) = &arr.rest {
                    self.record_derived_pattern(&rest.argument, root_node, semantics);
                }
            }
            BindingPattern::AssignmentPattern(assign) => {
                self.record_derived_pattern(&assign.left, root_node, semantics);
            }
        }
    }

    fn record_props_pattern(&mut self, pattern: &BindingPattern<'a>, root_node: OxcNodeId) {
        match pattern {
            BindingPattern::BindingIdentifier(ident) => {
                let Some(sym) = ident.symbol_id.get() else {
                    return;
                };
                self.record_prop_binding(
                    sym,
                    PropBindingFacts {
                        bindable: false,
                        is_rest: true,
                        is_source: false,
                        updated: false,
                        lowering_mode: self.prop_lowering_mode,
                        default_lowering: PropDefaultLowering::None,
                        default_needs_proxy: false,
                    },
                );
                self.data.reactivity.record_prop_declaration_v2(
                    root_node,
                    PropDeclarationSemantics {
                        lowering_mode: self.prop_lowering_mode,
                        kind: PropDeclarationKind::Identifier,
                    },
                );
                // `let props = $props()` behaves like a pure rest binding — any
                // `props.<key>` member access must rewrite to `$$props.<key>`.
                // No siblings shadow any key, so the excluded set is empty.
                self.rest_prop_excluded.insert(sym, FxHashSet::default());
            }
            BindingPattern::ObjectPattern(obj) => {
                let mut property_syms = Vec::with_capacity(obj.properties.len());
                let mut sibling_keys: FxHashSet<Ident<'a>> = FxHashSet::default();
                for prop in &obj.properties {
                    if let Some(key) = property_key_atom(&prop.key) {
                        sibling_keys.insert(key);
                    }
                    let Some(sym) = self.record_object_prop_pattern(&prop.value) else {
                        return;
                    };
                    property_syms.push(sym);
                }

                if let Some(rest) = &obj.rest {
                    match self.record_rest_prop_pattern(&rest.argument) {
                        Some(rest_sym) => {
                            self.rest_prop_excluded.insert(rest_sym, sibling_keys);
                        }
                        None => return,
                    }
                }
                self.pending_prop_objects
                    .push(PendingPropObjectDeclaration {
                        root_node,
                        property_syms,
                        has_rest: obj.rest.is_some(),
                    });
            }
            _ => {}
        }
    }

    fn record_object_prop_pattern(&mut self, pattern: &BindingPattern<'_>) -> Option<SymbolId> {
        match pattern {
            BindingPattern::BindingIdentifier(ident) => {
                let sym = ident.symbol_id.get()?;
                let is_source = matches!(self.prop_lowering_mode, PropLoweringMode::CustomElement)
                    || self.data.scoping.is_mutated(sym);
                self.record_prop_binding(
                    sym,
                    PropBindingFacts {
                        bindable: false,
                        is_rest: false,
                        is_source,
                        updated: self.data.scoping.is_mutated(sym),
                        lowering_mode: self.prop_lowering_mode,
                        default_lowering: PropDefaultLowering::None,
                        default_needs_proxy: false,
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
        self.record_prop_binding(
            sym,
            PropBindingFacts {
                bindable,
                is_rest: false,
                is_source: true,
                updated: self.data.scoping.is_mutated(sym),
                lowering_mode: self.prop_lowering_mode,
                default_lowering,
                default_needs_proxy,
            },
        );
        Some(sym)
    }

    fn record_rest_prop_pattern(&mut self, pattern: &BindingPattern<'_>) -> Option<SymbolId> {
        let BindingPattern::BindingIdentifier(ident) = pattern else {
            return None;
        };
        let sym = ident.symbol_id.get()?;
        self.record_prop_binding(
            sym,
            PropBindingFacts {
                bindable: false,
                is_rest: true,
                is_source: false,
                updated: false,
                lowering_mode: self.prop_lowering_mode,
                default_lowering: PropDefaultLowering::None,
                default_needs_proxy: false,
            },
        );
        Some(sym)
    }

    fn record_prop_binding(&mut self, sym: SymbolId, facts: PropBindingFacts) {
        let binding_node = self.data.scoping.symbol_declaration(sym);
        self.data.reactivity.record_prop_facts(sym, facts.clone());
        self.data.reactivity.record_prop_declaration_v2(
            binding_node,
            PropDeclarationSemantics {
                lowering_mode: facts.lowering_mode,
                kind: prop_binding_kind(&facts),
            },
        );
    }

    /// Classify a `<rest>.<key>` member access as `RestPropMemberRewrite` when
    /// the object identifier resolves to a `...rest` binding and the property
    /// key is NOT shadowed by a sibling named prop from the same `$props()`
    /// destructuring.
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
            .record_reference_semantics_v2(ref_id, V2ReferenceFacts::RestPropMemberRewrite);
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
    fn visit_variable_declaration(&mut self, decl: &VariableDeclaration<'a>) {
        let previous = self.current_decl_kind.replace(decl.kind);
        walk_variable_declaration(self, decl);
        self.current_decl_kind = previous;
    }

    fn visit_export_named_declaration(
        &mut self,
        export: &oxc_ast::ast::ExportNamedDeclaration<'a>,
    ) {
        // LEGACY(svelte4): defer classification until `finish()` so that
        // member-mutation tracking (`prop_member_updated`) is fully populated
        // before legacy-prop flags are computed.
        if !self.data.script.runes {
            self.legacy_export_node_ids.push(export.node_id());
        }
        oxc_ast_visit::walk::walk_export_named_declaration(self, export);
    }

    fn visit_variable_declarator(&mut self, declarator: &VariableDeclarator<'a>) {
        self.record_rune_declarator(declarator);
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

    if let Expression::Identifier(id) = expr {
        if id.name == JS_UNDEFINED_NAME {
            return false;
        }
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

pub(super) fn prop_binding_kind(facts: &PropBindingFacts) -> PropDeclarationKind {
    if facts.is_rest {
        PropDeclarationKind::Rest
    } else if facts.is_source {
        PropDeclarationKind::Source {
            bindable: facts.bindable,
            updated: facts.updated,
            default_lowering: facts.default_lowering,
            default_needs_proxy: facts.default_needs_proxy,
        }
    } else {
        PropDeclarationKind::NonSource
    }
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

/// Walk a `BindingPattern` and collect `symbol_declaration(sym)` for every
/// leaf `BindingIdentifier`. Needed so the eager-reactive path can key its
/// set by the same node ids used by `record_derived_declaration_v2`.
fn leaf_decl_nodes(
    scoping: &crate::scope::ComponentScoping<'_>,
    pattern: &BindingPattern<'_>,
) -> Vec<OxcNodeId> {
    let mut out = Vec::new();
    fn recur(
        scoping: &crate::scope::ComponentScoping<'_>,
        pattern: &BindingPattern<'_>,
        out: &mut Vec<OxcNodeId>,
    ) {
        match pattern {
            BindingPattern::BindingIdentifier(ident) => {
                if let Some(sym) = ident.symbol_id.get() {
                    out.push(scoping.symbol_declaration(sym));
                }
            }
            BindingPattern::ObjectPattern(obj) => {
                for prop in &obj.properties {
                    recur(scoping, &prop.value, out);
                }
                if let Some(rest) = &obj.rest {
                    recur(scoping, &rest.argument, out);
                }
            }
            BindingPattern::ArrayPattern(arr) => {
                for elem in arr.elements.iter().flatten() {
                    recur(scoping, elem, out);
                }
                if let Some(rest) = &arr.rest {
                    recur(scoping, &rest.argument, out);
                }
            }
            BindingPattern::AssignmentPattern(assign) => {
                recur(scoping, &assign.left, out);
            }
        }
    }
    recur(scoping, pattern, &mut out);
    out
}

/// Collects every resolved `ReferenceId` reachable from a JS subtree plus a
/// flag marking whether any runtime-reactive rune call (`$effect.pending()`,
/// `$effect.tracking()`, `$props.id()`, `$host()`, `$inspect.trace()`) appears
/// directly in the expression. The rune-call flag is needed because those
/// runes read from global symbols that don't have a local declaration to
/// resolve — dependency analysis via `ReferenceId`s alone misses them.
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
        if let Some(rune) = crate::utils::script_info::detect_rune_from_call(call) {
            if matches!(
                rune,
                RuneKind::EffectPending
                    | RuneKind::EffectTracking
                    | RuneKind::PropsId
                    | RuneKind::Host
                    | RuneKind::InspectTrace
            ) {
                *self.reactive_rune_call = true;
            }
        }
        oxc_ast_visit::walk::walk_call_expression(self, call);
    }
}
