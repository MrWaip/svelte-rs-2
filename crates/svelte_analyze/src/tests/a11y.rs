use super::*;

#[test]
fn a11y_unknown_aria_attribute_suggests_closest_match() {
    let diags = analyze_with_diags(r#"<div aria-labl="name"></div>"#);
    assert_has_warning_kind(&diags, |kind| {
        matches!(
            kind,
            svelte_diagnostics::DiagnosticKind::A11yUnknownAriaAttribute {
                attribute,
                suggestion: Some(suggestion),
            } if attribute == "labl" && suggestion == "aria-label"
        )
    });
}

#[test]
fn a11y_incorrect_aria_attribute_type_warns_for_invalid_number() {
    let diags = analyze_with_diags(r#"<div aria-valuenow="abc"></div>"#);
    assert_has_warning_kind(&diags, |kind| {
        matches!(
            kind,
            svelte_diagnostics::DiagnosticKind::A11yIncorrectAriaAttributeType {
                attribute,
                type_,
            } if attribute == "aria-valuenow" && type_ == "number"
        )
    });
}

#[test]
fn a11y_incorrect_aria_attribute_type_token_warns_with_allowed_values() {
    let diags = analyze_with_diags(r#"<div aria-autocomplete="bad"></div>"#);
    assert_has_warning_kind(&diags, |kind| {
        matches!(
            kind,
            svelte_diagnostics::DiagnosticKind::A11yIncorrectAriaAttributeTypeToken {
                attribute,
                values,
            } if attribute == "aria-autocomplete"
                && values == "\"inline\", \"list\", \"both\" or \"none\""
        )
    });
}

#[test]
fn a11y_unknown_role_suggests_closest_match() {
    let diags = analyze_with_diags(r#"<div role="buton"></div>"#);
    assert_has_warning_kind(&diags, |kind| {
        matches!(
            kind,
            svelte_diagnostics::DiagnosticKind::A11yUnknownRole {
                role,
                suggestion: Some(suggestion),
            } if role == "buton" && suggestion == "button"
        )
    });
}

#[test]
fn a11y_role_has_required_aria_props_warns_for_missing_props() {
    let diags = analyze_with_diags(r#"<div role="combobox"></div>"#);
    assert_has_warning(&diags, "a11y_role_has_required_aria_props");
    assert_has_warning_kind(&diags, |kind| {
        matches!(
            kind,
            svelte_diagnostics::DiagnosticKind::A11yRoleHasRequiredAriaProps { role, props }
                if role == "combobox"
                    && props == "\"aria-controls\" and \"aria-expanded\""
        )
    });
}

#[test]
fn a11y_role_supports_aria_props_warns_for_explicit_role() {
    let diags = analyze_with_diags(r#"<div role="button" aria-checked="true"></div>"#);
    assert_has_warning(&diags, "a11y_role_supports_aria_props");
    assert_has_warning_kind(&diags, |kind| {
        matches!(
            kind,
            svelte_diagnostics::DiagnosticKind::A11yRoleSupportsAriaProps { attribute, role }
                if attribute == "aria-checked" && role == "button"
        )
    });
}

#[test]
fn a11y_role_supports_aria_props_warns_for_implicit_role() {
    let diags = analyze_with_diags(r#"<button aria-checked="true"></button>"#);
    assert_has_warning(&diags, "a11y_role_supports_aria_props_implicit");
    assert_has_warning_kind(&diags, |kind| {
        matches!(
            kind,
            svelte_diagnostics::DiagnosticKind::A11yRoleSupportsAriaPropsImplicit {
                attribute,
                role,
                name,
            } if attribute == "aria-checked" && role == "button" && name == "button"
        )
    });
}

#[test]
fn a11y_interactive_supports_focus_warns_for_interactive_role_with_handler() {
    let diags = analyze_with_diags(r#"<div role="button" onclick={handle}></div>"#);
    assert_has_warning(&diags, "a11y_interactive_supports_focus");
    assert_has_warning_kind(&diags, |kind| {
        matches!(
            kind,
            svelte_diagnostics::DiagnosticKind::A11yInteractiveSupportsFocus { role }
                if role == "button"
        )
    });
}

#[test]
fn a11y_no_static_element_interactions_warns_without_role() {
    let diags = analyze_with_diags(r#"<div onmousedown={handle}></div>"#);
    assert_has_warning(&diags, "a11y_no_static_element_interactions");
    assert_has_warning_kind(&diags, |kind| {
        matches!(
            kind,
            svelte_diagnostics::DiagnosticKind::A11yNoStaticElementInteractions {
                element,
                handler,
            } if element == "div" && handler == "mousedown"
        )
    });
}

#[test]
fn a11y_mouse_events_have_key_events_warns_for_mouseover_without_focus() {
    let diags = analyze_with_diags(r#"<div onmouseover={handle}></div>"#);
    assert_has_warning_kind(&diags, |kind| {
        matches!(
            kind,
            svelte_diagnostics::DiagnosticKind::A11yMouseEventsHaveKeyEvents {
                event,
                accompanied_by,
            } if event == "mouseover" && accompanied_by == "focus"
        )
    });
}

#[test]
fn a11y_mouse_events_have_key_events_warns_for_mouseout_without_blur() {
    let diags = analyze_with_diags(r#"<div onmouseout={handle}></div>"#);
    assert_has_warning_kind(&diags, |kind| {
        matches!(
            kind,
            svelte_diagnostics::DiagnosticKind::A11yMouseEventsHaveKeyEvents {
                event,
                accompanied_by,
            } if event == "mouseout" && accompanied_by == "blur"
        )
    });
}

#[test]
#[ignore = "missing: noninteractive element to interactive role warning (analyze)"]
fn a11y_no_noninteractive_element_to_interactive_role_warns_for_div_role_button() {
    let diags = analyze_with_diags(r#"<div role="button"></div>"#);
    assert_has_warning(&diags, "a11y_no_noninteractive_element_to_interactive_role");
}
