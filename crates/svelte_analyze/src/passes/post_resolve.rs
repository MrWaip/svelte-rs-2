use crate::types::data::AnalysisData;
use crate::types::script::{DeclarationKind, RuneKind};

pub fn run_post_resolve_passes(data: &mut AnalysisData) {
    analyze_declarations(data);
    aggregate_store_needs_context(data);
}

fn analyze_declarations(data: &mut AnalysisData) {
    let script = match &data.script.info {
        Some(s) => s,
        None => return,
    };
    let root = data.scoping.root_scope_id();

    for decl in &script.declarations {
        if decl.is_rune == Some(RuneKind::PropsId) && data.script.props_id.is_none() {
            data.script.props_id = Some(decl.name.to_string());
        }

        let Some(ref lit) = decl.init_literal else {
            continue;
        };

        let sym_id = data.scoping.find_binding(root, &decl.name);
        let rune_kind = decl.is_rune;
        let is_mutated = sym_id.is_some_and(|id| data.scoping.is_mutated(id));

        let is_foldable_rune = rune_kind == Some(RuneKind::State) && !is_mutated;

        if rune_kind.is_some() && !is_foldable_rune {
            continue;
        }

        if rune_kind.is_none() && decl.kind != DeclarationKind::Const {
            continue;
        }

        if let Some(sym_id) = sym_id {
            data.scoping.set_known_value(sym_id, lit.to_string());
        }
    }

    mark_rest_prop_symbol(data);
}

fn mark_rest_prop_symbol(data: &mut AnalysisData) {
    let Some(decl) = data.script.props_declaration() else {
        return;
    };
    let root = data.scoping.root_scope_id();

    let rest_names: Vec<String> = decl
        .props
        .iter()
        .filter(|p| p.is_rest)
        .map(|p| p.local_name.to_string())
        .collect();
    for name in rest_names {
        if let Some(sym_id) = data.scoping.find_binding(root, &name) {
            data.scoping.mark_rest_prop_sym(sym_id);
        }
    }
}

fn aggregate_store_needs_context(data: &mut AnalysisData) {
    if data.output.needs_context {
        return;
    }

    let has_deep = data
        .expressions
        .values()
        .any(|i| i.has_store_member_mutation())
        || data
            .attr_expressions
            .values()
            .any(|i| i.has_store_member_mutation())
        || data.script.has_store_member_mutations;

    if has_deep {
        data.output.needs_context = true;
    }
}
