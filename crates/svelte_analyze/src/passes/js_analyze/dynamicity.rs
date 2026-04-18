use crate::scope::{ComponentScoping, SymbolId};
use crate::types::data::{
    AnalysisData, DeclarationSemantics, ExpressionInfo, ExpressionKind, PropDeclarationKind,
    PropDeclarationSemantics, ReactivitySemantics,
};

/// A symbol is "dynamic" when a template-level read of it may observe different
/// values across evaluations (so the surrounding expression must be wrapped in
/// reactive code). Reads directly from `ReactivitySemantics` declaration facts.
///
/// Classification per declaration kind:
/// - `State` / `Prop` / `Store` / `Contextual` / `RuntimeRune` → dynamic.
/// - `Derived` / `Const(ConstTag)` → dynamic iff their `reactive` flag is `true`
///   (computed by the builder v2 fix-point pass against init-expression refs).
/// - `OptimizedRune` with `proxy_init = true` (non-mutated `$state({...})`) →
///   dynamic: fields may still mutate through the proxy.
/// - `OptimizedRune` (scalar, no proxy) / `NonReactive` / `Unresolved` /
///   `LetCarrier` → dynamic only when captured from a nested scope (closure
///   capture of a top-level non-rune binding is still a static read).
/// - Each-index non-dynamic special case preserved.
pub(crate) fn is_symbol_dynamic(scoping: &ComponentScoping, reactivity: &ReactivitySemantics, sym_id: SymbolId) -> bool {
    use crate::types::data::ConstDeclarationSemantics;
    if scoping.is_each_index_non_dynamic(sym_id) {
        return false;
    }
    let decl = reactivity.declaration_semantics(scoping.symbol_declaration(sym_id));
    match decl {
        DeclarationSemantics::State(_)
        | DeclarationSemantics::Prop(_)
        | DeclarationSemantics::Store(_)
        | DeclarationSemantics::Contextual(_)
        | DeclarationSemantics::RuntimeRune { .. } => true,
        DeclarationSemantics::Derived(d) => d.reactive,
        DeclarationSemantics::Const(ConstDeclarationSemantics::ConstTag { reactive, .. }) => {
            reactive
        }
        // Proxy-backed unmutated $state: value itself is frozen but its
        // fields remain mutable through the proxy; treat as reactive.
        DeclarationSemantics::OptimizedRune(opt) if opt.proxy_init => true,
        DeclarationSemantics::NonReactive
        | DeclarationSemantics::Unresolved
        | DeclarationSemantics::OptimizedRune(_)
        | DeclarationSemantics::LetCarrier { .. } => {
            !scoping.is_component_top_level_symbol(sym_id)
        }
    }
}

pub(crate) fn classify_expression_dynamicity(data: &mut AnalysisData) {
    let has_class_state = data.script.has_class_state_fields;
    let scoping = &data.scoping;
    let reactivity = &data.reactivity;

    for info in data.expressions.values_mut() {
        let is_dynamic = is_dynamic_template(info, scoping, reactivity, has_class_state);
        info.set_dynamicity(is_dynamic, is_dynamic);
    }

    for info in data.attr_expressions.values_mut() {
        let is_dynamic = is_dynamic_element_attr(info, scoping, reactivity);
        let has_state = has_state_component_attr(info, scoping, reactivity);
        info.set_dynamicity(is_dynamic, has_state);
    }
}

fn is_dynamic_template(
    info: &ExpressionInfo,
    scoping: &ComponentScoping,
    reactivity: &ReactivitySemantics,
    has_class_state_fields: bool,
) -> bool {
    if info.has_await() || info.has_state_rune() || info.needs_context() {
        return true;
    }

    if matches!(info.kind(), ExpressionKind::CallExpression { .. }) {
        return info.has_store_ref()
            || info.ref_symbols().iter().any(|&sym_id| {
                is_symbol_dynamic(scoping, reactivity, sym_id)
                    || (scoping.is_component_top_level_symbol(sym_id) && !scoping.is_import(sym_id))
            });
    }

    if matches!(info.kind(), ExpressionKind::MemberExpression) {
        return info.has_store_ref() || !info.ref_symbols().is_empty();
    }

    if info.has_store_ref() {
        return true;
    }
    info.ref_symbols().iter().any(|&sym_id| {
        if is_symbol_dynamic(scoping, reactivity, sym_id) {
            return true;
        }
        if is_unified_prop_source(scoping, reactivity, sym_id) {
            return true;
        }
        if has_class_state_fields
            && scoping.is_component_top_level_symbol(sym_id)
            && is_unified_plain_symbol(scoping, reactivity, sym_id)
        {
            return true;
        }
        false
    })
}

fn is_dynamic_element_attr(
    info: &ExpressionInfo,
    scoping: &ComponentScoping,
    reactivity: &ReactivitySemantics,
) -> bool {
    if info.has_await() {
        return true;
    }
    attr_symbols(info, scoping).any(|sym_id| {
        matches!(
            reactivity.declaration_semantics(scoping.symbol_declaration(sym_id)),
            DeclarationSemantics::Prop(PropDeclarationSemantics {
                kind: PropDeclarationKind::NonSource,
                ..
            })
        ) || is_symbol_dynamic(scoping, reactivity, sym_id)
    })
}

fn has_state_component_attr(
    info: &ExpressionInfo,
    scoping: &ComponentScoping,
    reactivity: &ReactivitySemantics,
) -> bool {
    if info.has_await() {
        return true;
    }
    attr_symbols(info, scoping).any(|sym_id| {
        !scoping.is_component_top_level_symbol(sym_id)
            || !is_unified_plain_symbol(scoping, reactivity, sym_id)
    })
}

fn attr_symbols<'a>(
    info: &'a ExpressionInfo,
    scoping: &'a crate::scope::ComponentScoping,
) -> impl Iterator<Item = crate::scope::SymbolId> + 'a {
    let fallback = if info.ref_symbols().is_empty() {
        info.identifier_name()
            .and_then(|name| scoping.find_binding(scoping.root_scope_id(), name))
    } else {
        None
    };

    info.ref_symbols().iter().copied().chain(fallback)
}

/// Resolve the `DeclarationSemantics` for a symbol, falling back to the
/// declarator root when the direct declaration node has no recorded fact.
fn symbol_declaration_semantics(
    scoping: &ComponentScoping,
    reactivity: &ReactivitySemantics,
    sym_id: SymbolId,
) -> DeclarationSemantics {
    reactivity.declaration_semantics(scoping.symbol_declaration(sym_id))
}

fn is_unified_prop_source(
    scoping: &ComponentScoping,
    reactivity: &ReactivitySemantics,
    sym_id: SymbolId,
) -> bool {
    matches!(
        symbol_declaration_semantics(scoping, reactivity, sym_id),
        DeclarationSemantics::Prop(PropDeclarationSemantics {
            kind: PropDeclarationKind::Source { .. },
            ..
        })
    )
}

fn is_unified_plain_symbol(
    scoping: &ComponentScoping,
    reactivity: &ReactivitySemantics,
    sym_id: SymbolId,
) -> bool {
    // `OptimizedRune` is explicitly excluded: even if its declaration lowers
    // as a plain `let`, the binding stays reassignable from the outside, so
    // template reads of it are dynamic.
    matches!(
        symbol_declaration_semantics(scoping, reactivity, sym_id),
        DeclarationSemantics::NonReactive | DeclarationSemantics::Const(_)
    )
}
