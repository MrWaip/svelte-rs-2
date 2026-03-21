use scanner::{
    token::{self, ExpressionTag, Token, TokenType},
    Scanner,
};
use svelte_span::Span;

use svelte_ast::{
    AnimateDirective, Attribute, AwaitBlock, BindDirective, BooleanAttribute, ClassDirective,
    Comment, Component, ComponentNode, ConcatPart, ConcatenationAttribute, ConstTag, CssMode,
    CustomElementConfig, DebugTag, EachBlock, Element, ExpressionAttribute, Fragment, HtmlTag,
    IfBlock, KeyBlock, Namespace, Node, NodeIdAllocator, OnDirectiveLegacy, RawBlock, RenderTag,
    Script, ScriptContext, ScriptLanguage, Shorthand, SnippetBlock, SpreadAttribute,
    StringAttribute, StyleDirective, StyleDirectiveValue, SvelteBody, SvelteBoundary,
    SvelteDocument, SvelteHead, SvelteOptions, SvelteWindow, Text, TransitionDirection,
    TransitionDirective, UseDirective,
};

use svelte_diagnostics::Diagnostic;

pub mod js_parse;
mod parse_js;
pub mod scanner;

// Re-export parsing functions used by svelte_analyze
pub use js_parse::{parse_script_with_alloc, parse_expression_with_alloc, parse_snippet_params, parse_await_binding};

/// Parse a Svelte source file and all embedded JS expressions.
///
/// Returns the parsed component AST, JS parse results (expression metadata + ASTs),
/// and any diagnostics from both the Svelte parser and JS expression parsing.
pub fn parse_with_js<'a>(
    alloc: &'a oxc_allocator::Allocator,
    source: &str,
) -> (
    svelte_ast::Component,
    svelte_types::JsParseResult<'a>,
    Vec<Diagnostic>,
) {
    let (component, mut diagnostics) = Parser::new(source).parse();
    let mut js_result = svelte_types::JsParseResult::new();
    parse_js::parse_js(alloc, &component, &mut js_result, &mut diagnostics);
    (component, js_result, diagnostics)
}

// ---------------------------------------------------------------------------
// Stack entry — stores partial data while we parse nested structures
// ---------------------------------------------------------------------------

enum StackEntry {
    Element(ElementEntry),
    IfBlock(IfBlockEntry),
    EachBlock(EachBlockEntry),
    SnippetBlock(SnippetBlockEntry),
    KeyBlock(KeyBlockEntry),
    AwaitBlock(AwaitBlockEntry),
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
    /// Body children, set when `{:else}` switches to fallback collection.
    body_children: Option<Vec<Node>>,
    in_fallback: bool,
}

struct SnippetBlockEntry {
    span_start: Span,
    name: String,
    params_span: Option<Span>,
}

/// Tracks which sub-fragment is currently being collected.
enum AwaitPhase {
    Pending,
    Then,
    Catch,
}

struct AwaitBlockEntry {
    span: Span,
    expression_span: Span,
    value_span: Option<Span>,
    error_span: Option<Span>,
    /// Which phase we are currently collecting children for.
    phase: AwaitPhase,
    /// Pending children (collected before {:then}).
    pending_children: Option<Vec<Node>>,
    /// Then children (collected between {:then} and {:catch}).
    then_children: Option<Vec<Node>>,
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
                    self.handle_end_tag(&tag, token.span, &mut entry_stack, &mut children_stack);
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
                    self.close_if_chain(token.span, &mut entry_stack, &mut children_stack);
                }
                TokenType::StartEachTag(each) => {
                    entry_stack.push(StackEntry::EachBlock(EachBlockEntry {
                        span: token.span,
                        expression_span: each.collection_span,
                        context_span: each.context_span,
                        index_span: each.index_span,
                        key_span: each.key_span,
                        body_children: None,
                        in_fallback: false,
                    }));
                    children_stack.push(vec![]); // body children
                }
                TokenType::EndEachTag => {
                    self.handle_end_each_tag(token.span, &mut entry_stack, &mut children_stack);
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
                    self.handle_end_snippet_tag(token.span, &mut entry_stack, &mut children_stack);
                }
                TokenType::StartKeyTag(key_tag) => {
                    entry_stack.push(StackEntry::KeyBlock(KeyBlockEntry {
                        span: token.span,
                        expression_span: key_tag.expression_span,
                    }));
                    children_stack.push(vec![]);
                }
                TokenType::EndKeyTag => {
                    self.handle_end_key_tag(token.span, &mut entry_stack, &mut children_stack);
                }
                TokenType::StartAwaitTag(await_tag) => {
                    use scanner::token::AwaitInitialClause;
                    let phase = match await_tag.initial_clause {
                        AwaitInitialClause::Pending => AwaitPhase::Pending,
                        AwaitInitialClause::Then => AwaitPhase::Then,
                        AwaitInitialClause::Catch => AwaitPhase::Catch,
                    };
                    entry_stack.push(StackEntry::AwaitBlock(AwaitBlockEntry {
                        span: token.span,
                        expression_span: await_tag.expression_span,
                        value_span: await_tag.value_span,
                        error_span: await_tag.error_span,
                        phase,
                        pending_children: None,
                        then_children: None,
                    }));
                    children_stack.push(vec![]);
                }
                TokenType::AwaitClauseTag(clause_tag) => {
                    self.handle_await_clause_tag(
                        &clause_tag,
                        &mut entry_stack,
                        &mut children_stack,
                    );
                }
                TokenType::EndAwaitTag => {
                    self.handle_end_await_tag(token.span, &mut entry_stack, &mut children_stack);
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
                TokenType::DebugTag(dt) => {
                    let node = Node::DebugTag(DebugTag {
                        id: self.ids.next(),
                        span: token.span,
                        identifiers: dt.identifiers,
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

        let mut component =
            Component::new(self.source.to_string(), Fragment::new(roots), script, css);
        // Extract <svelte:options> from fragment (must be top-level)
        self.extract_svelte_options(&mut component);

        // Convert <svelte:head> elements to SvelteHead nodes
        Self::convert_svelte_head(&mut component);

        // Convert <svelte:window> elements to SvelteWindow nodes
        Self::convert_svelte_window(&mut component);

        // Convert <svelte:document> elements to SvelteDocument nodes
        Self::convert_svelte_document(&mut component);

        // Convert <svelte:body> elements to SvelteBody nodes
        Self::convert_svelte_body(&mut component);

        // Convert <svelte:element> elements to SvelteElement nodes
        Self::convert_svelte_element(&mut component.fragment);

        // Convert <svelte:boundary> elements to SvelteBoundary nodes
        Self::convert_svelte_boundary(&mut component.fragment);

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

        let last_children = pop_children(children_stack);
        let merged_span = eb.span.merge(&span);

        // If {:else} was encountered, body_children were saved and last_children are fallback
        let (body_children, fallback) = if eb.in_fallback {
            let body = eb.body_children.unwrap_or_default();
            (body, Some(Fragment::new(last_children)))
        } else {
            (last_children, None)
        };

        let node = Node::EachBlock(EachBlock {
            id: self.ids.next(),
            span: merged_span,
            expression_span: eb.expression_span,
            context_span: eb.context_span,
            index_span: eb.index_span,
            key_span: eb.key_span,
            body: Fragment::new(body_children),
            fallback,
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

    fn handle_await_clause_tag(
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

    fn handle_end_await_tag(
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
    fn auto_close_entry(&mut self, entry: StackEntry, children_stack: &mut Vec<Vec<Node>>) {
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

                let node = Node::EachBlock(EachBlock {
                    id: self.ids.next(),
                    span: merged_span,
                    expression_span: eb.expression_span,
                    context_span: eb.context_span,
                    index_span: eb.index_span,
                    key_span: eb.key_span,
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

    fn convert_attributes(&mut self, token_attrs: &[token::Attribute]) -> Vec<Attribute> {
        let mut attributes = Vec::new();

        for attr in token_attrs {
            match attr {
                token::Attribute::HTMLAttribute(html_attr) => {
                    let result = match &html_attr.value {
                        token::AttributeValue::String(span) => {
                            Attribute::StringAttribute(StringAttribute {
                                id: self.ids.next(),
                                name: html_attr.name_span.source_text(self.source).to_string(),
                                value_span: *span,
                            })
                        }
                        token::AttributeValue::ExpressionTag(expr_tag) => {
                            let name = html_attr.name_span.source_text(self.source).to_string();
                            let event_name = name.strip_prefix("on").map(|s| s.to_string());
                            Attribute::ExpressionAttribute(ExpressionAttribute {
                                id: self.ids.next(),
                                name,
                                expression_span: expr_tag.expression_span,
                                shorthand: false,
                                event_name,
                            })
                        }
                        token::AttributeValue::Concatenation(concat) => {
                            let parts = concat
                                .parts
                                .iter()
                                .map(|part| match part {
                                    token::ConcatenationPart::String(span) => ConcatPart::Static(
                                        span.source_text(self.source).to_string(),
                                    ),
                                    token::ConcatenationPart::Expression(et) => {
                                        ConcatPart::Dynamic(et.expression_span)
                                    }
                                })
                                .collect();

                            Attribute::ConcatenationAttribute(ConcatenationAttribute {
                                id: self.ids.next(),
                                name: html_attr.name_span.source_text(self.source).to_string(),
                                parts,
                            })
                        }
                        token::AttributeValue::Empty => {
                            Attribute::BooleanAttribute(BooleanAttribute {
                                id: self.ids.next(),
                                name: html_attr.name_span.source_text(self.source).to_string(),
                            })
                        }
                    };
                    attributes.push(result);
                }
                token::Attribute::ExpressionTag(expr_tag) => {
                    if expr_tag
                        .expression_span
                        .source_text(self.source)
                        .starts_with("...")
                    {
                        attributes.push(Attribute::SpreadAttribute(SpreadAttribute {
                            id: self.ids.next(),
                            expression_span: expr_tag.expression_span,
                        }));
                    } else {
                        attributes.push(Attribute::Shorthand(Shorthand {
                            id: self.ids.next(),
                            expression_span: expr_tag.expression_span,
                        }));
                    }
                }
                token::Attribute::ClassDirective(cd) => {
                    let expression_span = if cd.shorthand {
                        None
                    } else {
                        Some(cd.expression_span)
                    };
                    attributes.push(Attribute::ClassDirective(ClassDirective {
                        id: self.ids.next(),
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
                            token::AttributeValue::String(span) => StyleDirectiveValue::String(
                                span.source_text(self.source).to_string(),
                            ),
                            token::AttributeValue::Concatenation(c) => {
                                StyleDirectiveValue::Concatenation(
                                    c.parts
                                        .iter()
                                        .map(|p| match p {
                                            token::ConcatenationPart::String(span) => {
                                                ConcatPart::Static(
                                                    span.source_text(self.source).to_string(),
                                                )
                                            }
                                            token::ConcatenationPart::Expression(et) => {
                                                ConcatPart::Dynamic(et.expression_span)
                                            }
                                        })
                                        .collect(),
                                )
                            }
                            token::AttributeValue::Empty => {
                                debug_assert!(
                                    sd.shorthand,
                                    "Empty value on non-shorthand style directive"
                                );
                                StyleDirectiveValue::Shorthand
                            }
                        }
                    };
                    attributes.push(Attribute::StyleDirective(StyleDirective {
                        id: self.ids.next(),
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
                        id: self.ids.next(),
                        name: bd.name_span.source_text(self.source).to_string(),
                        expression_span,
                        shorthand: bd.shorthand,
                    }));
                }
                token::Attribute::UseDirective(ud) => {
                    let expression_span = if ud.shorthand {
                        None
                    } else {
                        Some(ud.expression_span)
                    };
                    attributes.push(Attribute::UseDirective(UseDirective {
                        id: self.ids.next(),
                        name: ud.name_span,
                        expression_span,
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
                        id: self.ids.next(),
                        name: od.name_span.source_text(self.source).to_string(),
                        expression_span,
                        modifiers: od
                            .modifiers
                            .iter()
                            .map(|m| m.source_text(self.source).to_string())
                            .collect(),
                    }));
                }
                token::Attribute::TransitionDirective(td) => {
                    let expression_span = if td.has_expression {
                        Some(td.expression_span)
                    } else {
                        None
                    };
                    let direction = match td.direction_prefix.as_str() {
                        "in" => TransitionDirection::In,
                        "out" => TransitionDirection::Out,
                        _ => TransitionDirection::Both,
                    };
                    attributes.push(Attribute::TransitionDirective(TransitionDirective {
                        id: self.ids.next(),
                        name: td.name_span,
                        expression_span,
                        modifiers: td
                            .modifiers
                            .iter()
                            .map(|m| m.source_text(self.source).to_string())
                            .collect(),
                        direction,
                    }));
                }
                token::Attribute::AnimateDirective(ad) => {
                    let expression_span = if ad.has_expression {
                        Some(ad.expression_span)
                    } else {
                        None
                    };
                    attributes.push(Attribute::AnimateDirective(AnimateDirective {
                        id: self.ids.next(),
                        name: ad.name_span,
                        expression_span,
                    }));
                }
                token::Attribute::AttachTag(at) => {
                    attributes.push(Attribute::AttachTag(svelte_ast::AttachTag {
                        id: self.ids.next(),
                        expression_span: at.expression_span,
                    }));
                }
            }
        }

        attributes
    }

    // -----------------------------------------------------------------------
    // <svelte:options> extraction
    // -----------------------------------------------------------------------

    fn extract_svelte_options(&mut self, component: &mut Component) {
        let options_idx = component
            .fragment
            .nodes
            .iter()
            .position(|n| n.as_element().is_some_and(|el| el.name == "svelte:options"));

        let Some(idx) = options_idx else {
            return;
        };

        let node = component.fragment.nodes.remove(idx);
        let Node::Element(el) = node else {
            unreachable!();
        };

        // Check for duplicate <svelte:options>
        let has_another = component
            .fragment
            .nodes
            .iter()
            .any(|n| n.as_element().is_some_and(|e| e.name == "svelte:options"));
        if has_another {
            self.recover(Diagnostic::svelte_options_duplicate(el.span));
        }

        // Validate no children
        if !el.fragment.is_empty() {
            self.recover(Diagnostic::svelte_options_no_children(el.span));
        }

        component.options = Some(self.read_svelte_options(&el));
    }

    fn read_svelte_options(&mut self, el: &Element) -> SvelteOptions {
        let mut options = SvelteOptions {
            span: el.span,
            runes: None,
            namespace: None,
            css: None,
            custom_element: None,
            immutable: None,
            accessors: None,
            preserve_whitespace: None,
            attributes: el.attributes.clone(),
        };

        for attr in &el.attributes {
            match attr {
                Attribute::BooleanAttribute(ba) => {
                    self.process_svelte_option_bool(&ba.name, true, el.span, &mut options);
                }
                Attribute::StringAttribute(sa) => {
                    let value = sa.value_span.source_text(self.source).to_string();
                    self.process_svelte_option_string(&sa.name, &value, el.span, &mut options);
                }
                Attribute::ExpressionAttribute(ea) => {
                    let expr_text = ea.expression_span.source_text(self.source).trim();
                    match expr_text {
                        "true" => {
                            self.process_svelte_option_bool(&ea.name, true, el.span, &mut options);
                        }
                        "false" => {
                            self.process_svelte_option_bool(&ea.name, false, el.span, &mut options);
                        }
                        _ => {
                            // Could be an object expression for customElement
                            if ea.name == "customElement" {
                                self.process_custom_element_expression(
                                    ea.expression_span,
                                    el.span,
                                    &mut options,
                                );
                            } else {
                                self.recover(Diagnostic::svelte_options_invalid_attribute(el.span));
                            }
                        }
                    }
                }
                _ => {
                    // Directives and other non-standard attributes are not allowed
                    self.recover(Diagnostic::svelte_options_invalid_attribute(el.span));
                }
            }
        }

        options
    }

    fn process_svelte_option_bool(
        &mut self,
        name: &str,
        value: bool,
        span: Span,
        options: &mut SvelteOptions,
    ) {
        match name {
            "runes" => options.runes = Some(value),
            "immutable" => options.immutable = Some(value),
            "accessors" => options.accessors = Some(value),
            "preserveWhitespace" => options.preserve_whitespace = Some(value),
            "namespace" | "css" | "customElement" => {
                self.recover(Diagnostic::svelte_options_invalid_attribute_value(
                    span,
                    "a string value".into(),
                ));
            }
            // LEGACY(svelte4): `tag` renamed to `customElement`
            "tag" => {
                self.recover(Diagnostic::svelte_options_deprecated_tag(span));
            }
            _ => {
                self.recover(Diagnostic::svelte_options_unknown_attribute(
                    span,
                    name.to_string(),
                ));
            }
        }
    }

    fn process_svelte_option_string(
        &mut self,
        name: &str,
        value: &str,
        span: Span,
        options: &mut SvelteOptions,
    ) {
        match name {
            "namespace" => match value {
                "html" => options.namespace = Some(Namespace::Html),
                "svg" | "http://www.w3.org/2000/svg" => options.namespace = Some(Namespace::Svg),
                "mathml" | "http://www.w3.org/1998/Math/MathML" => {
                    options.namespace = Some(Namespace::Mathml)
                }
                _ => {
                    self.recover(Diagnostic::svelte_options_invalid_attribute_value(
                        span,
                        r#""html", "mathml" or "svg""#.into(),
                    ));
                }
            },
            "css" => {
                if value == "injected" {
                    options.css = Some(CssMode::Injected);
                } else {
                    self.recover(Diagnostic::svelte_options_invalid_attribute_value(
                        span,
                        r#""injected""#.into(),
                    ));
                }
            }
            "customElement" => {
                if let Some(tag_err) = validate_custom_element_tag(value) {
                    match tag_err {
                        TagError::Invalid => {
                            self.recover(Diagnostic::svelte_options_invalid_custom_element_tag(
                                span,
                            ));
                        }
                        TagError::Reserved => {
                            self.recover(Diagnostic::svelte_options_reserved_tag_name(span));
                        }
                    }
                } else {
                    options.custom_element = Some(CustomElementConfig::Tag(value.to_string()));
                }
            }
            "runes" | "immutable" | "accessors" | "preserveWhitespace" => {
                self.recover(Diagnostic::svelte_options_invalid_attribute_value(
                    span,
                    "true or false".into(),
                ));
            }
            // LEGACY(svelte4): `tag` renamed to `customElement`
            "tag" => {
                self.recover(Diagnostic::svelte_options_deprecated_tag(span));
            }
            _ => {
                self.recover(Diagnostic::svelte_options_unknown_attribute(
                    span,
                    name.to_string(),
                ));
            }
        }
    }

    fn process_custom_element_expression(
        &mut self,
        expression_span: Span,
        el_span: Span,
        options: &mut SvelteOptions,
    ) {
        let expr_text = expression_span.source_text(self.source).trim();

        // `null` is backwards compat from Svelte 4 — just ignore
        if expr_text == "null" {
            return;
        }

        // Must be an object expression
        if !expr_text.starts_with('{') {
            self.recover(Diagnostic::svelte_options_invalid_attribute(el_span));
            return;
        }

        // Store the expression span; full object parsing deferred to analysis
        options.custom_element = Some(CustomElementConfig::Expression(expression_span));
    }

    // -----------------------------------------------------------------------
    // <svelte:head> conversion
    // -----------------------------------------------------------------------

    /// Convert `<svelte:head>` Element nodes in the root fragment to SvelteHead nodes.
    fn convert_svelte_head(component: &mut Component) {
        for node in &mut component.fragment.nodes {
            if let Node::Element(el) = node {
                if el.name == "svelte:head" {
                    let head = SvelteHead {
                        id: el.id,
                        span: el.span,
                        fragment: std::mem::replace(&mut el.fragment, Fragment::empty()),
                    };
                    *node = Node::SvelteHead(head);
                }
            }
        }
    }

    /// Convert `<svelte:window>` Element nodes in the root fragment to SvelteWindow nodes.
    fn convert_svelte_window(component: &mut Component) {
        for node in &mut component.fragment.nodes {
            if let Node::Element(el) = node {
                if el.name == "svelte:window" {
                    let window = SvelteWindow {
                        id: el.id,
                        span: el.span,
                        attributes: std::mem::take(&mut el.attributes),
                        fragment: std::mem::replace(&mut el.fragment, Fragment::empty()),
                    };
                    *node = Node::SvelteWindow(window);
                }
            }
        }
    }

    /// Convert `<svelte:document>` Element nodes in the root fragment to SvelteDocument nodes.
    fn convert_svelte_document(component: &mut Component) {
        for node in &mut component.fragment.nodes {
            if let Node::Element(el) = node {
                if el.name == "svelte:document" {
                    let doc = SvelteDocument {
                        id: el.id,
                        span: el.span,
                        attributes: std::mem::take(&mut el.attributes),
                        fragment: std::mem::replace(&mut el.fragment, Fragment::empty()),
                    };
                    *node = Node::SvelteDocument(doc);
                }
            }
        }
    }

    /// Convert `<svelte:body>` Element nodes in the root fragment to SvelteBody nodes.
    fn convert_svelte_body(component: &mut Component) {
        for node in &mut component.fragment.nodes {
            if let Node::Element(el) = node {
                if el.name == "svelte:body" {
                    let body = SvelteBody {
                        id: el.id,
                        span: el.span,
                        attributes: std::mem::take(&mut el.attributes),
                        fragment: std::mem::replace(&mut el.fragment, Fragment::empty()),
                    };
                    *node = Node::SvelteBody(body);
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // <svelte:element> conversion
    // -----------------------------------------------------------------------

    /// Convert `<svelte:element this={expr}>` Element nodes to SvelteElement nodes.
    /// Unlike svelte:head, these can appear anywhere in the tree, so we walk recursively.
    fn convert_svelte_element(fragment: &mut Fragment) {
        for node in &mut fragment.nodes {
            match node {
                Node::Element(el) if el.name == "svelte:element" => {
                    let (tag_span, static_tag) = Self::extract_this_attribute(&mut el.attributes);
                    let mut svelte_el = svelte_ast::SvelteElement {
                        id: el.id,
                        span: el.span,
                        tag_span,
                        static_tag,
                        attributes: std::mem::take(&mut el.attributes),
                        fragment: std::mem::replace(&mut el.fragment, Fragment::empty()),
                    };
                    Self::convert_svelte_element(&mut svelte_el.fragment);
                    *node = Node::SvelteElement(svelte_el);
                }
                Node::Element(el) => Self::convert_svelte_element(&mut el.fragment),
                Node::ComponentNode(cn) => Self::convert_svelte_element(&mut cn.fragment),
                Node::IfBlock(block) => {
                    Self::convert_svelte_element(&mut block.consequent);
                    if let Some(alt) = &mut block.alternate {
                        Self::convert_svelte_element(alt);
                    }
                }
                Node::EachBlock(block) => {
                    Self::convert_svelte_element(&mut block.body);
                    if let Some(fallback) = &mut block.fallback {
                        Self::convert_svelte_element(fallback);
                    }
                }
                Node::SnippetBlock(block) => Self::convert_svelte_element(&mut block.body),
                Node::KeyBlock(block) => Self::convert_svelte_element(&mut block.fragment),
                Node::SvelteHead(head) => Self::convert_svelte_element(&mut head.fragment),
                Node::SvelteElement(el) => Self::convert_svelte_element(&mut el.fragment),
                Node::SvelteBoundary(b) => Self::convert_svelte_element(&mut b.fragment),
                _ => {}
            }
        }
    }

    /// Convert `<svelte:boundary>` Element nodes to SvelteBoundary nodes.
    /// Recursive — boundary can appear anywhere in the template.
    fn convert_svelte_boundary(fragment: &mut Fragment) {
        for node in &mut fragment.nodes {
            match node {
                Node::Element(el) if el.name == "svelte:boundary" => {
                    let mut boundary = SvelteBoundary {
                        id: el.id,
                        span: el.span,
                        attributes: std::mem::take(&mut el.attributes),
                        fragment: std::mem::replace(&mut el.fragment, Fragment::empty()),
                    };
                    Self::convert_svelte_boundary(&mut boundary.fragment);
                    *node = Node::SvelteBoundary(boundary);
                }
                Node::Element(el) => Self::convert_svelte_boundary(&mut el.fragment),
                Node::ComponentNode(cn) => Self::convert_svelte_boundary(&mut cn.fragment),
                Node::IfBlock(block) => {
                    Self::convert_svelte_boundary(&mut block.consequent);
                    if let Some(alt) = &mut block.alternate {
                        Self::convert_svelte_boundary(alt);
                    }
                }
                Node::EachBlock(block) => {
                    Self::convert_svelte_boundary(&mut block.body);
                    if let Some(fallback) = &mut block.fallback {
                        Self::convert_svelte_boundary(fallback);
                    }
                }
                Node::SnippetBlock(block) => Self::convert_svelte_boundary(&mut block.body),
                Node::KeyBlock(block) => Self::convert_svelte_boundary(&mut block.fragment),
                Node::SvelteHead(head) => Self::convert_svelte_boundary(&mut head.fragment),
                Node::SvelteElement(el) => Self::convert_svelte_boundary(&mut el.fragment),
                Node::SvelteBoundary(b) => Self::convert_svelte_boundary(&mut b.fragment),
                _ => {}
            }
        }
    }

    /// Extract the `this` attribute from an attribute list, returning its expression span.
    /// Removes the `this` attribute from the vec.
    /// Returns (tag_span, is_static) — is_static is true for `this="literal"`.
    fn extract_this_attribute(attributes: &mut Vec<svelte_ast::Attribute>) -> (Span, bool) {
        let pos = attributes.iter().position(|attr| match attr {
            svelte_ast::Attribute::ExpressionAttribute(a) => a.name == "this",
            svelte_ast::Attribute::StringAttribute(a) => a.name == "this",
            _ => false,
        });

        if let Some(idx) = pos {
            let attr = attributes.remove(idx);
            match attr {
                svelte_ast::Attribute::ExpressionAttribute(a) => (a.expression_span, false),
                svelte_ast::Attribute::StringAttribute(a) => (a.value_span, true),
                _ => unreachable!(),
            }
        } else {
            // Missing `this` attribute — use empty span as fallback
            (Span::new(0, 0), false)
        }
    }
}

// Custom element tag name validation
enum TagError {
    Invalid,
    Reserved,
}

fn validate_custom_element_tag(tag: &str) -> Option<TagError> {
    if tag.is_empty() {
        return None; // Empty tag is allowed (means "no tag")
    }

    // Must start with lowercase letter, contain a hyphen, and only valid chars
    let is_valid = tag.starts_with(|c: char| c.is_ascii_lowercase())
        && tag.contains('-')
        && tag.chars().all(|c| {
            c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '.' || c == '_'
        });

    if !is_valid {
        return Some(TagError::Invalid);
    }

    const RESERVED: &[&str] = &[
        "annotation-xml",
        "color-profile",
        "font-face",
        "font-face-src",
        "font-face-uri",
        "font-face-format",
        "font-face-name",
        "missing-glyph",
    ];

    if RESERVED.contains(&tag) {
        return Some(TagError::Reserved);
    }

    None
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

#[cfg(test)]
mod tests;
