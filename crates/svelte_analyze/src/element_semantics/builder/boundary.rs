use oxc_ast::ast::{BindingIdentifier, BindingPattern, Statement, VariableDeclarator};
use svelte_ast::{Component, Node, SnippetBlock, SvelteBoundary};

use super::super::{BoundaryBranch, BoundarySemantics};
use crate::types::data::JsAst;

pub(super) fn classify(
    component: &Component,
    parsed: &JsAst<'_>,
    boundary: &SvelteBoundary,
) -> BoundarySemantics {
    let mut failed_snippet = BoundaryBranch::None;
    let mut pending_snippet = BoundaryBranch::None;
    let mut failed_attr = BoundaryBranch::None;
    let mut pending_attr = BoundaryBranch::None;

    for &nid in component.store.fragment_nodes(boundary.fragment) {
        let Node::SnippetBlock(block) = component.store.get(nid) else {
            continue;
        };
        match snippet_name(parsed, block).as_deref() {
            Some("failed") => failed_snippet = BoundaryBranch::Snippet(nid),
            Some("pending") => pending_snippet = BoundaryBranch::Snippet(nid),
            _ => {}
        }
    }

    for attr in &boundary.attributes {
        match attr.name() {
            Some("failed") => failed_attr = BoundaryBranch::Attribute(attr.id()),
            Some("pending") => pending_attr = BoundaryBranch::Attribute(attr.id()),
            _ => {}
        }
    }

    let failed = pick(failed_snippet, failed_attr);
    let pending = pick(pending_attr, pending_snippet);

    BoundarySemantics { failed, pending }
}

fn pick(primary: BoundaryBranch, fallback: BoundaryBranch) -> BoundaryBranch {
    match primary {
        BoundaryBranch::None => fallback,
        other => other,
    }
}

fn snippet_name(parsed: &JsAst<'_>, block: &SnippetBlock) -> Option<String> {
    let stmt = parsed.stmt(block.decl.id())?;
    let declarator = declarator_from_stmt(stmt)?;
    let ident = binding_ident_of(&declarator.id)?;
    Some(ident.name.to_string())
}

fn declarator_from_stmt<'a>(stmt: &'a Statement<'a>) -> Option<&'a VariableDeclarator<'a>> {
    let Statement::VariableDeclaration(decl) = stmt else {
        return None;
    };
    decl.declarations.first()
}

fn binding_ident_of<'a>(pattern: &'a BindingPattern<'a>) -> Option<&'a BindingIdentifier<'a>> {
    match pattern {
        BindingPattern::BindingIdentifier(ident) => Some(ident),
        _ => None,
    }
}
