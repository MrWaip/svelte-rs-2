use oxc_ast::ast::{
    AssignmentTarget, Expression, IdentifierReference, SimpleAssignmentTarget, Statement,
};
use oxc_ast_visit::{Visit, walk};
use oxc_span::GetSpan;
use svelte_ast::{
    AnimateDirective, Attribute, AwaitBlock, BindDirective, ComponentNode, ConcatPart, ConstTag,
    DebugTag, EachBlock, Element, ExpressionAttribute, ExpressionTag, IfBlock, KeyBlock,
    LetDirectiveLegacy, Node, NodeId, OnDirectiveLegacy, RenderTag, SVELTE_BODY, SVELTE_COMPONENT,
    SVELTE_DOCUMENT, SVELTE_ELEMENT, SVELTE_SELF, SVELTE_WINDOW, SlotElementLegacy, SnippetBlock,
    SvelteBody, SvelteDocument, SvelteElement, SvelteFragmentLegacy, SvelteWindow, Text,
    TransitionDirection, TransitionDirective, UseDirective, is_svg,
};
use svelte_component_semantics::SymbolFlags;
use svelte_diagnostics::codes::fuzzymatch;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::types::data::{BindHostKind, BindPropertyKind};
use crate::walker::{ParentKind, ParentRef, TemplateVisitor, VisitContext};
use crate::{AnalysisData, EventModifier};

mod a11y;

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

const A11Y_ARIA_AUTOCOMPLETE_VALUES: &[&str] = &["inline", "list", "both", "none"];
const A11Y_ARIA_CURRENT_VALUES: &[&str] =
    &["page", "step", "location", "date", "time", "true", "false"];
const A11Y_ARIA_HASPOPUP_VALUES: &[&str] =
    &["false", "true", "menu", "listbox", "tree", "grid", "dialog"];
const A11Y_ARIA_INVALID_VALUES: &[&str] = &["grammar", "false", "spelling", "true"];
const A11Y_ARIA_LIVE_VALUES: &[&str] = &["assertive", "off", "polite"];
const A11Y_ARIA_ORIENTATION_VALUES: &[&str] = &["vertical", "undefined", "horizontal"];
const A11Y_ARIA_SORT_VALUES: &[&str] = &["ascending", "descending", "none", "other"];
const A11Y_ARIA_DROPEFFECT_VALUES: &[&str] = &["copy", "execute", "link", "move", "none", "popup"];
const A11Y_ARIA_RELEVANT_VALUES: &[&str] = &["additions", "all", "removals", "text"];

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

const A11Y_NON_INTERACTIVE_ELEMENT_TO_INTERACTIVE_ROLE_EXCEPTIONS: &[(&str, &[&str])] = &[
    (
        "ul",
        &[
            "listbox",
            "menu",
            "menubar",
            "radiogroup",
            "tablist",
            "tree",
            "treegrid",
        ],
    ),
    (
        "ol",
        &[
            "listbox",
            "menu",
            "menubar",
            "radiogroup",
            "tablist",
            "tree",
            "treegrid",
        ],
    ),
    (
        "menu",
        &[
            "listbox",
            "menu",
            "menubar",
            "radiogroup",
            "tablist",
            "tree",
            "treegrid",
        ],
    ),
    ("li", &["menuitem", "option", "row", "tab", "treeitem"]),
    ("table", &["grid"]),
    ("td", &["gridcell"]),
    ("fieldset", &["radiogroup", "presentation"]),
];

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

const A11Y_NON_INTERACTIVE_ROLES: &[&str] = &[
    "alert",
    "alertdialog",
    "application",
    "article",
    "banner",
    "blockquote",
    "caption",
    "code",
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
    "group",
    "heading",
    "img",
    "insertion",
    "list",
    "listitem",
    "log",
    "main",
    "mark",
    "marquee",
    "math",
    "meter",
    "navigation",
    "note",
    "paragraph",
    "region",
    "rowgroup",
    "search",
    "separator",
    "status",
    "strong",
    "subscript",
    "superscript",
    "table",
    "term",
    "time",
    "timer",
    "tooltip",
    "doc-abstract",
    "doc-acknowledgments",
    "doc-afterword",
    "doc-appendix",
    "doc-biblioentry",
    "doc-bibliography",
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
    "doc-index",
    "doc-introduction",
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

const A11Y_RECOMMENDED_INTERACTIVE_HANDLERS: &[&str] = &[
    "click",
    "mousedown",
    "mouseup",
    "keypress",
    "keydown",
    "keyup",
];

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

#[derive(Clone, Copy, PartialEq, Eq)]
enum TransitionDirectiveKind {
    Transition,
    In,
    Out,
}

impl TransitionDirectiveKind {
    fn from_direction(direction: &TransitionDirection) -> Self {
        match direction {
            TransitionDirection::Both => Self::Transition,
            TransitionDirection::In => Self::In,
            TransitionDirection::Out => Self::Out,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Transition => "transition",
            Self::In => "in",
            Self::Out => "out",
        }
    }

    fn occupies_intro(self) -> bool {
        !matches!(self, Self::Out)
    }

    fn occupies_outro(self) -> bool {
        !matches!(self, Self::In)
    }
}

#[derive(Default)]
struct ElementEventState {
    has_s5_events: bool,
    has_animate_directive: bool,
    intro_transition: Option<TransitionDirectiveKind>,
    outro_transition: Option<TransitionDirectiveKind>,

    first_on_directive: Option<(Span, String)>,
}

pub(crate) struct TemplateValidationVisitor {
    current_expr_offset: u32,

    element_event_state: Vec<ElementEventState>,

    dialog_depth: u32,

    first_legacy_slot_span: Option<Span>,
    saw_render_tag: bool,
    emitted_slot_snippet_conflict: bool,
}

impl TemplateValidationVisitor {
    pub(crate) fn new() -> Self {
        Self {
            current_expr_offset: 0,
            element_event_state: Vec::new(),
            dialog_depth: 0,
            first_legacy_slot_span: None,
            saw_render_tag: false,
            emitted_slot_snippet_conflict: false,
        }
    }

    fn oxc_to_svelte(&self, span: oxc_span::Span) -> Span {
        Span::new(
            self.current_expr_offset + span.start,
            self.current_expr_offset + span.end,
        )
    }

    fn emit_mixed_syntax_if_needed(&mut self, ctx: &mut VisitContext<'_, '_>) {
        if let Some(state) = self.element_event_state.pop()
            && state.has_s5_events
            && let Some((span, name)) = state.first_on_directive
        {
            ctx.warnings_mut().push(Diagnostic::error(
                DiagnosticKind::MixedEventHandlerSyntaxes { name },
                span,
            ));
        }
    }

    fn maybe_warn_legacy_special_element(
        &mut self,
        name: &str,
        span: Span,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        if !ctx.runes {
            return;
        }

        if name == SVELTE_COMPONENT {
            ctx.warnings_mut().push(Diagnostic::warning(
                DiagnosticKind::SvelteComponentDeprecated,
                span,
            ));
        } else if name == SVELTE_SELF {
            let name = ctx.component_name().to_string();
            let basename = ctx.filename_basename().to_string();
            ctx.warnings_mut().push(Diagnostic::warning(
                DiagnosticKind::SvelteSelfDeprecated { name, basename },
                span,
            ));
        }
    }

    fn note_legacy_slot_element(&mut self, span: Span, ctx: &mut VisitContext<'_, '_>) {
        if ctx.data.output.custom_element {
            return;
        }

        if self.saw_render_tag {
            self.emit_slot_snippet_conflict(span, ctx);
            return;
        }

        self.first_legacy_slot_span.get_or_insert(span);
    }

    fn note_render_tag(&mut self, ctx: &mut VisitContext<'_, '_>) {
        self.saw_render_tag = true;

        if ctx.data.output.custom_element {
            return;
        }

        if let Some(span) = self.first_legacy_slot_span {
            self.emit_slot_snippet_conflict(span, ctx);
        }
    }

    fn emit_slot_snippet_conflict(&mut self, span: Span, ctx: &mut VisitContext<'_, '_>) {
        if self.emitted_slot_snippet_conflict {
            return;
        }

        ctx.warnings_mut()
            .push(Diagnostic::error(DiagnosticKind::SlotSnippetConflict, span));
        self.emitted_slot_snippet_conflict = true;
    }

    fn visit_special_element(
        &mut self,
        kind: SpecialElementKind,
        attributes: &[Attribute],
        fragment_id: svelte_ast::FragmentId,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        if let Some(span) = fragment_content_span(fragment_id, ctx) {
            ctx.warnings_mut().push(Diagnostic::error(
                DiagnosticKind::SvelteMetaInvalidContent {
                    name: kind.element_name().to_string(),
                },
                span,
            ));
        }

        for attr in attributes {
            if kind.allows_attribute(attr) {
                continue;
            }

            ctx.warnings_mut()
                .push(Diagnostic::error(kind.illegal_attr_kind(), attr.span()));
        }
    }
}

impl TemplateVisitor for TemplateValidationVisitor {
    fn visit_js_statement(
        &mut self,
        node_id: NodeId,
        stmt: &Statement<'_>,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        validate_const_tag_invalid_reference_stmt(node_id, stmt, self.current_expr_offset, ctx);
    }

    fn visit_snippet_block(&mut self, block: &SnippetBlock, ctx: &mut VisitContext<'_, '_>) {
        validate_snippet_rest_params(block, ctx);
        validate_snippet_shadowing_prop(block, ctx);
        validate_snippet_children_conflict(block, ctx);
    }

    fn visit_render_tag(&mut self, _tag: &RenderTag, ctx: &mut VisitContext<'_, '_>) {
        self.note_render_tag(ctx);
    }

    fn visit_const_tag(&mut self, tag: &ConstTag, ctx: &mut VisitContext<'_, '_>) {
        if let Some(parsed) = ctx.parsed()
            && let Some(oxc_ast::ast::Statement::VariableDeclaration(decl)) =
                parsed.stmt(tag.decl.id())
            && decl.declarations.len() > 1
        {
            ctx.warnings_mut().push(Diagnostic::error(
                DiagnosticKind::ConstTagInvalidExpression,
                tag.span,
            ));
        }

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

    fn visit_element(&mut self, el: &Element, ctx: &mut VisitContext<'_, '_>) {
        self.element_event_state.push(ElementEventState::default());
        self.maybe_warn_legacy_special_element(&el.name, el.span, ctx);

        if el.name == "dialog" {
            self.dialog_depth += 1;
        }

        let source = ctx.source;

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

        if el.name == "textarea"
            && has_value_attr
            && ctx.data.fragment_has_children_by_id(el.fragment)
        {
            ctx.warnings_mut().push(Diagnostic::error(
                DiagnosticKind::TextareaInvalidContent,
                el.span,
            ));
        }

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
        } else if has_slot && let Some(attr) = slot_attr {
            if !matches!(attr, Attribute::StringAttribute(_)) {
                ctx.warnings_mut().push(Diagnostic::error(
                    DiagnosticKind::SlotAttributeInvalid,
                    attr_value_span(attr),
                ));
            } else {
                validate_component_slot_conflicts(el, attr, ctx);
            }
        }

        a11y::check_element_warnings(
            el,
            &el.attributes,
            accesskey_attr,
            tabindex_attr,
            has_autofocus,
            self.dialog_depth,
            missing_attr_diag,
            ctx,
        );

        check_component_name_lowercase(el, ctx);
        check_plain_attr_warnings(el.id, el.span, &el.attributes, ctx);
        check_attribute_unquoted_sequence(&el.attributes, ctx);

        if el.name.contains('-') {
            check_attribute_quoted(&el.attributes, ctx);
        }

        let _ = has_spread;
    }

    fn visit_slot_element_legacy(
        &mut self,
        el: &SlotElementLegacy,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        self.element_event_state.push(ElementEventState::default());
        self.note_legacy_slot_element(el.span, ctx);

        if ctx.runes && !ctx.data.output.custom_element {
            ctx.warnings_mut().push(Diagnostic::warning(
                DiagnosticKind::SlotElementDeprecated,
                el.span,
            ));
        }

        for attr in &el.attributes {
            match attr {
                Attribute::StringAttribute(attr) if attr.name == "name" => {
                    if attr.value_span.source_text(ctx.source) == "default" {
                        ctx.warnings_mut().push(Diagnostic::error(
                            DiagnosticKind::SlotElementInvalidNameDefault,
                            attr.span,
                        ));
                    }
                }
                Attribute::ExpressionAttribute(attr) if attr.name == "name" => {
                    ctx.warnings_mut().push(Diagnostic::error(
                        DiagnosticKind::SlotElementInvalidName,
                        attr.span,
                    ));
                }
                Attribute::ConcatenationAttribute(attr) if attr.name == "name" => {
                    ctx.warnings_mut().push(Diagnostic::error(
                        DiagnosticKind::SlotElementInvalidName,
                        attr.span,
                    ));
                }
                Attribute::BooleanAttribute(attr) if attr.name == "name" => {
                    ctx.warnings_mut().push(Diagnostic::error(
                        DiagnosticKind::SlotElementInvalidName,
                        attr.span,
                    ));
                }
                Attribute::StringAttribute(_)
                | Attribute::ExpressionAttribute(_)
                | Attribute::ConcatenationAttribute(_)
                | Attribute::BooleanAttribute(_)
                | Attribute::SpreadAttribute(_)
                | Attribute::LetDirectiveLegacy(_) => {}
                _ => ctx.warnings_mut().push(Diagnostic::error(
                    DiagnosticKind::SlotElementInvalidAttribute,
                    attr.span(),
                )),
            }
        }
    }

    fn leave_element(&mut self, el: &Element, ctx: &mut VisitContext<'_, '_>) {
        if el.name == "dialog" {
            self.dialog_depth -= 1;
        }
        self.emit_mixed_syntax_if_needed(ctx);
    }

    fn leave_slot_element_legacy(
        &mut self,
        _el: &svelte_ast::SlotElementLegacy,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        self.emit_mixed_syntax_if_needed(ctx);
    }

    fn visit_component_node(&mut self, cn: &ComponentNode, ctx: &mut VisitContext<'_, '_>) {
        self.maybe_warn_legacy_special_element(&cn.name, cn.span, ctx);
        check_component_directives(&cn.attributes, ctx);
        check_component_attribute_warnings(&cn.attributes, ctx);
        check_attribute_unquoted_sequence(&cn.attributes, ctx);
        check_attribute_quoted(&cn.attributes, ctx);
    }

    fn visit_svelte_fragment_legacy(
        &mut self,
        el: &SvelteFragmentLegacy,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        let is_direct_child_of_component = ctx
            .parent()
            .is_some_and(|parent| parent.kind == ParentKind::ComponentNode);

        if !is_direct_child_of_component {
            ctx.warnings_mut().push(Diagnostic::error(
                DiagnosticKind::SvelteFragmentInvalidPlacement,
                el.span,
            ));
        }

        for attr in &el.attributes {
            match attr {
                Attribute::StringAttribute(attr) if attr.name == "slot" => {
                    if !is_direct_child_of_component {
                        ctx.warnings_mut().push(Diagnostic::error(
                            DiagnosticKind::SlotAttributeInvalidPlacement,
                            el.span,
                        ));
                    }
                }
                Attribute::ExpressionAttribute(attr) if attr.name == "slot" => {
                    if !is_direct_child_of_component {
                        ctx.warnings_mut().push(Diagnostic::error(
                            DiagnosticKind::SlotAttributeInvalidPlacement,
                            el.span,
                        ));
                    } else {
                        ctx.warnings_mut().push(Diagnostic::error(
                            DiagnosticKind::SlotAttributeInvalid,
                            attr.span,
                        ));
                    }
                }
                Attribute::ConcatenationAttribute(attr) if attr.name == "slot" => {
                    if !is_direct_child_of_component {
                        ctx.warnings_mut().push(Diagnostic::error(
                            DiagnosticKind::SlotAttributeInvalidPlacement,
                            el.span,
                        ));
                    } else {
                        ctx.warnings_mut().push(Diagnostic::error(
                            DiagnosticKind::SlotAttributeInvalid,
                            attr.span,
                        ));
                    }
                }
                Attribute::BooleanAttribute(attr) if attr.name == "slot" => {
                    if !is_direct_child_of_component {
                        ctx.warnings_mut().push(Diagnostic::error(
                            DiagnosticKind::SlotAttributeInvalidPlacement,
                            el.span,
                        ));
                    } else {
                        ctx.warnings_mut().push(Diagnostic::error(
                            DiagnosticKind::SlotAttributeInvalid,
                            attr.span,
                        ));
                    }
                }
                Attribute::StringAttribute(_)
                | Attribute::ExpressionAttribute(_)
                | Attribute::ConcatenationAttribute(_)
                | Attribute::BooleanAttribute(_)
                | Attribute::LetDirectiveLegacy(_) => {}
                _ => ctx.warnings_mut().push(Diagnostic::error(
                    DiagnosticKind::SvelteFragmentInvalidAttribute,
                    attr.span(),
                )),
            }
        }
    }

    fn visit_svelte_element(&mut self, el: &SvelteElement, ctx: &mut VisitContext<'_, '_>) {
        self.element_event_state.push(ElementEventState::default());
        check_plain_attr_warnings(el.id, el.span, &el.attributes, ctx);
        check_attribute_unquoted_sequence(&el.attributes, ctx);
    }

    fn visit_svelte_window(&mut self, window: &SvelteWindow, ctx: &mut VisitContext<'_, '_>) {
        self.visit_special_element(
            SpecialElementKind::Window,
            &window.attributes,
            window.fragment,
            ctx,
        );
    }

    fn visit_svelte_document(&mut self, document: &SvelteDocument, ctx: &mut VisitContext<'_, '_>) {
        self.visit_special_element(
            SpecialElementKind::Document,
            &document.attributes,
            document.fragment,
            ctx,
        );
    }

    fn visit_svelte_body(&mut self, body: &SvelteBody, ctx: &mut VisitContext<'_, '_>) {
        self.visit_special_element(
            SpecialElementKind::Body,
            &body.attributes,
            body.fragment,
            ctx,
        );
    }

    fn leave_svelte_element(&mut self, _el: &SvelteElement, ctx: &mut VisitContext<'_, '_>) {
        self.emit_mixed_syntax_if_needed(ctx);
    }

    fn visit_expression_attribute(
        &mut self,
        attr: &ExpressionAttribute,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        if attr.event_name.is_some()
            && let Some(Expression::Identifier(ident)) =
                ctx.parsed().and_then(|p| p.expr(attr.expression.id()))
            && ident.name.as_str() == attr.name.as_str()
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
                attr.expression.span,
            ));
        }

        if attr.event_name.is_some()
            && let Some(state) = self.element_event_state.last_mut()
        {
            state.has_s5_events = true;
        }
    }

    fn visit_bind_directive(&mut self, dir: &BindDirective, ctx: &mut VisitContext<'_, '_>) {
        if let Some(parent) = current_bind_parent(dir.id, ctx) {
            validate_bind_name_and_target(dir, &parent, ctx);
            validate_bind_parent_specifics(dir, &parent, ctx);
        }

        let is_identifier_target = if dir.shorthand {
            true
        } else {
            ctx.data
                .attr_expression(dir.id)
                .is_some_and(|info| info.is_identifier())
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
                    dir.expression.span,
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

    fn visit_let_directive_legacy(
        &mut self,
        dir: &LetDirectiveLegacy,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        let is_valid_parent = ctx.parent().is_some_and(|parent| {
            matches!(
                parent.kind,
                ParentKind::ComponentNode
                    | ParentKind::Element
                    | ParentKind::SlotElementLegacy
                    | ParentKind::SvelteElement
                    | ParentKind::SvelteFragmentLegacy
            )
        });

        if !is_valid_parent {
            ctx.warnings_mut().push(Diagnostic::error(
                DiagnosticKind::LetDirectiveInvalidPlacement,
                dir.span,
            ));
        }
    }

    fn visit_use_directive(&mut self, dir: &UseDirective, ctx: &mut VisitContext<'_, '_>) {
        let Some(expression_span) = dir.expression.as_ref().map(|r| r.span) else {
            return;
        };

        if ctx
            .data
            .attr_expression(dir.id)
            .is_some_and(|info| info.has_await())
        {
            ctx.warnings_mut().push(Diagnostic::error(
                DiagnosticKind::IllegalAwaitExpression,
                expression_span,
            ));
        }
    }

    fn visit_transition_directive(
        &mut self,
        dir: &TransitionDirective,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        let kind = TransitionDirectiveKind::from_direction(&dir.direction);

        if let Some(state) = self.element_event_state.last_mut() {
            let existing = if kind.occupies_intro() {
                state.intro_transition.or_else(|| {
                    kind.occupies_outro()
                        .then_some(state.outro_transition)
                        .flatten()
                })
            } else if kind.occupies_outro() {
                state.outro_transition
            } else {
                None
            };

            if let Some(existing) = existing {
                let diagnostic = if existing == kind {
                    DiagnosticKind::TransitionDuplicate {
                        type_: kind.label().to_string(),
                    }
                } else {
                    DiagnosticKind::TransitionConflict {
                        type_: kind.label().to_string(),
                        existing: existing.label().to_string(),
                    }
                };
                ctx.warnings_mut()
                    .push(Diagnostic::error(diagnostic, dir.name_ref.span));
            }

            if kind.occupies_intro() {
                state.intro_transition = Some(kind);
            }
            if kind.occupies_outro() {
                state.outro_transition = Some(kind);
            }
        }

        if let Some(expression_span) = dir.expression.as_ref().map(|r| r.span)
            && ctx
                .data
                .attr_expression(dir.id)
                .is_some_and(|info| info.has_await())
        {
            ctx.warnings_mut().push(Diagnostic::error(
                DiagnosticKind::IllegalAwaitExpression,
                expression_span,
            ));
        }
    }

    fn visit_on_directive_legacy(
        &mut self,
        dir: &OnDirectiveLegacy,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        let is_component = ctx
            .data
            .parent(dir.id)
            .is_some_and(|p| p.kind == ParentKind::ComponentNode);

        if !is_component {
            let list = EVENT_MODIFIERS.join(", ");
            for modifier in &dir.modifiers {
                if !EVENT_MODIFIERS.contains(&modifier.as_str()) {
                    ctx.warnings_mut().push(Diagnostic::error(
                        DiagnosticKind::EventHandlerInvalidModifier { list: list.clone() },
                        dir.name_span,
                    ));
                }
            }

            let flags = ctx.data.event_modifiers(dir.id);
            let has_passive = flags.contains(EventModifier::PASSIVE);
            let has_nonpassive = flags.contains(EventModifier::NONPASSIVE);
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

        if ctx.runes && !is_component {
            ctx.warnings_mut().push(Diagnostic::warning(
                DiagnosticKind::EventDirectiveDeprecated {
                    name: dir.name.clone(),
                },
                dir.name_span,
            ));
        }

        if !is_component && let Some(state) = self.element_event_state.last_mut() {
            state
                .first_on_directive
                .get_or_insert((dir.name_span, dir.name.clone()));
        }
    }

    fn visit_text(&mut self, text: &Text, ctx: &mut VisitContext<'_, '_>) {
        let value = text.value(ctx.source);

        if contains_non_whitespace_text(value)
            && let Some(message) = invalid_text_parent_message(text.id, ctx)
        {
            ctx.warnings_mut().push(Diagnostic::error(
                DiagnosticKind::NodeInvalidPlacement { message },
                text.span,
            ));
        }

        for (offset, ch) in value.char_indices() {
            if !is_bidi_control(ch) {
                continue;
            }
            if ctx
                .data
                .output
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

    fn visit_expression_tag(&mut self, tag: &ExpressionTag, ctx: &mut VisitContext<'_, '_>) {
        if let Some(message) = invalid_text_parent_message(tag.id, ctx) {
            ctx.warnings_mut().push(Diagnostic::error(
                DiagnosticKind::NodeInvalidPlacement { message },
                tag.span,
            ));
        }
    }

    fn visit_debug_tag(&mut self, tag: &DebugTag, ctx: &mut VisitContext<'_, '_>) {
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

    fn visit_key_block(&mut self, block: &KeyBlock, ctx: &mut VisitContext<'_, '_>) {
        check_empty_fragment(block.fragment, ctx);

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

    fn visit_if_block(&mut self, block: &IfBlock, ctx: &mut VisitContext<'_, '_>) {
        check_empty_fragment(block.consequent, ctx);
        if let Some(alt) = block.alternate {
            check_empty_fragment(alt, ctx);
        }

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

    fn visit_await_block(&mut self, block: &AwaitBlock, ctx: &mut VisitContext<'_, '_>) {
        if !ctx.runes {
            return;
        }
        for (span_opt, clause) in [
            (block.value.as_ref().map(|r| r.span), ":then"),
            (block.error.as_ref().map(|r| r.span), ":catch"),
        ] {
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

    fn visit_each_block(&mut self, block: &EachBlock, ctx: &mut VisitContext<'_, '_>) {
        if let Some(key_ref) = block.key.as_ref()
            && block.context.is_none()
        {
            ctx.warnings_mut().push(Diagnostic::error(
                DiagnosticKind::EachKeyWithoutAs,
                key_ref.span,
            ));
        }
    }

    fn visit_animate_directive(&mut self, dir: &AnimateDirective, ctx: &mut VisitContext<'_, '_>) {
        if let Some(state) = self.element_event_state.last_mut() {
            if state.has_animate_directive {
                ctx.warnings_mut().push(Diagnostic::error(
                    DiagnosticKind::AnimationDuplicate,
                    dir.name_ref.span,
                ));
            } else {
                state.has_animate_directive = true;
            }
        }

        if let Some(expression_span) = dir.expression.as_ref().map(|r| r.span)
            && ctx
                .data
                .attr_expression(dir.id)
                .is_some_and(|info| info.has_await())
        {
            ctx.warnings_mut().push(Diagnostic::error(
                DiagnosticKind::IllegalAwaitExpression,
                expression_span,
            ));
        }

        let (parent, grandparent) = {
            let parent = ctx.data.parent(dir.id);
            let mut ancestors = ctx.data.ancestors(dir.id);
            ancestors.next();
            (parent, ancestors.next())
        };

        let diag_kind = match grandparent.map(|p| p.kind) {
            Some(ParentKind::EachBlock) => {
                let each_id = grandparent
                    .expect("EachBlock parent implies grandparent is the each block")
                    .id;
                let each_block = ctx
                    .store
                    .get(each_id)
                    .as_each_block()
                    .expect("grandparent id resolved from ParentKind::EachBlock");
                if each_block.key.is_none() {
                    Some(DiagnosticKind::AnimationMissingKey)
                } else {
                    let _ = each_id;
                    let only_child = ctx
                        .data
                        .fragment_single_non_trivial_child_by_id(each_block.body);
                    if only_child != parent.map(|p| p.id) {
                        Some(DiagnosticKind::AnimationInvalidPlacement)
                    } else {
                        None
                    }
                }
            }
            _ => Some(DiagnosticKind::AnimationInvalidPlacement),
        };

        if let Some(kind) = diag_kind {
            ctx.warnings_mut()
                .push(Diagnostic::error(kind, dir.name_ref.span));
        }
    }

    fn visit_expression(&mut self, _id: NodeId, span: Span, _ctx: &mut VisitContext<'_, '_>) {
        self.current_expr_offset = span.start;
    }

    fn visit_statement(&mut self, _id: NodeId, span: Span, _ctx: &mut VisitContext<'_, '_>) {
        self.current_expr_offset = span.start;
    }

    fn visit_js_expression(
        &mut self,
        id: NodeId,
        expr: &Expression<'_>,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        validate_const_tag_invalid_reference_expr(id, expr, self.current_expr_offset, ctx);

        if !ctx.runes {
            return;
        }
        let is_bind = ctx
            .parent()
            .is_some_and(|p| p.kind == ParentKind::BindDirective);

        match expr {
            Expression::Identifier(ident) if is_bind && is_snippet_param_ref(ident, ctx.data) => {
                let span = self.oxc_to_svelte(expr.span());
                ctx.warnings_mut().push(Diagnostic::error(
                    DiagnosticKind::SnippetParameterAssignment,
                    span,
                ));
            }
            Expression::Identifier(ident) if is_bind && is_each_block_var_ref(ident, ctx.data) => {
                let span = self.oxc_to_svelte(expr.span());
                ctx.warnings_mut().push(Diagnostic::error(
                    DiagnosticKind::EachItemInvalidAssignment,
                    span,
                ));
            }
            _ if !is_bind && contains_invalid_each_assignment(expr, ctx.data) => {
                let span = self.oxc_to_svelte(expr.span());
                ctx.warnings_mut().push(Diagnostic::error(
                    DiagnosticKind::EachItemInvalidAssignment,
                    span,
                ));
            }
            _ if !is_bind && contains_invalid_snippet_param_assignment(expr, ctx.data) => {
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

fn current_bind_parent(bind_id: NodeId, ctx: &VisitContext<'_, '_>) -> Option<BindParentInfo> {
    let parent = ctx.data.parent(bind_id)?;
    match ctx.store.get(parent.id) {
        Node::Element(el) => Some(BindParentInfo {
            id: el.id,
            name: el.name.clone(),
            attrs: el.attributes.clone(),
        }),
        Node::ComponentNode(node) => Some(BindParentInfo {
            id: node.id,
            name: node.name.clone(),
            attrs: node.attributes.clone(),
        }),
        Node::SvelteElement(el) => Some(BindParentInfo {
            id: el.id,
            name: SVELTE_ELEMENT.to_string(),
            attrs: el.attributes.clone(),
        }),
        Node::SvelteWindow(el) => Some(BindParentInfo {
            id: el.id,
            name: SVELTE_WINDOW.to_string(),
            attrs: el.attributes.clone(),
        }),
        Node::SvelteDocument(el) => Some(BindParentInfo {
            id: el.id,
            name: SVELTE_DOCUMENT.to_string(),
            attrs: el.attributes.clone(),
        }),
        Node::SvelteBody(el) => Some(BindParentInfo {
            id: el.id,
            name: SVELTE_BODY.to_string(),
            attrs: el.attributes.clone(),
        }),
        _ => None,
    }
}

fn bind_expression_shape(
    dir: &BindDirective,
    ctx: &VisitContext<'_, '_>,
) -> Option<BindExpressionShape> {
    let parsed = ctx.parsed()?;
    let expr = parsed.expr(dir.expression.id())?;
    Some(classify_bind_expression(expr))
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

fn emit_bind_error(ctx: &mut VisitContext<'_, '_>, span: Span, kind: DiagnosticKind) {
    ctx.warnings_mut().push(Diagnostic::error(kind, span));
}

fn validate_bind_name_and_target(
    dir: &BindDirective,
    parent: &BindParentInfo,
    ctx: &mut VisitContext<'_, '_>,
) {
    let Some(bind_semantics) = ctx.data.bind_target_semantics(dir.id).copied() else {
        let explanation =
            fuzzymatch(dir.name.as_str(), BindPropertyKind::KNOWN_NAMES).and_then(|suggestion| {
                BindPropertyKind::from_host_and_name(BindHostKind::Element, suggestion)
                    .map(|property| property.validation_spec())
                    .is_some_and(|spec| spec.allows(&parent.name))
                    .then(|| format!("Did you mean '{suggestion}'?"))
            });

        emit_bind_error(
            ctx,
            dir.expression.span,
            DiagnosticKind::BindInvalidName {
                name: dir.name.clone(),
                explanation,
            },
        );
        return;
    };

    if bind_semantics.host() == BindHostKind::Component {
        return;
    }

    let validation = bind_semantics.validation_spec();

    if !validation.valid_elements().is_empty() && !validation.allows(&parent.name) {
        let elements = validation
            .valid_elements()
            .iter()
            .map(|name| format!("`<{name}>`"))
            .collect::<Vec<_>>()
            .join(", ");
        emit_bind_error(
            ctx,
            dir.expression.span,
            DiagnosticKind::BindInvalidTarget {
                name: dir.name.clone(),
                elements,
            },
        );
        return;
    }

    if validation
        .invalid_elements()
        .contains(&parent.name.as_str())
    {
        let mut valid_bindings = BindPropertyKind::KNOWN_NAMES
            .iter()
            .copied()
            .filter(|candidate| {
                BindPropertyKind::from_host_and_name(BindHostKind::Element, candidate)
                    .map(|property| property.validation_spec())
                    .is_some_and(|spec| spec.allows(&parent.name))
            })
            .collect::<Vec<_>>();
        valid_bindings.sort_unstable();

        emit_bind_error(
            ctx,
            dir.expression.span,
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
    ctx: &mut VisitContext<'_, '_>,
) {
    let bind_semantics = ctx.data.bind_target_semantics(dir.id).copied();
    let bind_property = bind_semantics.map(|semantics| semantics.property());

    if parent.name == "input" && bind_semantics.is_none_or(|semantics| !semantics.is_this()) {
        validate_input_bindings(dir, parent, ctx);
    }

    if parent.name == "select" && bind_semantics.is_none_or(|semantics| !semantics.is_this()) {
        let multiple = ctx.data.attribute(parent.id, &parent.attrs, "multiple");
        if let Some(a) = multiple
            && !attr_is_text(a)
            && !matches!(a, Attribute::BooleanAttribute(_))
        {
            ctx.warnings_mut().push(Diagnostic::error(
                DiagnosticKind::AttributeInvalidMultiple,
                attr_value_span(a),
            ));
            return;
        }
    }

    if matches!(
        bind_property,
        Some(BindPropertyKind::ElementSize(
            crate::types::data::ElementSizeKind::OffsetWidth
        ))
    ) && is_svg(&parent.name)
    {
        emit_bind_error(
            ctx,
            dir.expression.span,
            DiagnosticKind::BindInvalidTarget {
                name: dir.name.clone(),
                elements: "non-`<svg>` elements. Use `bind:clientWidth` for `<svg>` instead"
                    .to_string(),
            },
        );
        return;
    }

    if bind_semantics.is_some_and(|semantics| semantics.is_contenteditable()) {
        let contenteditable = ctx
            .data
            .attribute(parent.id, &parent.attrs, "contenteditable");
        match contenteditable {
            None => emit_bind_error(
                ctx,
                dir.expression.span,
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
    ctx: &mut VisitContext<'_, '_>,
) {
    let bind_property = ctx
        .data
        .bind_target_semantics(dir.id)
        .map(|semantics| semantics.property());
    let Some(type_attr) = ctx.data.attribute(parent.id, &parent.attrs, "type") else {
        return;
    };

    if !attr_is_text(type_attr) {
        if bind_property != Some(BindPropertyKind::Value)
            || matches!(type_attr, Attribute::BooleanAttribute(_))
        {
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
    if bind_property == Some(BindPropertyKind::Checked) && type_value != "checkbox" {
        let elements = if type_value == "radio" {
            "`<input type=\"checkbox\">` — for `<input type=\"radio\">`, use `bind:group`"
                .to_string()
        } else {
            "`<input type=\"checkbox\">`".to_string()
        };
        emit_bind_error(
            ctx,
            dir.expression.span,
            DiagnosticKind::BindInvalidTarget {
                name: dir.name.clone(),
                elements,
            },
        );
    } else if bind_property == Some(BindPropertyKind::Files) && type_value != "file" {
        emit_bind_error(
            ctx,
            dir.expression.span,
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
    ctx: &mut VisitContext<'_, '_>,
) {
    if ctx
        .data
        .bind_target_semantics(dir.id)
        .is_some_and(|semantics| semantics.is_group())
    {
        emit_bind_error(
            ctx,
            dir.expression.span,
            DiagnosticKind::BindGroupInvalidExpression,
        );
    }

    if has_parens {
        emit_bind_error(
            ctx,
            dir.expression.span,
            DiagnosticKind::BindInvalidParens {
                name: dir.name.clone(),
            },
        );
    }

    if len != 2 {
        emit_bind_error(
            ctx,
            dir.expression.span,
            DiagnosticKind::BindInvalidExpression,
        );
    }
}

fn validate_bind_identifier_value(dir: &BindDirective, ctx: &mut VisitContext<'_, '_>) {
    if ctx
        .data
        .bind_target_semantics(dir.id)
        .is_some_and(|semantics| !semantics.requires_mutable_target())
    {
        return;
    }

    let Some(sym_id) = bind_base_symbol(dir, ctx) else {
        return;
    };

    let decl = ctx.data.reactivity.binding_semantics(sym_id);
    let valid = matches!(
        decl,
        crate::BindingSemantics::State(_)
            | crate::BindingSemantics::Store(_)
            | crate::BindingSemantics::Prop(crate::PropBindingSemantics {
                kind: crate::PropBindingKind::Source { .. } | crate::PropBindingKind::NonSource,
                ..
            })
    ) || matches!(
        decl,
        crate::BindingSemantics::Contextual(crate::ContextualBindingSemantics::EachItem(_),),
    ) && bind_targets_each_context(sym_id, dir.id, ctx)
        || {
            let flags = ctx.data.scoping.symbol_flags(sym_id);
            flags.intersects(SymbolFlags::BlockScopedVariable | SymbolFlags::FunctionScopedVariable)
                && !flags.contains(SymbolFlags::ConstVariable)
        };

    if !valid {
        emit_bind_error(ctx, dir.expression.span, DiagnosticKind::BindInvalidValue);
    }
}

fn validate_bind_group_binding(dir: &BindDirective, ctx: &mut VisitContext<'_, '_>) {
    if !ctx
        .data
        .bind_target_semantics(dir.id)
        .is_some_and(|semantics| semantics.is_group())
    {
        return;
    }

    let Some(sym_id) = bind_base_symbol(dir, ctx) else {
        return;
    };

    if matches!(
        ctx.data.reactivity.binding_semantics(sym_id),
        crate::BindingSemantics::Contextual(crate::ContextualBindingSemantics::SnippetParam(_),),
    ) {
        emit_bind_error(
            ctx,
            dir.expression.span,
            DiagnosticKind::BindGroupInvalidSnippetParameter,
        );
        return;
    }

    if ctx.data.reactivity.is_each_rest(sym_id) {
        let name = ctx.data.scoping.symbol_name(sym_id).to_string();
        ctx.warnings_mut().push(Diagnostic::warning(
            DiagnosticKind::BindInvalidEachRest { name },
            dir.expression.span,
        ));
    }
}

fn bind_base_symbol(
    dir: &BindDirective,
    ctx: &VisitContext<'_, '_>,
) -> Option<crate::scope::SymbolId> {
    if dir.shorthand {
        return ctx.data.shorthand_symbol(dir.id);
    }

    let info = ctx.data.attr_expression(dir.id)?;
    info.is_identifier_or_member_expression()
        .then(|| info.ref_symbols().first().copied())
        .flatten()
}

fn bind_targets_each_context(
    sym_id: crate::scope::SymbolId,
    bind_id: NodeId,
    ctx: &VisitContext<'_, '_>,
) -> bool {
    let sym_scope = ctx.data.scoping.symbol_scope_id(sym_id);
    ctx.data
        .ancestors(bind_id)
        .filter(|parent| parent.kind == ParentKind::EachBlock)
        .any(|parent| {
            let Some(block) = ctx.store.get(parent.id).as_each_block() else {
                return false;
            };
            ctx.data.effective_fragment_scope(block.body, ctx.scope) == sym_scope
        })
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
        Attribute::ExpressionAttribute(attr) => attr.expression.span,
        Attribute::ConcatenationAttribute(attr) => attr
            .parts
            .iter()
            .filter_map(|part| match part {
                ConcatPart::Dynamic { expr, .. } => Some(expr.span),
                ConcatPart::Static(_) => None,
            })
            .reduce(|left, right| left.merge(&right))
            .unwrap_or(Span::new(0, 0)),
        Attribute::StringAttribute(attr) => attr.value_span,
        Attribute::BooleanAttribute(_) => Span::new(0, 0),
        Attribute::BindDirective(attr) => attr.expression.span,
        Attribute::LetDirectiveLegacy(attr) => attr
            .binding
            .as_ref()
            .map(|r| r.span)
            .unwrap_or(attr.name_span),
        Attribute::SpreadAttribute(attr) => attr.expression.span,
        Attribute::ClassDirective(attr) => attr.expression.span,
        Attribute::StyleDirective(attr) => match &attr.value {
            svelte_ast::StyleDirectiveValue::Expression => attr.expression.span,
            svelte_ast::StyleDirectiveValue::Concatenation(parts) => parts
                .iter()
                .filter_map(|part| match part {
                    ConcatPart::Dynamic { expr, .. } => Some(expr.span),
                    ConcatPart::Static(_) => None,
                })
                .reduce(|left, right| left.merge(&right))
                .unwrap_or(Span::new(0, 0)),
            svelte_ast::StyleDirectiveValue::String(_) => Span::new(0, 0),
        },
        Attribute::UseDirective(attr) => attr
            .expression
            .as_ref()
            .map(|r| r.span)
            .unwrap_or(attr.name_ref.span),
        Attribute::OnDirectiveLegacy(attr) => attr
            .expression
            .as_ref()
            .map(|r| r.span)
            .unwrap_or(attr.name_span),
        Attribute::TransitionDirective(attr) => attr
            .expression
            .as_ref()
            .map(|r| r.span)
            .unwrap_or(attr.name_ref.span),
        Attribute::AnimateDirective(attr) => attr
            .expression
            .as_ref()
            .map(|r| r.span)
            .unwrap_or(attr.name_ref.span),
        Attribute::AttachTag(attr) => attr.expression.span,
    }
}

#[derive(Clone, Copy)]
enum SpecialElementKind {
    Window,
    Document,
    Body,
}

impl SpecialElementKind {
    fn element_name(self) -> &'static str {
        match self {
            Self::Window => SVELTE_WINDOW,
            Self::Document => SVELTE_DOCUMENT,
            Self::Body => SVELTE_BODY,
        }
    }

    fn illegal_attr_kind(self) -> DiagnosticKind {
        match self {
            Self::Window | Self::Document => DiagnosticKind::IllegalElementAttribute {
                name: self.element_name().to_string(),
            },
            Self::Body => DiagnosticKind::SvelteBodyIllegalAttribute,
        }
    }

    fn allows_attribute(self, attr: &Attribute) -> bool {
        match attr {
            Attribute::ExpressionAttribute(attr) => attr.event_name.is_some(),
            Attribute::LetDirectiveLegacy(_) => true,
            Attribute::OnDirectiveLegacy(_) => true,
            Attribute::BindDirective(_) => matches!(self, Self::Window | Self::Document),
            Attribute::UseDirective(_) => matches!(self, Self::Body),
            Attribute::AttachTag(_) => matches!(self, Self::Document),
            _ => false,
        }
    }
}

fn fragment_content_span(
    fragment_id: svelte_ast::FragmentId,
    ctx: &VisitContext<'_, '_>,
) -> Option<Span> {
    let nodes = &ctx.store.fragment_nodes(fragment_id);
    let first = nodes.first()?;
    let last = nodes.last()?;
    Some(
        ctx.store
            .get(*first)
            .span()
            .merge(&ctx.store.get(*last).span()),
    )
}

fn validate_snippet_rest_params(block: &SnippetBlock, ctx: &mut VisitContext<'_, '_>) {
    let Some(parsed) = ctx.parsed() else {
        return;
    };
    let Some(stmt) = parsed.stmt(block.decl.id()) else {
        return;
    };
    let Some(params) = extract_arrow_params(stmt) else {
        return;
    };

    if params.rest.is_some() {
        ctx.warnings_mut().push(Diagnostic::error(
            DiagnosticKind::SnippetInvalidRestParameter,
            block.decl.span,
        ));
    }
}

fn validate_snippet_shadowing_prop(block: &SnippetBlock, ctx: &mut VisitContext<'_, '_>) {
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
            block.decl.span,
        ));
    }
}

fn validate_snippet_children_conflict(block: &SnippetBlock, ctx: &mut VisitContext<'_, '_>) {
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
    if component_has_implicit_default_children(component, Some(block.id), ctx).is_some() {
        ctx.warnings_mut().push(Diagnostic::error(
            DiagnosticKind::SnippetConflict,
            block.decl.span,
        ));
    }
}

fn validate_component_slot_conflicts(
    el: &Element,
    slot_attr: &Attribute,
    ctx: &mut VisitContext<'_, '_>,
) {
    let Some(parent) = ctx.data.parent(el.id) else {
        return;
    };
    if parent.kind != ParentKind::ComponentNode {
        return;
    }

    let Node::ComponentNode(component) = ctx.store.get(parent.id) else {
        return;
    };
    let Attribute::StringAttribute(slot_attr) = slot_attr else {
        return;
    };
    let slot_name = slot_attr.value_span.source_text(ctx.source);

    if has_prior_named_slot(component, el.id, slot_name, ctx) {
        ctx.warnings_mut().push(Diagnostic::error(
            DiagnosticKind::SlotAttributeDuplicate {
                name: slot_name.to_string(),
                component: component.name.clone(),
            },
            slot_attr.value_span,
        ));
    }

    if slot_name == "default"
        && let Some(conflict_span) =
            component_has_implicit_default_children(component, Some(el.id), ctx)
    {
        ctx.warnings_mut().push(Diagnostic::error(
            DiagnosticKind::SlotDefaultDuplicate,
            conflict_span,
        ));
    }
}

fn check_empty_fragment(fragment_id: svelte_ast::FragmentId, ctx: &mut VisitContext<'_, '_>) {
    if let Some(node_id) = ctx.data.fragment_single_child_by_id(fragment_id)
        && let Node::Text(text) = ctx.store.get(node_id)
        && text.value(ctx.source).trim().is_empty()
    {
        ctx.warnings_mut()
            .push(Diagnostic::warning(DiagnosticKind::BlockEmpty, text.span));
    }
}

fn element_has_slot_attr(parent: ParentRef, ctx: &VisitContext<'_, '_>) -> bool {
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

fn invalid_text_parent_message(id: NodeId, ctx: &VisitContext<'_, '_>) -> Option<String> {
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
    data: &AnalysisData,
) -> bool {
    ident
        .reference_id
        .get()
        .and_then(|ref_id| data.scoping.get_reference(ref_id).symbol_id())
        .is_some_and(|sym| {
            matches!(
                data.reactivity.binding_semantics(sym),
                crate::BindingSemantics::Contextual(
                    crate::ContextualBindingSemantics::EachItem(_)
                        | crate::ContextualBindingSemantics::EachIndex(_),
                ),
            )
        })
}

fn is_snippet_param_ref(
    ident: &oxc_ast::ast::IdentifierReference<'_>,
    data: &AnalysisData,
) -> bool {
    ident
        .reference_id
        .get()
        .and_then(|ref_id| data.scoping.get_reference(ref_id).symbol_id())
        .is_some_and(|sym| {
            matches!(
                data.reactivity.binding_semantics(sym),
                crate::BindingSemantics::Contextual(
                    crate::ContextualBindingSemantics::SnippetParam(_),
                ),
            )
        })
}

fn validate_const_tag_invalid_reference_expr(
    node_id: NodeId,
    expr: &Expression<'_>,
    source_offset: u32,
    ctx: &mut VisitContext<'_, '_>,
) {
    if !ctx.data.script.experimental_async {
        return;
    }

    let diagnostics = {
        let mut visitor = ConstTagInvalidReferenceVisitor::new(node_id, source_offset, &*ctx);
        visitor.visit_expression(expr);
        visitor.diagnostics
    };
    ctx.warnings_mut().extend(diagnostics);
}

fn validate_const_tag_invalid_reference_stmt(
    node_id: NodeId,
    stmt: &Statement<'_>,
    source_offset: u32,
    ctx: &mut VisitContext<'_, '_>,
) {
    if !ctx.data.script.experimental_async {
        return;
    }

    let diagnostics = {
        let mut visitor = ConstTagInvalidReferenceVisitor::new(node_id, source_offset, &*ctx);
        visitor.visit_statement(stmt);
        visitor.diagnostics
    };
    ctx.warnings_mut().extend(diagnostics);
}

fn maybe_const_tag_invalid_reference(
    node_id: NodeId,
    source_offset: u32,
    ident: &IdentifierReference<'_>,
    ctx: &VisitContext<'_, '_>,
) -> Option<Diagnostic> {
    let sym_id = ident
        .reference_id
        .get()
        .and_then(|ref_id| ctx.data.scoping.get_reference(ref_id).symbol_id())?;

    if !matches!(
        ctx.data.binding_semantics(sym_id),
        crate::types::data::BindingSemantics::Const(
            crate::types::data::ConstBindingSemantics::ConstTag { .. }
        )
    ) {
        return None;
    }

    let binding_scope = ctx.data.scoping.symbol_scope_id(sym_id);
    let mut snippet_name = None;
    for parent in ctx.data.ancestors(node_id) {
        if parent.kind == ParentKind::SnippetBlock {
            let Node::SnippetBlock(block) = ctx.store.get(parent.id) else {
                continue;
            };
            snippet_name = Some(block.name(ctx.source).to_string());
            continue;
        }

        let Some(snippet_name) = snippet_name.as_deref() else {
            continue;
        };

        match parent.kind {
            ParentKind::ComponentNode => {
                let Node::ComponentNode(cn) = ctx.store.get(parent.id) else {
                    break;
                };
                let component_scope = ctx.data.scoping.fragment_scope_by_id(cn.fragment);
                if component_scope == Some(binding_scope) {
                    return Some(Diagnostic::error(
                        DiagnosticKind::ConstTagInvalidReference {
                            name: ident.name.to_string(),
                        },
                        Span::new(
                            source_offset + ident.span.start,
                            source_offset + ident.span.end,
                        ),
                    ));
                }
                break;
            }
            ParentKind::SvelteBoundary if matches!(snippet_name, "failed" | "pending") => {
                let Node::SvelteBoundary(b) = ctx.store.get(parent.id) else {
                    break;
                };
                let boundary_scope = ctx.data.scoping.fragment_scope_by_id(b.fragment);
                if boundary_scope == Some(binding_scope) {
                    return Some(Diagnostic::error(
                        DiagnosticKind::ConstTagInvalidReference {
                            name: ident.name.to_string(),
                        },
                        Span::new(
                            source_offset + ident.span.start,
                            source_offset + ident.span.end,
                        ),
                    ));
                }
                break;
            }
            _ => {}
        }
    }

    None
}

struct ConstTagInvalidReferenceVisitor<'c, 'a> {
    node_id: NodeId,
    source_offset: u32,
    ctx: &'c VisitContext<'c, 'a>,
    diagnostics: Vec<Diagnostic>,
}

impl<'c, 'a> ConstTagInvalidReferenceVisitor<'c, 'a> {
    fn new(node_id: NodeId, source_offset: u32, ctx: &'c VisitContext<'c, 'a>) -> Self {
        Self {
            node_id,
            source_offset,
            ctx,
            diagnostics: Vec::new(),
        }
    }
}

impl<'a> Visit<'a> for ConstTagInvalidReferenceVisitor<'_, '_> {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        if let Some(diag) =
            maybe_const_tag_invalid_reference(self.node_id, self.source_offset, ident, self.ctx)
        {
            self.diagnostics.push(diag);
        }
    }
}

fn named_component_attr(attr: &Attribute, name: &str) -> bool {
    match attr {
        Attribute::StringAttribute(attr) => attr.name == name,
        Attribute::ExpressionAttribute(attr) => attr.name == name,
        Attribute::BooleanAttribute(attr) => attr.name == name,
        Attribute::ConcatenationAttribute(attr) => attr.name == name,
        Attribute::BindDirective(attr) => attr.name == name,
        Attribute::SpreadAttribute(_)
        | Attribute::ClassDirective(_)
        | Attribute::LetDirectiveLegacy(_)
        | Attribute::StyleDirective(_)
        | Attribute::UseDirective(_)
        | Attribute::OnDirectiveLegacy(_)
        | Attribute::TransitionDirective(_)
        | Attribute::AnimateDirective(_)
        | Attribute::AttachTag(_) => false,
    }
}

fn has_prior_named_slot(
    component: &ComponentNode,
    current_child_id: NodeId,
    slot_name: &str,
    ctx: &VisitContext<'_, '_>,
) -> bool {
    let Some(slot) = component.legacy_slots.iter().find(|s| s.name == slot_name) else {
        return false;
    };
    let nodes = &ctx.store.fragment_nodes(slot.fragment);
    nodes.first().copied() != Some(current_child_id) && nodes.contains(&current_child_id)
}

fn component_has_implicit_default_children(
    component: &ComponentNode,
    excluded_child_id: Option<NodeId>,
    ctx: &VisitContext<'_, '_>,
) -> Option<Span> {
    let cn_fragment_nodes = ctx.store.fragment_nodes(component.fragment).to_vec();
    for child_id in cn_fragment_nodes {
        if Some(child_id) == excluded_child_id {
            continue;
        }

        let child = ctx.store.get(child_id);
        match child {
            Node::Text(text) if text.value(ctx.source).trim().is_empty() => continue,
            Node::Comment(_) => continue,
            _ => {}
        }

        return Some(child.span());
    }

    for slot in &component.legacy_slots {
        let slot_nodes = ctx.store.fragment_nodes(slot.fragment).to_vec();
        for wrapper_id in slot_nodes {
            if Some(wrapper_id) == excluded_child_id {
                continue;
            }
            let child = ctx.store.get(wrapper_id);
            if matches!(child, Node::Element(_) | Node::SvelteFragmentLegacy(_)) {
                continue;
            }
            return Some(child.span());
        }
    }

    None
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

struct InvalidEachAssignmentVisitor<'s> {
    data: &'s AnalysisData<'s>,
    found: bool,
}

impl<'a> Visit<'a> for InvalidEachAssignmentVisitor<'_> {
    fn visit_assignment_expression(&mut self, expr: &oxc_ast::ast::AssignmentExpression<'a>) {
        if let AssignmentTarget::AssignmentTargetIdentifier(id) = &expr.left
            && is_each_block_var_ref(id, self.data)
        {
            self.found = true;
            return;
        }
        walk::walk_assignment_expression(self, expr);
    }

    fn visit_update_expression(&mut self, expr: &oxc_ast::ast::UpdateExpression<'a>) {
        if let SimpleAssignmentTarget::AssignmentTargetIdentifier(id) = &expr.argument
            && is_each_block_var_ref(id, self.data)
        {
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

fn contains_invalid_each_assignment(expr: &Expression<'_>, data: &AnalysisData<'_>) -> bool {
    let mut visitor = InvalidEachAssignmentVisitor { data, found: false };
    visitor.visit_expression(expr);
    visitor.found
}

struct InvalidSnippetParamAssignmentVisitor<'s> {
    data: &'s AnalysisData<'s>,
    found: bool,
}

impl<'a> Visit<'a> for InvalidSnippetParamAssignmentVisitor<'_> {
    fn visit_assignment_expression(&mut self, expr: &oxc_ast::ast::AssignmentExpression<'a>) {
        if let AssignmentTarget::AssignmentTargetIdentifier(id) = &expr.left
            && is_snippet_param_ref(id, self.data)
        {
            self.found = true;
            return;
        }
        walk::walk_assignment_expression(self, expr);
    }

    fn visit_update_expression(&mut self, expr: &oxc_ast::ast::UpdateExpression<'a>) {
        if let SimpleAssignmentTarget::AssignmentTargetIdentifier(id) = &expr.argument
            && is_snippet_param_ref(id, self.data)
        {
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

fn contains_invalid_snippet_param_assignment(expr: &Expression<'_>, data: &AnalysisData) -> bool {
    let mut visitor = InvalidSnippetParamAssignmentVisitor { data, found: false };
    visitor.visit_expression(expr);
    visitor.found
}

fn check_plain_attr_warnings(
    id: NodeId,
    span: Span,
    attrs: &[Attribute],
    ctx: &mut VisitContext<'_, '_>,
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

fn check_component_directives(attrs: &[Attribute], ctx: &mut VisitContext<'_, '_>) {
    for attr in attrs {
        match attr {
            Attribute::StringAttribute(_)
            | Attribute::ExpressionAttribute(_)
            | Attribute::BooleanAttribute(_)
            | Attribute::ConcatenationAttribute(_)
            | Attribute::SpreadAttribute(_)
            | Attribute::BindDirective(_)
            | Attribute::LetDirectiveLegacy(_)
            | Attribute::AttachTag(_) => {}
            Attribute::OnDirectiveLegacy(dir) => {
                let has_only_once = dir.modifiers.len() == 1
                    && ctx
                        .data
                        .event_modifiers(dir.id)
                        .contains(EventModifier::ONCE);
                if !dir.modifiers.is_empty() && !has_only_once {
                    ctx.warnings_mut().push(Diagnostic::error(
                        DiagnosticKind::EventHandlerInvalidComponentModifier,
                        dir.name_span,
                    ));
                }
            }
            Attribute::ClassDirective(_)
            | Attribute::StyleDirective(_)
            | Attribute::UseDirective(_)
            | Attribute::TransitionDirective(_)
            | Attribute::AnimateDirective(_) => {
                ctx.warnings_mut().push(Diagnostic::error(
                    DiagnosticKind::ComponentInvalidDirective,
                    attr_value_span(attr),
                ));
            }
        }
    }
}

fn check_component_attribute_warnings(attrs: &[Attribute], ctx: &mut VisitContext<'_, '_>) {
    let has_illegal_colon = attrs.iter().any(|attr| {
        let name = attr.html_name();
        name.contains(':')
            && !name.starts_with("xml:")
            && !name.starts_with("xlink:")
            && !name.starts_with("xmlns:")
    });

    if has_illegal_colon {
        let span = attrs
            .iter()
            .find(|attr| {
                let name = attr.html_name();
                name.contains(':')
                    && !name.starts_with("xml:")
                    && !name.starts_with("xlink:")
                    && !name.starts_with("xmlns:")
            })
            .map(attr_value_span)
            .unwrap_or(Span::new(0, 0));
        ctx.warnings_mut().push(Diagnostic::warning(
            DiagnosticKind::AttributeIllegalColon,
            span,
        ));
    }
}

fn check_attribute_unquoted_sequence(attrs: &[Attribute], ctx: &mut VisitContext<'_, '_>) {
    for attr in attrs {
        let Attribute::ConcatenationAttribute(concat) = attr else {
            continue;
        };

        if !concat.quoted && concat.parts.len() > 1 {
            ctx.warnings_mut().push(Diagnostic::error(
                DiagnosticKind::AttributeUnquotedSequence,
                attr_value_span(attr),
            ));
        }
    }
}

fn check_component_name_lowercase(el: &Element, ctx: &mut VisitContext<'_, '_>) {
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

fn check_attribute_quoted(attrs: &[Attribute], ctx: &mut VisitContext<'_, '_>) {
    if !ctx.runes {
        return;
    }
    for attr in attrs {
        if let Attribute::ConcatenationAttribute(ca) = attr
            && ca.parts.len() == 1
            && matches!(ca.parts[0], ConcatPart::Dynamic { .. })
        {
            ctx.warnings_mut().push(Diagnostic::warning(
                DiagnosticKind::AttributeQuoted,
                attr_value_span(attr),
            ));
        }
    }
}

fn check_a11y_missing_attribute(
    el: &Element,
    data: &AnalysisData,
    source: &str,
) -> Option<Diagnostic> {
    match el.name.as_str() {
        "img" => (!data.has_attribute(el.id, "alt")).then(|| warn_missing_attr(el, &["alt"])),

        "area" => (!data.has_attribute(el.id, "alt")
            && !data.has_attribute(el.id, "aria-label")
            && !data.has_attribute(el.id, "aria-labelledby"))
        .then(|| warn_missing_attr(el, &["alt", "aria-label", "aria-labelledby"])),

        "iframe" => {
            (!data.has_attribute(el.id, "title")).then(|| warn_missing_attr(el, &["title"]))
        }

        "object" => (!data.has_attribute(el.id, "title")
            && !data.has_attribute(el.id, "aria-label")
            && !data.has_attribute(el.id, "aria-labelledby"))
        .then(|| warn_missing_attr(el, &["title", "aria-label", "aria-labelledby"])),

        "a" => {
            if data.has_attribute(el.id, "href") || data.has_attribute(el.id, "xlink:href") {
                return None;
            }

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

fn warn_missing_attr(el: &Element, attrs: &[&str]) -> Diagnostic {
    let article = attrs
        .first()
        .copied()
        .map(|attr| match attr.chars().next() {
            _ if attr == "href" => "an",
            Some('a' | 'e' | 'i' | 'o' | 'u') => "an",
            _ => "a",
        })
        .unwrap_or("a")
        .to_string();
    let sequence = match attrs {
        [] => String::new(),
        [single] => (*single).to_string(),
        [left, right] => format!("{left} or {right}"),
        _ => {
            let last = attrs.last().copied().unwrap_or_default();
            let leading = &attrs[..attrs.len() - 1];
            format!("{} or {}", leading.join(", "), last)
        }
    };

    Diagnostic::warning(
        DiagnosticKind::A11yMissingAttribute {
            name: el.name.clone(),
            article,
            sequence,
        },
        el.span,
    )
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
        assert!(!has_whitespace_before_clause(" expr then ", ":then"));
    }
}
