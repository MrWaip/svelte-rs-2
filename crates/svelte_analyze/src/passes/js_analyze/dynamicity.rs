use crate::types::data::{AnalysisData, ExpressionInfo, ExpressionKind};

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

fn is_dynamic_template(
    info: &ExpressionInfo,
    scoping: &crate::scope::ComponentScoping,
    root: oxc_semantic::ScopeId,
    has_class_state_fields: bool,
) -> bool {
    if info.has_await || info.has_state_rune || info.needs_context {
        return true;
    }

    if matches!(info.kind, ExpressionKind::CallExpression { .. }) {
        return info.has_store_ref
            || info.ref_symbols.iter().any(|&sym_id| {
                scoping.is_dynamic_by_id(sym_id)
                    || (scoping.symbol_scope_id(sym_id) == root && !scoping.is_import(sym_id))
            });
    }

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
        if has_class_state_fields
            && scoping.symbol_scope_id(sym_id) == root
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
    if info.has_await {
        return true;
    }
    info.ref_symbols.iter().any(|&sym_id| {
        scoping.prop_non_source_name(sym_id).is_some() || scoping.is_dynamic_by_id(sym_id)
    })
}

fn has_state_component_attr(
    info: &ExpressionInfo,
    scoping: &crate::scope::ComponentScoping,
    root: oxc_semantic::ScopeId,
) -> bool {
    if info.has_await {
        return true;
    }
    info.ref_symbols
        .iter()
        .any(|&sym_id| scoping.symbol_scope_id(sym_id) != root || scoping.is_rune(sym_id))
}
