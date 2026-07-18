#![allow(clippy::ptr_arg)]

use svelte_ast::{
    AwaitBlock, ComponentNode, EachBlock, Element, ExprRef, FragmentRole, IfBlock, KeyBlock, Node,
    NodeId, SVELTE_COMPONENT, SVELTE_SELF, SnippetBlock, StmtRef, SvelteComponentLegacy,
    SvelteSelf,
};
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::scanner::{self, token};
use crate::{
    AwaitBlockEntry, AwaitPhase, EachBlockEntry, ElementEntry, IfBlockEntry, KeyBlockEntry, Parser,
    SnippetBlockEntry, StackEntry, is_component_name, pop_children, push_child,
};

pub(crate) enum CloseReason {
    Eof,
    ClosingTagOverrun {
        closing_start: u32,
        tag_name: String,
    },
    ImplicitOpen {
        following_tag: String,
        following_start: u32,
    },
}

impl<'a> Parser<'a> {
    fn finish_element(&mut self, el: ElementEntry, children: Vec<NodeId>, span: Span) -> Node {
        if el.name == SVELTE_COMPONENT {
            let (default_children, legacy_slots) = self.partition_component_children(children);
            let fragment = self.new_fragment(FragmentRole::ComponentChildren, default_children);
            Node::SvelteComponentLegacy(SvelteComponentLegacy {
                id: NodeId(0),
                span,
                self_closing: false,
                attributes: el.attributes,
                fragment,
                legacy_slots,
            })
        } else if el.name == SVELTE_SELF {
            let (default_children, legacy_slots) = self.partition_component_children(children);
            let fragment = self.new_fragment(FragmentRole::ComponentChildren, default_children);
            Node::SvelteSelf(SvelteSelf {
                id: NodeId(0),
                span,
                self_closing: false,
                attributes: el.attributes,
                fragment,
                legacy_slots,
            })
        } else if is_component_name(&el.name) {
            let (default_children, legacy_slots) = self.partition_component_children(children);
            let fragment = self.new_fragment(FragmentRole::ComponentChildren, default_children);
            Node::ComponentNode(ComponentNode {
                id: NodeId(0),
                span,
                name: ExprRef::new(el.name_span),
                self_closing: false,
                attributes: el.attributes,
                fragment,
                legacy_slots,
            })
        } else {
            let fragment = self.new_fragment(FragmentRole::Element, children);
            Node::Element(Element {
                id: NodeId(0),
                span,
                name: el.name,
                self_closing: false,
                attributes: el.attributes,
                fragment,
            })
        }
    }

    fn finish_each(&mut self, eb: EachBlockEntry, last_children: Vec<NodeId>, span: Span) -> Node {
        let (body_children, fallback) = if eb.in_fallback {
            let body = eb.body_children.unwrap_or_default();
            let fb = self.new_fragment(FragmentRole::EachFallback, last_children);
            (body, Some(fb))
        } else {
            (last_children, None)
        };

        let body = self.new_fragment(FragmentRole::EachBody, body_children);
        let key_id = eb.key_span.map(|_| self.reserve_id());
        Node::EachBlock(EachBlock {
            id: NodeId(0),
            span,
            expression: ExprRef::new(eb.expression_span),
            context: eb.context_span.map(StmtRef::new),
            index: eb.index_span.map(StmtRef::new),
            key: eb.key_span.map(ExprRef::new),
            key_id,
            body,
            fallback,
        })
    }

    fn finish_await(
        &mut self,
        ab: AwaitBlockEntry,
        current_children: Vec<NodeId>,
        span: Span,
    ) -> Node {
        let (pending, then, catch) = match ab.phase {
            AwaitPhase::Pending => {
                let p = self.new_fragment(FragmentRole::AwaitPending, current_children);
                (Some(p), None, None)
            }
            AwaitPhase::Then => {
                let pending = ab
                    .pending_children
                    .map(|c| self.new_fragment(FragmentRole::AwaitPending, c));
                let then = self.new_fragment(FragmentRole::AwaitThen, current_children);
                let catch = ab
                    .catch_children
                    .map(|c| self.new_fragment(FragmentRole::AwaitCatch, c));
                (pending, Some(then), catch)
            }
            AwaitPhase::Catch => {
                let pending = ab
                    .pending_children
                    .map(|c| self.new_fragment(FragmentRole::AwaitPending, c));
                let then = ab
                    .then_children
                    .map(|c| self.new_fragment(FragmentRole::AwaitThen, c));
                let catch = self.new_fragment(FragmentRole::AwaitCatch, current_children);
                (pending, then, Some(catch))
            }
        };

        Node::AwaitBlock(AwaitBlock {
            id: NodeId(0),
            span,
            expression: ExprRef::new(ab.expression_span),
            value: ab.value_span.map(StmtRef::new),
            error: ab.error_span.map(StmtRef::new),
            pending,
            then,
            catch,
        })
    }

    fn finish_if(&mut self, ib: IfBlockEntry, last_children: Vec<NodeId>, span: Span) -> Node {
        let (consequent, alternate) = if let Some(cons) = ib.consequent {
            let alt = self.new_fragment(FragmentRole::IfAlternate, last_children);
            (cons, Some(alt))
        } else {
            (last_children, None)
        };

        let consequent_fragment = self.new_fragment(FragmentRole::IfConsequent, consequent);
        Node::IfBlock(IfBlock {
            id: NodeId(0),
            span,
            test: ExprRef::new(ib.test_span),
            elseif: ib.elseif,
            consequent: consequent_fragment,
            alternate,
        })
    }

    fn finish_key(&mut self, kb: KeyBlockEntry, body_children: Vec<NodeId>, span: Span) -> Node {
        let fragment = self.new_fragment(FragmentRole::KeyBlockBody, body_children);
        Node::KeyBlock(KeyBlock {
            id: NodeId(0),
            span,
            expression: ExprRef::new(kb.expression_span),
            fragment,
        })
    }

    fn finish_snippet(
        &mut self,
        sb: SnippetBlockEntry,
        body_children: Vec<NodeId>,
        span: Span,
    ) -> Node {
        let body = self.new_fragment(FragmentRole::SnippetBody, body_children);
        Node::SnippetBlock(SnippetBlock {
            id: NodeId(0),
            span,
            decl: StmtRef::new(sb.expression_span),
            body,
        })
    }

    pub(crate) fn handle_end_tag(
        &mut self,
        tag: &token::EndTag,
        span: Span,
        entry_stack: &mut Vec<StackEntry>,
        children_stack: &mut Vec<Vec<NodeId>>,
    ) {
        let tag_name = tag.name_span.source_text(self.source);

        if scanner::is_void(tag_name) {
            self.recover(Diagnostic::void_element_invalid_content(Span::new(
                span.start, span.start,
            )));
            let id = self.push_node(Node::Error(svelte_ast::ErrorNode {
                id: NodeId(0),
                span,
            }));
            push_child(children_stack, id);
            return;
        }

        let match_idx = entry_stack
            .iter()
            .rposition(|e| matches!(e, StackEntry::Element(el) if el.name == tag_name));

        match match_idx {
            None => {
                let autoclosed_reason = self
                    .last_auto_closed_tag
                    .as_ref()
                    .filter(|a| a.tag == tag_name)
                    .map(|a| a.reason.clone());
                if let Some(reason) = autoclosed_reason {
                    self.recover(Diagnostic::error(
                        DiagnosticKind::ElementInvalidClosingTagAutoclosed {
                            name: tag_name.to_string(),
                            reason,
                        },
                        Span::new(span.start, span.start),
                    ));
                } else {
                    self.recover(Diagnostic::element_invalid_closing_tag(
                        Span::new(span.start, span.start),
                        tag_name.to_string(),
                    ));
                }
                let id = self.push_node(Node::Error(svelte_ast::ErrorNode {
                    id: NodeId(0),
                    span,
                }));
                push_child(children_stack, id);
            }
            Some(idx) => {
                let overrun = entry_stack.split_off(idx + 1);
                if !overrun.is_empty() {
                    let reason = CloseReason::ClosingTagOverrun {
                        closing_start: span.start,
                        tag_name: tag_name.to_string(),
                    };
                    for entry in overrun.into_iter().rev() {
                        self.auto_close_entry(entry, children_stack, &reason);
                    }
                }

                let Some(StackEntry::Element(el)) = entry_stack.pop() else {
                    return;
                };

                let children = pop_children(children_stack);
                let merged_span = el.span_start.merge(&span);

                let node = self.finish_element(el, children, merged_span);

                let id = self.push_node(node);
                push_child(children_stack, id);

                if self
                    .last_auto_closed_tag
                    .as_ref()
                    .is_some_and(|a| entry_stack.len() < a.depth)
                {
                    self.last_auto_closed_tag = None;
                }
            }
        }
    }

    pub(crate) fn handle_else_tag(
        &mut self,
        else_tag: &token::ElseTag,
        span: Span,
        entry_stack: &mut Vec<StackEntry>,
        children_stack: &mut Vec<Vec<NodeId>>,
    ) {
        match entry_stack.last() {
            Some(StackEntry::IfBlock(_)) | Some(StackEntry::EachBlock(_)) => {}
            Some(StackEntry::AwaitBlock(_)) => {
                let point = Span::new(span.start + 1, span.start + 1);
                self.recover(Diagnostic::error(
                    DiagnosticKind::ExpectedToken {
                        token: "{:then ...} or {:catch ...}".into(),
                    },
                    point,
                ));
                return;
            }
            _ => {
                let point = Span::new(span.start + 1, span.start + 1);
                self.recover(Diagnostic::error(
                    DiagnosticKind::BlockInvalidContinuationPlacement,
                    point,
                ));
                return;
            }
        }

        let consequent_children = pop_children(children_stack);

        if else_tag.elseif {
            let valid = entry_stack
                .last()
                .is_some_and(|e| matches!(e, StackEntry::IfBlock(_)));
            if !valid {
                self.recover(Diagnostic::no_if_block_for_else(span));
                children_stack.push(consequent_children);
                return;
            }
            let entry = entry_stack
                .last_mut()
                .expect("valid check above guarantees non-empty stack");
            let StackEntry::IfBlock(ib) = entry else {
                unreachable!()
            };
            ib.consequent = Some(consequent_children);
            ib.in_alternate = true;

            children_stack.push(vec![]);

            let expr_span = else_tag
                .expression_span
                .expect("elseif tag always carries an expression span");
            entry_stack.push(StackEntry::IfBlock(IfBlockEntry {
                span,
                test_span: expr_span,
                elseif: true,
                consequent: None,
                in_alternate: false,
            }));
            children_stack.push(vec![]);
        } else {
            match entry_stack.last_mut() {
                Some(StackEntry::IfBlock(ib)) => {
                    ib.consequent = Some(consequent_children);
                    ib.in_alternate = true;
                    ib.span = ib.span.merge(&span);
                    children_stack.push(vec![]);
                }
                Some(StackEntry::EachBlock(eb)) => {
                    eb.body_children = Some(consequent_children);
                    eb.in_fallback = true;
                    children_stack.push(vec![]);
                }
                _ => {
                    self.recover(Diagnostic::no_if_block_for_else(span));
                    children_stack.push(consequent_children);
                }
            }
        }
    }

    pub(crate) fn handle_end_each_tag(
        &mut self,
        span: Span,
        entry_stack: &mut Vec<StackEntry>,
        children_stack: &mut Vec<Vec<NodeId>>,
    ) {
        let entry = entry_stack.pop();

        let Some(StackEntry::EachBlock(eb)) = entry else {
            self.recover(Diagnostic::no_each_block_to_close(span));
            if let Some(entry) = entry {
                entry_stack.push(entry);
            }
            return;
        };

        let last_children = pop_children(children_stack);
        let merged_span = eb.span.merge(&span);

        let node = self.finish_each(eb, last_children, merged_span);
        let id = self.push_node(node);

        push_child(children_stack, id);
    }

    pub(crate) fn handle_end_snippet_tag(
        &mut self,
        span: Span,
        entry_stack: &mut Vec<StackEntry>,
        children_stack: &mut Vec<Vec<NodeId>>,
    ) {
        let entry = entry_stack.pop();

        let Some(StackEntry::SnippetBlock(sb)) = entry else {
            self.recover(Diagnostic::unexpected_token(span));
            if let Some(entry) = entry {
                entry_stack.push(entry);
            }
            return;
        };

        let body_children = pop_children(children_stack);
        let merged_span = sb.span_start.merge(&span);

        let node = self.finish_snippet(sb, body_children, merged_span);
        let id = self.push_node(node);

        push_child(children_stack, id);
    }

    pub(crate) fn handle_end_key_tag(
        &mut self,
        span: Span,
        entry_stack: &mut Vec<StackEntry>,
        children_stack: &mut Vec<Vec<NodeId>>,
    ) {
        let entry = entry_stack.pop();

        let Some(StackEntry::KeyBlock(kb)) = entry else {
            self.recover(Diagnostic::no_key_block_to_close(span));
            if let Some(entry) = entry {
                entry_stack.push(entry);
            }
            return;
        };

        let body_children = pop_children(children_stack);
        let merged_span = kb.span.merge(&span);

        let node = self.finish_key(kb, body_children, merged_span);
        let id = self.push_node(node);

        push_child(children_stack, id);
    }

    pub(crate) fn handle_await_clause_tag(
        &mut self,
        clause_tag: &scanner::token::AwaitClauseTag,
        span: Span,
        entry_stack: &mut Vec<StackEntry>,
        children_stack: &mut Vec<Vec<NodeId>>,
    ) {
        let point = Span::new(span.start + 1, span.start + 1);
        let ab = match entry_stack.last_mut() {
            Some(StackEntry::AwaitBlock(ab)) => ab,
            Some(StackEntry::IfBlock(_)) => {
                self.recover(Diagnostic::error(
                    DiagnosticKind::ExpectedToken {
                        token: "{:else} or {:else if}".into(),
                    },
                    point,
                ));
                return;
            }
            Some(StackEntry::EachBlock(_)) => {
                self.recover(Diagnostic::error(
                    DiagnosticKind::ExpectedToken {
                        token: "{:else}".into(),
                    },
                    point,
                ));
                return;
            }
            _ => {
                self.recover(Diagnostic::error(
                    DiagnosticKind::BlockInvalidContinuationPlacement,
                    point,
                ));
                return;
            }
        };

        let is_dup = match clause_tag.clause {
            token::AwaitClause::Then => {
                matches!(ab.phase, AwaitPhase::Then) || ab.then_children.is_some()
            }
            token::AwaitClause::Catch => matches!(ab.phase, AwaitPhase::Catch),
        };
        if is_dup {
            let name = match clause_tag.clause {
                token::AwaitClause::Then => "{:then}",
                token::AwaitClause::Catch => "{:catch}",
            };
            self.recover(Diagnostic::error(
                DiagnosticKind::BlockDuplicateClause {
                    name: name.to_string(),
                },
                span,
            ));
            return;
        }

        let current_children = pop_children(children_stack);
        match ab.phase {
            AwaitPhase::Pending => {
                ab.pending_children = Some(current_children);
            }
            AwaitPhase::Then => {
                ab.then_children = Some(current_children);
            }

            AwaitPhase::Catch => {
                ab.catch_children = Some(current_children);
            }
        }

        match clause_tag.clause {
            token::AwaitClause::Then => {
                ab.value_span = clause_tag.binding_span;
                ab.phase = AwaitPhase::Then;
            }
            token::AwaitClause::Catch => {
                ab.error_span = clause_tag.binding_span;
                ab.phase = AwaitPhase::Catch;
            }
        }

        children_stack.push(vec![]);
    }

    pub(crate) fn handle_end_await_tag(
        &mut self,
        span: Span,
        entry_stack: &mut Vec<StackEntry>,
        children_stack: &mut Vec<Vec<NodeId>>,
    ) {
        let entry = entry_stack.pop();

        let Some(StackEntry::AwaitBlock(ab)) = entry else {
            self.recover(Diagnostic::unexpected_token(span));
            if let Some(entry) = entry {
                entry_stack.push(entry);
            }
            return;
        };

        let current_children = pop_children(children_stack);
        let merged_span = ab.span.merge(&span);

        let node = self.finish_await(ab, current_children, merged_span);
        let id = self.push_node(node);

        push_child(children_stack, id);
    }

    pub(crate) fn auto_close_entries(
        &mut self,
        entry_stack: &mut Vec<StackEntry>,
        children_stack: &mut Vec<Vec<NodeId>>,
    ) {
        while let Some(entry) = entry_stack.pop() {
            self.auto_close_entry(entry, children_stack, &CloseReason::Eof);
        }
    }

    fn recover_block_close(&mut self, block_span: Span, reason: &CloseReason) {
        match reason {
            CloseReason::Eof => {
                if !self.has_error {
                    self.recover(Diagnostic::error(
                        DiagnosticKind::BlockUnclosed,
                        Span::new(block_span.start, block_span.start + 1),
                    ));
                }
            }
            CloseReason::ClosingTagOverrun {
                closing_start,
                tag_name,
            } => {
                let span = Span::new(*closing_start, *closing_start);
                self.recover(Diagnostic::element_invalid_closing_tag(
                    span,
                    tag_name.clone(),
                ));
            }
            CloseReason::ImplicitOpen { .. } => {}
        }
    }

    pub(crate) fn auto_close_entry(
        &mut self,
        entry: StackEntry,
        children_stack: &mut Vec<Vec<NodeId>>,
        reason: &CloseReason,
    ) {
        let boundary = match reason {
            CloseReason::Eof => {
                let eof_pos = self.source.len() as u32;
                Span::new(eof_pos, eof_pos)
            }
            CloseReason::ClosingTagOverrun { closing_start, .. } => {
                Span::new(*closing_start, *closing_start)
            }
            CloseReason::ImplicitOpen {
                following_start, ..
            } => Span::new(*following_start, *following_start),
        };

        match entry {
            StackEntry::Element(el) => {
                let children = pop_children(children_stack);
                match reason {
                    CloseReason::Eof => {
                        if !self.has_error {
                            self.recover(Diagnostic::error(
                                DiagnosticKind::ElementUnclosed {
                                    name: el.name.clone(),
                                },
                                Span::new(el.span_start.start, el.span_start.start + 1),
                            ));
                        }
                    }
                    CloseReason::ClosingTagOverrun {
                        closing_start,
                        tag_name,
                    } => {
                        let diag_end = children
                            .first()
                            .map(|id| self.store.get(*id).span().start)
                            .unwrap_or(*closing_start);
                        let diag_span = Span::new(el.span_start.start, diag_end);
                        let tag = format!("</{tag_name}>");
                        let closing = format!("</{}>", el.name);
                        self.recover(Diagnostic::element_implicitly_closed(
                            diag_span, tag, closing,
                        ));
                    }
                    CloseReason::ImplicitOpen {
                        following_tag,
                        following_start,
                    } => {
                        let diag_end = children
                            .first()
                            .map(|id| self.store.get(*id).span().start)
                            .unwrap_or(*following_start);
                        let diag_span = Span::new(el.span_start.start, diag_end);
                        let tag = format!("<{following_tag}>");
                        let closing = format!("</{}>", el.name);
                        self.recover(Diagnostic::element_implicitly_closed(
                            diag_span, tag, closing,
                        ));
                    }
                }
                let merged_span = el.span_start.merge(&boundary);

                let node = self.finish_element(el, children, merged_span);

                let id = self.push_node(node);
                push_child(children_stack, id);
            }
            StackEntry::IfBlock(ib) => {
                self.recover_block_close(ib.span, reason);
                let last_children = pop_children(children_stack);
                let merged_span = ib.span.merge(&boundary);

                let node = self.finish_if(ib, last_children, merged_span);
                let id = self.push_node(node);

                push_child(children_stack, id);
            }
            StackEntry::EachBlock(eb) => {
                self.recover_block_close(eb.span, reason);
                let last_children = pop_children(children_stack);
                let merged_span = eb.span.merge(&boundary);

                let node = self.finish_each(eb, last_children, merged_span);
                let id = self.push_node(node);

                push_child(children_stack, id);
            }
            StackEntry::SnippetBlock(sb) => {
                self.recover_block_close(sb.span_start, reason);
                let body_children = pop_children(children_stack);
                let merged_span = sb.span_start.merge(&boundary);

                let node = self.finish_snippet(sb, body_children, merged_span);
                let id = self.push_node(node);

                push_child(children_stack, id);
            }
            StackEntry::KeyBlock(kb) => {
                self.recover_block_close(kb.span, reason);
                let body_children = pop_children(children_stack);
                let merged_span = kb.span.merge(&boundary);

                let node = self.finish_key(kb, body_children, merged_span);
                let id = self.push_node(node);

                push_child(children_stack, id);
            }
            StackEntry::AwaitBlock(ab) => {
                self.recover_block_close(ab.span, reason);
                let current_children = pop_children(children_stack);
                let merged_span = ab.span.merge(&boundary);

                let node = self.finish_await(ab, current_children, merged_span);
                let id = self.push_node(node);

                push_child(children_stack, id);
            }
        }
    }

    pub(crate) fn close_if_chain(
        &mut self,
        end_span: Span,
        entry_stack: &mut Vec<StackEntry>,
        children_stack: &mut Vec<Vec<NodeId>>,
    ) {
        loop {
            let Some(entry) = entry_stack.pop() else {
                self.recover(Diagnostic::no_if_block_to_close(end_span));
                return;
            };

            let StackEntry::IfBlock(ib) = entry else {
                self.recover(Diagnostic::no_if_block_to_close(end_span));
                entry_stack.push(entry);
                return;
            };

            let last_children = pop_children(children_stack);
            let merged_span = ib.span.merge(&end_span);
            let elseif = ib.elseif;

            let node = self.finish_if(ib, last_children, merged_span);
            let id = self.push_node(node);

            if elseif {
                push_child(children_stack, id);

                if entry_stack
                    .last()
                    .is_some_and(|e| matches!(e, StackEntry::IfBlock(_)))
                {
                    continue;
                } else {
                    break;
                }
            } else {
                push_child(children_stack, id);
                break;
            }
        }
    }
}
