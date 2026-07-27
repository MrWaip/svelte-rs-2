use std::borrow::Cow;

use svelte_component_semantics::SymbolId;

use super::data::ReactivitySemantics;
use crate::scope::ComponentScoping;

pub(crate) fn finalize_component_prop_facts(
    reactivity: &mut ReactivitySemantics,
    scoping: &ComponentScoping,
) {
    let runes = has_named_runes_prop(reactivity, scoping);
    let legacy = has_named_legacy_prop(reactivity, scoping);
    reactivity.set_named_prop_flags(runes, legacy);
}

fn has_named_runes_prop(reactivity: &ReactivitySemantics, scoping: &ComponentScoping) -> bool {
    reactivity.iter_runes_prop_symbols().any(|sym| {
        let key = scoping
            .binding_origin_key(sym)
            .map(|(key, _)| key)
            .unwrap_or_else(|| Cow::Borrowed(scoping.symbol_name(sym)));
        !key.starts_with("$$")
    })
}

fn has_named_legacy_prop(reactivity: &ReactivitySemantics, scoping: &ComponentScoping) -> bool {
    reactivity
        .iter_legacy_bindable_prop_symbols()
        .any(|sym| !legacy_bindable_prop_key(reactivity, scoping, sym).starts_with("$$"))
}

fn legacy_bindable_prop_key<'d>(
    reactivity: &'d ReactivitySemantics,
    scoping: &'d ComponentScoping,
    sym: SymbolId,
) -> &'d str {
    match reactivity.legacy_bindable_prop_alias(sym) {
        Some(alias) => alias,
        None => scoping.symbol_name(sym),
    }
}
