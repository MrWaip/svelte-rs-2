use svelte_ast::{
    AwaitBlock, ComponentNode, EachBlock, Element, Fragment, IfBlock, KeyBlock, Node, SnippetBlock,
};
use svelte_diagnostics::Diagnostic;
use svelte_span::Span;

use crate::scanner::{self, token};
use crate::{
    is_component_name, pop_children, push_child, AwaitPhase, IfBlockEntry, Parser, StackEntry,
};

impl<'a> Parser<'a> {
    pub(crate) fn handle_end_tag(
        &mut self,
        tag: &token::EndTag,
        span: Span,
        entry_stack: &mut Vec<StackEntry>,
        children_stack: &mut Vec<Vec<Node>>,
    ) {
        let tag_name = tag.name_span.source_text(self.source);

        // Void elements cannot have closing tags
        if scanner::is_void(tag_name) {
            self.recover(Diagnostic::void_element_invalid_content(span));
            let node = Node::Error(svelte_ast::ErrorNode {
                id: self.ids.next(),
                span,
            });
            push_child(children_stack, node);
            return;
        }

        // Try to find a matching element in the stack
        let match_idx = entry_stack
            .iter()
            .rposition(|e| matches!(e, StackEntry::Element(el) if el.name == tag_name));

        match match_idx {
            None => {
                // No matching open tag — emit error node
                self.recover(Diagnostic::no_element_to_close(span));
                let node = Node::Error(svelte_ast::ErrorNode {
                    id: self.ids.next(),
                    span,
                });
                push_child(children_stack, node);
            }
            Some(idx) => {
                // Auto-close any intervening entries
                let entries_to_close = entry_stack.len() - 1 - idx;
                for _ in 0..entries_to_close {
                    let entry = entry_stack.pop().unwrap();
                    self.auto_close_entry(entry, children_stack);
                }

                // Now close the matching element
                let entry = entry_stack.pop().unwrap();
                let StackEntry::Element(el) = entry else {
                    unreachable!();
                };

                let children = pop_children(children_stack);
                let merged_span = el.span_start.merge(&span);

                let node = if is_component_name(&el.name) {
                    Node::ComponentNode(ComponentNode {
                        id: self.ids.next(),
                        span: merged_span,
                        name: el.name,
                        self_closing: false,
                        attributes: el.attributes,
                        fragment: Fragment::new(children),
                    })
                } else {
                    Node::Element(Element {
                        id: self.ids.next(),
                        span: merged_span,
                        name: el.name,
                        self_closing: false,
                        attributes: el.attributes,
                        fragment: Fragment::new(children),
                    })
                };

                push_child(children_stack, node);
            }
        }
    }

    pub(crate) fn handle_else_tag(
        &mut self,
        else_tag: &token::ElseTag,
        span: Span,
        entry_stack: &mut Vec<StackEntry>,
        children_stack: &mut Vec<Vec<Node>>,
    ) {
        let consequent_children = pop_children(children_stack);

        if else_tag.elseif {
            // {:else if expr}
            let valid = entry_stack
                .last()
                .is_some_and(|e| matches!(e, StackEntry::IfBlock(_)));
            if !valid {
                self.recover(Diagnostic::no_if_block_for_else(span));
                children_stack.push(consequent_children);
                return;
            }
            let entry = entry_stack.last_mut().unwrap();
            let StackEntry::IfBlock(ref mut ib) = entry else {
                unreachable!()
            };
            ib.consequent = Some(consequent_children);
            ib.in_alternate = true;

            children_stack.push(vec![]);

            let expr_span = else_tag.expression_span.unwrap();
            entry_stack.push(StackEntry::IfBlock(IfBlockEntry {
                span,
                test_span: expr_span,
                elseif: true,
                consequent: None,
                in_alternate: false,
            }));
            children_stack.push(vec![]);
        } else {
            // {:else} — can appear in IfBlock or EachBlock
            match entry_stack.last_mut() {
                Some(StackEntry::IfBlock(ref mut ib)) => {
                    ib.consequent = Some(consequent_children);
                    ib.in_alternate = true;
                    ib.span = ib.span.merge(&span);
                    children_stack.push(vec![]);
                }
                Some(StackEntry::EachBlock(ref mut eb)) => {
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
        children_stack: &mut Vec<Vec<Node>>,
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

        // If {:else} was encountered, body_children were saved and last_children are fallback
        let (body_children, fallback) = if eb.in_fallback {
            let body = eb.body_children.unwrap_or_default();
            (body, Some(Fragment::new(last_children)))
        } else {
            (last_children, None)
        };

        let key_id = eb.key_span.map(|_| self.ids.next());
        let node = Node::EachBlock(EachBlock {
            id: self.ids.next(),
            span: merged_span,
            expression_span: eb.expression_span,
            context_span: eb.context_span,
            index_span: eb.index_span,
            key_span: eb.key_span,
            key_id,
            body: Fragment::new(body_children),
            fallback,
        });

        push_child(children_stack, node);
    }

    pub(crate) fn handle_end_snippet_tag(
        &mut self,
        span: Span,
        entry_stack: &mut Vec<StackEntry>,
        children_stack: &mut Vec<Vec<Node>>,
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

        let node = Node::SnippetBlock(SnippetBlock {
            id: self.ids.next(),
            span: merged_span,
            expression_span: sb.expression_span,
            body: Fragment::new(body_children),
        });

        push_child(children_stack, node);
    }

    pub(crate) fn handle_end_key_tag(
        &mut self,
        span: Span,
        entry_stack: &mut Vec<StackEntry>,
        children_stack: &mut Vec<Vec<Node>>,
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

        let node = Node::KeyBlock(KeyBlock {
            id: self.ids.next(),
            span: merged_span,
            expression_span: kb.expression_span,
            fragment: Fragment::new(body_children),
        });

        push_child(children_stack, node);
    }

    pub(crate) fn handle_await_clause_tag(
        &mut self,
        clause_tag: &scanner::token::AwaitClauseTag,
        entry_stack: &mut Vec<StackEntry>,
        children_stack: &mut Vec<Vec<Node>>,
    ) {
        let entry = entry_stack.last_mut();

        let Some(StackEntry::AwaitBlock(ab)) = entry else {
            self.recover(Diagnostic::unexpected_token(Span::new(0, 0)));
            return;
        };

        // Save current children to the appropriate phase
        let current_children = pop_children(children_stack);
        match ab.phase {
            AwaitPhase::Pending => {
                ab.pending_children = Some(current_children);
            }
            AwaitPhase::Then => {
                ab.then_children = Some(current_children);
            }
            AwaitPhase::Catch => {
                // Shouldn't happen — {:catch} after {:catch}
                self.recover(Diagnostic::unexpected_token(Span::new(0, 0)));
                children_stack.push(vec![]);
                return;
            }
        }

        match clause_tag.clause {
            scanner::token::AwaitClause::Then => {
                ab.value_span = clause_tag.binding_span;
                ab.phase = AwaitPhase::Then;
            }
            scanner::token::AwaitClause::Catch => {
                ab.error_span = clause_tag.binding_span;
                ab.phase = AwaitPhase::Catch;
            }
        }

        // Push new children list for the next phase
        children_stack.push(vec![]);
    }

    pub(crate) fn handle_end_await_tag(
        &mut self,
        span: Span,
        entry_stack: &mut Vec<StackEntry>,
        children_stack: &mut Vec<Vec<Node>>,
    ) {
        let entry = entry_stack.pop();

        let Some(StackEntry::AwaitBlock(ab)) = entry else {
            self.recover(Diagnostic::unexpected_token(span));
            if let Some(entry) = entry {
                entry_stack.push(entry);
            }
            return;
        };

        // Save current children to the appropriate phase
        let current_children = pop_children(children_stack);
        let merged_span = ab.span.merge(&span);

        let (pending, then, catch) = match ab.phase {
            AwaitPhase::Pending => (Some(Fragment::new(current_children)), None, None),
            AwaitPhase::Then => {
                let pending = ab.pending_children.map(Fragment::new);
                (pending, Some(Fragment::new(current_children)), None)
            }
            AwaitPhase::Catch => {
                let pending = ab.pending_children.map(Fragment::new);
                let then = ab.then_children.map(Fragment::new);
                (pending, then, Some(Fragment::new(current_children)))
            }
        };

        let node = Node::AwaitBlock(AwaitBlock {
            id: self.ids.next(),
            span: merged_span,
            expression_span: ab.expression_span,
            value_span: ab.value_span,
            error_span: ab.error_span,
            pending,
            then,
            catch,
        });

        push_child(children_stack, node);
    }

    /// Auto-close all remaining open entries at EOF.
    pub(crate) fn auto_close_entries(
        &mut self,
        entry_stack: &mut Vec<StackEntry>,
        children_stack: &mut Vec<Vec<Node>>,
    ) {
        while let Some(entry) = entry_stack.pop() {
            self.auto_close_entry(entry, children_stack);
        }
    }

    /// Auto-close a single entry, producing a node with span extended to end of source.
    pub(crate) fn auto_close_entry(
        &mut self,
        entry: StackEntry,
        children_stack: &mut Vec<Vec<Node>>,
    ) {
        let eof_pos = self.source.len() as u32;
        let eof_span = Span::new(eof_pos, eof_pos);

        match entry {
            StackEntry::Element(el) => {
                self.recover(Diagnostic::unclosed_node(el.span_start));
                let children = pop_children(children_stack);
                let merged_span = el.span_start.merge(&eof_span);

                let node = if is_component_name(&el.name) {
                    Node::ComponentNode(ComponentNode {
                        id: self.ids.next(),
                        span: merged_span,
                        name: el.name,
                        self_closing: false,
                        attributes: el.attributes,
                        fragment: Fragment::new(children),
                    })
                } else {
                    Node::Element(Element {
                        id: self.ids.next(),
                        span: merged_span,
                        name: el.name,
                        self_closing: false,
                        attributes: el.attributes,
                        fragment: Fragment::new(children),
                    })
                };

                push_child(children_stack, node);
            }
            StackEntry::IfBlock(ib) => {
                self.recover(Diagnostic::unclosed_node(ib.span));
                let last_children = pop_children(children_stack);

                let (consequent, alternate) = if let Some(cons) = ib.consequent {
                    (cons, Some(Fragment::new(last_children)))
                } else {
                    (last_children, None)
                };

                let merged_span = ib.span.merge(&eof_span);

                let node = Node::IfBlock(IfBlock {
                    id: self.ids.next(),
                    span: merged_span,
                    test_span: ib.test_span,
                    elseif: ib.elseif,
                    consequent: Fragment::new(consequent),
                    alternate,
                });

                if ib.elseif {
                    push_child(children_stack, node);
                    // Continue unwinding parent if-blocks
                    if children_stack.len() > 1 {
                        // Parent if-block will be auto-closed in the next iteration
                    }
                } else {
                    push_child(children_stack, node);
                }
            }
            StackEntry::EachBlock(eb) => {
                self.recover(Diagnostic::unclosed_node(eb.span));
                let last_children = pop_children(children_stack);
                let merged_span = eb.span.merge(&eof_span);

                let (body_children, fallback) = if eb.in_fallback {
                    let body = eb.body_children.unwrap_or_default();
                    (body, Some(Fragment::new(last_children)))
                } else {
                    (last_children, None)
                };

                let key_id = eb.key_span.map(|_| self.ids.next());
                let node = Node::EachBlock(EachBlock {
                    id: self.ids.next(),
                    span: merged_span,
                    expression_span: eb.expression_span,
                    context_span: eb.context_span,
                    index_span: eb.index_span,
                    key_span: eb.key_span,
                    key_id,
                    body: Fragment::new(body_children),
                    fallback,
                });

                push_child(children_stack, node);
            }
            StackEntry::SnippetBlock(sb) => {
                self.recover(Diagnostic::unclosed_node(sb.span_start));
                let body_children = pop_children(children_stack);
                let merged_span = sb.span_start.merge(&eof_span);

                let node = Node::SnippetBlock(SnippetBlock {
                    id: self.ids.next(),
                    span: merged_span,
                    expression_span: sb.expression_span,
                    body: Fragment::new(body_children),
                });

                push_child(children_stack, node);
            }
            StackEntry::KeyBlock(kb) => {
                self.recover(Diagnostic::unclosed_node(kb.span));
                let body_children = pop_children(children_stack);
                let merged_span = kb.span.merge(&eof_span);

                let node = Node::KeyBlock(KeyBlock {
                    id: self.ids.next(),
                    span: merged_span,
                    expression_span: kb.expression_span,
                    fragment: Fragment::new(body_children),
                });

                push_child(children_stack, node);
            }
            StackEntry::AwaitBlock(ab) => {
                self.recover(Diagnostic::unclosed_node(ab.span));
                let current_children = pop_children(children_stack);
                let merged_span = ab.span.merge(&eof_span);

                let (pending, then, catch) = match ab.phase {
                    AwaitPhase::Pending => (Some(Fragment::new(current_children)), None, None),
                    AwaitPhase::Then => (
                        ab.pending_children.map(Fragment::new),
                        Some(Fragment::new(current_children)),
                        None,
                    ),
                    AwaitPhase::Catch => (
                        ab.pending_children.map(Fragment::new),
                        ab.then_children.map(Fragment::new),
                        Some(Fragment::new(current_children)),
                    ),
                };

                let node = Node::AwaitBlock(AwaitBlock {
                    id: self.ids.next(),
                    span: merged_span,
                    expression_span: ab.expression_span,
                    value_span: ab.value_span,
                    error_span: ab.error_span,
                    pending,
                    then,
                    catch,
                });

                push_child(children_stack, node);
            }
        }
    }

    /// Close the if-block chain. Handles nested else-if blocks.
    pub(crate) fn close_if_chain(
        &mut self,
        end_span: Span,
        entry_stack: &mut Vec<StackEntry>,
        children_stack: &mut Vec<Vec<Node>>,
    ) {
        // Process from innermost to outermost if-block
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
                // We had {:else} or {:else if}, so cons = consequent, last_children = alternate
                (cons, Some(Fragment::new(last_children)))
            } else {
                // No else branch, last_children = consequent
                (last_children, None)
            };

            let merged_span = ib.span.merge(&end_span);

            let node = Node::IfBlock(IfBlock {
                id: self.ids.next(),
                span: merged_span,
                test_span: ib.test_span,
                elseif: ib.elseif,
                consequent: Fragment::new(consequent),
                alternate,
            });

            if ib.elseif {
                // This is an else-if: it becomes the alternate of the parent if-block.
                push_child(children_stack, node);

                // Check if parent entry is also an IfBlock — if so, continue the loop.
                if entry_stack
                    .last()
                    .is_some_and(|e| matches!(e, StackEntry::IfBlock(_)))
                {
                    continue;
                } else {
                    break;
                }
            } else {
                // This is the outermost {#if}
                push_child(children_stack, node);
                break;
            }
        }
    }
}
