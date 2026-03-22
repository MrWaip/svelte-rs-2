use scanner::{
    token::{ExpressionTag, Token, TokenType},
    Scanner,
};
use svelte_span::Span;

use svelte_ast::{
    Attribute, Comment, Component, ComponentNode, ConstTag, DebugTag, Element, Fragment, HtmlTag,
    Node, NodeIdAllocator, RawBlock, RenderTag, Script, ScriptContext, ScriptLanguage, Text,
};

use svelte_diagnostics::Diagnostic;

pub mod parse_js;
pub mod scanner;
pub mod types;
mod walk_js;

mod attr_convert;
mod handlers;
mod svelte_elements;

// Re-export all shared types for convenience
pub use types::*;

// Re-export parsing functions used by svelte_analyze
pub use parse_js::{parse_script_with_alloc, parse_expression_with_alloc, parse_snippet_params, parse_await_binding};

/// Parse a standalone `.svelte.js`/`.svelte.ts` module.
///
/// Returns `(Program, Scoping)` or diagnostics on parse failure.
/// The caller (svelte_analyze) uses these to build scoping and detect runes.
pub fn parse_module<'a>(
    alloc: &'a oxc_allocator::Allocator,
    source: &str,
    is_ts: bool,
) -> Result<(oxc_ast::ast::Program<'a>, oxc_semantic::Scoping), Vec<Diagnostic>> {
    let arena_source: &'a str = alloc.alloc_str(source);
    let program = parse_js::parse_script_with_alloc(alloc, arena_source, 0, is_ts)?;
    let scoping = oxc_semantic::SemanticBuilder::new().build(&program).semantic.into_scoping();
    Ok((program, scoping))
}

/// Parse a Svelte source file and all embedded JS expressions.
///
/// Returns the parsed component AST, JS parse results (expression metadata + ASTs),
/// and any diagnostics from both the Svelte parser and JS expression parsing.
pub fn parse_with_js<'a>(
    alloc: &'a oxc_allocator::Allocator,
    source: &str,
) -> (
    svelte_ast::Component,
    crate::types::JsParseResult<'a>,
    Vec<Diagnostic>,
) {
    let (component, mut diagnostics) = Parser::new(source).parse();
    let mut js_result = crate::types::JsParseResult::new();
    walk_js::parse_js(alloc, &component, &mut js_result, &mut diagnostics);
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
