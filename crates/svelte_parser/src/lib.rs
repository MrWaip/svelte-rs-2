use scanner::{
    token::{self, ExpressionTag, Token, TokenType},
    Scanner,
};
use svelte_span::Span;

use svelte_ast::{
    Attribute, BindDirective, BooleanAttribute, ClassDirective, Comment,
    ConcatPart, ConcatenationAttribute, Component, EachBlock, Element,
    ExpressionAttribute, Fragment, IfBlock, Node, NodeIdAllocator, RenderTag, Script, ScriptContext,
    ScriptLanguage, ShorthandOrSpread, SnippetBlock, StringAttribute, Text,
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
// Parser
// ---------------------------------------------------------------------------

pub struct Parser<'a> {
    source: &'a str,
    ids: NodeIdAllocator,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Parser<'a> {
        Parser {
            source,
            ids: NodeIdAllocator::new(),
        }
    }

    pub fn parse(mut self) -> Result<Component, Diagnostic> {
        let mut scanner = Scanner::new(self.source);
        let tokens = scanner.scan_tokens()?;

        // children_stack[i] = children being collected for the i-th nesting level.
        // children_stack[0] = root level.
        let mut children_stack: Vec<Vec<Node>> = vec![vec![]];
        let mut entry_stack: Vec<StackEntry> = vec![];
        let mut script_data: Option<ScriptData> = None;

        for token in tokens {
            match token.token_type {
                TokenType::Text => {
                    let node = self.make_text(&token);
                    children_stack.last_mut().unwrap().push(node);
                }
                TokenType::Comment => {
                    let node = self.make_comment(&token);
                    children_stack.last_mut().unwrap().push(node);
                }
                TokenType::Interpolation(interpolation) => {
                    let node = self.make_expression_tag(&interpolation);
                    children_stack.last_mut().unwrap().push(node);
                }
                TokenType::StartTag(tag) => {
                    let attrs = self.convert_attributes(&tag.attributes)?;
                    if tag.self_closing {
                        let node = Node::Element(Element {
                            id: self.ids.next(),
                            span: token.span,
                            name: tag.name.to_string(),
                            self_closing: true,
                            attributes: attrs,
                            fragment: Fragment::empty(),
                        });
                        children_stack.last_mut().unwrap().push(node);
                    } else {
                        entry_stack.push(StackEntry::Element(ElementEntry {
                            name: tag.name.to_string(),
                            span_start: token.span,
                            attributes: attrs,
                        }));
                        children_stack.push(vec![]);
                    }
                }
                TokenType::EndTag(tag) => {
                    let entry = entry_stack
                        .pop()
                        .ok_or_else(|| Diagnostic::no_element_to_close(token.span))?;

                    let StackEntry::Element(el) = entry else {
                        return Err(Diagnostic::no_element_to_close(token.span));
                    };

                    if el.name != tag.name {
                        return Err(Diagnostic::no_element_to_close(token.span));
                    }

                    let children = children_stack.pop().unwrap();
                    let merged_span = el.span_start.merge(&token.span);

                    let node = Node::Element(Element {
                        id: self.ids.next(),
                        span: merged_span,
                        name: el.name,
                        self_closing: false,
                        attributes: el.attributes,
                        fragment: Fragment::new(children),
                    });

                    children_stack.last_mut().unwrap().push(node);
                }
                TokenType::StartIfTag(start_if) => {
                    entry_stack.push(StackEntry::IfBlock(IfBlockEntry {
                        span: token.span,
                        test_span: start_if.expression.span,
                        elseif: false,
                        consequent: None,
                        in_alternate: false,
                    }));
                    children_stack.push(vec![]); // consequent children
                }
                TokenType::ElseTag(else_tag) => {
                    // Finalize consequent: take current children as consequent
                    let consequent_children = children_stack.pop().unwrap();

                    if else_tag.elseif {
                        // {:else if expr}
                        // Store consequent in the current IfBlock entry
                        let entry = entry_stack.last_mut().ok_or_else(|| {
                            Diagnostic::no_if_block_for_else(token.span)
                        })?;
                        let StackEntry::IfBlock(ref mut ib) = entry else {
                            return Err(Diagnostic::no_if_block_for_else(token.span));
                        };
                        ib.consequent = Some(consequent_children);
                        ib.in_alternate = true;

                        // Push a children level for the parent's alternate
                        children_stack.push(vec![]);

                        // Push a new IfBlock for the else-if
                        let expr = else_tag.expression.as_ref().unwrap();
                        entry_stack.push(StackEntry::IfBlock(IfBlockEntry {
                            span: token.span,
                            test_span: expr.span,
                            elseif: true,
                            consequent: None,
                            in_alternate: false,
                        }));
                        children_stack.push(vec![]); // new consequent for else-if
                    } else {
                        // {:else}
                        let entry = entry_stack.last_mut().ok_or_else(|| {
                            Diagnostic::no_if_block_for_else(token.span)
                        })?;
                        let StackEntry::IfBlock(ref mut ib) = entry else {
                            return Err(Diagnostic::no_if_block_for_else(token.span));
                        };
                        ib.consequent = Some(consequent_children);
                        ib.in_alternate = true;
                        ib.span = ib.span.merge(&token.span);
                        children_stack.push(vec![]); // alternate children
                    }
                }
                TokenType::EndIfTag => {
                    self.close_if_chain(
                        token.span,
                        &mut entry_stack,
                        &mut children_stack,
                    )?;
                }
                TokenType::StartEachTag(each) => {
                    entry_stack.push(StackEntry::EachBlock(EachBlockEntry {
                        span: token.span,
                        expression_span: each.collection.span,
                        context_span: each.item.span,
                        index_span: each.index.map(|i| i.span),
                        key_span: each.key.map(|k| k.span),
                    }));
                    children_stack.push(vec![]); // body children
                }
                TokenType::EndEachTag => {
                    let entry = entry_stack
                        .pop()
                        .ok_or_else(|| Diagnostic::no_each_block_to_close(token.span))?;

                    let StackEntry::EachBlock(eb) = entry else {
                        return Err(Diagnostic::no_each_block_to_close(token.span));
                    };

                    let body_children = children_stack.pop().unwrap();
                    let merged_span = eb.span.merge(&token.span);

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

                    children_stack.last_mut().unwrap().push(node);
                }
                TokenType::StartSnippetTag(snippet_tag) => {
                    entry_stack.push(StackEntry::SnippetBlock(SnippetBlockEntry {
                        span_start: token.span,
                        name: snippet_tag.name.to_string(),
                        params_span: snippet_tag.params.map(|p| p.span),
                    }));
                    children_stack.push(vec![]);
                }
                TokenType::EndSnippetTag => {
                    let entry = entry_stack
                        .pop()
                        .ok_or_else(|| Diagnostic::unexpected_token(token.span))?;

                    let StackEntry::SnippetBlock(sb) = entry else {
                        return Err(Diagnostic::unexpected_token(token.span));
                    };

                    let body_children = children_stack.pop().unwrap();
                    let merged_span = sb.span_start.merge(&token.span);

                    let node = Node::SnippetBlock(SnippetBlock {
                        id: self.ids.next(),
                        span: merged_span,
                        name: sb.name,
                        params_span: sb.params_span,
                        body: Fragment::new(body_children),
                    });

                    children_stack.last_mut().unwrap().push(node);
                }
                TokenType::RenderTag(render_tag) => {
                    let node = Node::RenderTag(RenderTag {
                        id: self.ids.next(),
                        span: token.span,
                        expression_span: render_tag.expression.span,
                    });
                    children_stack.last_mut().unwrap().push(node);
                }
                TokenType::ScriptTag(script_tag) => {
                    if script_data.is_some() {
                        return Diagnostic::only_single_top_level_script(token.span).as_err();
                    }

                    let content_start = self.offset_of(script_tag.source);
                    let content_end = content_start + script_tag.source.len();

                    let language = if script_tag.is_typescript() {
                        ScriptLanguage::TypeScript
                    } else {
                        ScriptLanguage::JavaScript
                    };

                    let context = if script_tag.is_module() {
                        ScriptContext::Module
                    } else {
                        ScriptContext::Default
                    };

                    script_data = Some(ScriptData {
                        span: token.span,
                        content_span: Span::new(content_start as u32, content_end as u32),
                        language,
                        context,
                    });
                }
                TokenType::EOF => break,
            }
        }

        if !entry_stack.is_empty() {
            return Diagnostic::unclosed_node(Span::new(0, self.source.len() as u32)).as_err();
        }

        let roots = children_stack.pop().unwrap();

        let script = script_data.map(|sd| Script {
            id: self.ids.next(),
            span: sd.span,
            content_span: sd.content_span,
            context: sd.context,
            language: sd.language,
        });

        let mut component = Component::new(
            self.source.to_string(),
            Fragment::new(roots),
            script,
            None,
        );
        component.set_next_node_id(self.ids.current());

        Ok(component)
    }

    /// Close the if-block chain. Handles nested else-if blocks.
    fn close_if_chain(
        &mut self,
        end_span: Span,
        entry_stack: &mut Vec<StackEntry>,
        children_stack: &mut Vec<Vec<Node>>,
    ) -> Result<(), Diagnostic> {
        // Process from innermost to outermost if-block
        loop {
            let entry = entry_stack
                .pop()
                .ok_or_else(|| Diagnostic::no_if_block_to_close(end_span))?;

            let StackEntry::IfBlock(ib) = entry else {
                return Err(Diagnostic::no_if_block_to_close(end_span));
            };

            let last_children = children_stack.pop().unwrap();

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
                // The parent is still on the stack and is in_alternate mode.
                // Push this node as a child of the parent's alternate.
                children_stack.last_mut().unwrap().push(node);
                // Don't break — but actually the parent's {:else if} already pushed its
                // consequent. We need to check if the parent is also an if-block.

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
                children_stack.last_mut().unwrap().push(node);
                break;
            }
        }

        Ok(())
    }

    fn make_text(&mut self, token: &Token<'a>) -> Node {
        Node::Text(Text {
            id: self.ids.next(),
            span: token.span,
        })
    }

    fn make_comment(&mut self, token: &Token<'a>) -> Node {
        Node::Comment(Comment {
            id: self.ids.next(),
            span: token.span,
        })
    }

    fn make_expression_tag(&mut self, interpolation: &ExpressionTag<'a>) -> Node {
        Node::ExpressionTag(svelte_ast::ExpressionTag {
            id: self.ids.next(),
            span: interpolation.span,
            expression_span: interpolation.expression.span,
        })
    }

    fn convert_attributes(
        &mut self,
        token_attrs: &[token::Attribute<'a>],
    ) -> Result<Vec<Attribute>, Diagnostic> {
        let mut attributes = Vec::new();

        for attr in token_attrs {
            match attr {
                token::Attribute::HTMLAttribute(html_attr) => {
                    let result = match &html_attr.value {
                        token::AttributeValue::String(value) => {
                            let start = self.offset_of(value);
                            Attribute::StringAttribute(StringAttribute {
                                name: html_attr.name.to_string(),
                                value_span: Span::new(start as u32, (start + value.len()) as u32),
                            })
                        }
                        token::AttributeValue::ExpressionTag(expr_tag) => {
                            Attribute::ExpressionAttribute(ExpressionAttribute {
                                name: html_attr.name.to_string(),
                                expression_span: expr_tag.expression.span,
                                shorthand: false,
                            })
                        }
                        token::AttributeValue::Concatenation(concat) => {
                            let parts = concat
                                .parts
                                .iter()
                                .map(|part| match part {
                                    token::ConcatenationPart::String(s) => {
                                        ConcatPart::Static(s.to_string())
                                    }
                                    token::ConcatenationPart::Expression(et) => {
                                        ConcatPart::Dynamic(et.expression.span)
                                    }
                                })
                                .collect();

                            Attribute::ConcatenationAttribute(ConcatenationAttribute {
                                name: html_attr.name.to_string(),
                                parts,
                            })
                        }
                        token::AttributeValue::Empty => {
                            Attribute::BooleanAttribute(BooleanAttribute {
                                name: html_attr.name.to_string(),
                            })
                        }
                    };
                    attributes.push(result);
                }
                token::Attribute::ExpressionTag(expr_tag) => {
                    let is_spread = expr_tag.expression.value.starts_with("...");
                    attributes.push(Attribute::ShorthandOrSpread(ShorthandOrSpread {
                        expression_span: expr_tag.expression.span,
                        is_spread,
                    }));
                }
                token::Attribute::ClassDirective(cd) => {
                    let expression_span = if cd.shorthand {
                        None
                    } else {
                        Some(cd.expression.span)
                    };
                    attributes.push(Attribute::ClassDirective(ClassDirective {
                        name: cd.name.to_string(),
                        expression_span,
                        shorthand: cd.shorthand,
                    }));
                }
                token::Attribute::BindDirective(bd) => {
                    let expression_span = if bd.shorthand {
                        None
                    } else {
                        Some(bd.expression.span)
                    };
                    attributes.push(Attribute::BindDirective(BindDirective {
                        name: bd.name.to_string(),
                        expression_span,
                        shorthand: bd.shorthand,
                    }));
                }
            }
        }

        Ok(attributes)
    }

    fn offset_of(&self, s: &str) -> usize {
        let source_ptr = self.source.as_ptr() as usize;
        let s_ptr = s.as_ptr() as usize;
        debug_assert!(
            s_ptr >= source_ptr && s_ptr <= source_ptr + self.source.len(),
            "slice is not within source"
        );
        s_ptr - source_ptr
    }
}

struct ScriptData {
    span: Span,
    content_span: Span,
    language: ScriptLanguage,
    context: ScriptContext,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(source: &str) -> Component {
        Parser::new(source).parse().unwrap()
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
    fn unclosed_element_returns_error() {
        let result = Parser::new("<div>").parse();
        assert!(result.is_err());
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
}
