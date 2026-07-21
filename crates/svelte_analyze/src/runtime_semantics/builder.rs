use super::data::{
    ChildPropMode, ComponentBindOwnership, ComponentFrame, ContextScope, LegacyInit,
    LegacySlotSanitization, PropAccessors, PropsInput, RuntimeSemantics, StoreBindings,
};
use crate::expression_semantics::ExpressionSemanticsStore;
use crate::types::data::{
    ApiExport, ElementAnalysis, LegacySummary, ReactivitySemantics, ReactivitySummary,
    ScriptAnalysis,
};

#[allow(clippy::too_many_arguments)]
pub(crate) fn build(
    script: &ScriptAnalysis,
    reactivity: &ReactivitySemantics,
    elements: &ElementAnalysis,
    expressions: &ExpressionSemanticsStore,
    api_exports: &[ApiExport],
    legacy_has_export_declaration: bool,
    dev: bool,
) -> RuntimeSemantics {
    let uses_runes = reactivity.uses_runes();
    let summary = reactivity.summary();
    let has_exports = !api_exports.is_empty();
    let named_prop_accessors = if uses_runes {
        summary.has_named_runes_prop
    } else {
        summary.has_named_legacy_prop
    };
    let ce_props = summary.props.has_custom_element && named_prop_accessors;
    let has_legacy_accessor_props =
        !uses_runes && script.accessors && summary.has_named_legacy_prop;
    let public_surface = has_exports || ce_props || has_legacy_accessor_props;
    let observes_context = expressions.is_context_required();
    let has_legacy_export = legacy_has_export_declaration;
    let has_bind_props =
        summary.has_runes_bindable || summary.legacy.has_bindable_prop || has_exports;

    let frame = if public_surface || dev {
        ComponentFrame::Exposed
    } else if scopes_frame(uses_runes, summary, observes_context) {
        ComponentFrame::Scoped
    } else {
        ComponentFrame::Frameless
    };

    let context_ssr = if dev || observes_context || summary.has_runes_bindable {
        ContextScope::Wrapped
    } else {
        ContextScope::Direct
    };

    let props_input_ssr = if context_ssr == ContextScope::Wrapped
        || has_bind_props
        || summary.props.has_props
        || summary.legacy.reads_props_object
        || summary.legacy.reads_rest_props_object
        || summary.legacy.reads_slots_object
        || elements.renders_legacy_slot
        || has_legacy_export
    {
        PropsInput::Consumed
    } else {
        PropsInput::Ignored
    };

    RuntimeSemantics {
        child_prop_mode: if elements.has_child_component_bind {
            ChildPropMode::InOut
        } else {
            ChildPropMode::In
        },
        frame,
        prop_accessors: if script.accessors || ce_props {
            PropAccessors::Exposed
        } else {
            PropAccessors::Hidden
        },
        props_input: if needs_props_param(
            summary,
            elements,
            has_legacy_export,
            frame != ComponentFrame::Frameless,
        ) {
            PropsInput::Consumed
        } else {
            PropsInput::Ignored
        },
        stores: if summary.has_store_bindings {
            StoreBindings::Present
        } else {
            StoreBindings::Absent
        },
        legacy_init: legacy_init(uses_runes, script, summary.legacy, observes_context),
        sanitized_legacy_slots: if summary.legacy.reads_slots_object {
            LegacySlotSanitization::Needed
        } else {
            LegacySlotSanitization::Unneeded
        },
        component_bind_ownership: if elements.needs_component_bind_ownership {
            ComponentBindOwnership::Tracked
        } else {
            ComponentBindOwnership::Untracked
        },
        context_ssr,
        props_input_ssr,
    }
}

fn scopes_frame(uses_runes: bool, summary: ReactivitySummary, observes_context: bool) -> bool {
    if summary.props.has_bindable {
        return true;
    }
    if observes_context {
        return true;
    }
    if uses_runes {
        return false;
    }
    if summary.legacy.has_member_mutated {
        return true;
    }
    summary.has_legacy_reactive_statements
}

fn legacy_init(
    uses_runes: bool,
    script: &ScriptAnalysis,
    legacy: LegacySummary,
    observes_context: bool,
) -> LegacyInit {
    if uses_runes {
        return LegacyInit::None;
    }
    let needs_init = legacy.has_member_mutated || observes_context;
    if !needs_init {
        return LegacyInit::None;
    }
    if script.immutable {
        return LegacyInit::Immutable;
    }
    LegacyInit::Plain
}

fn needs_props_param(
    summary: ReactivitySummary,
    elements: &ElementAnalysis,
    has_legacy_export: bool,
    needs_frame: bool,
) -> bool {
    if needs_frame || summary.props.has_props {
        return true;
    }
    if summary.legacy.has_bindable_prop {
        return true;
    }
    if summary.legacy.reads_props_object || summary.legacy.reads_rest_props_object {
        return true;
    }
    if has_legacy_export {
        return true;
    }
    if elements.renders_legacy_slot || summary.legacy.reads_slots_object {
        return true;
    }
    elements.has_legacy_event_forward
}
