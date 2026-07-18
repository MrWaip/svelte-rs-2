use oxc_ast::ast::Statement;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use svelte_ast::{Component, Node};
use svelte_component_semantics::{ScopeId, SymbolId, SymbolOwner, walk_bindings};
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

    let mut declared: Vec<SymbolId> = Vec::new();
    for i in 0..component.store.len() {
        let node = component.store.get(svelte_ast::NodeId(i));
        match node {
            Node::ConstTag(tag) => {
                collect_declaration_bindings(parsed, tag.decl.id(), &mut declared)
            }
            Node::EachBlock(block) => {
                if let Some(context) = &block.context {
                    collect_declaration_bindings(parsed, context.id(), &mut declared);
                }
                if let Some(index) = &block.index {
                    collect_declaration_bindings(parsed, index.id(), &mut declared);
                }
            }
            Node::SnippetBlock(block) => {
                collect_declaration_bindings(parsed, block.decl.id(), &mut declared);
            }
            _ => {}
        }
    }

    let mut groups: FxHashMap<(ScopeId, &str), SmallVec<[SymbolId; 2]>> = FxHashMap::default();
    for symbol_id in declared {
        let key = (sem.symbol_scope_id(symbol_id), sem.symbol_name(symbol_id));
        groups.entry(key).or_default().push(symbol_id);
    }

    for ((_, name), mut symbols) in groups {
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

    validate_top_level_snippet_conflicts(component, data, parsed, diags);
}

fn validate_top_level_snippet_conflicts(
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
        let Some(snippet) = component.store.get(node_id).as_snippet_block() else {
            continue;
        };
        let mut names: Vec<SymbolId> = Vec::new();
        collect_declaration_bindings(parsed, snippet.decl.id(), &mut names);
        for symbol_id in names {
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
    out: &mut Vec<SymbolId>,
) {
    let Some(Statement::VariableDeclaration(decl)) = parsed.stmt(stmt_id) else {
        return;
    };
    for declarator in &decl.declarations {
        walk_bindings(&declarator.id, |visit| out.push(visit.symbol));
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
