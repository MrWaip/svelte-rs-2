use scanner::{
    token::{self, ExpressionTag, Token, TokenType},
    Scanner,
};
use svelte_span::Span;

use svelte_ast::{
    Attribute, BindDirective, BooleanAttribute, ClassDirective, Comment, ComponentNode, ConstTag,
    ConcatPart, ConcatenationAttribute, Component, EachBlock, Element, OnDirectiveLegacy, StyleDirective, StyleDirectiveValue,
    ExpressionAttribute, Fragment, HtmlTag, IfBlock, KeyBlock, Node, NodeIdAllocator, RawBlock, RenderTag, Script,
    ScriptContext, ScriptLanguage, ShorthandOrSpread, SnippetBlock, StringAttribute, Text,
};

use svelte_diagnostics::Diagnostic;

pub mod scanner;

// ---------------------------------------------------------------------------
// Stack entry — stores partial data while we parse nested structures
// ---------------------------------------------------------------------------

enum StackEntry {
    Element(ElementEntry),
    IfBlock(IfBlockEntry),
    EachBlock(EachBlockEntry),
    SnippetBlock(SnippetBlockEntry),
    KeyBlock(KeyBlockEntry),
}

struct KeyBlockEntry {
    span: Span,
    expression_span: Span,
}

struct ElementEntry {
    name: String,
    span_start: Span, // opening tag span
    attributes: Vec<Attribute>,
}

struct IfBlockEntry {
    span: Span,
    test_span: Span,
    elseif: bool,
    /// Children collected for the consequent branch.
    /// Once we see {:else}, these are moved out and we start collecting alternate.
    consequent: Option<Vec<Node>>,
    /// Whether we are currently collecting alternate children.
    in_alternate: bool,
}

struct EachBlockEntry {
    span: Span,
    expression_span: Span,
    context_span: Span,
    index_span: Option<Span>,
    key_span: Option<Span>,
}

struct SnippetBlockEntry {
    span_start: Span,
    name: String,
    params_span: Option<Span>,
}

// ---------------------------------------------------------------------------
// Stack helpers — safe wrappers around children_stack operations
// ---------------------------------------------------------------------------

/// Push a node onto the current children list.
/// Debug-asserts the stack is non-empty; gracefully no-ops in release.
fn push_child(children_stack: &mut Vec<Vec<Node>>, node: Node) {
    debug_assert!(
        !children_stack.is_empty(),
        "children_stack empty when pushing child"
    );
    if let Some(children) = children_stack.last_mut() {
        children.push(node);
    }
}

/// Pop the current children list.
/// Debug-asserts the stack is non-empty; returns empty vec in release.
fn pop_children(children_stack: &mut Vec<Vec<Node>>) -> Vec<Node> {
    debug_assert!(
        !children_stack.is_empty(),
        "children_stack empty when popping"
    );
    children_stack.pop().unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

pub struct Parser<'a> {
    source: &'a str,
    ids: NodeIdAllocator,
    diagnostics: Vec<Diagnostic>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Parser<'a> {
        Parser {
            source,
            ids: NodeIdAllocator::new(),
            diagnostics: Vec::new(),
        }
    }

    fn recover(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn parse(mut self) -> (Component, Vec<Diagnostic>) {
        let mut scanner = Scanner::new(self.source);
        let (tokens, scan_diagnostics) = scanner.scan_tokens();
        self.diagnostics.extend(scan_diagnostics);

        // children_stack[i] = children being collected for the i-th nesting level.
        // children_stack[0] = root level.
        let mut children_stack: Vec<Vec<Node>> = vec![vec![]];
        let mut entry_stack: Vec<StackEntry> = vec![];
        let mut script_data: Option<ScriptData> = None;
        let mut css_data: Option<CssData> = None;

        for token in tokens {
            match token.token_type {
                TokenType::Text => {
                    let node = self.make_text(&token);
                    push_child(&mut children_stack, node);
                }
                TokenType::Comment => {
                    let node = self.make_comment(&token);
                    push_child(&mut children_stack, node);
                }
                TokenType::Interpolation(interpolation) => {
                    let node = self.make_expression_tag(&interpolation);
                    push_child(&mut children_stack, node);
                }
                TokenType::StartTag(tag) => {
                    let name = tag.name_span.source_text(self.source);
                    let attrs = self.convert_attributes(&tag.attributes);
                    if tag.self_closing {
                        let name = name.to_string();
                        let node = if is_component_name(&name) {
                            Node::ComponentNode(ComponentNode {
                                id: self.ids.next(),
                                span: token.span,
                                name,
                                self_closing: true,
                                attributes: attrs,
                                fragment: Fragment::empty(),
                            })
                        } else {
                            Node::Element(Element {
                                id: self.ids.next(),
                                span: token.span,
                                name,
                                self_closing: true,
                                attributes: attrs,
                                fragment: Fragment::empty(),
                            })
                        };
                        push_child(&mut children_stack, node);
                    } else {
                        entry_stack.push(StackEntry::Element(ElementEntry {
                            name: name.to_string(),
                            span_start: token.span,
                            attributes: attrs,
                        }));
                        children_stack.push(vec![]);
                    }
                }
                TokenType::EndTag(tag) => {
                    self.handle_end_tag(
                        &tag,
                        token.span,
                        &mut entry_stack,
                        &mut children_stack,
                    );
                }
                TokenType::StartIfTag(start_if) => {
                    entry_stack.push(StackEntry::IfBlock(IfBlockEntry {
                        span: token.span,
                        test_span: start_if.expression_span,
                        elseif: false,
                        consequent: None,
                        in_alternate: false,
                    }));
                    children_stack.push(vec![]); // consequent children
                }
                TokenType::ElseTag(else_tag) => {
                    self.handle_else_tag(
                        &else_tag,
                        token.span,
                        &mut entry_stack,
                        &mut children_stack,
                    );
                }
                TokenType::EndIfTag => {
                    self.close_if_chain(
                        token.span,
                        &mut entry_stack,
                        &mut children_stack,
                    );
                }
                TokenType::StartEachTag(each) => {
                    entry_stack.push(StackEntry::EachBlock(EachBlockEntry {
                        span: token.span,
                        expression_span: each.collection_span,
                        context_span: each.context_span,
                        index_span: each.index_span,
                        key_span: each.key_span,
                    }));
                    children_stack.push(vec![]); // body children
                }
                TokenType::EndEachTag => {
                    self.handle_end_each_tag(
                        token.span,
                        &mut entry_stack,
                        &mut children_stack,
                    );
                }
                TokenType::StartSnippetTag(snippet_tag) => {
                    entry_stack.push(StackEntry::SnippetBlock(SnippetBlockEntry {
                        span_start: token.span,
                        name: snippet_tag.name_span.source_text(self.source).to_string(),
                        params_span: snippet_tag.params_span,
                    }));
                    children_stack.push(vec![]);
                }
                TokenType::EndSnippetTag => {
                    self.handle_end_snippet_tag(
                        token.span,
                        &mut entry_stack,
                        &mut children_stack,
                    );
                }
                TokenType::StartKeyTag(key_tag) => {
                    entry_stack.push(StackEntry::KeyBlock(KeyBlockEntry {
                        span: token.span,
                        expression_span: key_tag.expression_span,
                    }));
                    children_stack.push(vec![]);
                }
                TokenType::EndKeyTag => {
                    self.handle_end_key_tag(
                        token.span,
                        &mut entry_stack,
                        &mut children_stack,
                    );
                }
                TokenType::RenderTag(render_tag) => {
                    let node = Node::RenderTag(RenderTag {
                        id: self.ids.next(),
                        span: token.span,
                        expression_span: render_tag.expression_span,
                    });
                    push_child(&mut children_stack, node);
                }
                TokenType::HtmlTag(html_tag) => {
                    let node = Node::HtmlTag(HtmlTag {
                        id: self.ids.next(),
                        span: token.span,
                        expression_span: html_tag.expression_span,
                    });
                    push_child(&mut children_stack, node);
                }
                TokenType::ConstTag(ct) => {
                    let node = Node::ConstTag(ConstTag {
                        id: self.ids.next(),
                        span: token.span,
                        declaration_span: ct.declaration_span,
                    });
                    push_child(&mut children_stack, node);
                }
                TokenType::ScriptTag(script_tag) => {
                    if script_data.is_some() {
                        self.recover(Diagnostic::only_single_top_level_script(token.span));
                        continue;
                    }

                    let language = if script_tag.is_typescript {
                        ScriptLanguage::TypeScript
                    } else {
                        ScriptLanguage::JavaScript
                    };

                    let context = if script_tag.is_module {
                        ScriptContext::Module
                    } else {
                        ScriptContext::Default
                    };

                    script_data = Some(ScriptData {
                        span: token.span,
                        content_span: script_tag.content_span,
                        language,
                        context,
                    });
                }
                TokenType::StyleTag(style_tag) => {
                    if css_data.is_some() {
                        self.recover(Diagnostic::only_single_top_level_style(token.span));
                        continue;
                    }

                    css_data = Some(CssData {
                        span: token.span,
                        content_span: style_tag.content_span,
                    });
                }
                TokenType::EOF => break,
            }
        }

        // Auto-close any remaining open entries
        self.auto_close_entries(&mut entry_stack, &mut children_stack);

        let roots = pop_children(&mut children_stack);

        let script = script_data.map(|sd| Script {
            id: self.ids.next(),
            span: sd.span,
            content_span: sd.content_span,
            context: sd.context,
            language: sd.language,
        });

        let css = css_data.map(|cd| RawBlock {
            span: cd.span,
            content_span: cd.content_span,
        });

        let mut component = Component::new(
            self.source.to_string(),
            Fragment::new(roots),
            script,
            css,
        );
        component.set_next_node_id(self.ids.current());

        (component, self.diagnostics)
    }

    fn handle_end_tag(
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
        let match_idx = entry_stack.iter().rposition(|e| {
            matches!(e, StackEntry::Element(el) if el.name == tag_name)
        });

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

    fn handle_else_tag(
        &mut self,
        else_tag: &token::ElseTag,
        span: Span,
        entry_stack: &mut Vec<StackEntry>,
        children_stack: &mut Vec<Vec<Node>>,
    ) {
        let consequent_children = pop_children(children_stack);

        if else_tag.elseif {
            // {:else if expr}
            let valid = entry_stack.last().is_some_and(|e| matches!(e, StackEntry::IfBlock(_)));
            if !valid {
                self.recover(Diagnostic::no_if_block_for_else(span));
                children_stack.push(consequent_children);
                return;
            }
            let entry = entry_stack.last_mut().unwrap();
            let StackEntry::IfBlock(ref mut ib) = entry else { unreachable!() };
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
            // {:else}
            let valid = entry_stack.last().is_some_and(|e| matches!(e, StackEntry::IfBlock(_)));
            if !valid {
                self.recover(Diagnostic::no_if_block_for_else(span));
                children_stack.push(consequent_children);
                return;
            }
            let entry = entry_stack.last_mut().unwrap();
            let StackEntry::IfBlock(ref mut ib) = entry else { unreachable!() };
            ib.consequent = Some(consequent_children);
            ib.in_alternate = true;
            ib.span = ib.span.merge(&span);
            children_stack.push(vec![]);
        }
    }

    fn handle_end_each_tag(
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

        let body_children = pop_children(children_stack);
        let merged_span = eb.span.merge(&span);

        let node = Node::EachBlock(EachBlock {
            id: self.ids.next(),
            span: merged_span,
            expression_span: eb.expression_span,
            context_span: eb.context_span,
            index_span: eb.index_span,
            key_span: eb.key_span,
            body: Fragment::new(body_children),
            fallback: None,
        });

        push_child(children_stack, node);
    }

    fn handle_end_snippet_tag(
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
            name: sb.name,
            params_span: sb.params_span,
            body: Fragment::new(body_children),
        });

        push_child(children_stack, node);
    }

    fn handle_end_key_tag(
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

    /// Auto-close all remaining open entries at EOF.
    fn auto_close_entries(
        &mut self,
        entry_stack: &mut Vec<StackEntry>,
        children_stack: &mut Vec<Vec<Node>>,
    ) {
        while let Some(entry) = entry_stack.pop() {
            self.auto_close_entry(entry, children_stack);
        }
    }

    /// Auto-close a single entry, producing a node with span extended to end of source.
    fn auto_close_entry(
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
                let body_children = pop_children(children_stack);
                let merged_span = eb.span.merge(&eof_span);

                let node = Node::EachBlock(EachBlock {
                    id: self.ids.next(),
                    span: merged_span,
                    expression_span: eb.expression_span,
                    context_span: eb.context_span,
                    index_span: eb.index_span,
                    key_span: eb.key_span,
                    body: Fragment::new(body_children),
                    fallback: None,
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
                    name: sb.name,
                    params_span: sb.params_span,
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
        }
    }

    /// Close the if-block chain. Handles nested else-if blocks.
    fn close_if_chain(
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

    fn make_text(&mut self, token: &Token) -> Node {
        Node::Text(Text {
            id: self.ids.next(),
            span: token.span,
        })
    }

    fn make_comment(&mut self, token: &Token) -> Node {
        Node::Comment(Comment {
            id: self.ids.next(),
            span: token.span,
        })
    }

    fn make_expression_tag(&mut self, interpolation: &ExpressionTag) -> Node {
        Node::ExpressionTag(svelte_ast::ExpressionTag {
            id: self.ids.next(),
            span: interpolation.span,
            expression_span: interpolation.expression_span,
        })
    }

    fn convert_attributes(
        &mut self,
        token_attrs: &[token::Attribute],
    ) -> Vec<Attribute> {
        let mut attributes = Vec::new();

        for attr in token_attrs {
            match attr {
                token::Attribute::HTMLAttribute(html_attr) => {
                    let result = match &html_attr.value {
                        token::AttributeValue::String(span) => {
                            Attribute::StringAttribute(StringAttribute {
                                name: html_attr.name_span.source_text(self.source).to_string(),
                                value_span: *span,
                            })
                        }
                        token::AttributeValue::ExpressionTag(expr_tag) => {
                            Attribute::ExpressionAttribute(ExpressionAttribute {
                                name: html_attr.name_span.source_text(self.source).to_string(),
                                expression_span: expr_tag.expression_span,
                                shorthand: false,
                            })
                        }
                        token::AttributeValue::Concatenation(concat) => {
                            let parts = concat
                                .parts
                                .iter()
                                .map(|part| match part {
                                    token::ConcatenationPart::String(span) => {
                                        ConcatPart::Static(span.source_text(self.source).to_string())
                                    }
                                    token::ConcatenationPart::Expression(et) => {
                                        ConcatPart::Dynamic(et.expression_span)
                                    }
                                })
                                .collect();

                            Attribute::ConcatenationAttribute(ConcatenationAttribute {
                                name: html_attr.name_span.source_text(self.source).to_string(),
                                parts,
                            })
                        }
                        token::AttributeValue::Empty => {
                            Attribute::BooleanAttribute(BooleanAttribute {
                                name: html_attr.name_span.source_text(self.source).to_string(),
                            })
                        }
                    };
                    attributes.push(result);
                }
                token::Attribute::ExpressionTag(expr_tag) => {
                    let is_spread = expr_tag.expression_span.source_text(self.source).starts_with("...");
                    attributes.push(Attribute::ShorthandOrSpread(ShorthandOrSpread {
                        expression_span: expr_tag.expression_span,
                        is_spread,
                    }));
                }
                token::Attribute::ClassDirective(cd) => {
                    let expression_span = if cd.shorthand {
                        None
                    } else {
                        Some(cd.expression_span)
                    };
                    attributes.push(Attribute::ClassDirective(ClassDirective {
                        name: cd.name_span.source_text(self.source).to_string(),
                        expression_span,
                        shorthand: cd.shorthand,
                    }));
                }
                token::Attribute::StyleDirective(sd) => {
                    let value = if sd.shorthand {
                        StyleDirectiveValue::Shorthand
                    } else {
                        match &sd.value {
                            token::AttributeValue::ExpressionTag(et) => {
                                StyleDirectiveValue::Expression(et.expression_span)
                            }
                            token::AttributeValue::String(span) => {
                                StyleDirectiveValue::String(span.source_text(self.source).to_string())
                            }
                            token::AttributeValue::Concatenation(c) => {
                                StyleDirectiveValue::Concatenation(
                                    c.parts.iter().map(|p| match p {
                                        token::ConcatenationPart::String(span) => ConcatPart::Static(span.source_text(self.source).to_string()),
                                        token::ConcatenationPart::Expression(et) => ConcatPart::Dynamic(et.expression_span),
                                    }).collect(),
                                )
                            }
                            token::AttributeValue::Empty => {
                                debug_assert!(sd.shorthand, "Empty value on non-shorthand style directive");
                                StyleDirectiveValue::Shorthand
                            }
                        }
                    };
                    attributes.push(Attribute::StyleDirective(StyleDirective {
                        name: sd.name_span.source_text(self.source).to_string(),
                        value,
                        important: sd.important,
                    }));
                }
                token::Attribute::BindDirective(bd) => {
                    let expression_span = if bd.shorthand {
                        None
                    } else {
                        Some(bd.expression_span)
                    };
                    attributes.push(Attribute::BindDirective(BindDirective {
                        name: bd.name_span.source_text(self.source).to_string(),
                        expression_span,
                        shorthand: bd.shorthand,
                    }));
                }
                // LEGACY(svelte4): on:directive
                token::Attribute::OnDirectiveLegacy(od) => {
                    let expression_span = if od.has_expression {
                        Some(od.expression_span)
                    } else {
                        None
                    };
                    attributes.push(Attribute::OnDirectiveLegacy(OnDirectiveLegacy {
                        name: od.name_span.source_text(self.source).to_string(),
                        expression_span,
                        modifiers: od.modifiers.iter().map(|m| m.source_text(self.source).to_string()).collect(),
                    }));
                }
            }
        }

        attributes
    }
}

fn is_component_name(name: &str) -> bool {
    name.starts_with(|c: char| c.is_uppercase()) || name.contains('.')
}

struct ScriptData {
    span: Span,
    content_span: Span,
    language: ScriptLanguage,
    context: ScriptContext,
}

struct CssData {
    span: Span,
    content_span: Span,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(source: &str) -> Component {
        let (component, diagnostics) = Parser::new(source).parse();
        assert!(diagnostics.is_empty(), "unexpected diagnostics: {diagnostics:?}");
        component
    }

    fn assert_node(c: &Component, index: usize, expected: &str) {
        assert_eq!(c.source_text(c.fragment.nodes[index].span()), expected);
    }

    fn assert_script(c: &Component, expected: &str) {
        let script = c.script.as_ref().expect("expected script");
        assert_eq!(c.source_text(script.content_span), expected);
    }

    fn assert_if_block(c: &Component, index: usize, expected_test: &str) {
        if let Node::IfBlock(ref ib) = c.fragment.nodes[index] {
            assert_eq!(c.source_text(ib.test_span), expected_test);
        } else {
            panic!("expected IfBlock at index {index}");
        }
    }

    #[test]
    fn smoke_text_and_element() {
        let c = parse("prefix <div>text</div>");
        assert_node(&c, 0, "prefix ");
        assert_node(&c, 1, "<div>text</div>");
    }

    #[test]
    fn self_closed_element() {
        let c = parse("<img /><body><input/></body>");
        assert_node(&c, 0, "<img />");
        assert_node(&c, 1, "<body><input/></body>");
    }

    #[test]
    fn interpolation() {
        let c = parse("{ id - 22 + 1 }");
        assert_node(&c, 0, "{ id - 22 + 1 }");
    }

    #[test]
    fn if_block() {
        let c = parse("{#if true}<div>title</div>{/if}");
        assert_node(&c, 0, "{#if true}<div>title</div>{/if}");
        assert_if_block(&c, 0, "true");
    }

    #[test]
    fn if_else_block() {
        let c = parse("{#if true}<div>title</div>{:else}<h1>big</h1>{/if}");
        assert_node(&c, 0, "{#if true}<div>title</div>{:else}<h1>big</h1>{/if}");
        assert_if_block(&c, 0, "true");
    }

    #[test]
    fn if_elseif_else_block() {
        let c = parse("{#if false}one{:else if true}two{:else}three{/if}");
        assert_node(&c, 0, "{#if false}one{:else if true}two{:else}three{/if}");
        assert_if_block(&c, 0, "false");
    }

    #[test]
    fn each_block() {
        let c = parse("{#each values as value}item: {value}{/each}");
        assert_node(&c, 0, "{#each values as value}item: {value}{/each}");
    }

    #[test]
    fn script_tag() {
        let c = parse("<script>const i = 10;</script>");
        assert_script(&c, "const i = 10;");
    }

    #[test]
    fn script_tag_lang_ts() {
        let c = parse(r#"<script lang="ts">const i: number = 10;</script>"#);
        let script = c.script.as_ref().expect("expected script");
        assert_eq!(script.language, ScriptLanguage::TypeScript);
    }

    #[test]
    fn comment() {
        let c = parse("<!-- some comment -->");
        assert_node(&c, 0, "<!-- some comment -->");
    }

    #[test]
    fn element_with_attributes() {
        let c = parse(r#"<div lang="ts" disabled value={expr}>text</div>"#);
        assert_node(&c, 0, r#"<div lang="ts" disabled value={expr}>text</div>"#);
    }

    #[test]
    fn nested_if_in_element() {
        let c = parse("<div>{#if true}inside{/if}</div>");
        assert_node(&c, 0, "<div>{#if true}inside{/if}</div>");
    }

    #[test]
    fn unclosed_element_returns_diagnostic() {
        let (component, diagnostics) = Parser::new("<div>").parse();
        assert!(!diagnostics.is_empty(), "expected diagnostics for unclosed element");
        // AST should still contain the auto-closed element
        assert_eq!(component.fragment.nodes.len(), 1);
        assert!(component.fragment.nodes[0].is_element());
    }

    #[test]
    fn multiple_roots() {
        let c = parse("<div>a</div><span>b</span>text");
        assert_node(&c, 0, "<div>a</div>");
        assert_node(&c, 1, "<span>b</span>");
        assert_node(&c, 2, "text");
    }

    #[test]
    fn script_context_default() {
        let c = parse("<script>let x = 1;</script>");
        let script = c.script.as_ref().expect("expected script");
        assert_eq!(script.context, ScriptContext::Default);
    }

    #[test]
    fn script_context_module_attribute() {
        let c = parse("<script module>let x = 1;</script>");
        let script = c.script.as_ref().expect("expected script");
        assert_eq!(script.context, ScriptContext::Module);
    }

    #[test]
    fn script_context_module_context_attribute() {
        let c = parse(r#"<script context="module">let x = 1;</script>"#);
        let script = c.script.as_ref().expect("expected script");
        assert_eq!(script.context, ScriptContext::Module);
    }

    fn assert_snippet_block(c: &Component, index: usize, expected_name: &str, expected_params: Option<&str>) {
        if let Node::SnippetBlock(ref sb) = c.fragment.nodes[index] {
            assert_eq!(sb.name, expected_name);
            let actual_params = sb.params_span.map(|s| c.source_text(s));
            assert_eq!(actual_params, expected_params);
        } else {
            panic!("expected SnippetBlock at index {index}");
        }
    }

    fn assert_render_tag(c: &Component, index: usize, expected_expr: &str) {
        if let Node::RenderTag(ref rt) = c.fragment.nodes[index] {
            assert_eq!(c.source_text(rt.expression_span), expected_expr);
        } else {
            panic!("expected RenderTag at index {index}");
        }
    }

    #[test]
    fn snippet_block_basic() {
        let c = parse("{#snippet greeting(name)}<p>Hello {name}</p>{/snippet}");
        assert_node(&c, 0, "{#snippet greeting(name)}<p>Hello {name}</p>{/snippet}");
        assert_snippet_block(&c, 0, "greeting", Some("name"));
    }

    #[test]
    fn snippet_block_no_params() {
        let c = parse("{#snippet footer()}<p>footer</p>{/snippet}");
        assert_snippet_block(&c, 0, "footer", None);
    }

    #[test]
    fn snippet_block_multiple_params() {
        let c = parse("{#snippet card(title, body)}<div>{title} {body}</div>{/snippet}");
        assert_snippet_block(&c, 0, "card", Some("title, body"));
    }

    #[test]
    fn render_tag_basic() {
        let c = parse("{@render greeting(message)}");
        assert_render_tag(&c, 0, "greeting(message)");
    }

    #[test]
    fn render_tag_no_args() {
        let c = parse("{@render footer()}");
        assert_render_tag(&c, 0, "footer()");
    }

    #[test]
    fn render_tag_multiple_args() {
        let c = parse("{@render card(title, body)}");
        assert_render_tag(&c, 0, "card(title, body)");
    }

    #[test]
    fn snippet_and_render_together() {
        let c = parse("{#snippet greet(name)}<p>{name}</p>{/snippet}{@render greet(x)}");
        assert_snippet_block(&c, 0, "greet", Some("name"));
        assert_render_tag(&c, 1, "greet(x)");
    }

    // --- HtmlTag tests ---

    fn assert_html_tag(c: &Component, index: usize, expected_expr: &str) {
        if let Node::HtmlTag(ref ht) = c.fragment.nodes[index] {
            assert_eq!(c.source_text(ht.expression_span), expected_expr);
        } else {
            panic!("expected HtmlTag at index {index}");
        }
    }

    #[test]
    fn html_tag_basic() {
        let c = parse("{@html content}");
        assert_html_tag(&c, 0, "content");
    }

    #[test]
    fn html_tag_complex_expression() {
        let c = parse("{@html '<p>' + name + '</p>'}");
        assert_html_tag(&c, 0, "'<p>' + name + '</p>'");
    }

    // --- ConstTag tests ---

    fn assert_const_tag(c: &Component, fragment: &Fragment, index: usize, expected_decl: &str) {
        if let Node::ConstTag(ref ct) = fragment.nodes[index] {
            assert_eq!(c.source_text(ct.declaration_span), expected_decl);
        } else {
            panic!("expected ConstTag at index {index}");
        }
    }

    #[test]
    fn const_tag_basic() {
        let c = parse("{#each items as item}{@const doubled = item * 2}<p>{doubled}</p>{/each}");
        if let Node::EachBlock(ref eb) = c.fragment.nodes[0] {
            assert_const_tag(&c, &eb.body, 0, "doubled = item * 2");
        } else {
            panic!("expected EachBlock at index 0");
        }
    }

    #[test]
    fn const_tag_in_if_block() {
        let c = parse("{#if show}{@const x = count + 1}<p>{x}</p>{/if}");
        if let Node::IfBlock(ref ib) = c.fragment.nodes[0] {
            assert_const_tag(&c, &ib.consequent, 0, "x = count + 1");
        } else {
            panic!("expected IfBlock at index 0");
        }
    }

    // --- KeyBlock tests ---

    fn assert_key_block(c: &Component, index: usize, expected_expr: &str) {
        if let Node::KeyBlock(ref kb) = c.fragment.nodes[index] {
            assert_eq!(c.source_text(kb.expression_span), expected_expr);
        } else {
            panic!("expected KeyBlock at index {index}");
        }
    }

    #[test]
    fn key_block_basic() {
        let c = parse("{#key count}<div>{count}</div>{/key}");
        assert_node(&c, 0, "{#key count}<div>{count}</div>{/key}");
        assert_key_block(&c, 0, "count");
    }

    #[test]
    fn key_block_complex_expr() {
        let c = parse("{#key item.id}content{/key}");
        assert_key_block(&c, 0, "item.id");
    }

    // --- Escape sequence tests (Bug #1) ---

    #[test]
    fn interpolation_with_escaped_quotes() {
        let c = parse(r#"{ name.replace("\"", "'") }"#);
        assert_node(&c, 0, r#"{ name.replace("\"", "'") }"#);
    }

    // --- Style tag tests (Bug #2) ---

    fn assert_css(c: &Component, expected_content: &str) {
        let css = c.css.as_ref().expect("expected css");
        assert_eq!(c.source_text(css.content_span), expected_content);
    }

    #[test]
    fn style_tag() {
        let c = parse("<style>.foo { color: red; }</style>");
        assert_css(&c, ".foo { color: red; }");
    }

    #[test]
    fn style_tag_with_selectors() {
        let c = parse("<style>a > b { color: red; }</style>");
        assert_css(&c, "a > b { color: red; }");
    }

    #[test]
    fn style_tag_with_script() {
        let c = parse("<script>let x = 1;</script><style>.foo { color: red; }</style>");
        assert_script(&c, "let x = 1;");
        assert_css(&c, ".foo { color: red; }");
    }

    #[test]
    fn duplicate_style_tag_returns_diagnostic() {
        let (_, diagnostics) = Parser::new("<style>a{}</style><style>b{}</style>").parse();
        assert!(!diagnostics.is_empty(), "expected diagnostic for duplicate style");
    }

    // --- Each block key tests (Bug #3) ---

    fn assert_each_block(c: &Component, index: usize, expected_expr: &str, expected_key: Option<&str>) {
        if let Node::EachBlock(ref eb) = c.fragment.nodes[index] {
            assert_eq!(c.source_text(eb.expression_span), expected_expr);
            let actual_key = eb.key_span.map(|s| c.source_text(s));
            assert_eq!(actual_key, expected_key);
        } else {
            panic!("expected EachBlock at index {index}");
        }
    }

    #[test]
    fn each_block_with_key() {
        let c = parse("{#each items as item (item.id)}content{/each}");
        assert_each_block(&c, 0, "items", Some("item.id"));
    }

    #[test]
    fn each_block_with_index_and_key() {
        let c = parse("{#each items as item, i (item.id)}content{/each}");
        assert_each_block(&c, 0, "items", Some("item.id"));
    }

    // --- Deep nesting tests ---

    #[test]
    fn deeply_nested_blocks() {
        let c = parse("{#if a}<div>{#each items as item}{#if b}inner{/if}{/each}</div>{/if}");
        assert_node(&c, 0, "{#if a}<div>{#each items as item}{#if b}inner{/if}{/each}</div>{/if}");
    }

    // --- Directive tests at parser level ---

    #[test]
    fn class_directive_on_element() {
        let c = parse("<div class:active={isActive}>text</div>");
        assert_node(&c, 0, "<div class:active={isActive}>text</div>");
        if let Node::Element(ref el) = c.fragment.nodes[0] {
            assert_eq!(el.attributes.len(), 1);
            assert!(matches!(el.attributes[0], svelte_ast::Attribute::ClassDirective(_)));
        } else {
            panic!("expected Element");
        }
    }

    #[test]
    fn bind_directive_on_element() {
        let c = parse("<input bind:value={name}/>");
        assert_node(&c, 0, "<input bind:value={name}/>");
        if let Node::Element(ref el) = c.fragment.nodes[0] {
            assert_eq!(el.attributes.len(), 1);
            assert!(matches!(el.attributes[0], svelte_ast::Attribute::BindDirective(_)));
        } else {
            panic!("expected Element");
        }
    }

    #[test]
    fn spread_attribute_on_element() {
        let c = parse("<div {...props}>text</div>");
        if let Node::Element(ref el) = c.fragment.nodes[0] {
            assert_eq!(el.attributes.len(), 1);
            if let svelte_ast::Attribute::ShorthandOrSpread(ref sos) = el.attributes[0] {
                assert!(sos.is_spread);
            } else {
                panic!("expected ShorthandOrSpread");
            }
        } else {
            panic!("expected Element");
        }
    }

    // --- Error recovery tests ---

    fn parse_with_diagnostics(source: &str) -> (Component, Vec<Diagnostic>) {
        Parser::new(source).parse()
    }

    #[test]
    fn recovery_unclosed_element_with_text() {
        let (c, diags) = parse_with_diagnostics("<div><span>text");
        assert!(!diags.is_empty());
        // Auto-closed: div contains span, span contains text
        assert_eq!(c.fragment.nodes.len(), 1);
        assert!(c.fragment.nodes[0].is_element());
    }

    #[test]
    fn recovery_mismatched_close_tag() {
        let (c, diags) = parse_with_diagnostics("<div></span></div>");
        assert!(!diags.is_empty());
        // div should still be properly closed
        assert_eq!(c.fragment.nodes.len(), 1);
    }

    #[test]
    fn recovery_unclosed_if_block() {
        let (c, diags) = parse_with_diagnostics("{#if x}hello");
        assert!(!diags.is_empty());
        // Should produce an auto-closed IfBlock
        assert_eq!(c.fragment.nodes.len(), 1);
        assert!(c.fragment.nodes[0].is_if_block());
    }

    #[test]
    fn recovery_multiple_errors() {
        let (c, diags) = parse_with_diagnostics("<div><span>");
        assert!(diags.len() >= 2, "expected multiple diagnostics, got {}", diags.len());
        // Both unclosed elements should be auto-closed
        assert_eq!(c.fragment.nodes.len(), 1);
    }

    #[test]
    fn recovery_empty_input() {
        let (c, diags) = parse_with_diagnostics("");
        assert!(diags.is_empty());
        assert!(c.fragment.nodes.is_empty());
    }

    #[test]
    fn recovery_text_only() {
        let (c, diags) = parse_with_diagnostics("just text");
        assert!(diags.is_empty());
        assert_eq!(c.fragment.nodes.len(), 1);
        assert!(c.fragment.nodes[0].is_text());
    }

    #[test]
    fn recovery_close_tag_no_matching_open() {
        let (c, diags) = parse_with_diagnostics("</div>");
        assert!(!diags.is_empty());
        // Error node for the orphan close tag
        assert_eq!(c.fragment.nodes.len(), 1);
        assert!(matches!(c.fragment.nodes[0], Node::Error(_)));
    }

    #[test]
    fn recovery_duplicate_script_continues() {
        let (c, diags) = parse_with_diagnostics(
            "<script>let a = 1;</script><script>let b = 2;</script><div>ok</div>"
        );
        assert!(!diags.is_empty());
        // First script is kept, second is skipped, div is parsed
        assert!(c.script.is_some());
        assert_eq!(c.fragment.nodes.len(), 1);
        assert!(c.fragment.nodes[0].is_element());
    }

    #[test]
    fn recovery_unclosed_each_block() {
        let (c, diags) = parse_with_diagnostics("{#each items as item}content");
        assert!(!diags.is_empty());
        assert_eq!(c.fragment.nodes.len(), 1);
    }

    fn assert_element(c: &Component, index: usize, name: &str, self_closing: bool) {
        if let Node::Element(ref el) = c.fragment.nodes[index] {
            assert_eq!(el.name, name, "expected element name '{name}'");
            assert_eq!(el.self_closing, self_closing, "expected self_closing={self_closing} for <{name}>");
        } else {
            panic!("expected Element at index {index}");
        }
    }

    #[test]
    fn void_element_without_slash() {
        let c = parse("<input>");
        assert_element(&c, 0, "input", true);
    }

    #[test]
    fn void_element_with_slash() {
        let c = parse("<input />");
        assert_element(&c, 0, "input", true);
    }

    #[test]
    fn void_element_with_attributes() {
        let c = parse(r#"<input type="text">"#);
        assert_element(&c, 0, "input", true);
    }

    #[test]
    fn void_element_br() {
        let c = parse("<br>");
        assert_element(&c, 0, "br", true);
    }

    #[test]
    fn void_element_img() {
        let c = parse(r#"<img src="x.png">"#);
        assert_element(&c, 0, "img", true);
    }

    #[test]
    fn void_element_closing_tag_error() {
        let (_, diags) = parse_with_diagnostics("</input>");
        assert!(!diags.is_empty());
        assert!(diags.iter().any(|d| d.kind == svelte_diagnostics::DiagnosticKind::VoidElementInvalidContent));
    }

    #[test]
    fn void_element_multiple() {
        let c = parse("<input><br><hr>");
        assert_eq!(c.fragment.nodes.len(), 3);
        assert_element(&c, 0, "input", true);
        assert_element(&c, 1, "br", true);
        assert_element(&c, 2, "hr", true);
    }
}
