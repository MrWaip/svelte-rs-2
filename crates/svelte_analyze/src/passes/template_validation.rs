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
    AnimateDirective, EachBlock, Element, ExpressionAttribute, ExpressionTag, Node, NodeId,
    OnDirectiveLegacy, SvelteElement, Text,
};
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::scope::ComponentScoping;
use crate::walker::{ParentKind, TemplateVisitor, VisitContext};

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
}

impl TemplateValidationVisitor {
    pub(crate) fn new() -> Self {
        Self {
            current_expr_offset: 0,
            element_event_state: Vec::new(),
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
}

impl TemplateVisitor for TemplateValidationVisitor {
    fn visit_element(&mut self, _el: &Element, _ctx: &mut VisitContext<'_>) {
        self.element_event_state.push(ElementEventState::default());
    }

    fn leave_element(&mut self, _el: &Element, ctx: &mut VisitContext<'_>) {
        self.emit_mixed_syntax_if_needed(ctx);
    }

    fn visit_svelte_element(&mut self, _el: &SvelteElement, _ctx: &mut VisitContext<'_>) {
        self.element_event_state.push(ElementEventState::default());
    }

    fn leave_svelte_element(&mut self, _el: &SvelteElement, ctx: &mut VisitContext<'_>) {
        self.emit_mixed_syntax_if_needed(ctx);
    }

    fn visit_expression_attribute(
        &mut self,
        attr: &ExpressionAttribute,
        _ctx: &mut VisitContext<'_>,
    ) {
        if attr.event_name.is_some() {
            if let Some(state) = self.element_event_state.last_mut() {
                state.has_s5_events = true;
            }
        }
    }

    // Use cases: event_handler_invalid_modifier, event_handler_invalid_modifier_combination,
    // event_directive_deprecated, mixed_event_handler_syntaxes
    fn visit_on_directive_legacy(
        &mut self,
        dir: &OnDirectiveLegacy,
        ctx: &mut VisitContext<'_>,
    ) {
        let is_component =
            ctx.parent().is_some_and(|p| p.kind == ParentKind::ComponentNode);

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
                DiagnosticKind::EventDirectiveDeprecated { name: dir.name.clone() },
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
            if let Some(message) = invalid_text_parent_message(ctx) {
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
        if let Some(message) = invalid_text_parent_message(ctx) {
            ctx.warnings_mut().push(Diagnostic::error(
                DiagnosticKind::NodeInvalidPlacement { message },
                tag.span,
            ));
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
        let grandparent = {
            let mut ancestors = ctx.ancestors();
            ancestors.next(); // skip Element (direct attr parent)
            ancestors.next().copied()
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
                        let non_trivial = each_block
                            .body
                            .nodes
                            .iter()
                            .filter(|&&nid| !is_trivial_node(ctx.store.get(nid), ctx.source))
                            .count();
                        if non_trivial > 1 {
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
            _ => {}
        }
    }
}

/// Trivial nodes are invisible non-content nodes that don't count as "children"
/// for the purpose of animate placement validation.
fn is_trivial_node(node: &Node, source: &str) -> bool {
    match node {
        Node::Comment(_) | Node::ConstTag(_) => true,
        Node::Text(t) => source[t.span.start as usize..t.span.end as usize]
            .trim()
            .is_empty(),
        _ => false,
    }
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

fn invalid_text_parent_message(ctx: &VisitContext<'_>) -> Option<String> {
    let parent = ctx
        .ancestors()
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
