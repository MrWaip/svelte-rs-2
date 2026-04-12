use super::*;
use svelte_diagnostics::codes::fuzzymatch;

#[derive(Clone, Copy)]
enum AriaValueKind {
    Id,
    String,
    Number,
    Boolean,
    Idlist,
    Integer,
    Token(&'static [&'static str]),
    Tokenlist(&'static [&'static str]),
    Tristate,
}

#[derive(Clone, Copy)]
enum StaticAttributeValue<'a> {
    Text(&'a str),
    True,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ElementInteractivity {
    Interactive,
    NonInteractive,
    Static,
}

const A11Y_LABELABLE_ELEMENTS: &[&str] = &[
    "button", "input", "keygen", "meter", "output", "progress", "select", "textarea",
];
const A11Y_REQUIRED_CONTENT: &[&str] = &["h1", "h2", "h3", "h4", "h5", "h6"];
const A11Y_AUTOCOMPLETE_ADDRESS_TYPE_TOKENS: &[&str] = &["shipping", "billing"];
const A11Y_AUTOCOMPLETE_FIELD_NAME_TOKENS: &[&str] = &[
    "",
    "on",
    "off",
    "name",
    "honorific-prefix",
    "given-name",
    "additional-name",
    "family-name",
    "honorific-suffix",
    "nickname",
    "username",
    "new-password",
    "current-password",
    "one-time-code",
    "organization-title",
    "organization",
    "street-address",
    "address-line1",
    "address-line2",
    "address-line3",
    "address-level4",
    "address-level3",
    "address-level2",
    "address-level1",
    "country",
    "country-name",
    "postal-code",
    "cc-name",
    "cc-given-name",
    "cc-additional-name",
    "cc-family-name",
    "cc-number",
    "cc-exp",
    "cc-exp-month",
    "cc-exp-year",
    "cc-csc",
    "cc-type",
    "transaction-currency",
    "transaction-amount",
    "language",
    "bday",
    "bday-day",
    "bday-month",
    "bday-year",
    "sex",
    "url",
    "photo",
];
const A11Y_AUTOCOMPLETE_CONTACT_TYPE_TOKENS: &[&str] = &["home", "work", "mobile", "fax", "pager"];
const A11Y_AUTOCOMPLETE_CONTACT_FIELD_NAME_TOKENS: &[&str] = &[
    "tel",
    "tel-country-code",
    "tel-national",
    "tel-area-code",
    "tel-local",
    "tel-local-prefix",
    "tel-local-suffix",
    "tel-extension",
    "email",
    "impp",
];
const A11Y_REDUNDANT_IMG_ALT_TOKENS: &[&str] = &["image", "picture", "photo"];

pub(super) fn check_element_warnings(
    el: &Element,
    attrs: &[Attribute],
    accesskey_attr: Option<&Attribute>,
    tabindex_attr: Option<&Attribute>,
    has_autofocus: bool,
    dialog_depth: u32,
    missing_attr_diag: Option<Diagnostic>,
    ctx: &mut VisitContext<'_>,
) {
    if let Some(attr) = ctx.data.attribute(el.id, attrs, "scope") {
        if el.name != "th" {
            ctx.push_warning_if_not_ignored(
                el.id,
                DiagnosticKind::A11yMisplacedScope,
                attr_value_span(attr),
            );
        }
    }

    if matches!(el.name.as_str(), "marquee" | "blink") {
        ctx.warnings_mut().push(Diagnostic::warning(
            DiagnosticKind::A11yDistractingElements {
                name: el.name.clone(),
            },
            el.span,
        ));
    }

    if let Some(attr) = accesskey_attr {
        ctx.warnings_mut().push(Diagnostic::warning(
            DiagnosticKind::A11yAccesskey,
            attr_value_span(attr),
        ));
    }

    if let Some(attr) = tabindex_attr {
        if let Some(text) = static_text_attr_value(attr, ctx.source) {
            if let Ok(n) = text.trim().parse::<i64>() {
                if n > 0 {
                    ctx.warnings_mut().push(Diagnostic::warning(
                        DiagnosticKind::A11yPositiveTabindex,
                        attr_value_span(attr),
                    ));
                }
            }
        }
    }

    if has_autofocus && el.name != "dialog" && dialog_depth == 0 {
        ctx.warnings_mut()
            .push(Diagnostic::warning(DiagnosticKind::A11yAutofocus, el.span));
    }

    if let Some(diag) = missing_attr_diag {
        ctx.warnings_mut().push(diag);
    }

    check_a11y_aria_attribute_warnings(el, attrs, ctx);
    check_a11y_role_warnings(el, attrs, ctx);
    check_a11y_role_supported_aria_props_warnings(el, attrs, ctx);
    check_a11y_role_attribute_interaction_warnings(el, attrs, ctx);
    check_a11y_element_specific_content_warnings(el, attrs, ctx);
}

fn static_attr_value<'a>(attr: &Attribute, source: &'a str) -> Option<StaticAttributeValue<'a>> {
    match attr {
        Attribute::StringAttribute(attr) => Some(StaticAttributeValue::Text(
            attr.value_span.source_text(source),
        )),
        Attribute::BooleanAttribute(_) => Some(StaticAttributeValue::True),
        _ => None,
    }
}

fn attr_named_name(attr: &Attribute) -> Option<&str> {
    match attr {
        Attribute::StringAttribute(attr) => Some(&attr.name),
        Attribute::ExpressionAttribute(attr) => Some(&attr.name),
        Attribute::BooleanAttribute(attr) => Some(&attr.name),
        Attribute::ConcatenationAttribute(attr) => Some(&attr.name),
        Attribute::BindDirective(attr) => Some(&attr.name),
        Attribute::Shorthand(_)
        | Attribute::SpreadAttribute(_)
        | Attribute::ClassDirective(_)
        | Attribute::LetDirectiveLegacy(_)
        | Attribute::StyleDirective(_)
        | Attribute::UseDirective(_)
        | Attribute::OnDirectiveLegacy(_)
        | Attribute::TransitionDirective(_)
        | Attribute::AnimateDirective(_)
        | Attribute::AttachTag(_) => None,
    }
}

fn is_heading_tag(name: &str) -> bool {
    matches!(name, "h1" | "h2" | "h3" | "h4" | "h5" | "h6")
}

fn aria_attribute_value_kind(name: &str) -> Option<AriaValueKind> {
    match name {
        "aria-activedescendant" | "aria-details" | "aria-errormessage" => Some(AriaValueKind::Id),
        "aria-braillelabel"
        | "aria-brailleroledescription"
        | "aria-description"
        | "aria-keyshortcuts"
        | "aria-label"
        | "aria-placeholder"
        | "aria-roledescription"
        | "aria-valuetext" => Some(AriaValueKind::String),
        "aria-valuemax" | "aria-valuemin" | "aria-valuenow" => Some(AriaValueKind::Number),
        "aria-atomic"
        | "aria-busy"
        | "aria-disabled"
        | "aria-expanded"
        | "aria-grabbed"
        | "aria-hidden"
        | "aria-modal"
        | "aria-multiline"
        | "aria-multiselectable"
        | "aria-readonly"
        | "aria-required"
        | "aria-selected" => Some(AriaValueKind::Boolean),
        "aria-controls" | "aria-describedby" | "aria-flowto" | "aria-labelledby" | "aria-owns" => {
            Some(AriaValueKind::Idlist)
        }
        "aria-colcount" | "aria-colindex" | "aria-colspan" | "aria-level" | "aria-posinset"
        | "aria-rowcount" | "aria-rowindex" | "aria-rowspan" | "aria-setsize" => {
            Some(AriaValueKind::Integer)
        }
        "aria-autocomplete" => Some(AriaValueKind::Token(super::A11Y_ARIA_AUTOCOMPLETE_VALUES)),
        "aria-current" => Some(AriaValueKind::Token(super::A11Y_ARIA_CURRENT_VALUES)),
        "aria-haspopup" => Some(AriaValueKind::Token(super::A11Y_ARIA_HASPOPUP_VALUES)),
        "aria-invalid" => Some(AriaValueKind::Token(super::A11Y_ARIA_INVALID_VALUES)),
        "aria-live" => Some(AriaValueKind::Token(super::A11Y_ARIA_LIVE_VALUES)),
        "aria-orientation" => Some(AriaValueKind::Token(super::A11Y_ARIA_ORIENTATION_VALUES)),
        "aria-sort" => Some(AriaValueKind::Token(super::A11Y_ARIA_SORT_VALUES)),
        "aria-dropeffect" => Some(AriaValueKind::Tokenlist(super::A11Y_ARIA_DROPEFFECT_VALUES)),
        "aria-relevant" => Some(AriaValueKind::Tokenlist(super::A11Y_ARIA_RELEVANT_VALUES)),
        "aria-checked" | "aria-pressed" => Some(AriaValueKind::Tristate),
        _ => None,
    }
}

fn format_quoted_list(values: &[&str]) -> String {
    let quoted = values
        .iter()
        .map(|value| format!("\"{value}\""))
        .collect::<Vec<_>>();

    match quoted.as_slice() {
        [] => String::new(),
        [single] => single.clone(),
        [left, right] => format!("{left} or {right}"),
        _ => {
            let last = quoted.last().cloned().unwrap_or_default();
            let leading = &quoted[..quoted.len() - 1];
            format!("{} or {}", leading.join(", "), last)
        }
    }
}

fn validate_aria_attribute_value(
    attr: &Attribute,
    name: &str,
    kind: AriaValueKind,
    ctx: &mut VisitContext<'_>,
) {
    let span = attr_value_span(attr);
    let Some(value) = static_attr_value(attr, ctx.source) else {
        return;
    };
    let value = match value {
        StaticAttributeValue::Text(text) => text,
        StaticAttributeValue::True => "",
    };

    let diagnostic = match kind {
        AriaValueKind::Id | AriaValueKind::String if value.is_empty() => {
            Some(DiagnosticKind::A11yIncorrectAriaAttributeType {
                attribute: name.to_string(),
                type_: "non-empty string".to_string(),
            })
        }
        AriaValueKind::Number if value.is_empty() || value.trim().parse::<f64>().is_err() => {
            Some(DiagnosticKind::A11yIncorrectAriaAttributeType {
                attribute: name.to_string(),
                type_: "number".to_string(),
            })
        }
        AriaValueKind::Boolean if !matches!(value, "true" | "false") => {
            Some(DiagnosticKind::A11yIncorrectAriaAttributeTypeBoolean {
                attribute: name.to_string(),
            })
        }
        AriaValueKind::Idlist if value.is_empty() => {
            Some(DiagnosticKind::A11yIncorrectAriaAttributeTypeIdlist {
                attribute: name.to_string(),
            })
        }
        AriaValueKind::Integer
            if value.is_empty()
                || value
                    .trim()
                    .parse::<f64>()
                    .ok()
                    .is_none_or(|number| !number.is_finite() || number.fract() != 0.0) =>
        {
            Some(DiagnosticKind::A11yIncorrectAriaAttributeTypeInteger {
                attribute: name.to_string(),
            })
        }
        AriaValueKind::Token(values) if !values.contains(&value.to_ascii_lowercase().as_str()) => {
            Some(DiagnosticKind::A11yIncorrectAriaAttributeTypeToken {
                attribute: name.to_string(),
                values: format_quoted_list(values),
            })
        }
        AriaValueKind::Tokenlist(values)
            if value
                .split_whitespace()
                .map(str::to_ascii_lowercase)
                .any(|token| !values.contains(&token.as_str())) =>
        {
            Some(DiagnosticKind::A11yIncorrectAriaAttributeTypeTokenlist {
                attribute: name.to_string(),
                values: format_quoted_list(values),
            })
        }
        AriaValueKind::Tristate if !matches!(value, "true" | "false" | "mixed") => {
            Some(DiagnosticKind::A11yIncorrectAriaAttributeTypeTristate {
                attribute: name.to_string(),
            })
        }
        _ => None,
    };

    if let Some(diagnostic) = diagnostic {
        ctx.warnings_mut()
            .push(Diagnostic::warning(diagnostic, span));
    }
}

fn check_a11y_aria_attribute_warnings(
    el: &Element,
    attrs: &[Attribute],
    ctx: &mut VisitContext<'_>,
) {
    for attr in attrs {
        let Some(name) = attr_named_name(attr) else {
            continue;
        };
        let name = name.to_ascii_lowercase();
        if !name.starts_with("aria-") {
            continue;
        }

        if super::A11Y_INVISIBLE_ELEMENTS.contains(&el.name.as_str()) {
            ctx.warnings_mut().push(Diagnostic::warning(
                DiagnosticKind::A11yAriaAttributes {
                    name: el.name.clone(),
                },
                attr_value_span(attr),
            ));
        }

        let attribute = name.trim_start_matches("aria-");
        if !super::A11Y_ARIA_ATTRIBUTES.contains(&attribute) {
            ctx.warnings_mut().push(Diagnostic::warning(
                DiagnosticKind::A11yUnknownAriaAttribute {
                    attribute: attribute.to_string(),
                    suggestion: fuzzymatch(attribute, super::A11Y_ARIA_ATTRIBUTES)
                        .map(|s| format!("aria-{s}")),
                },
                attr_value_span(attr),
            ));
            continue;
        }

        if name == "aria-hidden" && is_heading_tag(&el.name) {
            ctx.warnings_mut().push(Diagnostic::warning(
                DiagnosticKind::A11yHidden {
                    name: el.name.clone(),
                },
                attr_value_span(attr),
            ));
        }

        if let Some(kind) = aria_attribute_value_kind(name.as_str()) {
            validate_aria_attribute_value(attr, name.as_str(), kind, ctx);
        }
    }
}

fn check_a11y_role_warnings(el: &Element, attrs: &[Attribute], ctx: &mut VisitContext<'_>) {
    let has_spread = ctx.data.has_spread(el.id);
    let semantic_role = semantic_role_for_element(el, attrs, ctx);
    let interactivity = element_interactivity(el, attrs, ctx);
    let is_interactive = interactivity == ElementInteractivity::Interactive;
    let is_non_interactive = interactivity == ElementInteractivity::NonInteractive;

    for attr in attrs {
        let Some(name) = attr_named_name(attr) else {
            continue;
        };
        if !name.eq_ignore_ascii_case("role") {
            continue;
        }

        if super::A11Y_INVISIBLE_ELEMENTS.contains(&el.name.as_str()) {
            ctx.warnings_mut().push(Diagnostic::warning(
                DiagnosticKind::A11yMisplacedRole {
                    name: el.name.clone(),
                },
                attr_value_span(attr),
            ));
        }

        let Some(value) = static_text_attr_value(attr, ctx.source) else {
            continue;
        };

        for role in value.split_ascii_whitespace() {
            if role.is_empty() {
                continue;
            }

            if super::A11Y_ABSTRACT_ROLES.contains(&role) {
                ctx.warnings_mut().push(Diagnostic::warning(
                    DiagnosticKind::A11yNoAbstractRole {
                        role: role.to_string(),
                    },
                    attr_value_span(attr),
                ));
            } else if !super::A11Y_ARIA_ROLES.contains(&role) {
                ctx.warnings_mut().push(Diagnostic::warning(
                    DiagnosticKind::A11yUnknownRole {
                        role: role.to_string(),
                        suggestion: fuzzymatch(role, super::A11Y_ARIA_ROLES).map(str::to_string),
                    },
                    attr_value_span(attr),
                ));
            }

            if !has_spread
                && is_interactive
                && (is_non_interactive_role(Some(role)) || is_presentation_role(Some(role)))
            {
                ctx.warnings_mut().push(Diagnostic::warning(
                    DiagnosticKind::A11yNoInteractiveElementToNoninteractiveRole {
                        element: el.name.clone(),
                        role: role.to_string(),
                    },
                    el.span,
                ));
            }

            if !has_spread
                && is_non_interactive
                && is_interactive_role(Some(role))
                && !is_noninteractive_element_to_interactive_role_exception(el.name.as_str(), role)
            {
                ctx.warnings_mut().push(Diagnostic::warning(
                    DiagnosticKind::A11yNoNoninteractiveElementToInteractiveRole {
                        element: el.name.clone(),
                        role: role.to_string(),
                    },
                    el.span,
                ));
            }

            if semantic_role == Some(role)
                && !matches!(el.name.as_str(), "ul" | "ol" | "li" | "menu")
                && !(el.name == "a" && !ctx.data.has_attribute(el.id, "href"))
            {
                ctx.warnings_mut().push(Diagnostic::warning(
                    DiagnosticKind::A11yNoRedundantRoles {
                        role: role.to_string(),
                    },
                    attr_value_span(attr),
                ));
            }

            let Some(required_props) = required_role_props(role) else {
                continue;
            };

            if has_spread || is_semantic_role_element(el, attrs, ctx, role) {
                continue;
            }

            if required_props
                .iter()
                .any(|prop| ctx.data.attribute(el.id, attrs, prop).is_none())
            {
                ctx.warnings_mut().push(Diagnostic::warning(
                    DiagnosticKind::A11yRoleHasRequiredAriaProps {
                        role: role.to_string(),
                        props: format_required_role_props(required_props),
                    },
                    attr_value_span(attr),
                ));
            }
        }
    }
}

fn check_a11y_role_supported_aria_props_warnings(
    el: &Element,
    attrs: &[Attribute],
    ctx: &mut VisitContext<'_>,
) {
    let explicit_role_attr = ctx.data.attribute(el.id, attrs, "role");
    let role_value = explicit_role_attr
        .and_then(|attr| static_text_attr_value(attr, ctx.source))
        .or_else(|| {
            explicit_role_attr
                .is_none()
                .then(|| implicit_role_for_element(el, attrs, ctx))
                .flatten()
        });
    let Some(role_value) = role_value else {
        return;
    };

    if !is_known_role_for_prop_support(role_value) {
        return;
    }

    let is_implicit = explicit_role_attr.is_none();

    for attr in attrs {
        let Some(name) = attr_named_name(attr) else {
            continue;
        };
        let name = name.to_ascii_lowercase();
        if !name.starts_with("aria-") {
            continue;
        }

        let attribute = name.trim_start_matches("aria-");
        if !super::A11Y_ARIA_ATTRIBUTES.contains(&attribute) {
            continue;
        }

        if role_supports_aria_prop(role_value, name.as_str()) {
            continue;
        }

        let diag = if is_implicit {
            DiagnosticKind::A11yRoleSupportsAriaPropsImplicit {
                attribute: name.clone(),
                role: role_value.to_string(),
                name: el.name.clone(),
            }
        } else {
            DiagnosticKind::A11yRoleSupportsAriaProps {
                attribute: name.clone(),
                role: role_value.to_string(),
            }
        };
        ctx.warnings_mut()
            .push(Diagnostic::warning(diag, attr_value_span(attr)));
    }
}

fn check_a11y_role_attribute_interaction_warnings(
    el: &Element,
    attrs: &[Attribute],
    ctx: &mut VisitContext<'_>,
) {
    let interactivity = element_interactivity(el, attrs, ctx);
    let is_interactive = interactivity == ElementInteractivity::Interactive;
    let is_non_interactive = interactivity == ElementInteractivity::NonInteractive;
    let is_static = interactivity == ElementInteractivity::Static;
    let has_spread = ctx.data.has_spread(el.id);
    let has_tabindex = ctx.data.attribute(el.id, attrs, "tabindex");
    let role_attr = ctx.data.attribute(el.id, attrs, "role");
    let role_static_value = role_attr.and_then(|attr| static_text_attr_value(attr, ctx.source));
    let handlers = collect_element_handlers(attrs);
    let is_hidden = is_hidden_from_screen_reader(el, attrs, ctx);
    let has_contenteditable_attr = ctx.data.has_attribute(el.id, "contenteditable");

    for attr in attrs {
        let Some(name) = attr_named_name(attr) else {
            continue;
        };
        let name = name.to_ascii_lowercase();
        if name == "aria-activedescendant"
            && !is_interactive
            && has_tabindex.is_none()
            && !has_spread
        {
            ctx.warnings_mut().push(Diagnostic::warning(
                DiagnosticKind::A11yAriaActivedescendantHasTabindex,
                attr_value_span(attr),
            ));
        }
    }

    if handlers.iter().any(|handler| handler == "click") {
        let is_non_presentation_role =
            role_static_value.is_some_and(|role| !is_presentation_role(Some(role)));
        let has_key_event = handlers
            .iter()
            .any(|handler| matches!(handler.as_str(), "keydown" | "keyup" | "keypress"));
        if !is_hidden
            && (role_attr.is_none() || is_non_presentation_role)
            && !is_interactive
            && !has_spread
            && !has_key_event
        {
            ctx.warnings_mut().push(Diagnostic::warning(
                DiagnosticKind::A11yClickEventsHaveKeyEvents,
                el.span,
            ));
        }
    }

    if let Some(tabindex_attr) = has_tabindex {
        if !is_interactive && !is_interactive_role(role_static_value) {
            let should_warn = static_text_attr_value(tabindex_attr, ctx.source)
                .and_then(|value| value.trim().parse::<f64>().ok())
                .is_none_or(|value| value >= 0.0);
            if should_warn {
                ctx.warnings_mut().push(Diagnostic::warning(
                    DiagnosticKind::A11yNoNoninteractiveTabindex,
                    attr_value_span(tabindex_attr),
                ));
            }
        }
    }

    if !has_spread
        && !has_contenteditable_attr
        && !is_hidden
        && !is_presentation_role(role_static_value)
        && ((!is_interactive && is_non_interactive_role(role_static_value))
            || (is_non_interactive && role_attr.is_none()))
        && handlers
            .iter()
            .any(|handler| super::A11Y_RECOMMENDED_INTERACTIVE_HANDLERS.contains(&handler.as_str()))
    {
        ctx.warnings_mut().push(Diagnostic::warning(
            DiagnosticKind::A11yNoNoninteractiveElementInteractions {
                element: el.name.clone(),
            },
            el.span,
        ));
    }

    if !has_spread
        && (role_attr.is_none() || role_static_value.is_some())
        && !is_hidden
        && !is_interactive
        && !is_interactive_role(role_static_value)
        && !is_non_interactive
        && !is_non_interactive_role(role_static_value)
        && !is_abstract_role(role_static_value)
    {
        let interactive_handlers = handlers
            .iter()
            .filter(|handler| super::A11Y_INTERACTIVE_HANDLERS.contains(&handler.as_str()))
            .map(String::as_str)
            .collect::<Vec<_>>();
        if !interactive_handlers.is_empty() {
            ctx.push_warning_if_not_ignored(
                el.id,
                DiagnosticKind::A11yNoStaticElementInteractions {
                    element: el.name.clone(),
                    handler: format_handler_list(&interactive_handlers),
                },
                el.span,
            );
        }
    }

    if !has_spread && handlers.iter().any(|handler| handler == "mouseover") {
        if !handlers.iter().any(|handler| handler == "focus") {
            ctx.warnings_mut().push(Diagnostic::warning(
                DiagnosticKind::A11yMouseEventsHaveKeyEvents {
                    event: "mouseover".to_string(),
                    accompanied_by: "focus".to_string(),
                },
                el.span,
            ));
        }
    }

    if !has_spread && handlers.iter().any(|handler| handler == "mouseout") {
        if !handlers.iter().any(|handler| handler == "blur") {
            ctx.warnings_mut().push(Diagnostic::warning(
                DiagnosticKind::A11yMouseEventsHaveKeyEvents {
                    event: "mouseout".to_string(),
                    accompanied_by: "blur".to_string(),
                },
                el.span,
            ));
        }
    }

    let has_interactive_handlers = handlers
        .iter()
        .any(|handler| super::A11Y_INTERACTIVE_HANDLERS.contains(&handler.as_str()));
    if !has_interactive_handlers
        || has_spread
        || has_disabled_attribute(el.id, attrs, ctx)
        || is_hidden
        || has_tabindex.is_some()
        || !is_static
    {
        return;
    }

    let Some(role_attr) = role_attr else {
        return;
    };
    let Some(role_value) = static_text_attr_value(role_attr, ctx.source) else {
        return;
    };

    for role in role_value.split_ascii_whitespace() {
        if role.is_empty() || !is_interactive_role(Some(role)) || is_presentation_role(Some(role)) {
            continue;
        }

        ctx.warnings_mut().push(Diagnostic::warning(
            DiagnosticKind::A11yInteractiveSupportsFocus {
                role: role.to_string(),
            },
            el.span,
        ));
    }
}

fn check_a11y_element_specific_content_warnings(
    el: &Element,
    attrs: &[Attribute],
    ctx: &mut VisitContext<'_>,
) {
    let has_spread = ctx.data.has_spread(el.id);
    let is_labelled = ["aria-label", "aria-labelledby", "title"]
        .into_iter()
        .any(|name| ctx.data.has_attribute(el.id, name));

    match el.name.as_str() {
        "a" | "button" => {
            let inert_is_static = ctx
                .data
                .attribute(el.id, attrs, "inert")
                .and_then(|attr| static_attr_value(attr, ctx.source))
                .is_some();
            let is_hidden = ctx
                .data
                .attribute(el.id, attrs, "aria-hidden")
                .and_then(|attr| static_text_attr_value(attr, ctx.source))
                .is_some_and(|value| value == "true")
                || inert_is_static;

            if !has_spread && !is_hidden && !is_labelled && !has_content(&el.fragment, ctx) {
                ctx.push_warning_if_not_ignored(
                    el.id,
                    DiagnosticKind::A11yConsiderExplicitLabel,
                    el.span,
                );
            }

            if el.name == "button" {
                return;
            }

            let href_attr = ctx
                .data
                .attribute(el.id, attrs, "href")
                .or_else(|| ctx.data.attribute(el.id, attrs, "xlink:href"));
            let Some(href_attr) = href_attr else {
                return;
            };
            let Some(href_value) = static_text_attr_value(href_attr, ctx.source) else {
                return;
            };

            if href_value.is_empty() || href_value == "#" || has_javascript_prefix(href_value) {
                let href_attribute = attr_named_name(href_attr).unwrap_or("href").to_string();
                ctx.push_warning_if_not_ignored(
                    el.id,
                    DiagnosticKind::A11yInvalidAttribute {
                        href_value: href_value.to_string(),
                        href_attribute,
                    },
                    attr_value_span(href_attr),
                );
            }
        }
        "label" => {
            if has_spread
                || ctx.data.has_attribute(el.id, "for")
                || has_associated_control(&el.fragment, ctx)
            {
                return;
            }

            ctx.push_warning_if_not_ignored(
                el.id,
                DiagnosticKind::A11yLabelHasAssociatedControl,
                el.span,
            );
        }
        "input" => {
            let type_attr = ctx.data.attribute(el.id, attrs, "type");
            let autocomplete_attr = ctx.data.attribute(el.id, attrs, "autocomplete");

            if let (Some(type_attr), Some(autocomplete_attr)) = (type_attr, autocomplete_attr) {
                let autocomplete_value = static_attr_value(autocomplete_attr, ctx.source);
                if !is_valid_autocomplete(autocomplete_value) {
                    let value = static_attr_value_text(autocomplete_value).unwrap_or("true");
                    let type_ = static_text_attr_value(type_attr, ctx.source).unwrap_or("...");
                    ctx.push_warning_if_not_ignored(
                        el.id,
                        DiagnosticKind::A11yAutocompleteValid {
                            value: value.to_string(),
                            type_: type_.to_string(),
                        },
                        attr_value_span(autocomplete_attr),
                    );
                }
            }
        }
        "img" => {
            let alt_value = ctx
                .data
                .attribute(el.id, attrs, "alt")
                .and_then(|attr| static_text_attr_value(attr, ctx.source));
            let aria_hidden = ctx
                .data
                .attribute(el.id, attrs, "aria-hidden")
                .and_then(|attr| static_attr_value(attr, ctx.source));

            if !has_spread && aria_hidden.is_none() && alt_value.is_some_and(is_redundant_img_alt) {
                ctx.push_warning_if_not_ignored(
                    el.id,
                    DiagnosticKind::A11yImgRedundantAlt,
                    el.span,
                );
            }
        }
        "video" => {
            let aria_hidden_is_static = ctx
                .data
                .attribute(el.id, attrs, "aria-hidden")
                .and_then(|attr| static_attr_value(attr, ctx.source))
                .is_some_and(|value| matches!(value, StaticAttributeValue::Text("true")));
            if has_spread
                || ctx.data.has_attribute(el.id, "muted")
                || aria_hidden_is_static
                || !ctx.data.has_attribute(el.id, "src")
            {
                return;
            }

            if !video_has_caption_track(&el.fragment, ctx) {
                ctx.push_warning_if_not_ignored(
                    el.id,
                    DiagnosticKind::A11yMediaHasCaption,
                    el.span,
                );
            }
        }
        "figcaption" => {
            let is_direct_child_of_figure = ctx.data.parent(el.id).is_some_and(|parent| {
                parent.kind == ParentKind::Element
                    && ctx
                        .store
                        .get(parent.id)
                        .as_element()
                        .is_some_and(|parent_el| parent_el.name == "figure")
            });
            if !is_direct_child_of_figure {
                ctx.push_warning_if_not_ignored(
                    el.id,
                    DiagnosticKind::A11yFigcaptionParent,
                    el.span,
                );
            }
        }
        "figure" => {
            let children = visible_figure_children(&el.fragment, ctx);
            let Some(index) = children.iter().position(|&child_id| {
                ctx.store
                    .get(child_id)
                    .as_element()
                    .is_some_and(|child| child.name == "figcaption")
            }) else {
                return;
            };

            if index != 0 && index != children.len() - 1 {
                ctx.push_warning_if_not_ignored(
                    el.id,
                    DiagnosticKind::A11yFigcaptionIndex,
                    ctx.store.get(children[index]).span(),
                );
            }
        }
        _ => {}
    }

    if !has_spread
        && !is_labelled
        && !ctx.data.elements.flags.is_bound_contenteditable(el.id)
        && A11Y_REQUIRED_CONTENT.contains(&el.name.as_str())
        && !has_content(&el.fragment, ctx)
    {
        ctx.push_warning_if_not_ignored(
            el.id,
            DiagnosticKind::A11yMissingContent {
                name: el.name.clone(),
            },
            el.span,
        );
    }
}

fn implicit_role_for_element<'a>(
    el: &Element,
    attrs: &'a [Attribute],
    ctx: &VisitContext<'a>,
) -> Option<&'static str> {
    match el.name.as_str() {
        "menuitem" => {
            let type_attr = ctx.data.attribute(el.id, attrs, "type")?;
            let type_value = static_text_attr_value(type_attr, ctx.source)?;
            lookup_static_pair(super::A11Y_MENUITEM_IMPLICIT_ROLES, type_value)
        }
        "input" => {
            let type_attr = ctx.data.attribute(el.id, attrs, "type")?;
            let type_value = static_text_attr_value(type_attr, ctx.source)?;
            if ctx.data.has_attribute(el.id, "list")
                && super::A11Y_COMBOBOX_INPUT_TYPES.contains(&type_value)
            {
                Some("combobox")
            } else {
                lookup_static_pair(super::A11Y_INPUT_IMPLICIT_ROLES, type_value)
            }
        }
        _ => lookup_static_pair(super::A11Y_IMPLICIT_ROLES, el.name.as_str()),
    }
}

fn semantic_role_for_element<'a>(
    el: &Element,
    attrs: &'a [Attribute],
    ctx: &VisitContext<'a>,
) -> Option<&'static str> {
    implicit_role_for_element(el, attrs, ctx).or_else(|| {
        (!has_sectioning_ancestor(el.id, ctx))
            .then(|| nested_implicit_role(el.name.as_str()))
            .flatten()
    })
}

fn nested_implicit_role(name: &str) -> Option<&'static str> {
    lookup_static_pair(super::A11Y_NESTED_IMPLICIT_ROLES, name)
}

fn required_role_props(role: &str) -> Option<&'static [&'static str]> {
    super::A11Y_REQUIRED_ROLE_PROPS
        .iter()
        .find_map(|(name, props)| (*name == role).then_some(*props))
}

fn is_semantic_role_element<'a>(
    el: &Element,
    attrs: &'a [Attribute],
    ctx: &VisitContext<'a>,
    role: &str,
) -> bool {
    semantic_role_for_element(el, attrs, ctx).is_some_and(|semantic| semantic == role)
}

fn has_sectioning_ancestor(id: NodeId, ctx: &VisitContext<'_>) -> bool {
    ctx.data.ancestors(id).any(|parent| {
        if parent.kind != ParentKind::Element {
            return false;
        }

        ctx.store
            .get(parent.id)
            .as_element()
            .is_some_and(|element| matches!(element.name.as_str(), "section" | "article"))
    })
}

fn lookup_static_pair<'a>(pairs: &'a [(&'a str, &'a str)], needle: &str) -> Option<&'a str> {
    pairs
        .iter()
        .find_map(|(name, value)| (*name == needle).then_some(*value))
}

fn format_required_role_props(props: &[&str]) -> String {
    let quoted = props
        .iter()
        .map(|prop| format!("\"{prop}\""))
        .collect::<Vec<_>>();

    match quoted.as_slice() {
        [] => String::new(),
        [single] => single.clone(),
        [left, right] => format!("{left} and {right}"),
        _ => {
            let last = quoted.last().cloned().unwrap_or_default();
            let leading = &quoted[..quoted.len() - 1];
            format!("{}, and {}", leading.join(", "), last)
        }
    }
}

fn format_handler_list(handlers: &[&str]) -> String {
    match handlers {
        [] => String::new(),
        [single] => (*single).to_string(),
        [left, right] => format!("{left} or {right}"),
        _ => {
            let last = handlers.last().copied().unwrap_or_default();
            let leading = &handlers[..handlers.len() - 1];
            format!("{} or {}", leading.join(", "), last)
        }
    }
}

fn collect_element_handlers(attrs: &[Attribute]) -> Vec<String> {
    attrs
        .iter()
        .filter_map(|attr| match attr {
            Attribute::ExpressionAttribute(attr) => attr.event_name.clone(),
            Attribute::OnDirectiveLegacy(attr) => Some(attr.name.clone()),
            _ => None,
        })
        .collect()
}

fn has_content(fragment: &Fragment, ctx: &VisitContext<'_>) -> bool {
    for &child_id in &fragment.nodes {
        let child = ctx.store.get(child_id);
        match child {
            Node::Text(text) if text.value(ctx.source).trim().is_empty() => continue,
            Node::Element(child_el) => {
                let popover_is_static = ctx
                    .data
                    .attribute(child_el.id, &child_el.attributes, "popover")
                    .and_then(|attr| static_attr_value(attr, ctx.source))
                    .is_some();
                if popover_is_static {
                    continue;
                }

                if child_el.name == "img" && ctx.data.has_attribute(child_el.id, "alt") {
                    return true;
                }

                if child_el.name == "selectedcontent" {
                    return true;
                }

                if !has_content(&child_el.fragment, ctx) {
                    continue;
                }
            }
            _ => {}
        }

        // Follow the reference compiler's conservative behavior and treat any
        // remaining child kind as content once it survives the special cases above.
        return true;
    }

    false
}

fn static_attr_value_text(value: Option<StaticAttributeValue<'_>>) -> Option<&str> {
    match value {
        Some(StaticAttributeValue::Text(text)) => Some(text),
        Some(StaticAttributeValue::True) | None => None,
    }
}

fn has_associated_control(fragment: &Fragment, ctx: &VisitContext<'_>) -> bool {
    fragment
        .nodes
        .iter()
        .copied()
        .any(|child_id| node_has_associated_control(ctx.store.get(child_id), ctx))
}

fn is_valid_autocomplete(autocomplete: Option<StaticAttributeValue<'_>>) -> bool {
    let Some(autocomplete) = autocomplete else {
        return true;
    };

    let StaticAttributeValue::Text(autocomplete) = autocomplete else {
        return false;
    };

    if autocomplete.is_empty() {
        return true;
    }

    let lower = autocomplete.trim().to_ascii_lowercase();
    if lower.is_empty() {
        return true;
    }

    let tokens = lower.split_whitespace().collect::<Vec<_>>();
    let mut index = 0;

    if tokens
        .get(index)
        .is_some_and(|token| token.starts_with("section-"))
    {
        index += 1;
    }
    if tokens
        .get(index)
        .is_some_and(|token| A11Y_AUTOCOMPLETE_ADDRESS_TYPE_TOKENS.contains(token))
    {
        index += 1;
    }
    if tokens
        .get(index)
        .is_some_and(|token| A11Y_AUTOCOMPLETE_FIELD_NAME_TOKENS.contains(token))
    {
        index += 1;
    } else {
        if tokens
            .get(index)
            .is_some_and(|token| A11Y_AUTOCOMPLETE_CONTACT_TYPE_TOKENS.contains(token))
        {
            index += 1;
        }
        if tokens
            .get(index)
            .is_some_and(|token| A11Y_AUTOCOMPLETE_CONTACT_FIELD_NAME_TOKENS.contains(token))
        {
            index += 1;
        } else {
            return false;
        }
    }
    if tokens.get(index).is_some_and(|token| *token == "webauthn") {
        index += 1;
    }

    index == tokens.len()
}

fn is_redundant_img_alt(alt: &str) -> bool {
    alt.split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
        .any(|token| {
            let lower = token.to_ascii_lowercase();
            A11Y_REDUNDANT_IMG_ALT_TOKENS.contains(&lower.as_str())
        })
}

fn video_has_caption_track(fragment: &Fragment, ctx: &VisitContext<'_>) -> bool {
    let Some(track) = fragment.nodes.iter().find_map(|&child_id| {
        ctx.store
            .get(child_id)
            .as_element()
            .filter(|child| child.name == "track")
    }) else {
        return false;
    };

    track.attributes.iter().any(|attr| match attr {
        Attribute::SpreadAttribute(_) => true,
        Attribute::StringAttribute(attr) => {
            attr.name == "kind" && attr.value_span.source_text(ctx.source) == "captions"
        }
        _ => false,
    })
}

fn visible_figure_children(fragment: &Fragment, ctx: &VisitContext<'_>) -> Vec<NodeId> {
    fragment
        .nodes
        .iter()
        .copied()
        .filter(|&child_id| match ctx.store.get(child_id) {
            Node::Comment(_) => false,
            Node::Text(text) => !text.value(ctx.source).trim().is_empty(),
            _ => true,
        })
        .collect()
}

fn node_has_associated_control(node: &Node, ctx: &VisitContext<'_>) -> bool {
    match node {
        Node::Element(el) => {
            if A11Y_LABELABLE_ELEMENTS.contains(&el.name.as_str()) {
                return true;
            }

            has_associated_control(&el.fragment, ctx)
        }
        Node::SlotElementLegacy(_) => true,
        // Match the reference analyzer's conservative behavior for dynamic descendants:
        // if the label contains a component-like control target, don't warn.
        Node::ComponentNode(_) | Node::RenderTag(_) | Node::SvelteElement(_) => true,
        Node::IfBlock(block) => {
            has_associated_control(&block.consequent, ctx)
                || block
                    .alternate
                    .as_ref()
                    .is_some_and(|fragment| has_associated_control(fragment, ctx))
        }
        Node::EachBlock(block) => {
            has_associated_control(&block.body, ctx)
                || block
                    .fallback
                    .as_ref()
                    .is_some_and(|fragment| has_associated_control(fragment, ctx))
        }
        Node::SnippetBlock(block) => has_associated_control(&block.body, ctx),
        Node::KeyBlock(block) => has_associated_control(&block.fragment, ctx),
        Node::SvelteHead(node) => has_associated_control(&node.fragment, ctx),
        Node::SvelteFragmentLegacy(node) => has_associated_control(&node.fragment, ctx),
        Node::SvelteWindow(node) => has_associated_control(&node.fragment, ctx),
        Node::SvelteDocument(node) => has_associated_control(&node.fragment, ctx),
        Node::SvelteBody(node) => has_associated_control(&node.fragment, ctx),
        Node::SvelteBoundary(node) => has_associated_control(&node.fragment, ctx),
        Node::AwaitBlock(block) => {
            block
                .pending
                .as_ref()
                .is_some_and(|fragment| has_associated_control(fragment, ctx))
                || block
                    .then
                    .as_ref()
                    .is_some_and(|fragment| has_associated_control(fragment, ctx))
                || block
                    .catch
                    .as_ref()
                    .is_some_and(|fragment| has_associated_control(fragment, ctx))
        }
        Node::Text(_)
        | Node::Comment(_)
        | Node::ExpressionTag(_)
        | Node::HtmlTag(_)
        | Node::ConstTag(_)
        | Node::DebugTag(_)
        | Node::Error(_) => false,
    }
}

fn is_hidden_from_screen_reader(el: &Element, attrs: &[Attribute], ctx: &VisitContext<'_>) -> bool {
    if el.name == "input" {
        if let Some(input_type) = ctx
            .data
            .attribute(el.id, attrs, "type")
            .and_then(|attr| static_text_attr_value(attr, ctx.source))
        {
            if input_type == "hidden" {
                return true;
            }
        }
    }

    ctx.data
        .attribute(el.id, attrs, "aria-hidden")
        .map_or(false, |attr| match attr {
            Attribute::BooleanAttribute(_) => true,
            _ => static_text_attr_value(attr, ctx.source).is_some_and(|value| value == "true"),
        })
}

fn has_disabled_attribute(id: NodeId, attrs: &[Attribute], ctx: &VisitContext<'_>) -> bool {
    if ctx
        .data
        .attribute(id, attrs, "disabled")
        .is_some_and(|attr| {
            matches!(attr, Attribute::BooleanAttribute(_))
                || static_text_attr_value(attr, ctx.source).is_some_and(|value| !value.is_empty())
        })
    {
        return true;
    }

    ctx.data
        .attribute(id, attrs, "aria-disabled")
        .and_then(|attr| static_text_attr_value(attr, ctx.source))
        .is_some_and(|value| value == "true")
}

fn has_javascript_prefix(value: &str) -> bool {
    value
        .trim_start_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
        .get(..11)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("javascript:"))
}

fn element_interactivity(
    el: &Element,
    attrs: &[Attribute],
    ctx: &VisitContext<'_>,
) -> ElementInteractivity {
    if let Some(role) = semantic_role_for_element(el, attrs, ctx) {
        return if is_interactive_role(Some(role)) {
            ElementInteractivity::Interactive
        } else {
            ElementInteractivity::NonInteractive
        };
    }

    match el.name.as_str() {
        "button" | "details" | "embed" | "iframe" | "label" | "select" | "textarea" => {
            ElementInteractivity::Interactive
        }
        "a" | "area" => {
            if ctx.data.has_attribute(el.id, "href") {
                ElementInteractivity::Interactive
            } else {
                ElementInteractivity::Static
            }
        }
        "audio" | "video" => {
            if ctx.data.has_attribute(el.id, "controls") {
                ElementInteractivity::Interactive
            } else {
                ElementInteractivity::Static
            }
        }
        "img" => {
            if ctx.data.has_attribute(el.id, "usemap") {
                ElementInteractivity::Interactive
            } else {
                ElementInteractivity::Static
            }
        }
        "input" => {
            let input_type = ctx
                .data
                .attribute(el.id, attrs, "type")
                .and_then(|attr| static_text_attr_value(attr, ctx.source));
            if input_type == Some("hidden") {
                ElementInteractivity::NonInteractive
            } else {
                ElementInteractivity::Interactive
            }
        }
        _ => ElementInteractivity::Static,
    }
}

fn is_noninteractive_element_to_interactive_role_exception(name: &str, role: &str) -> bool {
    super::A11Y_NON_INTERACTIVE_ELEMENT_TO_INTERACTIVE_ROLE_EXCEPTIONS
        .iter()
        .find_map(|(element, roles)| (*element == name).then_some(*roles))
        .is_some_and(|roles| roles.contains(&role))
}

fn is_interactive_role(role: Option<&str>) -> bool {
    role.is_some_and(|role| super::A11Y_INTERACTIVE_ROLES.contains(&role))
}

fn is_non_interactive_role(role: Option<&str>) -> bool {
    role.is_some_and(|role| super::A11Y_NON_INTERACTIVE_ROLES.contains(&role))
}

fn is_presentation_role(role: Option<&str>) -> bool {
    role.is_some_and(|role| super::A11Y_PRESENTATION_ROLES.contains(&role))
}

fn is_abstract_role(role: Option<&str>) -> bool {
    role.is_some_and(|role| super::A11Y_ABSTRACT_ROLES.contains(&role))
}

fn is_known_role_for_prop_support(role: &str) -> bool {
    super::A11Y_ARIA_ROLES.contains(&role)
}

fn role_supports_aria_prop(role: &str, attr: &str) -> bool {
    if matches!(role, "none" | "doc-pullquote") {
        return false;
    }

    if super::A11Y_GLOBAL_ROLE_SUPPORTED_PROPS.contains(&attr) {
        return true;
    }

    match role {
        "alertdialog" | "dialog" | "window" => matches!(attr, "aria-modal"),
        "application" | "graphics-object" => matches!(
            attr,
            "aria-activedescendant"
                | "aria-disabled"
                | "aria-errormessage"
                | "aria-expanded"
                | "aria-haspopup"
                | "aria-invalid"
        ),
        "article" => matches!(attr, "aria-posinset" | "aria-setsize"),
        "button" => matches!(
            attr,
            "aria-disabled" | "aria-expanded" | "aria-haspopup" | "aria-pressed"
        ),
        "cell" => matches!(
            attr,
            "aria-colindex" | "aria-colspan" | "aria-rowindex" | "aria-rowspan"
        ),
        "checkbox" | "switch" => matches!(
            attr,
            "aria-checked"
                | "aria-disabled"
                | "aria-errormessage"
                | "aria-expanded"
                | "aria-invalid"
                | "aria-readonly"
                | "aria-required"
        ),
        "columnheader" | "rowheader" => matches!(
            attr,
            "aria-colindex"
                | "aria-colspan"
                | "aria-disabled"
                | "aria-errormessage"
                | "aria-expanded"
                | "aria-invalid"
                | "aria-readonly"
                | "aria-required"
                | "aria-rowindex"
                | "aria-rowspan"
                | "aria-selected"
                | "aria-sort"
        ),
        "combobox" => matches!(
            attr,
            "aria-activedescendant"
                | "aria-autocomplete"
                | "aria-errormessage"
                | "aria-expanded"
                | "aria-invalid"
                | "aria-readonly"
                | "aria-required"
        ),
        "grid" | "treegrid" => matches!(
            attr,
            "aria-activedescendant"
                | "aria-colcount"
                | "aria-colindex"
                | "aria-colspan"
                | "aria-multiselectable"
                | "aria-readonly"
                | "aria-rowcount"
                | "aria-rowindex"
                | "aria-rowspan"
        ),
        "gridcell" => matches!(
            attr,
            "aria-colindex"
                | "aria-colspan"
                | "aria-disabled"
                | "aria-errormessage"
                | "aria-expanded"
                | "aria-invalid"
                | "aria-readonly"
                | "aria-required"
                | "aria-rowindex"
                | "aria-rowspan"
                | "aria-selected"
        ),
        "heading" => matches!(attr, "aria-level"),
        "img" => matches!(attr, "aria-errormessage" | "aria-invalid"),
        "link" => matches!(attr, "aria-disabled" | "aria-expanded" | "aria-haspopup"),
        "listbox" => matches!(
            attr,
            "aria-activedescendant"
                | "aria-errormessage"
                | "aria-invalid"
                | "aria-multiselectable"
                | "aria-readonly"
                | "aria-required"
        ),
        "menuitem" | "menuitemcheckbox" | "menuitemradio" => matches!(
            attr,
            "aria-disabled" | "aria-expanded" | "aria-haspopup" | "aria-posinset" | "aria-setsize"
        ),
        "meter" => matches!(
            attr,
            "aria-valuemax" | "aria-valuemin" | "aria-valuenow" | "aria-valuetext"
        ),
        "option" => matches!(
            attr,
            "aria-checked" | "aria-posinset" | "aria-setsize" | "aria-selected"
        ),
        "progressbar" => matches!(
            attr,
            "aria-valuemax" | "aria-valuemin" | "aria-valuenow" | "aria-valuetext"
        ),
        "radio" => matches!(
            attr,
            "aria-checked"
                | "aria-disabled"
                | "aria-errormessage"
                | "aria-expanded"
                | "aria-invalid"
                | "aria-posinset"
                | "aria-readonly"
                | "aria-required"
                | "aria-setsize"
        ),
        "radiogroup" => matches!(
            attr,
            "aria-errormessage" | "aria-invalid" | "aria-readonly" | "aria-required"
        ),
        "range" | "scrollbar" | "separator" | "slider" | "spinbutton" => matches!(
            attr,
            "aria-errormessage"
                | "aria-invalid"
                | "aria-readonly"
                | "aria-required"
                | "aria-valuemax"
                | "aria-valuemin"
                | "aria-valuenow"
                | "aria-valuetext"
        ),
        "row" => matches!(
            attr,
            "aria-colindex"
                | "aria-expanded"
                | "aria-level"
                | "aria-posinset"
                | "aria-rowindex"
                | "aria-selected"
                | "aria-setsize"
        ),
        "searchbox" | "textbox" => matches!(
            attr,
            "aria-activedescendant"
                | "aria-autocomplete"
                | "aria-errormessage"
                | "aria-invalid"
                | "aria-multiline"
                | "aria-placeholder"
                | "aria-readonly"
                | "aria-required"
        ),
        "select" | "tree" => matches!(
            attr,
            "aria-activedescendant"
                | "aria-errormessage"
                | "aria-invalid"
                | "aria-multiselectable"
                | "aria-required"
        ),
        "tab" => matches!(
            attr,
            "aria-disabled"
                | "aria-expanded"
                | "aria-haspopup"
                | "aria-posinset"
                | "aria-setsize"
                | "aria-selected"
        ),
        "table" => matches!(attr, "aria-colcount" | "aria-rowcount"),
        "tabpanel" => matches!(attr, "aria-expanded"),
        "treeitem" => matches!(
            attr,
            "aria-checked"
                | "aria-disabled"
                | "aria-errormessage"
                | "aria-expanded"
                | "aria-invalid"
                | "aria-level"
                | "aria-posinset"
                | "aria-required"
                | "aria-selected"
                | "aria-setsize"
        ),
        _ => false,
    }
}
