use scanner::{token::TokenType, Scanner};
use svelte_span::Span;

use svelte_ast::{
    AstStore, Attribute, Comment, Component, ComponentNode, ConstTag, DebugTag, Element,
    FragmentId, FragmentRole, HtmlTag, Node, NodeId, RawBlock, RenderTag, Script, ScriptContext,
    ScriptLanguage, Text, SVELTE_COMPONENT, SVELTE_SELF,
};

use svelte_diagnostics::Diagnostic;

mod html;
mod html_entities;
pub mod parse_js;
pub mod scanner;
pub mod types;
mod walk_js;

mod attr_convert;
mod handlers;
mod svelte_elements;

// Re-export all shared types for convenience
pub use types::{CePropConfig, CeShadowMode, JsAst, ParsedCeConfig};

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
    let scoping = oxc_semantic::SemanticBuilder::new()
        .build(&program)
        .semantic
        .into_scoping();
    Ok((program, scoping))
}

/// Parse a Svelte source file and all embedded JS expressions.
///
/// Returns the parsed component AST, parse results (expression + statement ASTs),
/// and any diagnostics from both the Svelte parser and JS expression parsing.
pub fn parse_with_js<'a>(
    alloc: &'a oxc_allocator::Allocator,
    source: &str,
) -> (
    svelte_ast::Component,
    crate::types::JsAst<'a>,
    Vec<Diagnostic>,
) {
    let (component, mut diagnostics) = Parser::new(source).parse();
    let mut result = crate::types::JsAst::new();
    walk_js::parse_js(alloc, &component, &mut result, &mut diagnostics);

    (component, result, diagnostics)
}

/// Parse the CSS from a component's top-level `<style>` block.
///
/// Returns `None` when the component has no `<style>` block.
/// CSS parse diagnostics are returned separately and should be merged
/// into the main diagnostic list by the caller.
pub fn parse_css_block(
    component: &svelte_ast::Component,
) -> Option<(svelte_css::StyleSheet, Vec<svelte_diagnostics::Diagnostic>)> {
    let css_block = component.css.as_ref()?;
    let css_text = component.source_text(css_block.content_span);
    let (stylesheet, diags) = svelte_css::parse(css_text);
    Some((stylesheet, diags))
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
    consequent: Option<Vec<NodeId>>,
    /// Whether we are currently collecting alternate children.
    in_alternate: bool,
}

struct EachBlockEntry {
    span: Span,
    expression_span: Span,
    context_span: Option<Span>,
    index_span: Option<Span>,
    key_span: Option<Span>,
    /// Body children, set when `{:else}` switches to fallback collection.
    body_children: Option<Vec<NodeId>>,
    in_fallback: bool,
}

struct SnippetBlockEntry {
    span_start: Span,
    expression_span: Span,
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
    /// Pending children (collected before {:then} or {:catch}).
    pending_children: Option<Vec<NodeId>>,
    /// Then children (collected between {:then} and a following {:catch}).
    then_children: Option<Vec<NodeId>>,
    /// Catch children saved when {:then} follows {:catch} (out-of-order clauses).
    catch_children: Option<Vec<NodeId>>,
}

// ---------------------------------------------------------------------------
// Stack helpers — safe wrappers around children_stack operations
// ---------------------------------------------------------------------------

/// Push a node id onto the current children list.
/// Debug-asserts the stack is non-empty; gracefully no-ops in release.
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

/// Pop the current children list.
/// Debug-asserts the stack is non-empty; returns empty vec in release.
fn pop_children(children_stack: &mut Vec<Vec<NodeId>>) -> Vec<NodeId> {
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
    store: AstStore,
    diagnostics: Vec<Diagnostic>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Parser<'a> {
        Parser {
            source,
            store: AstStore::with_capacity(source.len() / 10),
            diagnostics: Vec::new(),
        }
    }

    fn recover(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Push a node into the store and return its NodeId.
    fn push_node(&mut self, node: Node) -> NodeId {
        self.store.push(node)
    }

    /// Reserve a NodeId slot for a non-node id (attributes, key expressions, etc.).
    fn reserve_id(&mut self) -> NodeId {
        self.store.reserve()
    }

    /// Allocate a fresh FragmentId in the AstStore arena with the given child nodes.
    pub(crate) fn new_fragment(&mut self, role: FragmentRole, nodes: Vec<NodeId>) -> FragmentId {
        self.store.push_fragment(role, nodes)
    }

    /// Allocate a fresh empty FragmentId in the AstStore arena.
    pub(crate) fn empty_fragment(&mut self, role: FragmentRole) -> FragmentId {
        self.store.reserve_fragment(role)
    }

    /// Split component children into default-slot bucket and named slot buckets.
    /// Each child whose `slot="name"` attribute resolves to a non-empty static name
    /// goes into its own `LegacySlot` fragment; everything else stays in default.
    /// `slot=` attribute remains on the child node — analysis still validates it.
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

    /// Return the static name from a `slot="name"` attribute on the given child node,
    /// if present. Returns `None` for nodes without attributes, without `slot=`,
    /// or when the value isn't a static string (dynamic / concatenation cases stay
    /// in the default bucket — analysis reports them).
    fn slot_name_of(&self, child: NodeId) -> Option<String> {
        let attrs = match self.store.get(child) {
            Node::Element(el) => &el.attributes,
            Node::ComponentNode(cn) => &cn.attributes,
            _ => return None,
        };

        for attr in attrs {
            if let Attribute::StringAttribute(sa) = attr {
                if sa.name == "slot" {
                    let value = sa.value_span.source_text(self.source);
                    if value.is_empty() {
                        return None;
                    }
                    return Some(value.to_string());
                }
            }
        }
        None
    }

    pub fn parse(mut self) -> (Component, Vec<Diagnostic>) {
        let mut scanner = Scanner::new(self.source);
        let (tokens, scan_diagnostics) = scanner.scan_tokens();
        self.diagnostics.extend(scan_diagnostics);

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
                        id: NodeId(0), // set by store.push
                        span: token.span,
                        decoded: html::decode_text(raw),
                    }));
                    push_child(&mut children_stack, id);
                }
                TokenType::Comment => {
                    let id = self.push_node(Node::Comment(Comment {
                        id: NodeId(0),
                        span: token.span,
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
                    let attrs = self.convert_attributes(&tag.attributes, is_component);
                    if tag.self_closing {
                        let name = name.to_string();
                        let role = if is_component {
                            FragmentRole::ComponentChildren
                        } else {
                            FragmentRole::Element
                        };
                        let fragment = self.empty_fragment(role);
                        let node = if is_component_name(&name) {
                            Node::ComponentNode(ComponentNode {
                                id: NodeId(0),
                                span: token.span,
                                name,
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
                        });
                    }
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

        let instance_script = instance_script_data.map(|sd| Script {
            id: self.reserve_id(),
            span: sd.span,
            content_span: sd.content_span,
            context: sd.context,
            language: sd.language,
            context_deprecated: sd.context_deprecated,
        });

        let module_script = module_script_data.map(|sd| Script {
            id: self.reserve_id(),
            span: sd.span,
            content_span: sd.content_span,
            context: sd.context,
            language: sd.language,
            context_deprecated: sd.context_deprecated,
        });

        let css = css_data.map(|cd| RawBlock {
            span: cd.span,
            content_span: cd.content_span,
        });

        let root_fragment = self.new_fragment(FragmentRole::Root, roots);
        let store = std::mem::take(&mut self.store);
        let mut component = Component::new(
            self.source.to_string(),
            root_fragment,
            store,
            instance_script,
            module_script,
            css,
        );
        // Extract <svelte:options> from fragment (must be top-level)
        self.extract_svelte_options(&mut component);

        // Root-only special elements need parser-owned duplicate / placement validation before
        // top-level conversion rewrites the valid cases into dedicated AST nodes.
        self.validate_root_only_special_elements(&component);

        // Convert <svelte:head> elements to SvelteHead nodes
        Self::convert_svelte_head(&mut component);

        // Convert <svelte:window> elements to SvelteWindow nodes
        Self::convert_svelte_window(&mut component);

        // Convert <svelte:document> elements to SvelteDocument nodes
        Self::convert_svelte_document(&mut component);

        // Convert <svelte:body> elements to SvelteBody nodes
        Self::convert_svelte_body(&mut component);

        let root_nodes = component.fragment_nodes(component.root).to_vec();

        Self::convert_slot_element_legacy(&mut component.store, &root_nodes);
        Self::convert_svelte_fragment_legacy(&mut component.store, &root_nodes);
        Self::convert_svelte_element(&mut component.store, &root_nodes);
        Self::convert_svelte_boundary(&mut component.store, &root_nodes);

        Self::populate_fragment_owners(&mut component.store);
        component.store.freeze_node_fragments();

        (component, self.diagnostics)
    }

    fn populate_fragment_owners(store: &mut AstStore) {
        let total = store.fragments_len();
        for fid_raw in 0..total {
            let fid = svelte_ast::FragmentId(fid_raw);
            let nodes_len = store.fragment_nodes(fid).len();
            for i in 0..nodes_len {
                let nid = store.fragment_nodes(fid)[i];
                let mut child_frags: Vec<svelte_ast::FragmentId> = Vec::new();
                let node = store.get(nid);
                match node {
                    Node::Element(el) => child_frags.push(el.fragment),
                    Node::SlotElementLegacy(el) => child_frags.push(el.fragment),
                    Node::ComponentNode(cn) => {
                        child_frags.push(cn.fragment);
                        for slot in &cn.legacy_slots {
                            child_frags.push(slot.fragment);
                        }
                    }
                    Node::IfBlock(b) => {
                        child_frags.push(b.consequent);
                        if let Some(alt) = b.alternate {
                            child_frags.push(alt);
                        }
                    }
                    Node::EachBlock(b) => {
                        child_frags.push(b.body);
                        if let Some(fb) = b.fallback {
                            child_frags.push(fb);
                        }
                    }
                    Node::SnippetBlock(b) => child_frags.push(b.body),
                    Node::KeyBlock(b) => child_frags.push(b.fragment),
                    Node::SvelteHead(h) => child_frags.push(h.fragment),
                    Node::SvelteFragmentLegacy(f) => child_frags.push(f.fragment),
                    Node::SvelteElement(el) => child_frags.push(el.fragment),
                    Node::SvelteWindow(w) => child_frags.push(w.fragment),
                    Node::SvelteDocument(d) => child_frags.push(d.fragment),
                    Node::SvelteBody(b) => child_frags.push(b.fragment),
                    Node::SvelteBoundary(b) => child_frags.push(b.fragment),
                    Node::AwaitBlock(b) => {
                        if let Some(p) = b.pending {
                            child_frags.push(p);
                        }
                        if let Some(t) = b.then {
                            child_frags.push(t);
                        }
                        if let Some(c) = b.catch {
                            child_frags.push(c);
                        }
                    }
                    _ => {}
                }
                for cf in child_frags {
                    store.set_fragment_owner(cf, nid);
                }
            }
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
}

struct CssData {
    span: Span,
    content_span: Span,
}

#[cfg(test)]
mod tests;
