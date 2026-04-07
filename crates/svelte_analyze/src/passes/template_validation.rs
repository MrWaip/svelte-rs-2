//! Template structure validation — structural diagnostics that depend on the resolved
//! AST shape (key presence, animate placement, each-block assignments).
//!
//! Runs after `CollectSymbols` so that reference IDs are resolved.
//!
//! OXC expression spans are 0-based relative to the expression text snippet.
//! `current_expr_offset` tracks the source-absolute start of the current expression
//! so that sub-expression spans can be reported correctly.

use oxc_ast::ast::{AssignmentTarget, Expression, SimpleAssignmentTarget};
use oxc_ast_visit::{walk, Visit};
use oxc_span::GetSpan;
use svelte_ast::{
    is_svg, AnimateDirective, Attribute, AwaitBlock, BindDirective, ComponentNode, ConcatPart,
    ConstTag, DebugTag, EachBlock, Element, ExpressionAttribute, ExpressionTag, Fragment, IfBlock,
    KeyBlock, Node, NodeId, OnDirectiveLegacy, SnippetBlock, SvelteElement, Text,
};
use svelte_component_semantics::SymbolFlags;
use svelte_diagnostics::codes::fuzzymatch;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::passes::binding_properties::{binding_property, BINDING_NAMES};
use crate::scope::ComponentScoping;
use crate::types::data::{ExpressionKind, FragmentKey};
use crate::walker::{ParentKind, ParentRef, TemplateVisitor, VisitContext};
use crate::AnalysisData;

const EVENT_MODIFIERS: &[&str] = &[
    "preventDefault",
    "stopPropagation",
    "stopImmediatePropagation",
    "self",
    "trusted",
    "once",
    "capture",
    "passive",
    "nonpassive",
];

const A11Y_ARIA_ATTRIBUTES: &[&str] = &[
    "activedescendant",
    "atomic",
    "autocomplete",
    "busy",
    "checked",
    "colcount",
    "colindex",
    "colspan",
    "controls",
    "current",
    "describedby",
    "description",
    "details",
    "disabled",
    "dropeffect",
    "errormessage",
    "expanded",
    "flowto",
    "grabbed",
    "haspopup",
    "hidden",
    "invalid",
    "keyshortcuts",
    "label",
    "labelledby",
    "level",
    "live",
    "modal",
    "multiline",
    "multiselectable",
    "orientation",
    "owns",
    "placeholder",
    "posinset",
    "pressed",
    "readonly",
    "relevant",
    "required",
    "roledescription",
    "rowcount",
    "rowindex",
    "rowspan",
    "selected",
    "setsize",
    "sort",
    "valuemax",
    "valuemin",
    "valuenow",
    "valuetext",
];

const A11Y_INVISIBLE_ELEMENTS: &[&str] = &["meta", "html", "script", "style"];

const A11Y_ARIA_ROLES: &[&str] = &[
    "command",
    "composite",
    "input",
    "landmark",
    "range",
    "roletype",
    "section",
    "sectionhead",
    "select",
    "structure",
    "widget",
    "window",
    "alert",
    "alertdialog",
    "application",
    "article",
    "banner",
    "blockquote",
    "button",
    "caption",
    "cell",
    "checkbox",
    "code",
    "columnheader",
    "combobox",
    "complementary",
    "contentinfo",
    "definition",
    "deletion",
    "dialog",
    "directory",
    "document",
    "emphasis",
    "feed",
    "figure",
    "form",
    "generic",
    "grid",
    "gridcell",
    "group",
    "heading",
    "img",
    "insertion",
    "link",
    "list",
    "listbox",
    "listitem",
    "log",
    "main",
    "mark",
    "marquee",
    "math",
    "menu",
    "menubar",
    "menuitem",
    "menuitemcheckbox",
    "menuitemradio",
    "meter",
    "navigation",
    "none",
    "note",
    "option",
    "paragraph",
    "presentation",
    "progressbar",
    "radio",
    "radiogroup",
    "region",
    "row",
    "rowgroup",
    "rowheader",
    "scrollbar",
    "search",
    "searchbox",
    "separator",
    "slider",
    "spinbutton",
    "status",
    "strong",
    "subscript",
    "superscript",
    "switch",
    "tab",
    "table",
    "tablist",
    "tabpanel",
    "term",
    "textbox",
    "time",
    "timer",
    "toolbar",
    "tooltip",
    "tree",
    "treegrid",
    "treeitem",
    "doc-abstract",
    "doc-acknowledgments",
    "doc-afterword",
    "doc-appendix",
    "doc-backlink",
    "doc-biblioentry",
    "doc-bibliography",
    "doc-biblioref",
    "doc-chapter",
    "doc-colophon",
    "doc-conclusion",
    "doc-cover",
    "doc-credit",
    "doc-credits",
    "doc-dedication",
    "doc-endnote",
    "doc-endnotes",
    "doc-epigraph",
    "doc-epilogue",
    "doc-errata",
    "doc-example",
    "doc-footnote",
    "doc-foreword",
    "doc-glossary",
    "doc-glossref",
    "doc-index",
    "doc-introduction",
    "doc-noteref",
    "doc-notice",
    "doc-pagebreak",
    "doc-pagefooter",
    "doc-pageheader",
    "doc-pagelist",
    "doc-part",
    "doc-preface",
    "doc-prologue",
    "doc-pullquote",
    "doc-qna",
    "doc-subtitle",
    "doc-tip",
    "doc-toc",
    "graphics-document",
    "graphics-object",
    "graphics-symbol",
];

const A11Y_ABSTRACT_ROLES: &[&str] = &[
    "command",
    "composite",
    "input",
    "landmark",
    "range",
    "roletype",
    "section",
    "sectionhead",
    "select",
    "structure",
    "widget",
    "window",
];

const A11Y_IMPLICIT_ROLES: &[(&str, &str)] = &[
    ("a", "link"),
    ("area", "link"),
    ("article", "article"),
    ("aside", "complementary"),
    ("body", "document"),
    ("button", "button"),
    ("datalist", "listbox"),
    ("dd", "definition"),
    ("dfn", "term"),
    ("dialog", "dialog"),
    ("details", "group"),
    ("dt", "term"),
    ("fieldset", "group"),
    ("figure", "figure"),
    ("form", "form"),
    ("h1", "heading"),
    ("h2", "heading"),
    ("h3", "heading"),
    ("h4", "heading"),
    ("h5", "heading"),
    ("h6", "heading"),
    ("hr", "separator"),
    ("img", "img"),
    ("li", "listitem"),
    ("link", "link"),
    ("main", "main"),
    ("menu", "list"),
    ("meter", "progressbar"),
    ("nav", "navigation"),
    ("ol", "list"),
    ("option", "option"),
    ("optgroup", "group"),
    ("output", "status"),
    ("progress", "progressbar"),
    ("section", "region"),
    ("summary", "button"),
    ("table", "table"),
    ("tbody", "rowgroup"),
    ("textarea", "textbox"),
    ("tfoot", "rowgroup"),
    ("thead", "rowgroup"),
    ("tr", "row"),
    ("ul", "list"),
];

const A11Y_NESTED_IMPLICIT_ROLES: &[(&str, &str)] =
    &[("header", "banner"), ("footer", "contentinfo")];

const A11Y_INPUT_IMPLICIT_ROLES: &[(&str, &str)] = &[
    ("button", "button"),
    ("image", "button"),
    ("reset", "button"),
    ("submit", "button"),
    ("checkbox", "checkbox"),
    ("radio", "radio"),
    ("range", "slider"),
    ("number", "spinbutton"),
    ("email", "textbox"),
    ("search", "searchbox"),
    ("tel", "textbox"),
    ("text", "textbox"),
    ("url", "textbox"),
];

const A11Y_COMBOBOX_INPUT_TYPES: &[&str] = &["email", "search", "tel", "text", "url"];

const A11Y_MENUITEM_IMPLICIT_ROLES: &[(&str, &str)] = &[
    ("command", "menuitem"),
    ("checkbox", "menuitemcheckbox"),
    ("radio", "menuitemradio"),
];

const A11Y_REQUIRED_ROLE_PROPS: &[(&str, &[&str])] = &[
    ("checkbox", &["aria-checked"]),
    ("combobox", &["aria-controls", "aria-expanded"]),
    ("heading", &["aria-level"]),
    ("menuitemcheckbox", &["aria-checked"]),
    ("menuitemradio", &["aria-checked"]),
    ("meter", &["aria-valuenow"]),
    ("option", &["aria-selected"]),
    ("radio", &["aria-checked"]),
    ("scrollbar", &["aria-controls", "aria-valuenow"]),
    ("slider", &["aria-valuenow"]),
    ("switch", &["aria-checked"]),
    ("treeitem", &["aria-selected"]),
];

const A11Y_INTERACTIVE_ROLES: &[&str] = &[
    "alertdialog",
    "button",
    "cell",
    "checkbox",
    "columnheader",
    "combobox",
    "dialog",
    "grid",
    "gridcell",
    "link",
    "listbox",
    "menu",
    "menubar",
    "menuitem",
    "menuitemcheckbox",
    "menuitemradio",
    "option",
    "radio",
    "radiogroup",
    "row",
    "rowheader",
    "scrollbar",
    "searchbox",
    "slider",
    "spinbutton",
    "switch",
    "tab",
    "tablist",
    "tabpanel",
    "textbox",
    "toolbar",
    "tree",
    "treegrid",
    "treeitem",
    "doc-backlink",
    "doc-biblioref",
    "doc-glossref",
    "doc-noteref",
];

const A11Y_PRESENTATION_ROLES: &[&str] = &["presentation", "none"];

const A11Y_INTERACTIVE_HANDLERS: &[&str] = &[
    "keypress",
    "keydown",
    "keyup",
    "click",
    "contextmenu",
    "dblclick",
    "drag",
    "dragend",
    "dragenter",
    "dragexit",
    "dragleave",
    "dragover",
    "dragstart",
    "drop",
    "mousedown",
    "mouseenter",
    "mouseleave",
    "mousemove",
    "mouseout",
    "mouseover",
    "mouseup",
    "pointerdown",
    "pointerup",
    "pointermove",
    "pointerenter",
    "pointerleave",
    "pointerover",
    "pointerout",
    "pointercancel",
    "touchstart",
    "touchend",
    "touchmove",
    "touchcancel",
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum ElementInteractivity {
    Interactive,
    NonInteractive,
    Static,
}

const A11Y_GLOBAL_ROLE_SUPPORTED_PROPS: &[&str] = &[
    "aria-atomic",
    "aria-busy",
    "aria-controls",
    "aria-current",
    "aria-describedby",
    "aria-details",
    "aria-dropeffect",
    "aria-flowto",
    "aria-grabbed",
    "aria-hidden",
    "aria-keyshortcuts",
    "aria-label",
    "aria-labelledby",
    "aria-live",
    "aria-owns",
    "aria-relevant",
    "aria-roledescription",
];

struct BindParentInfo {
    id: svelte_ast::NodeId,
    name: String,
    attrs: Vec<Attribute>,
}

enum BindExpressionShape {
    IdentifierOrMember,
    Sequence { len: usize, has_parens: bool },
    Invalid,
}

/// Per-element state for detecting mixed event syntax (S5 attributes + legacy on:).
#[derive(Default)]
struct ElementEventState {
    has_s5_events: bool,
    /// Span and event name of the first `on:` directive seen on this element.
    first_on_directive: Option<(Span, String)>,
}

pub(crate) struct TemplateValidationVisitor {
    /// Source-absolute start of the current expression being visited.
    /// Set by `visit_expression`, used in `visit_js_expression` to offset OXC sub-spans.
    current_expr_offset: u32,
    /// Stack of per-element event state, pushed on `visit_element` and popped on `leave_element`.
    element_event_state: Vec<ElementEventState>,
    /// Nesting depth of `<dialog>` elements; used to suppress `a11y_autofocus` inside dialogs.
    dialog_depth: u32,
}

impl TemplateValidationVisitor {
    pub(crate) fn new() -> Self {
        Self {
            current_expr_offset: 0,
            element_event_state: Vec::new(),
            dialog_depth: 0,
        }
    }

    fn oxc_to_svelte(&self, span: oxc_span::Span) -> Span {
        Span::new(
            self.current_expr_offset + span.start,
            self.current_expr_offset + span.end,
        )
    }

    fn emit_mixed_syntax_if_needed(&mut self, ctx: &mut VisitContext<'_>) {
        if let Some(state) = self.element_event_state.pop() {
            if state.has_s5_events {
                if let Some((span, name)) = state.first_on_directive {
                    ctx.warnings_mut().push(Diagnostic::error(
                        DiagnosticKind::MixedEventHandlerSyntaxes { name },
                        span,
                    ));
                }
            }
        }
    }

    fn maybe_warn_legacy_special_element(
        &mut self,
        name: &str,
        span: Span,
        ctx: &mut VisitContext<'_>,
    ) {
        if !ctx.runes {
            return;
        }

        match name {
            "svelte:component" => ctx.warnings_mut().push(Diagnostic::warning(
                DiagnosticKind::SvelteComponentDeprecated,
                span,
            )),
            "svelte:self" => {
                let name = ctx.component_name().to_string();
                let basename = ctx.filename_basename().to_string();
                ctx.warnings_mut().push(Diagnostic::warning(
                    DiagnosticKind::SvelteSelfDeprecated { name, basename },
                    span,
                ));
            }
            _ => {}
        }
    }
}

impl TemplateVisitor for TemplateValidationVisitor {
    fn visit_snippet_block(&mut self, block: &SnippetBlock, ctx: &mut VisitContext<'_>) {
        validate_snippet_rest_params(block, ctx);
        validate_snippet_shadowing_prop(block, ctx);
        validate_snippet_children_conflict(block, ctx);
    }

    fn visit_const_tag(&mut self, tag: &ConstTag, ctx: &mut VisitContext<'_>) {
        // const_tag_invalid_expression: an unparenthesized sequence expression in the init
        // (e.g. `{@const a = b, c = d}`) produces two declarators when OXC parses the
        // wrapped `const a = b, c = d;` form. Parenthesised sequences are fine.
        if let Some(parsed) = ctx.parsed() {
            if let Some(handle) = parsed.stmt_handle(tag.expression_span.start) {
                if let Some(oxc_ast::ast::Statement::VariableDeclaration(decl)) =
                    parsed.stmt(handle)
                {
                    if decl.declarations.len() > 1 {
                        ctx.warnings_mut().push(Diagnostic::error(
                            DiagnosticKind::ConstTagInvalidExpression,
                            tag.span,
                        ));
                    }
                }
            }
        }

        // const_tag_invalid_placement: {@const} must be a direct child of an allowed block.
        // TemplateTopology preserves the same direct-parent relationship the walker stack exposed.
        let is_valid_parent = ctx.data.parent(tag.id).is_some_and(|p| {
            matches!(
                p.kind,
                ParentKind::IfBlock
                    | ParentKind::EachBlock
                    | ParentKind::SnippetBlock
                    | ParentKind::ComponentNode
                    | ParentKind::AwaitBlock
                    | ParentKind::SvelteBoundary
                    | ParentKind::KeyBlock
            ) || element_has_slot_attr(p, ctx)
        });

        if !is_valid_parent {
            ctx.warnings_mut().push(Diagnostic::error(
                DiagnosticKind::ConstTagInvalidPlacement,
                tag.span,
            ));
        }
    }

    fn visit_element(&mut self, el: &Element, ctx: &mut VisitContext<'_>) {
        self.element_event_state.push(ElementEventState::default());
        self.maybe_warn_legacy_special_element(&el.name, el.span, ctx);

        // Track dialog nesting for a11y_autofocus suppression.
        if el.name == "dialog" {
            self.dialog_depth += 1;
        }

        // Copy source before borrowing ctx.data so the &str is available after
        // the idx borrow is released (needed by check_a11y_missing_attribute).
        let source = ctx.source;

        // Pre-compute all attribute-based flags and refs inside a block so that
        // the ctx.data borrow through `idx` is released before ctx.warnings_mut().
        // &Attribute results borrow `el`, not `ctx`, so they outlive the block.
        let (
            has_slot,
            has_spread,
            accesskey_attr,
            tabindex_attr,
            has_autofocus,
            missing_attr_diag,
            has_value_attr,
            slot_attr,
        ) = {
            (
                ctx.data.has_attribute(el.id, "slot"),
                ctx.data.has_spread(el.id),
                ctx.data.attribute(el.id, &el.attributes, "accesskey"),
                ctx.data.attribute(el.id, &el.attributes, "tabindex"),
                ctx.data.has_attribute(el.id, "autofocus"),
                if !ctx.data.has_spread(el.id) {
                    check_a11y_missing_attribute(el, ctx.data, source)
                } else {
                    None
                },
                ctx.data.has_attribute(el.id, "value"),
                ctx.data.attribute(el.id, &el.attributes, "slot"),
            )
        };

        // textarea_invalid_content: <textarea> may not have both a value attribute and children.
        if el.name == "textarea"
            && has_value_attr
            && ctx.data.fragment_has_children(&FragmentKey::Element(el.id))
        {
            ctx.warnings_mut().push(Diagnostic::error(
                DiagnosticKind::TextareaInvalidContent,
                el.span,
            ));
        }

        // slot_attribute_invalid_placement / slot_attribute_invalid:
        // - placement error when the element is NOT a direct child of a component.
        // - value error when it IS a direct child but slot value is not a plain string.
        if has_slot
            && !ctx
                .data
                .parent(el.id)
                .is_some_and(|p| p.kind == ParentKind::ComponentNode)
        {
            ctx.warnings_mut().push(Diagnostic::error(
                DiagnosticKind::SlotAttributeInvalidPlacement,
                el.span,
            ));
        } else if has_slot {
            if let Some(attr) = slot_attr {
                if !matches!(attr, Attribute::StringAttribute(_)) {
                    ctx.warnings_mut().push(Diagnostic::error(
                        DiagnosticKind::SlotAttributeInvalid,
                        attr_value_span(attr),
                    ));
                }
            }
        }

        // a11y_distracting_elements: <marquee> and <blink> are harmful to accessibility.
        if matches!(el.name.as_str(), "marquee" | "blink") {
            ctx.warnings_mut().push(Diagnostic::warning(
                DiagnosticKind::A11yDistractingElements {
                    name: el.name.clone(),
                },
                el.span,
            ));
        }

        // Attribute-level A11y checks.
        if let Some(attr) = accesskey_attr {
            ctx.warnings_mut().push(Diagnostic::warning(
                DiagnosticKind::A11yAccesskey,
                attr_value_span(attr),
            ));
        }
        if let Some(attr) = tabindex_attr {
            if let Some(text) = static_text_attr_value(attr, source) {
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
        if has_autofocus && el.name != "dialog" && self.dialog_depth == 0 {
            ctx.warnings_mut()
                .push(Diagnostic::warning(DiagnosticKind::A11yAutofocus, el.span));
        }

        // a11y_missing_attribute: certain elements require specific attributes.
        if let Some(diag) = missing_attr_diag {
            ctx.warnings_mut().push(diag);
        }

        check_a11y_aria_attribute_warnings(el, &el.attributes, ctx);
        check_a11y_role_warnings(el, &el.attributes, ctx);
        check_a11y_role_supported_aria_props_warnings(el, &el.attributes, ctx);
        check_a11y_role_attribute_interaction_warnings(el, &el.attributes, ctx);

        // slot_element_deprecated: <slot> is deprecated in runes mode; use {@render} instead.
        if el.name == "slot" && ctx.runes && !ctx.data.custom_element {
            ctx.warnings_mut().push(Diagnostic::warning(
                DiagnosticKind::SlotElementDeprecated,
                el.span,
            ));
        }

        check_component_name_lowercase(el, ctx);
        check_plain_attr_warnings(el.id, el.span, &el.attributes, ctx);

        // attribute_quoted: custom elements (names containing '-') get the same warning
        // as component attributes for quoted single-expression attrs in runes mode.
        if el.name.contains('-') {
            check_attribute_quoted(&el.attributes, ctx);
        }

        let _ = has_spread; // used only in pre-computation above
    }

    fn leave_element(&mut self, el: &Element, ctx: &mut VisitContext<'_>) {
        if el.name == "dialog" {
            self.dialog_depth -= 1;
        }
        self.emit_mixed_syntax_if_needed(ctx);
    }

    fn visit_component_node(&mut self, cn: &ComponentNode, ctx: &mut VisitContext<'_>) {
        self.maybe_warn_legacy_special_element(&cn.name, cn.span, ctx);
        check_attribute_quoted(&cn.attributes, ctx);
    }

    fn visit_svelte_element(&mut self, el: &SvelteElement, ctx: &mut VisitContext<'_>) {
        self.element_event_state.push(ElementEventState::default());
        check_plain_attr_warnings(el.id, el.span, &el.attributes, ctx);
    }

    fn leave_svelte_element(&mut self, _el: &SvelteElement, ctx: &mut VisitContext<'_>) {
        self.emit_mixed_syntax_if_needed(ctx);
    }

    fn visit_expression_attribute(
        &mut self,
        attr: &ExpressionAttribute,
        ctx: &mut VisitContext<'_>,
    ) {
        if attr.event_name.is_some() {
            if let Some(expr) = ctx
                .parsed()
                .and_then(|parsed| parsed.expr_handle(attr.expression_span.start))
                .and_then(|handle| ctx.parsed().and_then(|parsed| parsed.expr(handle)))
            {
                if let Expression::Identifier(ident) = expr {
                    if ident.name.as_str() == attr.name.as_str()
                        && ctx
                            .data
                            .scoping
                            .find_binding(ctx.scope, ident.name.as_str())
                            .is_none()
                    {
                        ctx.warnings_mut().push(Diagnostic::warning(
                            DiagnosticKind::AttributeGlobalEventReference {
                                name: attr.name.clone(),
                            },
                            attr.expression_span,
                        ));
                    }
                }
            }
        }

        if attr.event_name.is_some() {
            if let Some(state) = self.element_event_state.last_mut() {
                state.has_s5_events = true;
            }
        }
    }

    fn visit_bind_directive(&mut self, dir: &BindDirective, ctx: &mut VisitContext<'_>) {
        if let Some(parent) = current_bind_parent(dir.id, ctx) {
            validate_bind_name_and_target(dir, &parent, ctx);
            validate_bind_parent_specifics(dir, &parent, ctx);
        }

        let is_identifier_target = if dir.shorthand {
            true
        } else {
            ctx.data
                .attr_expression(dir.id)
                .is_some_and(|info| matches!(info.kind, ExpressionKind::Identifier(_)))
        };

        let Some(expr_shape) = bind_expression_shape(dir, ctx) else {
            if is_identifier_target {
                validate_bind_identifier_value(dir, ctx);
            }
            validate_bind_group_binding(dir, ctx);
            return;
        };

        match expr_shape {
            BindExpressionShape::Sequence { len, has_parens } => {
                validate_bind_sequence_expression(dir, len, has_parens, ctx);
                return;
            }
            BindExpressionShape::Invalid => {
                emit_bind_error(
                    ctx,
                    dir.expression_span,
                    DiagnosticKind::BindInvalidExpression,
                );
                return;
            }
            BindExpressionShape::IdentifierOrMember => {}
        }

        if is_identifier_target {
            validate_bind_identifier_value(dir, ctx);
        }

        validate_bind_group_binding(dir, ctx);
    }

    // Use cases: event_handler_invalid_modifier, event_handler_invalid_modifier_combination,
    // event_directive_deprecated, mixed_event_handler_syntaxes
    fn visit_on_directive_legacy(&mut self, dir: &OnDirectiveLegacy, ctx: &mut VisitContext<'_>) {
        let is_component = ctx
            .data
            .parent(dir.id)
            .is_some_and(|p| p.kind == ParentKind::ComponentNode);

        if !is_component {
            // Invalid modifier check
            let list = EVENT_MODIFIERS.join(", ");
            for modifier in &dir.modifiers {
                if !EVENT_MODIFIERS.contains(&modifier.as_str()) {
                    ctx.warnings_mut().push(Diagnostic::error(
                        DiagnosticKind::EventHandlerInvalidModifier { list: list.clone() },
                        dir.name_span,
                    ));
                }
            }

            // passive + nonpassive conflict
            let has_passive = dir.modifiers.iter().any(|m| m == "passive");
            let has_nonpassive = dir.modifiers.iter().any(|m| m == "nonpassive");
            if has_passive && has_nonpassive {
                ctx.warnings_mut().push(Diagnostic::error(
                    DiagnosticKind::EventHandlerInvalidModifierCombination {
                        modifier1: "passive".to_string(),
                        modifier2: "nonpassive".to_string(),
                    },
                    dir.name_span,
                ));
            }
        }

        // Runes-mode deprecation — all non-component on: directives
        if ctx.runes && !is_component {
            ctx.warnings_mut().push(Diagnostic::warning(
                DiagnosticKind::EventDirectiveDeprecated {
                    name: dir.name.clone(),
                },
                dir.name_span,
            ));
        }

        // Record first on: directive for mixed-syntax check (DOM elements only)
        if !is_component {
            if let Some(state) = self.element_event_state.last_mut() {
                state
                    .first_on_directive
                    .get_or_insert((dir.name_span, dir.name.clone()));
            }
        }
    }

    fn visit_text(&mut self, text: &Text, ctx: &mut VisitContext<'_>) {
        let value = text.value(ctx.source);

        if contains_non_whitespace_text(value) {
            if let Some(message) = invalid_text_parent_message(text.id, ctx) {
                ctx.warnings_mut().push(Diagnostic::error(
                    DiagnosticKind::NodeInvalidPlacement { message },
                    text.span,
                ));
            }
        }

        for (offset, ch) in value.char_indices() {
            if !is_bidi_control(ch) {
                continue;
            }
            if ctx
                .data
                .ignore_data
                .is_ignored(text.id, "bidirectional_control_characters")
            {
                break;
            }
            let start = text.span.start + offset as u32;
            ctx.warnings_mut().push(Diagnostic::warning(
                DiagnosticKind::BidirectionalControlCharacters,
                Span::new(start, start + ch.len_utf8() as u32),
            ));
        }
    }

    fn visit_expression_tag(&mut self, tag: &ExpressionTag, ctx: &mut VisitContext<'_>) {
        if let Some(message) = invalid_text_parent_message(tag.id, ctx) {
            ctx.warnings_mut().push(Diagnostic::error(
                DiagnosticKind::NodeInvalidPlacement { message },
                tag.span,
            ));
        }
    }

    // Use case: block_unexpected_character — runes mode only, char after `{` must be `@`
    fn visit_debug_tag(&mut self, tag: &DebugTag, ctx: &mut VisitContext<'_>) {
        if ctx.runes {
            let start = tag.span.start as usize;
            if ctx.source.as_bytes().get(start + 1) != Some(&b'@') {
                ctx.warnings_mut().push(Diagnostic::error(
                    DiagnosticKind::BlockUnexpectedCharacter {
                        character: "@".to_string(),
                    },
                    Span::new(tag.span.start, tag.span.start + 5),
                ));
            }
        }
    }

    // Use cases: block_empty, block_unexpected_character
    fn visit_key_block(&mut self, block: &KeyBlock, ctx: &mut VisitContext<'_>) {
        check_empty_fragment(&block.fragment, FragmentKey::KeyBlockBody(block.id), ctx);

        // block_unexpected_character: runes mode only — char after `{` must be `#`
        if ctx.runes {
            let start = block.span.start as usize;
            if ctx.source.as_bytes().get(start + 1) != Some(&b'#') {
                ctx.warnings_mut().push(Diagnostic::error(
                    DiagnosticKind::BlockUnexpectedCharacter {
                        character: "#".to_string(),
                    },
                    Span::new(block.span.start, block.span.start + 5),
                ));
            }
        }
    }

    // Use cases: block_empty (consequent + alternate), block_unexpected_character
    fn visit_if_block(&mut self, block: &IfBlock, ctx: &mut VisitContext<'_>) {
        check_empty_fragment(&block.consequent, FragmentKey::IfConsequent(block.id), ctx);
        if let Some(alt) = &block.alternate {
            check_empty_fragment(alt, FragmentKey::IfAlternate(block.id), ctx);
        }

        // block_unexpected_character: runes mode only — `{#if` needs `#`, `{:else if` needs `:`
        if ctx.runes {
            let expected: u8 = if block.elseif { b':' } else { b'#' };
            let start = block.span.start as usize;
            if ctx.source.as_bytes().get(start + 1) != Some(&expected) {
                ctx.warnings_mut().push(Diagnostic::error(
                    DiagnosticKind::BlockUnexpectedCharacter {
                        character: (expected as char).to_string(),
                    },
                    Span::new(block.span.start, block.span.start + 5),
                ));
            }
        }
    }

    // Use case: block_unexpected_character for {:then val} / {:catch err} with whitespace before `:`
    fn visit_await_block(&mut self, block: &AwaitBlock, ctx: &mut VisitContext<'_>) {
        if !ctx.runes {
            return;
        }
        for (span_opt, clause) in [(block.value_span, ":then"), (block.error_span, ":catch")] {
            if let Some(span) = span_opt {
                let start = span.start as usize;
                let win_start = start.saturating_sub(10);
                let window = &ctx.source[win_start..start];
                if has_whitespace_before_clause(window, clause) {
                    ctx.warnings_mut().push(Diagnostic::error(
                        DiagnosticKind::BlockUnexpectedCharacter {
                            character: ":".to_string(),
                        },
                        Span::new(win_start as u32, start as u32),
                    ));
                }
            }
        }
    }

    // Use case 2: each_key_without_as
    fn visit_each_block(&mut self, block: &EachBlock, ctx: &mut VisitContext<'_>) {
        if block.key_span.is_some() && block.context_span.is_none() {
            ctx.warnings_mut().push(Diagnostic::error(
                DiagnosticKind::EachKeyWithoutAs,
                block.key_span.unwrap(),
            ));
        }
    }

    // Use cases 3 & 4: animation_missing_key, animation_invalid_placement
    fn visit_animate_directive(&mut self, dir: &AnimateDirective, ctx: &mut VisitContext<'_>) {
        // Collect grandparent info in a block so the ancestors iterator is dropped
        // before we borrow ctx.store or ctx.warnings_mut().
        let (parent, grandparent) = {
            let parent = ctx.data.parent(dir.id);
            let mut ancestors = ctx.data.ancestors(dir.id);
            ancestors.next(); // skip Element (direct attr parent)
            (parent, ancestors.next())
        };

        let diag_kind = match grandparent.map(|p| p.kind) {
            Some(ParentKind::EachBlock) => {
                // Scope the store borrow so it ends before warnings_mut() is called.
                let diag = {
                    let each_id = grandparent.unwrap().id;
                    let each_block = ctx.store.get(each_id).as_each_block().unwrap();
                    if each_block.key_span.is_none() {
                        Some(DiagnosticKind::AnimationMissingKey)
                    } else {
                        let only_child = ctx
                            .data
                            .fragment_single_non_trivial_child(&FragmentKey::EachBody(each_id));
                        if only_child != parent.map(|p| p.id) {
                            Some(DiagnosticKind::AnimationInvalidPlacement)
                        } else {
                            None
                        }
                    }
                };
                diag
            }
            _ => Some(DiagnosticKind::AnimationInvalidPlacement),
        };

        if let Some(kind) = diag_kind {
            ctx.warnings_mut().push(Diagnostic::error(kind, dir.name));
        }
    }

    // Track current expression offset for sub-span conversion
    fn visit_expression(&mut self, _id: NodeId, span: Span, _ctx: &mut VisitContext<'_>) {
        self.current_expr_offset = span.start;
    }

    // Use case 5: each_item_invalid_assignment (runes mode only)
    fn visit_js_expression(
        &mut self,
        _id: NodeId,
        expr: &Expression<'_>,
        ctx: &mut VisitContext<'_>,
    ) {
        if !ctx.runes {
            return;
        }
        let is_bind = ctx
            .parent()
            .is_some_and(|p| p.kind == ParentKind::BindDirective);

        match expr {
            Expression::Identifier(ident)
                if is_bind && is_snippet_param_ref(ident, &ctx.data.scoping) =>
            {
                let span = self.oxc_to_svelte(expr.span());
                ctx.warnings_mut().push(Diagnostic::error(
                    DiagnosticKind::SnippetParameterAssignment,
                    span,
                ));
            }
            Expression::Identifier(ident)
                if is_bind && is_each_block_var_ref(ident, &ctx.data.scoping) =>
            {
                let span = self.oxc_to_svelte(expr.span());
                ctx.warnings_mut().push(Diagnostic::error(
                    DiagnosticKind::EachItemInvalidAssignment,
                    span,
                ));
            }
            _ if !is_bind && contains_invalid_each_assignment(expr, &ctx.data.scoping) => {
                let span = self.oxc_to_svelte(expr.span());
                ctx.warnings_mut().push(Diagnostic::error(
                    DiagnosticKind::EachItemInvalidAssignment,
                    span,
                ));
            }
            _ if !is_bind && contains_invalid_snippet_param_assignment(expr, &ctx.data.scoping) => {
                let span = self.oxc_to_svelte(expr.span());
                ctx.warnings_mut().push(Diagnostic::error(
                    DiagnosticKind::SnippetParameterAssignment,
                    span,
                ));
            }
            _ => {}
        }
    }
}

fn current_bind_parent(bind_id: NodeId, ctx: &VisitContext<'_>) -> Option<BindParentInfo> {
    let parent = ctx.data.parent(bind_id)?;
    match ctx.store.get(parent.id) {
        Node::Element(el) => Some(BindParentInfo {
            id: el.id,
            name: el.name.clone(),
            attrs: el.attributes.clone(),
        }),
        Node::SvelteElement(el) => Some(BindParentInfo {
            id: el.id,
            name: "svelte:element".to_string(),
            attrs: el.attributes.clone(),
        }),
        Node::SvelteWindow(el) => Some(BindParentInfo {
            id: el.id,
            name: "svelte:window".to_string(),
            attrs: el.attributes.clone(),
        }),
        Node::SvelteDocument(el) => Some(BindParentInfo {
            id: el.id,
            name: "svelte:document".to_string(),
            attrs: el.attributes.clone(),
        }),
        Node::SvelteBody(el) => Some(BindParentInfo {
            id: el.id,
            name: "svelte:body".to_string(),
            attrs: el.attributes.clone(),
        }),
        _ => None,
    }
}

fn bind_expression_shape(
    dir: &BindDirective,
    ctx: &VisitContext<'_>,
) -> Option<BindExpressionShape> {
    bind_expression(dir, ctx).map(classify_bind_expression)
}

fn classify_bind_expression(expr: &Expression<'_>) -> BindExpressionShape {
    match expr {
        Expression::Identifier(_) => BindExpressionShape::IdentifierOrMember,
        Expression::SequenceExpression(sequence) => BindExpressionShape::Sequence {
            len: sequence.expressions.len(),
            has_parens: false,
        },
        Expression::ParenthesizedExpression(expr) => match &expr.expression {
            Expression::SequenceExpression(sequence) => BindExpressionShape::Sequence {
                len: sequence.expressions.len(),
                has_parens: true,
            },
            inner => classify_bind_expression(inner),
        },
        _ if expr.as_member_expression().is_some() => BindExpressionShape::IdentifierOrMember,
        _ => BindExpressionShape::Invalid,
    }
}

fn emit_bind_error(ctx: &mut VisitContext<'_>, span: Option<Span>, kind: DiagnosticKind) {
    ctx.warnings_mut()
        .push(Diagnostic::error(kind, span.unwrap_or(Span::new(0, 0))));
}

fn validate_bind_name_and_target(
    dir: &BindDirective,
    parent: &BindParentInfo,
    ctx: &mut VisitContext<'_>,
) {
    let Some(property) = binding_property(dir.name.as_str()) else {
        let explanation = fuzzymatch(dir.name.as_str(), BINDING_NAMES).and_then(|suggestion| {
            binding_property(suggestion)
                .is_some_and(|property| property.allows(&parent.name))
                .then(|| format!("Did you mean '{suggestion}'?"))
        });

        emit_bind_error(
            ctx,
            dir.expression_span,
            DiagnosticKind::BindInvalidName {
                name: dir.name.clone(),
                explanation,
            },
        );
        return;
    };

    if !property.valid_elements.is_empty()
        && !property.valid_elements.contains(&parent.name.as_str())
    {
        let elements = property
            .valid_elements
            .iter()
            .map(|name| format!("`<{name}>`"))
            .collect::<Vec<_>>()
            .join(", ");
        emit_bind_error(
            ctx,
            dir.expression_span,
            DiagnosticKind::BindInvalidTarget {
                name: dir.name.clone(),
                elements,
            },
        );
        return;
    }

    if property.invalid_elements.contains(&parent.name.as_str()) {
        let mut valid_bindings = BINDING_NAMES
            .iter()
            .copied()
            .filter(|candidate| {
                binding_property(candidate).is_some_and(|property| property.allows(&parent.name))
            })
            .collect::<Vec<_>>();
        valid_bindings.sort_unstable();

        emit_bind_error(
            ctx,
            dir.expression_span,
            DiagnosticKind::BindInvalidName {
                name: dir.name.clone(),
                explanation: Some(format!(
                    "Possible bindings for <{}> are {}",
                    parent.name,
                    valid_bindings.join(", ")
                )),
            },
        );
    }
}

fn validate_bind_parent_specifics(
    dir: &BindDirective,
    parent: &BindParentInfo,
    ctx: &mut VisitContext<'_>,
) {
    if parent.name == "input" && dir.name != "this" {
        validate_input_bindings(dir, parent, ctx);
    }

    if parent.name == "select" && dir.name != "this" {
        let multiple = ctx.data.attribute(parent.id, &parent.attrs, "multiple");
        if let Some(a) = multiple {
            if !attr_is_text(a) && !matches!(a, Attribute::BooleanAttribute(_)) {
                ctx.warnings_mut().push(Diagnostic::error(
                    DiagnosticKind::AttributeInvalidMultiple,
                    attr_value_span(a),
                ));
                return;
            }
        }
    }

    if dir.name == "offsetWidth" && is_svg(&parent.name) {
        emit_bind_error(
            ctx,
            dir.expression_span,
            DiagnosticKind::BindInvalidTarget {
                name: dir.name.clone(),
                elements: "non-`<svg>` elements. Use `bind:clientWidth` for `<svg>` instead"
                    .to_string(),
            },
        );
        return;
    }

    if matches!(dir.name.as_str(), "innerHTML" | "innerText" | "textContent") {
        let contenteditable = ctx
            .data
            .attribute(parent.id, &parent.attrs, "contenteditable");
        match contenteditable {
            None => emit_bind_error(
                ctx,
                dir.expression_span,
                DiagnosticKind::AttributeContenteditableMissing,
            ),
            Some(attr)
                if !attr_is_text(attr) && !matches!(attr, Attribute::BooleanAttribute(_)) =>
            {
                ctx.warnings_mut().push(Diagnostic::error(
                    DiagnosticKind::AttributeContenteditableDynamic,
                    attr_value_span(attr),
                ));
            }
            _ => {}
        }
    }
}

fn validate_input_bindings(
    dir: &BindDirective,
    parent: &BindParentInfo,
    ctx: &mut VisitContext<'_>,
) {
    let Some(type_attr) = ctx.data.attribute(parent.id, &parent.attrs, "type") else {
        return;
    };

    if !attr_is_text(type_attr) {
        if dir.name != "value" || matches!(type_attr, Attribute::BooleanAttribute(_)) {
            ctx.warnings_mut().push(Diagnostic::error(
                DiagnosticKind::AttributeInvalidType,
                attr_value_span(type_attr),
            ));
        }
        return;
    }

    let type_value = ctx
        .data
        .static_text_attribute_value(parent.id, &parent.attrs, "type", ctx.source)
        .unwrap_or_default();
    if dir.name == "checked" && type_value != "checkbox" {
        let elements = if type_value == "radio" {
            "`<input type=\"checkbox\">` — for `<input type=\"radio\">`, use `bind:group`"
                .to_string()
        } else {
            "`<input type=\"checkbox\">`".to_string()
        };
        emit_bind_error(
            ctx,
            dir.expression_span,
            DiagnosticKind::BindInvalidTarget {
                name: dir.name.clone(),
                elements,
            },
        );
    } else if dir.name == "files" && type_value != "file" {
        emit_bind_error(
            ctx,
            dir.expression_span,
            DiagnosticKind::BindInvalidTarget {
                name: dir.name.clone(),
                elements: "`<input type=\"file\">`".to_string(),
            },
        );
    }
}

fn validate_bind_sequence_expression(
    dir: &BindDirective,
    len: usize,
    has_parens: bool,
    ctx: &mut VisitContext<'_>,
) {
    if dir.name == "group" {
        emit_bind_error(
            ctx,
            dir.expression_span,
            DiagnosticKind::BindGroupInvalidExpression,
        );
    }

    if has_parens {
        emit_bind_error(
            ctx,
            dir.expression_span,
            DiagnosticKind::BindInvalidParens {
                name: dir.name.clone(),
            },
        );
    }

    if len != 2 {
        emit_bind_error(
            ctx,
            dir.expression_span,
            DiagnosticKind::BindInvalidExpression,
        );
    }
}

fn validate_bind_identifier_value(dir: &BindDirective, ctx: &mut VisitContext<'_>) {
    if dir.name == "this" {
        return;
    }

    let Some(sym_id) = bind_base_symbol(dir, ctx) else {
        return;
    };

    let rune_kind = ctx.data.scoping.rune_kind(sym_id);
    let valid = matches!(
        rune_kind,
        Some(crate::types::script::RuneKind::State | crate::types::script::RuneKind::StateRaw)
    ) || ctx.data.scoping.is_prop_source(sym_id)
        || ctx.data.scoping.prop_non_source_name(sym_id).is_some()
        || ctx.data.scoping.is_each_block_var(sym_id)
        || bind_targets_each_context(sym_id, dir.id, ctx)
        || ctx.data.scoping.is_store(sym_id)
        // Plain mutable let/var (no rune) is bindable — the bind directive's setter writes to it.
        // This matches the reference compiler: any bind target is marked as `binding.updated`
        // by scope analysis, so the "not updated" guard never fires for plain let/var.
        || (rune_kind.is_none() && {
            let flags = ctx.data.scoping.symbol_flags(sym_id);
            flags.intersects(SymbolFlags::BlockScopedVariable | SymbolFlags::FunctionScopedVariable)
                && !flags.contains(SymbolFlags::ConstVariable)
        });

    if !valid {
        emit_bind_error(ctx, dir.expression_span, DiagnosticKind::BindInvalidValue);
    }
}

fn validate_bind_group_binding(dir: &BindDirective, ctx: &mut VisitContext<'_>) {
    if dir.name != "group" {
        return;
    }

    let Some(sym_id) = bind_base_symbol(dir, ctx) else {
        return;
    };

    if ctx.data.scoping.is_snippet_param(sym_id) {
        emit_bind_error(
            ctx,
            dir.expression_span,
            DiagnosticKind::BindGroupInvalidSnippetParameter,
        );
        return;
    }

    if ctx.data.scoping.is_each_rest(sym_id) {
        let name = ctx.data.scoping.symbol_name(sym_id).to_string();
        ctx.warnings_mut().push(Diagnostic::warning(
            DiagnosticKind::BindInvalidEachRest { name },
            dir.expression_span.unwrap_or(Span::new(0, 0)),
        ));
    }
}

fn bind_base_symbol(dir: &BindDirective, ctx: &VisitContext<'_>) -> Option<crate::scope::SymbolId> {
    if dir.shorthand {
        return ctx.data.shorthand_symbol(dir.id);
    }

    let info = ctx.data.attr_expression(dir.id)?;
    match info.kind {
        ExpressionKind::Identifier(_) | ExpressionKind::MemberExpression => {
            info.ref_symbols.first().copied()
        }
        _ => None,
    }
}

fn bind_expression<'a>(
    dir: &BindDirective,
    ctx: &'a VisitContext<'a>,
) -> Option<&'a Expression<'a>> {
    let span = dir.expression_span?;
    let parsed = ctx.parsed()?;
    parsed
        .expr_handle(span.start)
        .and_then(|handle| parsed.expr(handle))
}

fn bind_targets_each_context(
    sym_id: crate::scope::SymbolId,
    bind_id: NodeId,
    ctx: &VisitContext<'_>,
) -> bool {
    let sym_scope = ctx.data.scoping.symbol_scope_id(sym_id);
    ctx.data
        .ancestors(bind_id)
        .filter(|parent| parent.kind == ParentKind::EachBlock)
        .any(|parent| ctx.data.each_body_scope(parent.id, ctx.scope) == sym_scope)
}

fn attr_is_text(attr: &Attribute) -> bool {
    matches!(attr, Attribute::StringAttribute(_))
}

fn static_text_attr_value<'a>(attr: &Attribute, source: &'a str) -> Option<&'a str> {
    match attr {
        Attribute::StringAttribute(attr) => Some(attr.value_span.source_text(source)),
        _ => None,
    }
}

fn attr_value_span(attr: &Attribute) -> Span {
    match attr {
        Attribute::ExpressionAttribute(attr) => attr.expression_span,
        Attribute::ConcatenationAttribute(attr) => attr
            .parts
            .iter()
            .filter_map(|part| match part {
                ConcatPart::Dynamic { span, .. } => Some(*span),
                ConcatPart::Static(_) => None,
            })
            .reduce(|left, right| left.merge(&right))
            .unwrap_or(Span::new(0, 0)),
        Attribute::StringAttribute(attr) => attr.value_span,
        Attribute::BooleanAttribute(_) => Span::new(0, 0),
        Attribute::BindDirective(attr) => attr.expression_span.unwrap_or(Span::new(0, 0)),
        Attribute::Shorthand(attr) => attr.expression_span,
        Attribute::SpreadAttribute(attr) => attr.expression_span,
        Attribute::ClassDirective(attr) => attr.expression_span.unwrap_or(Span::new(0, 0)),
        Attribute::StyleDirective(attr) => match &attr.value {
            svelte_ast::StyleDirectiveValue::Expression(span) => *span,
            svelte_ast::StyleDirectiveValue::Concatenation(parts) => parts
                .iter()
                .filter_map(|part| match part {
                    ConcatPart::Dynamic { span, .. } => Some(*span),
                    ConcatPart::Static(_) => None,
                })
                .reduce(|left, right| left.merge(&right))
                .unwrap_or(Span::new(0, 0)),
            svelte_ast::StyleDirectiveValue::String(_)
            | svelte_ast::StyleDirectiveValue::Shorthand => Span::new(0, 0),
        },
        Attribute::UseDirective(attr) => attr.expression_span.unwrap_or(Span::new(0, 0)),
        Attribute::OnDirectiveLegacy(attr) => attr.expression_span.unwrap_or(attr.name_span),
        Attribute::TransitionDirective(attr) => attr.expression_span.unwrap_or(Span::new(0, 0)),
        Attribute::AnimateDirective(attr) => attr.expression_span.unwrap_or(Span::new(0, 0)),
        Attribute::AttachTag(attr) => attr.expression_span,
    }
}

fn validate_snippet_rest_params(block: &SnippetBlock, ctx: &mut VisitContext<'_>) {
    let Some(parsed) = ctx.parsed() else {
        return;
    };
    let Some(stmt) = parsed
        .stmt_handle(block.expression_span.start)
        .and_then(|handle| parsed.stmt(handle))
    else {
        return;
    };
    let Some(params) = extract_arrow_params(stmt) else {
        return;
    };

    if let Some(rest) = &params.rest {
        ctx.warnings_mut().push(Diagnostic::error(
            DiagnosticKind::SnippetInvalidRestParameter,
            block.expression_span,
        ));
        let _ = rest;
    }
}

fn validate_snippet_shadowing_prop(block: &SnippetBlock, ctx: &mut VisitContext<'_>) {
    let Some(parent) = ctx.data.parent(block.id) else {
        return;
    };
    if parent.kind != ParentKind::ComponentNode {
        return;
    }

    let Node::ComponentNode(component) = ctx.store.get(parent.id) else {
        return;
    };
    let snippet_name = block.name(ctx.source);
    if component
        .attributes
        .iter()
        .any(|attr| named_component_attr(attr, snippet_name))
    {
        ctx.warnings_mut().push(Diagnostic::error(
            DiagnosticKind::SnippetShadowingProp {
                prop: snippet_name.to_string(),
            },
            block.expression_span,
        ));
    }
}

fn validate_snippet_children_conflict(block: &SnippetBlock, ctx: &mut VisitContext<'_>) {
    if block.name(ctx.source) != "children" {
        return;
    }

    let Some(parent) = ctx.data.parent(block.id) else {
        return;
    };
    if parent.kind != ParentKind::ComponentNode {
        return;
    }

    let Node::ComponentNode(component) = ctx.store.get(parent.id) else {
        return;
    };
    if component_has_implicit_children(component, block.id, ctx) {
        ctx.warnings_mut().push(Diagnostic::error(
            DiagnosticKind::SnippetConflict,
            block.expression_span,
        ));
    }
}

/// Warns `BlockEmpty` when a fragment contains exactly one whitespace-only text node.
/// Mirrors `validate_block_not_empty` in the reference compiler's shared utils.
fn check_empty_fragment(_fragment: &Fragment, key: FragmentKey, ctx: &mut VisitContext<'_>) {
    if let Some(node_id) = ctx.data.fragment_single_child(&key) {
        if let Node::Text(text) = ctx.store.get(node_id) {
            if text.value(ctx.source).trim().is_empty() {
                ctx.warnings_mut()
                    .push(Diagnostic::warning(DiagnosticKind::BlockEmpty, text.span));
            }
        }
    }
}

/// Returns true when `parent` is a `RegularElement` or `SvelteElement` with a `slot="..."` attr.
///
/// Used by const_tag_invalid_placement to allow `{@const}` inside slotted elements,
/// matching the reference compiler's allowed-parent matrix.
fn element_has_slot_attr(parent: ParentRef, ctx: &VisitContext<'_>) -> bool {
    if !matches!(parent.kind, ParentKind::Element | ParentKind::SvelteElement) {
        return false;
    }
    let attrs = match ctx.store.get(parent.id) {
        Node::Element(el) => &el.attributes,
        Node::SvelteElement(el) => &el.attributes,
        _ => return false,
    };
    ctx.data
        .attribute(parent.id, attrs, "slot")
        .is_some_and(|attr| matches!(attr, Attribute::StringAttribute(_)))
}

fn contains_non_whitespace_text(text: &str) -> bool {
    text.chars()
        .any(|ch| !matches!(ch, ' ' | '\t' | '\r' | '\n'))
}

fn is_bidi_control(ch: char) -> bool {
    matches!(
        ch,
        '\u{202A}'
            | '\u{202B}'
            | '\u{202C}'
            | '\u{202D}'
            | '\u{202E}'
            | '\u{2066}'
            | '\u{2067}'
            | '\u{2068}'
            | '\u{2069}'
    )
}

fn invalid_text_parent_message(id: NodeId, ctx: &VisitContext<'_>) -> Option<String> {
    let parent = ctx
        .data
        .ancestors(id)
        .find(|parent| parent.kind == ParentKind::Element)?;
    let element = ctx.store.get(parent.id).as_element()?;
    let name = element.name.as_str();

    if matches!(
        name,
        "table" | "thead" | "tbody" | "tfoot" | "tr" | "colgroup" | "select" | "datalist"
    ) {
        Some(format!("`<{}>` cannot contain text nodes", name))
    } else {
        None
    }
}

fn is_each_block_var_ref(
    ident: &oxc_ast::ast::IdentifierReference<'_>,
    scoping: &ComponentScoping,
) -> bool {
    ident
        .reference_id
        .get()
        .and_then(|ref_id| scoping.get_reference(ref_id).symbol_id())
        .is_some_and(|sym| scoping.is_each_block_var(sym))
}

fn is_snippet_param_ref(
    ident: &oxc_ast::ast::IdentifierReference<'_>,
    scoping: &ComponentScoping,
) -> bool {
    ident
        .reference_id
        .get()
        .and_then(|ref_id| scoping.get_reference(ref_id).symbol_id())
        .is_some_and(|sym| scoping.is_snippet_param(sym))
}

fn named_component_attr(attr: &Attribute, name: &str) -> bool {
    match attr {
        Attribute::StringAttribute(attr) => attr.name == name,
        Attribute::ExpressionAttribute(attr) => attr.name == name,
        Attribute::BooleanAttribute(attr) => attr.name == name,
        Attribute::ConcatenationAttribute(attr) => attr.name == name,
        Attribute::BindDirective(attr) => attr.name == name,
        Attribute::Shorthand(_)
        | Attribute::SpreadAttribute(_)
        | Attribute::ClassDirective(_)
        | Attribute::StyleDirective(_)
        | Attribute::UseDirective(_)
        | Attribute::OnDirectiveLegacy(_)
        | Attribute::TransitionDirective(_)
        | Attribute::AnimateDirective(_)
        | Attribute::AttachTag(_) => false,
    }
}

fn component_has_implicit_children(
    component: &ComponentNode,
    current_snippet_id: NodeId,
    ctx: &VisitContext<'_>,
) -> bool {
    let key = FragmentKey::ComponentNode(component.id);
    match ctx.data.fragment_non_trivial_child_count(&key) {
        0 => false,
        1 => {
            let Some(node_id) = ctx.data.fragment_single_non_trivial_child(&key) else {
                return true;
            };
            !matches!(
                ctx.store.get(node_id),
                Node::SnippetBlock(snippet) if snippet.id == current_snippet_id
            )
        }
        _ => true,
    }
}

fn extract_arrow_params<'s, 'a: 's>(
    stmt: &'s oxc_ast::ast::Statement<'a>,
) -> Option<&'s oxc_ast::ast::FormalParameters<'a>> {
    let oxc_ast::ast::Statement::VariableDeclaration(decl) = stmt else {
        return None;
    };
    let declarator = decl.declarations.first()?;
    let Some(Expression::ArrowFunctionExpression(arrow)) = &declarator.init else {
        return None;
    };
    Some(&arrow.params)
}

struct EachBlockVarRefVisitor<'s> {
    scoping: &'s ComponentScoping,
    found: bool,
}

impl<'a> Visit<'a> for EachBlockVarRefVisitor<'_> {
    fn visit_identifier_reference(&mut self, ident: &oxc_ast::ast::IdentifierReference<'a>) {
        if is_each_block_var_ref(ident, self.scoping) {
            self.found = true;
        }
    }

    fn visit_assignment_target_property_identifier(
        &mut self,
        it: &oxc_ast::ast::AssignmentTargetPropertyIdentifier<'a>,
    ) {
        if is_each_block_var_ref(&it.binding, self.scoping) {
            self.found = true;
        }
        if let Some(init) = &it.init {
            self.visit_expression(init);
        }
    }

    fn visit_expression(&mut self, expr: &Expression<'a>) {
        if self.found {
            return;
        }
        walk::walk_expression(self, expr);
    }

    fn visit_assignment_target(&mut self, target: &AssignmentTarget<'a>) {
        if self.found {
            return;
        }
        walk::walk_assignment_target(self, target);
    }

    fn visit_simple_assignment_target(&mut self, target: &SimpleAssignmentTarget<'a>) {
        if self.found {
            return;
        }
        walk::walk_simple_assignment_target(self, target);
    }
}

fn contains_each_block_var_in_assignment_target(
    target: &AssignmentTarget<'_>,
    scoping: &ComponentScoping,
) -> bool {
    let mut visitor = EachBlockVarRefVisitor {
        scoping,
        found: false,
    };
    visitor.visit_assignment_target(target);
    visitor.found
}

fn contains_each_block_var_in_simple_target(
    target: &SimpleAssignmentTarget<'_>,
    scoping: &ComponentScoping,
) -> bool {
    let mut visitor = EachBlockVarRefVisitor {
        scoping,
        found: false,
    };
    visitor.visit_simple_assignment_target(target);
    visitor.found
}

struct InvalidEachAssignmentVisitor<'s> {
    scoping: &'s ComponentScoping,
    found: bool,
}

impl<'a> Visit<'a> for InvalidEachAssignmentVisitor<'_> {
    fn visit_assignment_expression(&mut self, expr: &oxc_ast::ast::AssignmentExpression<'a>) {
        if contains_each_block_var_in_assignment_target(&expr.left, self.scoping) {
            self.found = true;
            return;
        }
        walk::walk_assignment_expression(self, expr);
    }

    fn visit_update_expression(&mut self, expr: &oxc_ast::ast::UpdateExpression<'a>) {
        if contains_each_block_var_in_simple_target(&expr.argument, self.scoping) {
            self.found = true;
            return;
        }
        walk::walk_update_expression(self, expr);
    }

    fn visit_expression(&mut self, expr: &Expression<'a>) {
        if self.found {
            return;
        }
        walk::walk_expression(self, expr);
    }
}

fn contains_invalid_each_assignment(expr: &Expression<'_>, scoping: &ComponentScoping) -> bool {
    let mut visitor = InvalidEachAssignmentVisitor {
        scoping,
        found: false,
    };
    visitor.visit_expression(expr);
    visitor.found
}

struct SnippetParamRefVisitor<'s> {
    scoping: &'s ComponentScoping,
    found: bool,
}

impl<'a> Visit<'a> for SnippetParamRefVisitor<'_> {
    fn visit_identifier_reference(&mut self, ident: &oxc_ast::ast::IdentifierReference<'a>) {
        if is_snippet_param_ref(ident, self.scoping) {
            self.found = true;
        }
    }

    fn visit_assignment_target_property_identifier(
        &mut self,
        it: &oxc_ast::ast::AssignmentTargetPropertyIdentifier<'a>,
    ) {
        if is_snippet_param_ref(&it.binding, self.scoping) {
            self.found = true;
        }
        if let Some(init) = &it.init {
            self.visit_expression(init);
        }
    }

    fn visit_expression(&mut self, expr: &Expression<'a>) {
        if self.found {
            return;
        }
        walk::walk_expression(self, expr);
    }

    fn visit_assignment_target(&mut self, target: &AssignmentTarget<'a>) {
        if self.found {
            return;
        }
        walk::walk_assignment_target(self, target);
    }

    fn visit_simple_assignment_target(&mut self, target: &SimpleAssignmentTarget<'a>) {
        if self.found {
            return;
        }
        walk::walk_simple_assignment_target(self, target);
    }
}

fn contains_snippet_param_in_assignment_target(
    target: &AssignmentTarget<'_>,
    scoping: &ComponentScoping,
) -> bool {
    let mut visitor = SnippetParamRefVisitor {
        scoping,
        found: false,
    };
    visitor.visit_assignment_target(target);
    visitor.found
}

fn contains_snippet_param_in_simple_target(
    target: &SimpleAssignmentTarget<'_>,
    scoping: &ComponentScoping,
) -> bool {
    let mut visitor = SnippetParamRefVisitor {
        scoping,
        found: false,
    };
    visitor.visit_simple_assignment_target(target);
    visitor.found
}

struct InvalidSnippetParamAssignmentVisitor<'s> {
    scoping: &'s ComponentScoping,
    found: bool,
}

impl<'a> Visit<'a> for InvalidSnippetParamAssignmentVisitor<'_> {
    fn visit_assignment_expression(&mut self, expr: &oxc_ast::ast::AssignmentExpression<'a>) {
        if contains_snippet_param_in_assignment_target(&expr.left, self.scoping) {
            self.found = true;
            return;
        }
        walk::walk_assignment_expression(self, expr);
    }

    fn visit_update_expression(&mut self, expr: &oxc_ast::ast::UpdateExpression<'a>) {
        if contains_snippet_param_in_simple_target(&expr.argument, self.scoping) {
            self.found = true;
            return;
        }
        walk::walk_update_expression(self, expr);
    }

    fn visit_expression(&mut self, expr: &Expression<'a>) {
        if self.found {
            return;
        }
        walk::walk_expression(self, expr);
    }
}

/// Returns true when `window` (the source slice just before a binding pattern) contains
/// whitespace between the opening `{` and the clause keyword (`:then` or `:catch`).
/// Catches patterns like `{ :then val}` where a space precedes the colon.
fn has_whitespace_before_clause(window: &str, clause: &str) -> bool {
    if let Some(brace_pos) = window.rfind('{') {
        let between = &window[brace_pos + 1..];
        let ws_len = between.len() - between.trim_start().len();
        let rest = &between[ws_len..];
        rest.starts_with(clause) && ws_len > 0
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::has_whitespace_before_clause;

    #[test]
    fn no_whitespace_before_then() {
        assert!(!has_whitespace_before_clause("{:then ", ":then"));
    }

    #[test]
    fn whitespace_before_then() {
        assert!(has_whitespace_before_clause("{ :then ", ":then"));
    }

    #[test]
    fn multiple_spaces_before_catch() {
        assert!(has_whitespace_before_clause("{  :catch ", ":catch"));
    }

    #[test]
    fn no_brace_in_window() {
        assert!(!has_whitespace_before_clause(":then val", ":then"));
    }

    #[test]
    fn shorthand_then_form() {
        // {#await expr then val} — no `{:then` pattern before binding
        assert!(!has_whitespace_before_clause(" expr then ", ":then"));
    }
}

fn contains_invalid_snippet_param_assignment(
    expr: &Expression<'_>,
    scoping: &ComponentScoping,
) -> bool {
    let mut visitor = InvalidSnippetParamAssignmentVisitor {
        scoping,
        found: false,
    };
    visitor.visit_expression(expr);
    visitor.found
}

// ---------------------------------------------------------------------------
// A11y: missing required attributes
// ---------------------------------------------------------------------------

/// Emit `attribute_avoid_is`, `attribute_illegal_colon`, and
/// `attribute_invalid_property_name` warnings for plain HTML attributes on a
/// `RegularElement` or `SvelteElement`.  The borrow of `ctx.data` is confined
/// to the inner block so `ctx.warnings_mut()` can be called freely afterwards.
fn check_plain_attr_warnings(
    id: NodeId,
    span: Span,
    attrs: &[Attribute],
    ctx: &mut VisitContext<'_>,
) {
    let (has_is, has_colon, invalid_prop) = {
        let has_colon = attrs.iter().any(|a| {
            let n = a.html_name();
            n.contains(':')
                && !n.starts_with("xml:")
                && !n.starts_with("xlink:")
                && !n.starts_with("xmlns:")
        });
        let invalid_prop = if ctx.data.has_attribute(id, "className") {
            Some(("className", "class"))
        } else if ctx.data.has_attribute(id, "htmlFor") {
            Some(("htmlFor", "for"))
        } else {
            None
        };
        (ctx.data.has_attribute(id, "is"), has_colon, invalid_prop)
    };

    if has_is {
        ctx.warnings_mut()
            .push(Diagnostic::warning(DiagnosticKind::AttributeAvoidIs, span));
    }
    if has_colon {
        ctx.warnings_mut().push(Diagnostic::warning(
            DiagnosticKind::AttributeIllegalColon,
            span,
        ));
    }
    if let Some((wrong, right)) = invalid_prop {
        ctx.warnings_mut().push(Diagnostic::warning(
            DiagnosticKind::AttributeInvalidPropertyName {
                wrong: wrong.to_string(),
                right: right.to_string(),
            },
            span,
        ));
    }
}

fn check_component_name_lowercase(el: &Element, ctx: &mut VisitContext<'_>) {
    let Some(sym_id) = ctx.data.scoping.find_binding(ctx.scope, &el.name) else {
        return;
    };

    if ctx.data.scoping.is_import(sym_id)
        && ctx
            .data
            .scoping
            .get_resolved_reference_ids(sym_id)
            .is_empty()
    {
        ctx.warnings_mut().push(Diagnostic::warning(
            DiagnosticKind::ComponentNameLowercase {
                name: el.name.clone(),
            },
            el.span,
        ));
    }
}

/// `attribute_quoted` warning: fires in runes mode when a component or custom element
/// receives an attribute whose value is a quoted single expression (e.g. `foo="{expr}"`).
/// In the AST this appears as a `ConcatenationAttribute` with exactly one `Dynamic` part.
fn check_attribute_quoted(attrs: &[Attribute], ctx: &mut VisitContext<'_>) {
    if !ctx.runes {
        return;
    }
    for attr in attrs {
        if let Attribute::ConcatenationAttribute(ca) = attr {
            if ca.parts.len() == 1 && matches!(ca.parts[0], ConcatPart::Dynamic { .. }) {
                ctx.warnings_mut().push(Diagnostic::warning(
                    DiagnosticKind::AttributeQuoted,
                    attr_value_span(attr),
                ));
            }
        }
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

fn check_a11y_aria_attribute_warnings(el: &Element, attrs: &[Attribute], ctx: &mut VisitContext<'_>) {
    for attr in attrs {
        let Some(name) = attr_named_name(attr) else {
            continue;
        };
        let name = name.to_ascii_lowercase();
        if !name.starts_with("aria-") {
            continue;
        }

        if A11Y_INVISIBLE_ELEMENTS.contains(&el.name.as_str()) {
            ctx.warnings_mut().push(Diagnostic::warning(
                DiagnosticKind::A11yAriaAttributes {
                    name: el.name.clone(),
                },
                attr_value_span(attr),
            ));
        }

        let attribute = name.trim_start_matches("aria-");
        if !A11Y_ARIA_ATTRIBUTES.contains(&attribute) {
            ctx.warnings_mut().push(Diagnostic::warning(
                DiagnosticKind::A11yUnknownAriaAttribute {
                    attribute: attribute.to_string(),
                    suggestion: fuzzymatch(attribute, A11Y_ARIA_ATTRIBUTES)
                        .map(|s| format!("aria-{s}")),
                },
                attr_value_span(attr),
            ));
        }

        if name == "aria-hidden" && is_heading_tag(&el.name) {
            ctx.warnings_mut().push(Diagnostic::warning(
                DiagnosticKind::A11yHidden {
                    name: el.name.clone(),
                },
                attr_value_span(attr),
            ));
        }
    }
}

fn check_a11y_role_warnings(el: &Element, attrs: &[Attribute], ctx: &mut VisitContext<'_>) {
    let has_spread = ctx.data.has_spread(el.id);
    let implicit_role = implicit_role_for_element(el, attrs, ctx);
    let is_parent_section_or_article = has_sectioning_ancestor(el.id, ctx);

    for attr in attrs {
        let Some(name) = attr_named_name(attr) else {
            continue;
        };
        if !name.eq_ignore_ascii_case("role") {
            continue;
        }

        if A11Y_INVISIBLE_ELEMENTS.contains(&el.name.as_str()) {
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

            if A11Y_ABSTRACT_ROLES.contains(&role) {
                ctx.warnings_mut().push(Diagnostic::warning(
                    DiagnosticKind::A11yNoAbstractRole {
                        role: role.to_string(),
                    },
                    attr_value_span(attr),
                ));
            } else if !A11Y_ARIA_ROLES.contains(&role) {
                ctx.warnings_mut().push(Diagnostic::warning(
                    DiagnosticKind::A11yUnknownRole {
                        role: role.to_string(),
                        suggestion: fuzzymatch(role, A11Y_ARIA_ROLES).map(str::to_string),
                    },
                    attr_value_span(attr),
                ));
            }

            if implicit_role == Some(role)
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

            if !is_parent_section_or_article
                && nested_implicit_role(el.name.as_str()).is_some_and(|nested| nested == role)
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
        .or_else(|| explicit_role_attr.is_none().then(|| implicit_role_for_element(el, attrs, ctx)).flatten());
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
        if !A11Y_ARIA_ATTRIBUTES.contains(&attribute) {
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
    let is_static = interactivity == ElementInteractivity::Static;
    let has_spread = ctx.data.has_spread(el.id);
    let has_tabindex = ctx.data.attribute(el.id, attrs, "tabindex");
    let role_attr = ctx.data.attribute(el.id, attrs, "role");
    let role_static_value = role_attr.and_then(|attr| static_text_attr_value(attr, ctx.source));
    let handlers = collect_element_handlers(attrs);

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

    let has_interactive_handlers = handlers
        .iter()
        .any(|handler| A11Y_INTERACTIVE_HANDLERS.contains(&handler.as_str()));
    if !has_interactive_handlers
        || has_spread
        || has_disabled_attribute(el.id, attrs, ctx)
        || is_hidden_from_screen_reader(el, attrs, ctx)
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
        if role.is_empty()
            || !is_interactive_role(Some(role))
            || is_presentation_role(Some(role))
        {
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

fn implicit_role_for_element<'a>(
    el: &Element,
    attrs: &'a [Attribute],
    ctx: &VisitContext<'a>,
) -> Option<&'static str> {
    match el.name.as_str() {
        "menuitem" => {
            let type_attr = ctx.data.attribute(el.id, attrs, "type")?;
            let type_value = static_text_attr_value(type_attr, ctx.source)?;
            lookup_static_pair(A11Y_MENUITEM_IMPLICIT_ROLES, type_value)
        }
        "input" => {
            let type_attr = ctx.data.attribute(el.id, attrs, "type")?;
            let type_value = static_text_attr_value(type_attr, ctx.source)?;
            if ctx.data.has_attribute(el.id, "list") && A11Y_COMBOBOX_INPUT_TYPES.contains(&type_value)
            {
                Some("combobox")
            } else {
                lookup_static_pair(A11Y_INPUT_IMPLICIT_ROLES, type_value)
            }
        }
        _ => lookup_static_pair(A11Y_IMPLICIT_ROLES, el.name.as_str()),
    }
}

fn nested_implicit_role(name: &str) -> Option<&'static str> {
    lookup_static_pair(A11Y_NESTED_IMPLICIT_ROLES, name)
}

fn required_role_props(role: &str) -> Option<&'static [&'static str]> {
    A11Y_REQUIRED_ROLE_PROPS
        .iter()
        .find_map(|(name, props)| (*name == role).then_some(*props))
}

fn is_semantic_role_element<'a>(
    el: &Element,
    attrs: &'a [Attribute],
    ctx: &VisitContext<'a>,
    role: &str,
) -> bool {
    if implicit_role_for_element(el, attrs, ctx).is_some_and(|implicit| implicit == role) {
        return true;
    }

    !has_sectioning_ancestor(el.id, ctx)
        && matches!(
            (el.name.as_str(), role),
            ("header", "banner") | ("footer", "contentinfo")
        )
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

fn collect_element_handlers(attrs: &[Attribute]) -> Vec<String> {
    attrs.iter()
        .filter_map(|attr| match attr {
            Attribute::ExpressionAttribute(attr) => attr.event_name.clone(),
            Attribute::OnDirectiveLegacy(attr) => Some(attr.name.clone()),
            _ => None,
        })
        .collect()
}

fn is_hidden_from_screen_reader(
    el: &Element,
    attrs: &[Attribute],
    ctx: &VisitContext<'_>,
) -> bool {
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
            _ => static_text_attr_value(attr, ctx.source)
                .is_some_and(|value| value == "true"),
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

fn element_interactivity(
    el: &Element,
    attrs: &[Attribute],
    ctx: &VisitContext<'_>,
) -> ElementInteractivity {
    if let Some(role) = implicit_role_for_element(el, attrs, ctx) {
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

fn is_interactive_role(role: Option<&str>) -> bool {
    role.is_some_and(|role| A11Y_INTERACTIVE_ROLES.contains(&role))
}

fn is_presentation_role(role: Option<&str>) -> bool {
    role.is_some_and(|role| A11Y_PRESENTATION_ROLES.contains(&role))
}

fn is_known_role_for_prop_support(role: &str) -> bool {
    A11Y_ARIA_ROLES.contains(&role)
}

fn role_supports_aria_prop(role: &str, attr: &str) -> bool {
    if matches!(role, "none" | "doc-pullquote") {
        return false;
    }

    if A11Y_GLOBAL_ROLE_SUPPORTED_PROPS.contains(&attr) {
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
                | "aria-haspopup"
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
                | "aria-disabled"
                | "aria-errormessage"
                | "aria-expanded"
                | "aria-haspopup"
                | "aria-invalid"
                | "aria-readonly"
                | "aria-required"
        ),
        "composite" | "group" => matches!(attr, "aria-activedescendant" | "aria-disabled"),
        "doc-abstract"
        | "doc-acknowledgments"
        | "doc-afterword"
        | "doc-appendix"
        | "doc-backlink"
        | "doc-bibliography"
        | "doc-biblioref"
        | "doc-chapter"
        | "doc-colophon"
        | "doc-conclusion"
        | "doc-cover"
        | "doc-credit"
        | "doc-credits"
        | "doc-dedication"
        | "doc-endnotes"
        | "doc-epigraph"
        | "doc-epilogue"
        | "doc-errata"
        | "doc-example"
        | "doc-footnote"
        | "doc-foreword"
        | "doc-glossary"
        | "doc-glossref"
        | "doc-index"
        | "doc-introduction"
        | "doc-noteref"
        | "doc-notice"
        | "doc-pagelist"
        | "doc-part"
        | "doc-preface"
        | "doc-prologue"
        | "doc-qna"
        | "doc-subtitle"
        | "doc-tip"
        | "doc-toc"
        | "graphics-document"
        | "graphics-symbol" => matches!(
            attr,
            "aria-disabled"
                | "aria-errormessage"
                | "aria-expanded"
                | "aria-haspopup"
                | "aria-invalid"
        ),
        "doc-biblioentry" | "doc-endnote" => matches!(
            attr,
            "aria-disabled"
                | "aria-errormessage"
                | "aria-expanded"
                | "aria-haspopup"
                | "aria-invalid"
                | "aria-level"
                | "aria-posinset"
                | "aria-setsize"
        ),
        "doc-pagebreak" => matches!(
            attr,
            "aria-disabled"
                | "aria-errormessage"
                | "aria-expanded"
                | "aria-haspopup"
                | "aria-invalid"
                | "aria-orientation"
                | "aria-valuemax"
                | "aria-valuemin"
                | "aria-valuenow"
                | "aria-valuetext"
        ),
        "doc-pagefooter" | "doc-pageheader" => matches!(
            attr,
            "aria-braillelabel"
                | "aria-brailleroledescription"
                | "aria-description"
                | "aria-disabled"
                | "aria-errormessage"
                | "aria-haspopup"
                | "aria-invalid"
        ),
        "grid" => matches!(
            attr,
            "aria-activedescendant"
                | "aria-colcount"
                | "aria-disabled"
                | "aria-multiselectable"
                | "aria-readonly"
                | "aria-rowcount"
        ),
        "gridcell" => matches!(
            attr,
            "aria-colindex"
                | "aria-colspan"
                | "aria-disabled"
                | "aria-errormessage"
                | "aria-expanded"
                | "aria-haspopup"
                | "aria-invalid"
                | "aria-readonly"
                | "aria-required"
                | "aria-rowindex"
                | "aria-rowspan"
                | "aria-selected"
        ),
        "heading" => matches!(attr, "aria-level"),
        "input" => matches!(attr, "aria-disabled"),
        "link" => matches!(attr, "aria-disabled" | "aria-expanded" | "aria-haspopup"),
        "listbox" => matches!(
            attr,
            "aria-activedescendant"
                | "aria-disabled"
                | "aria-errormessage"
                | "aria-expanded"
                | "aria-invalid"
                | "aria-multiselectable"
                | "aria-orientation"
                | "aria-readonly"
                | "aria-required"
        ),
        "listitem" => matches!(attr, "aria-level" | "aria-posinset" | "aria-setsize"),
        "mark" => matches!(
            attr,
            "aria-braillelabel" | "aria-brailleroledescription" | "aria-description"
        ),
        "menu" | "menubar" | "select" | "toolbar" => matches!(
            attr,
            "aria-activedescendant" | "aria-disabled" | "aria-orientation"
        ),
        "menuitem" => matches!(
            attr,
            "aria-disabled"
                | "aria-expanded"
                | "aria-haspopup"
                | "aria-posinset"
                | "aria-setsize"
        ),
        "menuitemcheckbox" | "menuitemradio" => matches!(
            attr,
            "aria-checked"
                | "aria-disabled"
                | "aria-errormessage"
                | "aria-expanded"
                | "aria-haspopup"
                | "aria-invalid"
                | "aria-posinset"
                | "aria-readonly"
                | "aria-required"
                | "aria-setsize"
        ),
        "meter" | "progressbar" => matches!(
            attr,
            "aria-valuemax" | "aria-valuemin" | "aria-valuenow" | "aria-valuetext"
        ),
        "option" => matches!(
            attr,
            "aria-checked"
                | "aria-disabled"
                | "aria-posinset"
                | "aria-selected"
                | "aria-setsize"
        ),
        "radio" => matches!(
            attr,
            "aria-checked" | "aria-disabled" | "aria-posinset" | "aria-setsize"
        ),
        "radiogroup" => matches!(
            attr,
            "aria-activedescendant"
                | "aria-disabled"
                | "aria-errormessage"
                | "aria-invalid"
                | "aria-orientation"
                | "aria-readonly"
                | "aria-required"
        ),
        "range" => matches!(attr, "aria-valuemax" | "aria-valuemin" | "aria-valuenow"),
        "row" => matches!(
            attr,
            "aria-activedescendant"
                | "aria-colindex"
                | "aria-disabled"
                | "aria-expanded"
                | "aria-level"
                | "aria-posinset"
                | "aria-rowindex"
                | "aria-selected"
                | "aria-setsize"
        ),
        "scrollbar" | "separator" => matches!(
            attr,
            "aria-disabled"
                | "aria-orientation"
                | "aria-valuemax"
                | "aria-valuemin"
                | "aria-valuenow"
                | "aria-valuetext"
        ),
        "searchbox" | "textbox" => matches!(
            attr,
            "aria-activedescendant"
                | "aria-autocomplete"
                | "aria-disabled"
                | "aria-errormessage"
                | "aria-haspopup"
                | "aria-invalid"
                | "aria-multiline"
                | "aria-placeholder"
                | "aria-readonly"
                | "aria-required"
        ),
        "slider" => matches!(
            attr,
            "aria-disabled"
                | "aria-errormessage"
                | "aria-haspopup"
                | "aria-invalid"
                | "aria-orientation"
                | "aria-readonly"
                | "aria-valuemax"
                | "aria-valuemin"
                | "aria-valuenow"
                | "aria-valuetext"
        ),
        "spinbutton" => matches!(
            attr,
            "aria-activedescendant"
                | "aria-disabled"
                | "aria-errormessage"
                | "aria-invalid"
                | "aria-readonly"
                | "aria-required"
                | "aria-valuemax"
                | "aria-valuemin"
                | "aria-valuenow"
                | "aria-valuetext"
        ),
        "tab" => matches!(
            attr,
            "aria-disabled"
                | "aria-expanded"
                | "aria-haspopup"
                | "aria-posinset"
                | "aria-selected"
                | "aria-setsize"
        ),
        "table" => matches!(attr, "aria-colcount" | "aria-rowcount"),
        "tablist" => matches!(
            attr,
            "aria-activedescendant"
                | "aria-disabled"
                | "aria-level"
                | "aria-multiselectable"
                | "aria-orientation"
        ),
        "tree" => matches!(
            attr,
            "aria-activedescendant"
                | "aria-disabled"
                | "aria-errormessage"
                | "aria-invalid"
                | "aria-multiselectable"
                | "aria-orientation"
                | "aria-required"
        ),
        "treegrid" => matches!(
            attr,
            "aria-activedescendant"
                | "aria-colcount"
                | "aria-disabled"
                | "aria-errormessage"
                | "aria-invalid"
                | "aria-multiselectable"
                | "aria-orientation"
                | "aria-readonly"
                | "aria-required"
                | "aria-rowcount"
        ),
        "treeitem" => matches!(
            attr,
            "aria-checked"
                | "aria-disabled"
                | "aria-expanded"
                | "aria-haspopup"
                | "aria-level"
                | "aria-posinset"
                | "aria-selected"
                | "aria-setsize"
        ),
        _ => false,
    }
}

fn warn_missing_attr(el: &Element, required: &[&str]) -> Diagnostic {
    let first = required[0];
    // "href" and vowel-starting names take "an"; everything else takes "a".
    let article = if first == "href" || first.starts_with(['a', 'e', 'i', 'o', 'u']) {
        "an"
    } else {
        "a"
    };
    let sequence = if required.len() == 1 {
        required[0].to_string()
    } else {
        let (last, rest) = required.split_last().unwrap();
        format!("{} or {last}", rest.join(", "))
    };
    Diagnostic::warning(
        DiagnosticKind::A11yMissingAttribute {
            name: el.name.clone(),
            article: article.to_string(),
            sequence,
        },
        el.span,
    )
}

/// Check `a11y_missing_attribute` for elements that require specific attributes.
/// Only called when no spread attribute is present on `el`.
/// Returns a diagnostic to emit, or `None` if the element is valid.
fn check_a11y_missing_attribute(
    el: &Element,
    data: &AnalysisData,
    source: &str,
) -> Option<Diagnostic> {
    match el.name.as_str() {
        // img needs alt
        "img" => (!data.has_attribute(el.id, "alt")).then(|| warn_missing_attr(el, &["alt"])),
        // area needs alt, aria-label, or aria-labelledby
        "area" => (!data.has_attribute(el.id, "alt")
            && !data.has_attribute(el.id, "aria-label")
            && !data.has_attribute(el.id, "aria-labelledby"))
        .then(|| warn_missing_attr(el, &["alt", "aria-label", "aria-labelledby"])),
        // iframe needs title
        "iframe" => {
            (!data.has_attribute(el.id, "title")).then(|| warn_missing_attr(el, &["title"]))
        }
        // object needs title, aria-label, or aria-labelledby
        "object" => (!data.has_attribute(el.id, "title")
            && !data.has_attribute(el.id, "aria-label")
            && !data.has_attribute(el.id, "aria-labelledby"))
        .then(|| warn_missing_attr(el, &["title", "aria-label", "aria-labelledby"])),
        // <a> without href is only valid as a named anchor (id/name) or disabled link
        "a" => {
            if data.has_attribute(el.id, "href") || data.has_attribute(el.id, "xlink:href") {
                return None;
            }
            // Named anchors and aria-disabled links don't require href.
            if data.has_attribute(el.id, "id") || data.has_attribute(el.id, "name") {
                return None;
            }
            if data.has_true_boolean_attribute(el.id, &el.attributes, "aria-disabled", source) {
                return None;
            }
            Some(warn_missing_attr(el, &["href"]))
        }
        _ => None,
    }
}
