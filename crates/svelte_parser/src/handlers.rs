#![allow(clippy::ptr_arg)]

use svelte_ast::{
    AwaitBlock, ComponentNode, EachBlock, Element, ExprRef, FragmentRole, IfBlock, KeyBlock, Node,
    NodeId, SnippetBlock, StmtRef,
};
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::scanner::{self, token};
use crate::{
    AwaitPhase, IfBlockEntry, Parser, StackEntry, is_component_name, pop_children, push_child,
};

impl<'a> Parser<'a> {
    pub(crate) fn handle_end_tag(
        &mut self,
        tag: &token::EndTag,
        span: Span,
        entry_stack: &mut Vec<StackEntry>,
        children_stack: &mut Vec<Vec<NodeId>>,
    ) {
        let tag_name = tag.name_span.source_text(self.source);

        if scanner::is_void(tag_name) {
            self.recover(Diagnostic::void_element_invalid_content(span));
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
                self.recover(Diagnostic::no_element_to_close(span));
                let id = self.push_node(Node::Error(svelte_ast::ErrorNode {
                    id: NodeId(0),
                    span,
                }));
                push_child(children_stack, id);
            }
            Some(idx) => {
                let entries_to_close = entry_stack.len() - 1 - idx;
                for _ in 0..entries_to_close {
                    let entry = entry_stack
                        .pop()
                        .expect("entries_to_close is derived from stack length");
                    self.auto_close_entry(entry, children_stack);
                }

                let entry = entry_stack
                    .pop()
                    .expect("matching element at idx guarantees stack is non-empty");
                let StackEntry::Element(el) = entry else {
                    unreachable!();
                };

                let children = pop_children(children_stack);
                let merged_span = el.span_start.merge(&span);

                let node = if is_component_name(&el.name) {
                    let (default_children, legacy_slots) =
                        self.partition_component_children(children);
                    let fragment =
                        self.new_fragment(FragmentRole::ComponentChildren, default_children);
                    Node::ComponentNode(ComponentNode {
                        id: NodeId(0),
                        span: merged_span,
                        name: el.name,
                        self_closing: false,
                        attributes: el.attributes,
                        fragment,
                        legacy_slots,
                    })
                } else {
                    let fragment = self.new_fragment(FragmentRole::Element, children);
                    Node::Element(Element {
                        id: NodeId(0),
                        span: merged_span,
                        name: el.name,
                        self_closing: false,
                        attributes: el.attributes,
                        fragment,
                    })
                };

                let id = self.push_node(node);
                push_child(children_stack, id);
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

        let (body_children, fallback) = if eb.in_fallback {
            let body = eb.body_children.unwrap_or_default();
            let fb = self.new_fragment(FragmentRole::EachFallback, last_children);
            (body, Some(fb))
        } else {
            (last_children, None)
        };

        let body = self.new_fragment(FragmentRole::EachBody, body_children);
        let key_id = eb.key_span.map(|_| self.reserve_id());
        let id = self.push_node(Node::EachBlock(EachBlock {
            id: NodeId(0),
            span: merged_span,
            expression: ExprRef::new(eb.expression_span),
            context: eb.context_span.map(StmtRef::new),
            index: eb.index_span.map(StmtRef::new),
            key: eb.key_span.map(ExprRef::new),
            key_id,
            body,
            fallback,
        }));

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

        let body = self.new_fragment(FragmentRole::SnippetBody, body_children);
        let id = self.push_node(Node::SnippetBlock(SnippetBlock {
            id: NodeId(0),
            span: merged_span,
            decl: StmtRef::new(sb.expression_span),
            body,
        }));

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

        let fragment = self.new_fragment(FragmentRole::KeyBlockBody, body_children);
        let id = self.push_node(Node::KeyBlock(KeyBlock {
            id: NodeId(0),
            span: merged_span,
            expression: ExprRef::new(kb.expression_span),
            fragment,
        }));

        push_child(children_stack, id);
    }

    pub(crate) fn handle_await_clause_tag(
        &mut self,
        clause_tag: &scanner::token::AwaitClauseTag,
        span: Span,
        entry_stack: &mut Vec<StackEntry>,
        children_stack: &mut Vec<Vec<NodeId>>,
    ) {
        let entry = entry_stack.last_mut();

        let Some(StackEntry::AwaitBlock(ab)) = entry else {
            self.recover(Diagnostic::unexpected_token(Span::new(0, 0)));
            return;
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

        let id = self.push_node(Node::AwaitBlock(AwaitBlock {
            id: NodeId(0),
            span: merged_span,
            expression: ExprRef::new(ab.expression_span),
            value: ab.value_span.map(StmtRef::new),
            error: ab.error_span.map(StmtRef::new),
            pending,
            then,
            catch,
        }));

        push_child(children_stack, id);
    }

    pub(crate) fn auto_close_entries(
        &mut self,
        entry_stack: &mut Vec<StackEntry>,
        children_stack: &mut Vec<Vec<NodeId>>,
    ) {
        while let Some(entry) = entry_stack.pop() {
            self.auto_close_entry(entry, children_stack);
        }
    }

    pub(crate) fn auto_close_entry(
        &mut self,
        entry: StackEntry,
        children_stack: &mut Vec<Vec<NodeId>>,
    ) {
        let eof_pos = self.source.len() as u32;
        let eof_span = Span::new(eof_pos, eof_pos);

        match entry {
            StackEntry::Element(el) => {
                self.recover(Diagnostic::unclosed_node(el.span_start));
                let children = pop_children(children_stack);
                let merged_span = el.span_start.merge(&eof_span);

                let node = if is_component_name(&el.name) {
                    let (default_children, legacy_slots) =
                        self.partition_component_children(children);
                    let fragment =
                        self.new_fragment(FragmentRole::ComponentChildren, default_children);
                    Node::ComponentNode(ComponentNode {
                        id: NodeId(0),
                        span: merged_span,
                        name: el.name,
                        self_closing: false,
                        attributes: el.attributes,
                        fragment,
                        legacy_slots,
                    })
                } else {
                    let fragment = self.new_fragment(FragmentRole::Element, children);
                    Node::Element(Element {
                        id: NodeId(0),
                        span: merged_span,
                        name: el.name,
                        self_closing: false,
                        attributes: el.attributes,
                        fragment,
                    })
                };

                let id = self.push_node(node);
                push_child(children_stack, id);
            }
            StackEntry::IfBlock(ib) => {
                self.recover(Diagnostic::unclosed_node(ib.span));
                let last_children = pop_children(children_stack);

                let (consequent, alternate) = if let Some(cons) = ib.consequent {
                    let alt = self.new_fragment(FragmentRole::IfAlternate, last_children);
                    (cons, Some(alt))
                } else {
                    (last_children, None)
                };

                let merged_span = ib.span.merge(&eof_span);

                let consequent_fragment = self.new_fragment(FragmentRole::IfConsequent, consequent);
                let id = self.push_node(Node::IfBlock(IfBlock {
                    id: NodeId(0),
                    span: merged_span,
                    test: ExprRef::new(ib.test_span),
                    elseif: ib.elseif,
                    consequent: consequent_fragment,
                    alternate,
                }));

                push_child(children_stack, id);
            }
            StackEntry::EachBlock(eb) => {
                self.recover(Diagnostic::unclosed_node(eb.span));
                let last_children = pop_children(children_stack);
                let merged_span = eb.span.merge(&eof_span);

                let (body_children, fallback) = if eb.in_fallback {
                    let body = eb.body_children.unwrap_or_default();
                    let fb = self.new_fragment(FragmentRole::EachFallback, last_children);
                    (body, Some(fb))
                } else {
                    (last_children, None)
                };

                let body = self.new_fragment(FragmentRole::EachBody, body_children);
                let key_id = eb.key_span.map(|_| self.reserve_id());
                let id = self.push_node(Node::EachBlock(EachBlock {
                    id: NodeId(0),
                    span: merged_span,
                    expression: ExprRef::new(eb.expression_span),
                    context: eb.context_span.map(StmtRef::new),
                    index: eb.index_span.map(StmtRef::new),
                    key: eb.key_span.map(ExprRef::new),
                    key_id,
                    body,
                    fallback,
                }));

                push_child(children_stack, id);
            }
            StackEntry::SnippetBlock(sb) => {
                self.recover(Diagnostic::unclosed_node(sb.span_start));
                let body_children = pop_children(children_stack);
                let merged_span = sb.span_start.merge(&eof_span);

                let body = self.new_fragment(FragmentRole::SnippetBody, body_children);
                let id = self.push_node(Node::SnippetBlock(SnippetBlock {
                    id: NodeId(0),
                    span: merged_span,
                    decl: StmtRef::new(sb.expression_span),
                    body,
                }));

                push_child(children_stack, id);
            }
            StackEntry::KeyBlock(kb) => {
                self.recover(Diagnostic::unclosed_node(kb.span));
                let body_children = pop_children(children_stack);
                let merged_span = kb.span.merge(&eof_span);

                let fragment = self.new_fragment(FragmentRole::KeyBlockBody, body_children);
                let id = self.push_node(Node::KeyBlock(KeyBlock {
                    id: NodeId(0),
                    span: merged_span,
                    expression: ExprRef::new(kb.expression_span),
                    fragment,
                }));

                push_child(children_stack, id);
            }
            StackEntry::AwaitBlock(ab) => {
                self.recover(Diagnostic::unclosed_node(ab.span));
                let current_children = pop_children(children_stack);
                let merged_span = ab.span.merge(&eof_span);

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

                let id = self.push_node(Node::AwaitBlock(AwaitBlock {
                    id: NodeId(0),
                    span: merged_span,
                    expression: ExprRef::new(ab.expression_span),
                    value: ab.value_span.map(StmtRef::new),
                    error: ab.error_span.map(StmtRef::new),
                    pending,
                    then,
                    catch,
                }));

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

            let (consequent, alternate) = if let Some(cons) = ib.consequent {
                let alt = self.new_fragment(FragmentRole::IfAlternate, last_children);
                (cons, Some(alt))
            } else {
                (last_children, None)
            };

            let merged_span = ib.span.merge(&end_span);

            let consequent_fragment = self.new_fragment(FragmentRole::IfConsequent, consequent);
            let id = self.push_node(Node::IfBlock(IfBlock {
                id: NodeId(0),
                span: merged_span,
                test: ExprRef::new(ib.test_span),
                elseif: ib.elseif,
                consequent: consequent_fragment,
                alternate,
            }));

            if ib.elseif {
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
