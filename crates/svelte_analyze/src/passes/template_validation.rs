//! Template structure validation — structural diagnostics that depend on the resolved
//! AST shape (key presence, animate placement, each-block assignments).
//!
//! Runs after `CollectSymbols` so that reference IDs are resolved.
//!
//! OXC expression spans are 0-based relative to the expression text snippet.
//! `current_expr_offset` tracks the source-absolute start of the current expression
//! so that sub-expression spans can be reported correctly.

use oxc_ast::ast::{AssignmentTarget, Expression, SimpleAssignmentTarget};
use svelte_ast::{AnimateDirective, EachBlock, Node, NodeId};
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::scope::ComponentScoping;
use crate::walker::{ParentKind, TemplateVisitor, VisitContext};

pub(crate) struct TemplateValidationVisitor {
    /// Source-absolute start of the current expression being visited.
    /// Set by `visit_expression`, used in `visit_js_expression` to offset OXC sub-spans.
    current_expr_offset: u32,
}

impl TemplateValidationVisitor {
    pub(crate) fn new() -> Self {
        Self { current_expr_offset: 0 }
    }

    fn oxc_to_svelte(&self, span: oxc_span::Span) -> Span {
        Span::new(
            self.current_expr_offset + span.start,
            self.current_expr_offset + span.end,
        )
    }
}

impl TemplateVisitor for TemplateValidationVisitor {
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
                        let non_trivial = each_block.body.nodes.iter().filter(|&&nid| {
                            !is_trivial_node(ctx.store.get(nid), ctx.source)
                        }).count();
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
    fn visit_js_expression(&mut self, _id: NodeId, expr: &Expression<'_>, ctx: &mut VisitContext<'_>) {
        if !ctx.runes {
            return;
        }
        let is_bind = ctx.parent().is_some_and(|p| p.kind == ParentKind::BindDirective);

        match expr {
            Expression::AssignmentExpression(assign) if !is_bind => {
                if let AssignmentTarget::AssignmentTargetIdentifier(ident) = &assign.left {
                    if is_each_block_var_ref(ident, &ctx.data.scoping) {
                        let span = self.oxc_to_svelte(assign.span);
                        ctx.warnings_mut().push(Diagnostic::error(
                            DiagnosticKind::EachItemInvalidAssignment,
                            span,
                        ));
                    }
                }
            }
            Expression::UpdateExpression(update) if !is_bind => {
                if let SimpleAssignmentTarget::AssignmentTargetIdentifier(ident) = &update.argument {
                    if is_each_block_var_ref(ident, &ctx.data.scoping) {
                        let span = self.oxc_to_svelte(update.span);
                        ctx.warnings_mut().push(Diagnostic::error(
                            DiagnosticKind::EachItemInvalidAssignment,
                            span,
                        ));
                    }
                }
            }
            Expression::Identifier(ident) if is_bind => {
                if let Some(ref_id) = ident.reference_id.get() {
                    if let Some(sym) = ctx.data.scoping.get_reference(ref_id).symbol_id() {
                        if ctx.data.scoping.is_each_block_var(sym) {
                            let span = self.oxc_to_svelte(ident.span);
                            ctx.warnings_mut().push(Diagnostic::error(
                                DiagnosticKind::EachItemInvalidAssignment,
                                span,
                            ));
                        }
                    }
                }
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
        Node::Text(t) => source[t.span.start as usize..t.span.end as usize].trim().is_empty(),
        _ => false,
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
