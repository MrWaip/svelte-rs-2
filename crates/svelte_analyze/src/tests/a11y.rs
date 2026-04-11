use super::*;

#[test]
fn a11y_distracting_elements_marquee() {
    let diags = analyze_with_diags(r#"<marquee>scroll</marquee>"#);
    assert_has_warning(&diags, "a11y_distracting_elements");
}

#[test]
fn a11y_distracting_elements_blink() {
    let diags = analyze_with_diags(r#"<blink>flash</blink>"#);
    assert_has_warning(&diags, "a11y_distracting_elements");
}

#[test]
fn a11y_positive_tabindex_warns() {
    let diags = analyze_with_diags(r#"<div tabindex="2">content</div>"#);
    assert_has_warning(&diags, "a11y_positive_tabindex");
}

#[test]
fn a11y_tabindex_zero_no_warning() {
    let diags = analyze_with_diags(r#"<div tabindex="0">content</div>"#);
    assert_no_warning(&diags, "a11y_positive_tabindex");
}

#[test]
fn a11y_tabindex_negative_no_warning() {
    let diags = analyze_with_diags(r#"<div tabindex="-1">content</div>"#);
    assert_no_warning(&diags, "a11y_positive_tabindex");
}

#[test]
fn a11y_tabindex_dynamic_no_warning() {
    let diags =
        analyze_with_diags(r#"<script>let n = $state(0);</script><div tabindex={n}>content</div>"#);
    assert_no_warning(&diags, "a11y_positive_tabindex");
}

#[test]
fn a11y_no_noninteractive_tabindex_warns_for_div() {
    let diags = analyze_with_diags(r#"<div tabindex="0">content</div>"#);
    assert_has_warning(&diags, "a11y_no_noninteractive_tabindex");
}

#[test]
fn a11y_no_noninteractive_tabindex_no_warning_for_negative_tabindex() {
    let diags = analyze_with_diags(r#"<div tabindex="-1">content</div>"#);
    assert_no_warning(&diags, "a11y_no_noninteractive_tabindex");
}

#[test]
fn a11y_no_noninteractive_tabindex_no_warning_for_interactive_element() {
    let diags = analyze_with_diags(r#"<button tabindex="0">content</button>"#);
    assert_no_warning(&diags, "a11y_no_noninteractive_tabindex");
}

#[test]
fn a11y_autofocus_warns() {
    let diags = analyze_with_diags(r#"<input autofocus />"#);
    assert_has_warning(&diags, "a11y_autofocus");
}

#[test]
fn a11y_autofocus_on_dialog_no_warning() {
    // autofocus is legitimate on a <dialog> element itself
    let diags = analyze_with_diags(r#"<dialog autofocus>content</dialog>"#);
    assert_no_warning(&diags, "a11y_autofocus");
}

#[test]
fn a11y_autofocus_inside_dialog_no_warning() {
    // autofocus is legitimate on elements nested inside a <dialog>
    let diags = analyze_with_diags(r#"<dialog><input autofocus /></dialog>"#);
    assert_no_warning(&diags, "a11y_autofocus");
}

#[test]
fn a11y_missing_attribute_img_no_alt() {
    let diags = analyze_with_diags(r#"<img src="cat.jpg" />"#);
    assert_has_warning(&diags, "a11y_missing_attribute");
}

#[test]
fn a11y_missing_attribute_img_with_alt_no_warning() {
    let diags = analyze_with_diags(r#"<img src="cat.jpg" alt="a cat" />"#);
    assert_no_warning(&diags, "a11y_missing_attribute");
}

#[test]
fn a11y_missing_attribute_img_spread_no_warning() {
    // spread may include alt — don't warn
    let diags = analyze_with_diags(r#"<script>let p = $state({});</script><img {...p} />"#);
    assert_no_warning(&diags, "a11y_missing_attribute");
}

#[test]
fn a11y_missing_attribute_area_no_alt() {
    let diags = analyze_with_diags(r#"<map name="m"><area /></map>"#);
    assert_has_warning(&diags, "a11y_missing_attribute");
}

#[test]
fn a11y_missing_attribute_area_with_aria_label_no_warning() {
    let diags = analyze_with_diags(r#"<map name="m"><area aria-label="region" /></map>"#);
    assert_no_warning(&diags, "a11y_missing_attribute");
}

#[test]
fn a11y_missing_attribute_iframe_no_title() {
    let diags = analyze_with_diags(r#"<iframe src="page.html"></iframe>"#);
    assert_has_warning(&diags, "a11y_missing_attribute");
}

#[test]
fn a11y_missing_attribute_iframe_with_title_no_warning() {
    let diags = analyze_with_diags(r#"<iframe src="page.html" title="page"></iframe>"#);
    assert_no_warning(&diags, "a11y_missing_attribute");
}

#[test]
fn a11y_missing_attribute_object_no_title() {
    let diags = analyze_with_diags(r#"<object data="file.swf"></object>"#);
    assert_has_warning(&diags, "a11y_missing_attribute");
}

#[test]
fn a11y_missing_attribute_object_with_aria_labelledby_no_warning() {
    let diags = analyze_with_diags(r#"<object data="file.swf" aria-labelledby="desc"></object>"#);
    assert_no_warning(&diags, "a11y_missing_attribute");
}

#[test]
fn a11y_missing_attribute_anchor_no_href() {
    // <a> without href, id, name, or aria-disabled=true should warn
    let diags = analyze_with_diags(r#"<a>link text</a>"#);
    assert_has_warning(&diags, "a11y_missing_attribute");
}

#[test]
fn a11y_missing_attribute_anchor_with_href_no_warning() {
    let diags = analyze_with_diags(r#"<a href="/page">link</a>"#);
    assert_no_warning(&diags, "a11y_missing_attribute");
}

#[test]
fn a11y_missing_attribute_anchor_with_id_no_warning() {
    // <a id="..."> is a named anchor — href not required
    let diags = analyze_with_diags(r#"<a id="anchor">anchor</a>"#);
    assert_no_warning(&diags, "a11y_missing_attribute");
}

#[test]
fn a11y_missing_attribute_anchor_with_name_no_warning() {
    let diags = analyze_with_diags(r#"<a name="anchor">anchor</a>"#);
    assert_no_warning(&diags, "a11y_missing_attribute");
}

#[test]
fn a11y_missing_attribute_anchor_aria_disabled_no_warning() {
    // aria-disabled="true" suppresses the href requirement
    let diags = analyze_with_diags(r#"<a aria-disabled="true">disabled link</a>"#);
    assert_no_warning(&diags, "a11y_missing_attribute");
}

#[test]
fn a11y_missing_attribute_anchor_spread_no_warning() {
    let diags = analyze_with_diags(r#"<script>let p = $state({});</script><a {...p}>link</a>"#);
    assert_no_warning(&diags, "a11y_missing_attribute");
}

#[test]
fn a11y_unknown_aria_attribute_warns() {
    let diags = analyze_with_diags(r#"<div aria-labl="name"></div>"#);
    assert_has_warning(&diags, "a11y_unknown_aria_attribute");
}

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
fn a11y_known_aria_attribute_no_unknown_warning() {
    let diags = analyze_with_diags(r#"<div aria-label="name"></div>"#);
    assert_no_warning(&diags, "a11y_unknown_aria_attribute");
}

#[test]
fn a11y_aria_attributes_warn_on_invisible_elements() {
    let diags = analyze_with_diags(r#"<meta aria-label="x" />"#);
    assert_has_warning(&diags, "a11y_aria_attributes");
}

#[test]
fn a11y_hidden_warns_on_heading_tags() {
    let diags = analyze_with_diags(r#"<h1 aria-hidden="true">Title</h1>"#);
    assert_has_warning(&diags, "a11y_hidden");
}

#[test]
fn a11y_hidden_no_warning_on_non_heading_tags() {
    let diags = analyze_with_diags(r#"<div aria-hidden="true">Title</div>"#);
    assert_no_warning(&diags, "a11y_hidden");
}

#[test]
fn a11y_incorrect_aria_attribute_type_idlist_warns_on_empty_value() {
    let diags = analyze_with_diags(r#"<div aria-labelledby=""></div>"#);
    assert_has_warning(&diags, "a11y_incorrect_aria_attribute_type_idlist");
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
fn a11y_incorrect_aria_attribute_type_integer_warns_for_non_integer() {
    let diags = analyze_with_diags(r#"<div aria-rowindex="1.5"></div>"#);
    assert_has_warning(&diags, "a11y_incorrect_aria_attribute_type_integer");
}

#[test]
fn a11y_incorrect_aria_attribute_type_boolean_warns_for_invalid_boolean() {
    let diags = analyze_with_diags(r#"<div aria-hidden="maybe"></div>"#);
    assert_has_warning(&diags, "a11y_incorrect_aria_attribute_type_boolean");
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
fn a11y_incorrect_aria_attribute_type_tokenlist_warns_for_invalid_token() {
    let diags = analyze_with_diags(r#"<div aria-dropeffect="copy wrong"></div>"#);
    assert_has_warning(&diags, "a11y_incorrect_aria_attribute_type_tokenlist");
}

#[test]
fn a11y_incorrect_aria_attribute_type_tristate_accepts_mixed() {
    let diags = analyze_with_diags(r#"<div aria-checked="mixed"></div>"#);
    assert_no_warning(&diags, "a11y_incorrect_aria_attribute_type_tristate");
}

#[test]
fn a11y_incorrect_aria_attribute_type_no_warning_for_valid_known_value() {
    let diags = analyze_with_diags(r#"<div aria-hidden="true"></div>"#);
    assert_no_warning(&diags, "a11y_incorrect_aria_attribute_type_boolean");
}

#[test]
fn a11y_incorrect_aria_attribute_type_no_warning_for_dynamic_value() {
    let diags = analyze_with_diags(
        r#"<script>let value = $state('maybe');</script><div aria-hidden={value}></div>"#,
    );
    assert_no_warning(&diags, "a11y_incorrect_aria_attribute_type_boolean");
}

#[test]
fn a11y_incorrect_aria_attribute_type_unknown_attribute_only_warns_once() {
    let diags = analyze_with_diags(r#"<div aria-labl="x"></div>"#);
    assert_has_warning(&diags, "a11y_unknown_aria_attribute");
    assert_no_warning(&diags, "a11y_incorrect_aria_attribute_type");
    assert_no_warning(&diags, "a11y_incorrect_aria_attribute_type_boolean");
    assert_no_warning(&diags, "a11y_incorrect_aria_attribute_type_idlist");
    assert_no_warning(&diags, "a11y_incorrect_aria_attribute_type_integer");
    assert_no_warning(&diags, "a11y_incorrect_aria_attribute_type_token");
    assert_no_warning(&diags, "a11y_incorrect_aria_attribute_type_tokenlist");
    assert_no_warning(&diags, "a11y_incorrect_aria_attribute_type_tristate");
}

#[test]
fn a11y_unknown_role_warns() {
    let diags = analyze_with_diags(r#"<div role="buton"></div>"#);
    assert_has_warning(&diags, "a11y_unknown_role");
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
fn a11y_abstract_role_warns() {
    let diags = analyze_with_diags(r#"<div role="widget"></div>"#);
    assert_has_warning(&diags, "a11y_no_abstract_role");
}

#[test]
fn a11y_valid_concrete_role_no_name_warning() {
    let diags = analyze_with_diags(r#"<div role="button"></div>"#);
    assert_no_warning(&diags, "a11y_unknown_role");
    assert_no_warning(&diags, "a11y_no_abstract_role");
}

#[test]
fn a11y_misplaced_role_warns_on_invisible_elements() {
    let diags = analyze_with_diags(r#"<meta role="button" />"#);
    assert_has_warning(&diags, "a11y_misplaced_role");
}

#[test]
fn a11y_role_whitespace_validates_each_token() {
    let diags = analyze_with_diags(r#"<div role="widget buton"></div>"#);
    assert_has_warning(&diags, "a11y_no_abstract_role");
    assert_has_warning(&diags, "a11y_unknown_role");
}

#[test]
fn a11y_no_redundant_roles_warns_for_native_semantics() {
    let diags = analyze_with_diags(r#"<button role="button"></button>"#);
    assert_has_warning(&diags, "a11y_no_redundant_roles");
}

#[test]
fn a11y_no_redundant_roles_warns_for_top_level_header_banner() {
    let diags = analyze_with_diags(r#"<header role="banner"></header>"#);
    assert_has_warning(&diags, "a11y_no_redundant_roles");
}

#[test]
fn a11y_no_redundant_roles_no_warning_for_section_header_banner() {
    let diags = analyze_with_diags(r#"<section><header role="banner"></header></section>"#);
    assert_no_warning(&diags, "a11y_no_redundant_roles");
}

#[test]
fn a11y_no_redundant_roles_no_warning_for_anchor_without_href() {
    let diags = analyze_with_diags(r#"<a role="link">link</a>"#);
    assert_no_warning(&diags, "a11y_no_redundant_roles");
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
fn a11y_role_has_required_aria_props_no_warning_when_props_present() {
    let diags = analyze_with_diags(
        r#"<div role="combobox" aria-controls="list" aria-expanded="false"></div>"#,
    );
    assert_no_warning(&diags, "a11y_role_has_required_aria_props");
}

#[test]
fn a11y_role_has_required_aria_props_no_warning_for_native_semantics() {
    let diags = analyze_with_diags(r#"<input type="checkbox" role="checkbox" />"#);
    assert_no_warning(&diags, "a11y_role_has_required_aria_props");
}

#[test]
fn a11y_role_has_required_aria_props_no_warning_with_spread() {
    let diags = analyze_with_diags(
        r#"<script>let props = $state({});</script><div role="combobox" {...props}></div>"#,
    );
    assert_no_warning(&diags, "a11y_role_has_required_aria_props");
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
fn a11y_role_supports_aria_props_no_warning_for_supported_explicit_role_prop() {
    let diags = analyze_with_diags(r#"<div role="button" aria-expanded="true"></div>"#);
    assert_no_warning(&diags, "a11y_role_supports_aria_props");
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
fn a11y_role_supports_aria_props_no_warning_for_supported_implicit_role_prop() {
    let diags = analyze_with_diags(r#"<button aria-expanded="true"></button>"#);
    assert_no_warning(&diags, "a11y_role_supports_aria_props_implicit");
}

#[test]
fn a11y_role_supports_aria_props_unknown_aria_attr_only_warns_once() {
    let diags = analyze_with_diags(r#"<button aria-labl="x"></button>"#);
    assert_has_warning(&diags, "a11y_unknown_aria_attribute");
    assert_no_warning(&diags, "a11y_role_supports_aria_props");
    assert_no_warning(&diags, "a11y_role_supports_aria_props_implicit");
}

#[test]
fn a11y_role_supports_aria_props_no_warning_without_role() {
    let diags = analyze_with_diags(r#"<div aria-checked="true"></div>"#);
    assert_no_warning(&diags, "a11y_role_supports_aria_props");
    assert_no_warning(&diags, "a11y_role_supports_aria_props_implicit");
}

#[test]
fn a11y_aria_activedescendant_has_tabindex_warns_without_tabindex() {
    let diags = analyze_with_diags(r#"<div aria-activedescendant="item"></div>"#);
    assert_has_warning(&diags, "a11y_aria_activedescendant_has_tabindex");
}

#[test]
fn a11y_aria_activedescendant_has_tabindex_no_warning_with_tabindex() {
    let diags = analyze_with_diags(r#"<div aria-activedescendant="item" tabindex="0"></div>"#);
    assert_no_warning(&diags, "a11y_aria_activedescendant_has_tabindex");
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
fn a11y_interactive_supports_focus_no_warning_with_tabindex() {
    let diags = analyze_with_diags(r#"<div role="button" tabindex="0" onclick={handle}></div>"#);
    assert_no_warning(&diags, "a11y_interactive_supports_focus");
}

#[test]
fn a11y_interactive_supports_focus_no_warning_when_disabled() {
    let diags =
        analyze_with_diags(r#"<div role="button" aria-disabled="true" onclick={handle}></div>"#);
    assert_no_warning(&diags, "a11y_interactive_supports_focus");
}

#[test]
fn a11y_interactive_supports_focus_no_warning_for_native_interactive_element() {
    let diags = analyze_with_diags(r#"<button onclick={handle}></button>"#);
    assert_no_warning(&diags, "a11y_interactive_supports_focus");
}

#[test]
fn a11y_click_events_have_key_events_warns_for_visible_noninteractive_element() {
    let diags = analyze_with_diags(r#"<div onclick={handle}></div>"#);
    assert_has_warning(&diags, "a11y_click_events_have_key_events");
}

#[test]
fn a11y_click_events_have_key_events_no_warning_with_keyboard_handler() {
    let diags = analyze_with_diags(r#"<div onclick={handle} onkeydown={handle}></div>"#);
    assert_no_warning(&diags, "a11y_click_events_have_key_events");
}

#[test]
fn a11y_click_events_have_key_events_no_warning_for_interactive_element() {
    let diags = analyze_with_diags(r#"<button onclick={handle}></button>"#);
    assert_no_warning(&diags, "a11y_click_events_have_key_events");
}

#[test]
fn a11y_no_noninteractive_element_interactions_warns_for_noninteractive_role() {
    let diags = analyze_with_diags(r#"<div role="article" onclick={handle}></div>"#);
    assert_has_warning(&diags, "a11y_no_noninteractive_element_interactions");
}

#[test]
fn a11y_no_noninteractive_element_interactions_no_warning_with_contenteditable() {
    let diags =
        analyze_with_diags(r#"<div role="article" contenteditable onclick={handle}></div>"#);
    assert_no_warning(&diags, "a11y_no_noninteractive_element_interactions");
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
fn a11y_no_static_element_interactions_no_warning_with_explicit_role() {
    let diags = analyze_with_diags(r#"<div role="button" onmousedown={handle}></div>"#);
    assert_no_warning(&diags, "a11y_no_static_element_interactions");
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
fn a11y_mouse_events_have_key_events_no_warning_with_paired_handler() {
    let diags = analyze_with_diags(r#"<div onmouseover={handle} onfocus={handle}></div>"#);
    assert_no_warning(&diags, "a11y_mouse_events_have_key_events");
}

#[test]
#[ignore = "missing: interactive element to noninteractive role warning (analyze)"]
fn a11y_no_interactive_element_to_noninteractive_role_warns_for_button_role_presentation() {
    let diags = analyze_with_diags(r#"<button role="presentation"></button>"#);
    assert_has_warning(&diags, "a11y_no_interactive_element_to_noninteractive_role");
}

#[test]
#[ignore = "missing: noninteractive element to interactive role warning (analyze)"]
fn a11y_no_noninteractive_element_to_interactive_role_warns_for_div_role_button() {
    let diags = analyze_with_diags(r#"<div role="button"></div>"#);
    assert_has_warning(&diags, "a11y_no_noninteractive_element_to_interactive_role");
}

#[test]
#[ignore = "missing: explicit label warning for unlabeled button/link content (analyze)"]
fn a11y_consider_explicit_label_warns_for_icon_button() {
    let diags = analyze_with_diags(r#"<button><svg aria-hidden="true"></svg></button>"#);
    assert_has_warning(&diags, "a11y_consider_explicit_label");
}

#[test]
#[ignore = "missing: invalid href attribute warning (analyze)"]
fn a11y_invalid_attribute_warns_for_anchor_hash_href() {
    let diags = analyze_with_diags(r##"<a href="#">jump</a>"##);
    assert_has_warning(&diags, "a11y_invalid_attribute");
}

#[test]
#[ignore = "missing: label association warning (analyze)"]
fn a11y_label_has_associated_control_warns_without_for_or_control() {
    let diags = analyze_with_diags(r#"<label>Username</label>"#);
    assert_has_warning(&diags, "a11y_label_has_associated_control");
}
