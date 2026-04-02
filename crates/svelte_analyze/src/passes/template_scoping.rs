//! Pure scope registration for template constructs.
//!
//! Walks the template tree and creates a child scope for every block that
//! introduces one. For snippets, also pre-sets the ArrowFunctionExpression
//! scope_id so SemanticCollector reuses it instead of creating a new scope.

use svelte_ast::{AstStore, Component, Fragment, Node};

use crate::scope::{ComponentScoping, ScopeId};
use crate::types::data::{FragmentKey, ParserResult};

pub(crate) fn create_template_scopes(
    component: &Component,
    scoping: &mut ComponentScoping,
    parsed: &ParserResult<'_>,
) {
    let root = scoping.root_scope_id();
    walk(&component.fragment, scoping, root, parsed, &component.store);
}

fn walk(
    fragment: &Fragment,
    scoping: &mut ComponentScoping,
    current_scope: ScopeId,
    parsed: &ParserResult<'_>,
    store: &AstStore,
) {
    for &id in &fragment.nodes {
        match store.get(id) {
            Node::EachBlock(block) => {
                let child_scope = scoping.add_child_scope(current_scope);
                scoping.set_fragment_scope(FragmentKey::EachBody(block.id), child_scope);
                walk(&block.body, scoping, child_scope, parsed, store);
                if let Some(fb) = &block.fallback {
                    walk(fb, scoping, current_scope, parsed, store);
                }
            }
            Node::IfBlock(block) => {
                let cons_scope = scoping.add_child_scope(current_scope);
                scoping.set_fragment_scope(FragmentKey::IfConsequent(block.id), cons_scope);
                walk(&block.consequent, scoping, cons_scope, parsed, store);
                if let Some(alt) = &block.alternate {
                    let alt_scope = scoping.add_child_scope(current_scope);
                    scoping.set_fragment_scope(FragmentKey::IfAlternate(block.id), alt_scope);
                    walk(alt, scoping, alt_scope, parsed, store);
                }
            }
            Node::SnippetBlock(block) => {
                let snippet_scope = scoping.add_child_scope(current_scope);
                scoping.set_fragment_scope(FragmentKey::SnippetBody(block.id), snippet_scope);
                // Pre-set arrow scope so SemanticCollector reuses it
                if let Some(arrow) = parsed
                    .stmt_handle(block.expression_span.start)
                    .and_then(|handle| parsed.stmt(handle))
                    .and_then(extract_arrow_from_const)
                {
                    arrow.scope_id.set(Some(snippet_scope));
                }
                walk(&block.body, scoping, snippet_scope, parsed, store);
            }
            Node::KeyBlock(block) => {
                let child_scope = scoping.add_child_scope(current_scope);
                scoping.set_fragment_scope(FragmentKey::KeyBlockBody(block.id), child_scope);
                walk(&block.fragment, scoping, child_scope, parsed, store);
            }
            Node::SvelteHead(head) => {
                let child_scope = scoping.add_child_scope(current_scope);
                scoping.set_fragment_scope(FragmentKey::SvelteHeadBody(head.id), child_scope);
                walk(&head.fragment, scoping, child_scope, parsed, store);
            }
            Node::SvelteElement(el) => {
                let child_scope = scoping.add_child_scope(current_scope);
                scoping.set_fragment_scope(FragmentKey::SvelteElementBody(el.id), child_scope);
                walk(&el.fragment, scoping, child_scope, parsed, store);
            }
            Node::SvelteBoundary(b) => {
                let child_scope = scoping.add_child_scope(current_scope);
                scoping.set_fragment_scope(FragmentKey::SvelteBoundaryBody(b.id), child_scope);
                walk(&b.fragment, scoping, child_scope, parsed, store);
            }
            Node::AwaitBlock(block) => {
                if let Some(ref p) = block.pending {
                    let scope = scoping.add_child_scope(current_scope);
                    scoping.set_fragment_scope(FragmentKey::AwaitPending(block.id), scope);
                    walk(p, scoping, scope, parsed, store);
                }
                if let Some(ref t) = block.then {
                    let scope = scoping.add_child_scope(current_scope);
                    scoping.set_fragment_scope(FragmentKey::AwaitThen(block.id), scope);
                    walk(t, scoping, scope, parsed, store);
                }
                if let Some(ref c) = block.catch {
                    let scope = scoping.add_child_scope(current_scope);
                    scoping.set_fragment_scope(FragmentKey::AwaitCatch(block.id), scope);
                    walk(c, scoping, scope, parsed, store);
                }
            }
            Node::Element(el) => walk(&el.fragment, scoping, current_scope, parsed, store),
            Node::ComponentNode(cn) => walk(&cn.fragment, scoping, current_scope, parsed, store),
            Node::ConstTag(_)
            | Node::SvelteWindow(_)
            | Node::SvelteDocument(_)
            | Node::SvelteBody(_)
            | Node::ExpressionTag(_)
            | Node::Text(_)
            | Node::Comment(_)
            | Node::RenderTag(_)
            | Node::HtmlTag(_)
            | Node::DebugTag(_)
            | Node::Error(_) => {}
        }
    }
}

/// Extract ArrowFunctionExpression from `const NAME = (...) => {}`.
fn extract_arrow_from_const<'a>(
    stmt: &'a oxc_ast::ast::Statement<'a>,
) -> Option<&'a oxc_ast::ast::ArrowFunctionExpression<'a>> {
    let oxc_ast::ast::Statement::VariableDeclaration(decl) = stmt else {
        return None;
    };
    let declarator = decl.declarations.first()?;
    let oxc_ast::ast::Expression::ArrowFunctionExpression(arrow) = declarator.init.as_ref()? else {
        return None;
    };
    Some(arrow)
}
