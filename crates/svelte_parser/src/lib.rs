use std::mem;

use oxc_allocator::Allocator;
use oxc_ast::ast::Program;
use scanner::{Scanner, token::TokenType};
use svelte_span::Span;

use svelte_ast::{
    AstStore, Attribute, Comment, Component, ComponentNode, ConstTag, DebugTag, DeclarationTag,
    Element, FragmentId, FragmentRole, HtmlTag, Node, NodeId, RawBlock, RenderTag,
    SVELTE_COMPONENT, SVELTE_SELF, Script, ScriptContext, ScriptLanguage, Text,
};

use svelte_diagnostics::Diagnostic;

mod html;
mod html_entities;
mod js_postprocess;
pub mod parse_js;
pub mod scanner;
pub mod types;
mod walk_js;

mod attr_convert;
mod handlers;
mod svelte_elements;
use svelte_elements::for_each_child_fragment;

pub use types::{CeDomMode, CePropConfig, JsAst, ParsedCeConfig};

pub fn parse_module<'a>(
    alloc: &'a Allocator,
    source: &str,
    is_ts: bool,
) -> Result<Program<'a>, Vec<Diagnostic>> {
    let arena_source: &'a str = alloc.alloc_str(source);
    parse_js::parse_script_with_alloc(alloc, arena_source, 0, is_ts)
}

pub fn parse_with_js<'a>(
    alloc: &'a Allocator,
    source: &str,
) -> (svelte_ast::Component, JsAst<'a>, Vec<Diagnostic>) {
    let trimmed = source.trim_end();
    let (component, mut diagnostics) = Parser::new(trimmed).parse();
    let mut result = JsAst::new();
    walk_js::parse_js(alloc, &component, &mut result, &mut diagnostics);
    reduce_to_first_error(&mut diagnostics);

    (component, result, diagnostics)
}

fn reduce_to_first_error(diagnostics: &mut Vec<Diagnostic>) {
    let mut seen_error = false;
    diagnostics.retain(|diagnostic| {
        if diagnostic.severity == svelte_diagnostics::Severity::Error {
            if seen_error {
                return false;
            }
            seen_error = true;
        }
        true
    });
}

pub fn parse_css_block(
    component: &svelte_ast::Component,
) -> Option<(svelte_css::StyleSheet, Vec<svelte_diagnostics::Diagnostic>)> {
    let css_block = component.css.as_ref()?;
    let css_text = component.source_text(css_block.content_span);
    let (stylesheet, diags) = svelte_css::parse(css_text);
    Some((stylesheet, diags))
}

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
    name_span: Span,
    span_start: Span,
    attributes: Vec<Attribute>,
}

struct IfBlockEntry {
    span: Span,
    test_span: Span,
    elseif: bool,

    consequent: Option<Vec<NodeId>>,

    in_alternate: bool,
}

struct EachBlockEntry {
    span: Span,
    expression_span: Span,
    context_span: Option<Span>,
    index_span: Option<Span>,
    key_span: Option<Span>,

    body_children: Option<Vec<NodeId>>,
    in_fallback: bool,
}

struct SnippetBlockEntry {
    span_start: Span,
    expression_span: Span,
}

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

    phase: AwaitPhase,

    pending_children: Option<Vec<NodeId>>,

    then_children: Option<Vec<NodeId>>,

    catch_children: Option<Vec<NodeId>>,
}

#[allow(clippy::ptr_arg)]
fn push_child(children_stack: &mut Vec<Vec<NodeId>>, id: NodeId) {
    debug_assert!(
        !children_stack.is_empty(),
        "children_stack empty when pushing child"
    );
    if let Some(children) = children_stack.last_mut() {
        children.push(id);
    }
}

fn pop_children(children_stack: &mut Vec<Vec<NodeId>>) -> Vec<NodeId> {
    debug_assert!(
        !children_stack.is_empty(),
        "children_stack empty when popping"
    );
    children_stack.pop().unwrap_or_default()
}

pub struct Parser<'a> {
    source: &'a str,
    store: AstStore,
    diagnostics: Vec<Diagnostic>,
    last_auto_closed_tag: Option<AutoClosedTag>,
    has_error: bool,
}

pub(crate) struct AutoClosedTag {
    pub(crate) tag: String,
    pub(crate) reason: String,
    pub(crate) depth: usize,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Parser<'a> {
        Parser {
            source,
            store: AstStore::with_capacity(source.len() / 10),
            diagnostics: Vec::new(),
            last_auto_closed_tag: None,
            has_error: false,
        }
    }

    fn recover(&mut self, diagnostic: Diagnostic) {
        if diagnostic.severity == svelte_diagnostics::Severity::Error {
            self.has_error = true;
        }
        self.diagnostics.push(diagnostic);
    }

    fn push_node(&mut self, node: Node) -> NodeId {
        self.store.push(node)
    }

    fn reserve_id(&mut self) -> NodeId {
        self.store.reserve()
    }

    fn preceding_comment_id(&self, siblings: &[NodeId]) -> Option<NodeId> {
        for &id in siblings.iter().rev() {
            match self.store.get(id) {
                Node::Comment(comment) => return Some(comment.id),
                Node::Text(text) => {
                    if text.span.source_text(self.source).trim().is_empty() {
                        continue;
                    }
                    return None;
                }
                _ => return None,
            }
        }
        None
    }

    pub(crate) fn new_fragment(&mut self, role: FragmentRole, nodes: Vec<NodeId>) -> FragmentId {
        self.store.push_fragment(role, nodes)
    }

    pub(crate) fn empty_fragment(&mut self, role: FragmentRole) -> FragmentId {
        self.store.reserve_fragment(role)
    }

    pub(crate) fn partition_component_children(
        &mut self,
        children: Vec<NodeId>,
    ) -> (Vec<NodeId>, Vec<svelte_ast::LegacySlot>) {
        let mut default = Vec::with_capacity(children.len());
        let mut slots: Vec<svelte_ast::LegacySlot> = Vec::new();

        for child in children {
            match self.slot_name_of(child) {
                Some(name) => {
                    if let Some(existing) = slots.iter_mut().find(|s| s.name == name) {
                        self.store.fragment_mut(existing.fragment).nodes.push(child);
                    } else {
                        let fragment = self.empty_fragment(FragmentRole::NamedSlot);
                        self.store.fragment_mut(fragment).nodes.push(child);
                        slots.push(svelte_ast::LegacySlot { name, fragment });
                    }
                }
                None => default.push(child),
            }
        }

        (default, slots)
    }

    fn slot_name_of(&self, child: NodeId) -> Option<String> {
        let attrs = match self.store.get(child) {
            Node::Element(el) => &el.attributes,
            Node::ComponentNode(cn) => &cn.attributes,
            Node::SvelteSelf(cn) => &cn.attributes,
            Node::SvelteComponentLegacy(cn) => &cn.attributes,
            _ => return None,
        };

        for attr in attrs {
            if let Attribute::StringAttribute(sa) = attr
                && sa.name == "slot"
            {
                let value = sa.value(self.source);
                if value.is_empty() || value == "default" {
                    return None;
                }
                return Some(value.to_string());
            }
        }
        None
    }

    pub fn parse(mut self) -> (Component, Vec<Diagnostic>) {
        let mut scanner = Scanner::new(self.source);
        let (tokens, scan_diagnostics) = scanner.scan_tokens();
        self.diagnostics.extend(scan_diagnostics);
        let mut js_comment_expr_starts = scanner.take_js_comment_expr_starts();
        js_comment_expr_starts.sort_unstable();

        let mut children_stack: Vec<Vec<NodeId>> = vec![vec![]];
        let mut entry_stack: Vec<StackEntry> = vec![];
        let mut instance_script_data: Option<ScriptData> = None;
        let mut module_script_data: Option<ScriptData> = None;
        let mut css_data: Option<CssData> = None;

        for token in tokens {
            match token.token_type {
                TokenType::Text => {
                    let raw = token.span.source_text(self.source);
                    let id = self.push_node(Node::Text(Text {
                        id: NodeId(0),
                        span: token.span,
                        decoded: html::decode_text(raw),
                    }));
                    push_child(&mut children_stack, id);
                }
                TokenType::Comment { content_span } => {
                    let id = self.push_node(Node::Comment(Comment {
                        id: NodeId(0),
                        span: token.span,
                        content_span,
                    }));
                    push_child(&mut children_stack, id);
                }
                TokenType::Interpolation(interpolation) => {
                    let id = self.push_node(Node::ExpressionTag(svelte_ast::ExpressionTag {
                        id: NodeId(0),
                        span: interpolation.span,
                        expression: svelte_ast::ExprRef::new(interpolation.expression_span),
                    }));
                    push_child(&mut children_stack, id);
                }
                TokenType::StartTag(tag) => {
                    let name = tag.name_span.source_text(self.source);
                    let is_component = is_component_name(name);

                    let closed_name = match entry_stack.last() {
                        Some(StackEntry::Element(top))
                            if svelte_ast::closing_tag_omitted(&top.name, name) =>
                        {
                            Some(top.name.clone())
                        }
                        _ => None,
                    };
                    if let Some(closed_name) = closed_name {
                        let following_tag = name.to_string();
                        let following_start = token.span.start;
                        if let Some(entry) = entry_stack.pop() {
                            self.auto_close_entry(
                                entry,
                                &mut children_stack,
                                &handlers::CloseReason::ImplicitOpen {
                                    following_tag: following_tag.clone(),
                                    following_start,
                                },
                            );
                        }
                        self.last_auto_closed_tag = Some(AutoClosedTag {
                            tag: closed_name,
                            reason: following_tag,
                            depth: entry_stack.len(),
                        });
                    }

                    let attrs = self.convert_attributes(&tag.attributes, is_component);
                    if tag.self_closing {
                        let name = name.to_string();
                        let role = if is_component {
                            FragmentRole::ComponentChildren
                        } else {
                            FragmentRole::Element
                        };
                        let fragment = self.empty_fragment(role);
                        let node = if name == SVELTE_COMPONENT {
                            Node::SvelteComponentLegacy(svelte_ast::SvelteComponentLegacy {
                                id: NodeId(0),
                                span: token.span,
                                self_closing: true,
                                attributes: attrs,
                                fragment,
                                legacy_slots: Vec::new(),
                            })
                        } else if name == SVELTE_SELF {
                            Node::SvelteSelf(svelte_ast::SvelteSelf {
                                id: NodeId(0),
                                span: token.span,
                                self_closing: true,
                                attributes: attrs,
                                fragment,
                                legacy_slots: Vec::new(),
                            })
                        } else if is_component_name(&name) {
                            Node::ComponentNode(ComponentNode {
                                id: NodeId(0),
                                span: token.span,
                                name: svelte_ast::ExprRef::new(tag.name_span),
                                self_closing: true,
                                attributes: attrs,
                                fragment,
                                legacy_slots: Vec::new(),
                            })
                        } else {
                            Node::Element(Element {
                                id: NodeId(0),
                                span: token.span,
                                name,
                                self_closing: true,
                                attributes: attrs,
                                fragment,
                            })
                        };
                        let id = self.push_node(node);
                        push_child(&mut children_stack, id);
                    } else {
                        entry_stack.push(StackEntry::Element(ElementEntry {
                            name: name.to_string(),
                            name_span: tag.name_span,
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
                    children_stack.push(vec![]);
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
                    children_stack.push(vec![]);
                }
                TokenType::EndEachTag => {
                    self.handle_end_each_tag(token.span, &mut entry_stack, &mut children_stack);
                }
                TokenType::StartSnippetTag(snippet_tag) => {
                    entry_stack.push(StackEntry::SnippetBlock(SnippetBlockEntry {
                        span_start: token.span,
                        expression_span: snippet_tag.expression_span,
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
                        catch_children: None,
                    }));
                    children_stack.push(vec![]);
                }
                TokenType::AwaitClauseTag(clause_tag) => {
                    self.handle_await_clause_tag(
                        &clause_tag,
                        token.span,
                        &mut entry_stack,
                        &mut children_stack,
                    );
                }
                TokenType::EndAwaitTag => {
                    self.handle_end_await_tag(token.span, &mut entry_stack, &mut children_stack);
                }
                TokenType::RenderTag(render_tag) => {
                    let id = self.push_node(Node::RenderTag(RenderTag {
                        id: NodeId(0),
                        span: token.span,
                        expression: svelte_ast::ExprRef::new(render_tag.expression_span),
                    }));
                    push_child(&mut children_stack, id);
                }
                TokenType::HtmlTag(html_tag) => {
                    let id = self.push_node(Node::HtmlTag(HtmlTag {
                        id: NodeId(0),
                        span: token.span,
                        expression: svelte_ast::ExprRef::new(html_tag.expression_span),
                    }));
                    push_child(&mut children_stack, id);
                }
                TokenType::ConstTag(ct) => {
                    let id = self.push_node(Node::ConstTag(ConstTag {
                        id: NodeId(0),
                        span: token.span,
                        decl: svelte_ast::StmtRef::new(ct.expression_span),
                    }));
                    push_child(&mut children_stack, id);
                }
                TokenType::DeclarationTag(dt) => {
                    let id = self.push_node(Node::DeclarationTag(DeclarationTag {
                        id: NodeId(0),
                        span: token.span,
                        declaration: svelte_ast::StmtRef::new(dt.statement_span),
                    }));
                    push_child(&mut children_stack, id);
                }
                TokenType::DebugTag(dt) => {
                    let identifier_refs = dt
                        .identifiers
                        .iter()
                        .map(|s| svelte_ast::ExprRef::new(*s))
                        .collect();
                    let id = self.push_node(Node::DebugTag(DebugTag {
                        id: NodeId(0),
                        span: token.span,
                        identifier_refs,
                    }));
                    push_child(&mut children_stack, id);
                }
                TokenType::ScriptTag(script_tag) => {
                    let language = if script_tag.is_typescript {
                        ScriptLanguage::TypeScript
                    } else {
                        ScriptLanguage::JavaScript
                    };

                    if script_tag.is_module {
                        if module_script_data.is_some() {
                            self.recover(Diagnostic::error(
                                svelte_diagnostics::DiagnosticKind::ScriptDuplicate,
                                token.span,
                            ));
                            continue;
                        }
                        module_script_data = Some(ScriptData {
                            span: token.span,
                            content_span: script_tag.content_span,
                            language,
                            context: ScriptContext::Module,
                            context_deprecated: script_tag.context_deprecated,
                            invalid_context: script_tag.invalid_context,
                        });
                    } else {
                        if instance_script_data.is_some() {
                            self.recover(Diagnostic::error(
                                svelte_diagnostics::DiagnosticKind::ScriptDuplicate,
                                token.span,
                            ));
                            continue;
                        }
                        instance_script_data = Some(ScriptData {
                            span: token.span,
                            content_span: script_tag.content_span,
                            language,
                            context: ScriptContext::Default,
                            context_deprecated: false,
                            invalid_context: script_tag.invalid_context,
                        });
                    }
                }
                TokenType::StyleTag(style_tag) => {
                    if css_data.is_some() {
                        self.recover(Diagnostic::error(
                            svelte_diagnostics::DiagnosticKind::StyleDuplicate,
                            Span::new(token.span.start, token.span.start),
                        ));
                        continue;
                    }

                    let preceding_comment = children_stack
                        .last()
                        .and_then(|siblings| self.preceding_comment_id(siblings));
                    css_data = Some(CssData {
                        span: token.span,
                        content_span: style_tag.content_span,
                        preceding_comment,
                    });
                }
                TokenType::EOF => break,
            }
        }

        self.auto_close_entries(&mut entry_stack, &mut children_stack);

        let roots = pop_children(&mut children_stack);

        let instance_script = instance_script_data.map(|sd| Script {
            id: self.reserve_id(),
            span: sd.span,
            content_span: sd.content_span,
            context: sd.context,
            language: sd.language,
            context_deprecated: sd.context_deprecated,
            invalid_context: sd.invalid_context,
        });

        let module_script = module_script_data.map(|sd| Script {
            id: self.reserve_id(),
            span: sd.span,
            content_span: sd.content_span,
            context: sd.context,
            language: sd.language,
            context_deprecated: sd.context_deprecated,
            invalid_context: sd.invalid_context,
        });

        let css = css_data.map(|cd| RawBlock {
            span: cd.span,
            content_span: cd.content_span,
            preceding_comment: cd.preceding_comment,
        });

        let root_fragment = self.new_fragment(FragmentRole::Root, roots);
        let store = mem::take(&mut self.store);
        let mut component = Component::new(
            self.source.to_string(),
            root_fragment,
            store,
            instance_script,
            module_script,
            css,
        );
        component.js_comment_expr_starts = js_comment_expr_starts;

        self.extract_svelte_options(&mut component);

        Self::convert_special_root_elements(&mut component);

        let root_nodes = component.fragment_nodes(component.root).to_vec();

        Self::convert_slot_element_legacy(&mut component.store, &root_nodes, false);
        Self::convert_svelte_fragment_legacy(&mut component.store, &root_nodes);
        Self::convert_svelte_element(&mut component.store, &mut self.diagnostics, &root_nodes);
        Self::convert_svelte_boundary(&mut component.store, &root_nodes);

        Self::populate_fragment_owners(&mut component.store);
        component.store.freeze_node_fragments();

        (component, self.diagnostics)
    }

    fn populate_fragment_owners(store: &mut AstStore) {
        let total = store.fragments_len();
        let mut child_frags: Vec<svelte_ast::FragmentId> = Vec::new();
        for fid_raw in 0..total {
            let fid = svelte_ast::FragmentId(fid_raw);
            let nodes_len = store.fragment_nodes(fid).len();
            for i in 0..nodes_len {
                let nid = store.fragment_nodes(fid)[i];
                child_frags.clear();
                for_each_child_fragment(store.get(nid), |cf| child_frags.push(cf));
                for &cf in &child_frags {
                    store.set_fragment_owner(cf, nid);
                }
            }
        }
    }
}

enum TagError {
    Invalid,
    Reserved,
}

fn is_custom_element_tag_char(c: char) -> bool {
    matches!(c,
        'a'..='z'
        | '0'..='9'
        | '_'
        | '.'
        | '-'
        | '\u{B7}'
        | '\u{C0}'..='\u{D6}'
        | '\u{D8}'..='\u{F6}'
        | '\u{F8}'..='\u{37D}'
        | '\u{37F}'..='\u{1FFF}'
        | '\u{200C}'..='\u{200D}'
        | '\u{203F}'..='\u{2040}'
        | '\u{2070}'..='\u{218F}'
        | '\u{2C00}'..='\u{2FEF}'
        | '\u{3001}'..='\u{D7FF}'
        | '\u{F900}'..='\u{FDCF}'
        | '\u{FDF0}'..='\u{FFFD}'
        | '\u{10000}'..='\u{EFFFF}'
    )
}

fn validate_custom_element_tag(tag: &str) -> Option<TagError> {
    if tag.is_empty() {
        return None;
    }

    let is_valid = tag.starts_with(|c: char| c.is_ascii_lowercase())
        && tag.contains('-')
        && tag.chars().all(is_custom_element_tag_char);

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
    name.starts_with(|c: char| c.is_uppercase())
        || name.contains('.')
        || name == SVELTE_COMPONENT
        || name == SVELTE_SELF
}

struct ScriptData {
    span: Span,
    content_span: Span,
    language: ScriptLanguage,
    context: ScriptContext,
    context_deprecated: bool,
    invalid_context: Option<Span>,
}

struct CssData {
    span: Span,
    content_span: Span,
    preceding_comment: Option<NodeId>,
}

#[cfg(test)]
mod tests;
