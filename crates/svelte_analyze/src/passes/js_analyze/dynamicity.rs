use crate::types::data::{AnalysisData, ExpressionInfo, ExpressionKind};

pub(crate) fn classify_expression_dynamicity(data: &mut AnalysisData) {
    let has_class_state = data.script.has_class_state_fields;

    for info in data.expressions.values_mut() {
        let is_dynamic = is_dynamic_template(info, &data.scoping, has_class_state);
        info.set_dynamicity(is_dynamic, is_dynamic);
    }

    for info in data.attr_expressions.values_mut() {
        let is_dynamic = is_dynamic_element_attr(info, &data.scoping);
        let has_state = has_state_component_attr(info, &data.scoping);
        info.set_dynamicity(is_dynamic, has_state);
    }
}

fn is_dynamic_template(
    info: &ExpressionInfo,
    scoping: &crate::scope::ComponentScoping,
    has_class_state_fields: bool,
) -> bool {
    if info.has_await() || info.has_state_rune() || info.needs_context() {
        return true;
    }

    if matches!(info.kind(), ExpressionKind::CallExpression { .. }) {
        return info.has_store_ref()
            || info.ref_symbols().iter().any(|&sym_id| {
                scoping.is_dynamic_by_id(sym_id)
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
        if scoping.is_dynamic_by_id(sym_id) {
            return true;
        }
        if scoping.is_prop_source(sym_id) {
            return true;
        }
        if has_class_state_fields
            && scoping.is_component_top_level_symbol(sym_id)
            && !scoping.is_rune(sym_id)
        {
            return true;
        }
        false
    })
}

fn is_dynamic_element_attr(
    info: &ExpressionInfo,
    scoping: &crate::scope::ComponentScoping,
) -> bool {
    if info.has_await() {
        return true;
    }
    attr_symbols(info, scoping).any(|sym_id| {
        scoping.prop_non_source_name(sym_id).is_some() || scoping.is_dynamic_by_id(sym_id)
    })
}

fn has_state_component_attr(
    info: &ExpressionInfo,
    scoping: &crate::scope::ComponentScoping,
) -> bool {
    if info.has_await() {
        return true;
    }
    attr_symbols(info, scoping)
        .any(|sym_id| !scoping.is_component_top_level_symbol(sym_id) || scoping.is_rune(sym_id))
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
