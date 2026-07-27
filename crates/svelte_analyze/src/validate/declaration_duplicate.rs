use oxc_ast::ast::Statement;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use svelte_ast::{Component, FragmentId, Node, NodeId};
use svelte_component_semantics::{SymbolId, SymbolOwner, walk_bindings};
use svelte_diagnostics::{Diagnostic, DiagnosticKind};

use crate::AnalysisData;
use crate::types::data::JsAst;

pub(super) fn validate(
    component: &Component,
    data: &AnalysisData<'_>,
    parsed: &JsAst<'_>,
    diags: &mut Vec<Diagnostic>,
) {
    let sem = data.scoping.semantics();

    let mut declared: Vec<(FragmentId, SymbolId)> = Vec::new();
    for i in 0..component.store.len() {
        let node_id = NodeId(i);
        let node = component.store.get(node_id);
        match node {
            Node::ConstTag(tag) => {
                if let Some(fragment) = component.store.node_fragment(node_id) {
                    collect_declaration_bindings(parsed, tag.decl.id(), fragment, &mut declared);
                }
            }
            Node::EachBlock(block) => {
                if let Some(context) = &block.context {
                    collect_declaration_bindings(parsed, context.id(), block.body, &mut declared);
                }
                if let Some(index) = &block.index {
                    collect_declaration_bindings(parsed, index.id(), block.body, &mut declared);
                }
            }
            Node::SnippetBlock(block) => {
                if let Some(fragment) = component.store.node_fragment(node_id) {
                    collect_declaration_bindings(parsed, block.decl.id(), fragment, &mut declared);
                }
            }
            _ => {}
        }
    }

    let mut groups: FxHashMap<(FragmentId, &str), SmallVec<[SymbolId; 2]>> = FxHashMap::default();
    for (fragment, symbol_id) in declared {
        let key = (fragment, sem.symbol_name(symbol_id));
        groups.entry(key).or_default().push(symbol_id);
    }

    for ((_fragment, name), mut symbols) in groups {
        symbols.sort_by_key(|&id| {
            let span = sem.symbol_span(id);
            (span.start, span.end)
        });
        symbols.dedup_by_key(|&mut id| {
            let span = sem.symbol_span(id);
            (span.start, span.end)
        });
        for &symbol_id in symbols.iter().skip(1) {
            emit(sem, symbol_id, name, diags);
        }
    }

    validate_top_level_conflicts(component, data, parsed, diags);
}

fn validate_top_level_conflicts(
    component: &Component,
    data: &AnalysisData<'_>,
    parsed: &JsAst<'_>,
    diags: &mut Vec<Diagnostic>,
) {
    let sem = data.scoping.semantics();
    let Some(instance_scope) = sem.instance_scope_id() else {
        return;
    };

    let mut instance_names: FxHashSet<&str> = FxHashSet::default();
    for symbol_id in sem.symbol_ids() {
        if sem.symbol_owner(symbol_id) == SymbolOwner::InstanceScript
            && sem.symbol_scope_id(symbol_id) == instance_scope
        {
            instance_names.insert(sem.symbol_name(symbol_id));
        }
    }

    for &node_id in &component.root_fragment().nodes {
        let node = component.store.get(node_id);
        let stmt_id = match node {
            Node::SnippetBlock(snippet) => snippet.decl.id(),
            Node::DeclarationTag(tag) => tag.declaration.id(),
            _ => continue,
        };
        let mut names: Vec<(FragmentId, SymbolId)> = Vec::new();
        collect_declaration_bindings(parsed, stmt_id, component.root, &mut names);
        for (_, symbol_id) in names {
            let name = sem.symbol_name(symbol_id);
            if instance_names.contains(name) {
                emit(sem, symbol_id, name, diags);
            }
        }
    }
}

fn collect_declaration_bindings(
    parsed: &JsAst<'_>,
    stmt_id: svelte_component_semantics::OxcNodeId,
    fragment: FragmentId,
    out: &mut Vec<(FragmentId, SymbolId)>,
) {
    let Some(Statement::VariableDeclaration(decl)) = parsed.stmt(stmt_id) else {
        return;
    };
    for declarator in &decl.declarations {
        walk_bindings(&declarator.id, |visit| out.push((fragment, visit.symbol)));
    }
}

fn emit(
    sem: &svelte_component_semantics::ComponentSemantics<'_>,
    symbol_id: SymbolId,
    name: &str,
    diags: &mut Vec<Diagnostic>,
) {
    let span = sem.symbol_span(symbol_id);
    diags.push(Diagnostic::error(
        DiagnosticKind::DeclarationDuplicate {
            name: name.to_string(),
        },
        svelte_span::Span::new(span.start, span.end),
    ));
}
