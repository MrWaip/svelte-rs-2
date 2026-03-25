use crate::types::script::{DeclarationKind, RuneKind};

use crate::types::data::{AnalysisData, PropAnalysis, PropsAnalysis};

/// Run all passes that depend on resolve_references but are independent of each other.
/// Combines known value collection, props analysis, and needs_context aggregation.
pub fn run_post_resolve_passes(data: &mut AnalysisData) {
    analyze_declarations(data);
    detect_each_index_usage(data);
    aggregate_store_needs_context(data);
}

/// Single pass over script declarations for both known-value collection and props-id detection.
/// Then processes props_declaration separately (only needed for props analysis).
fn analyze_declarations(data: &mut AnalysisData) {
    let script = match &data.script {
        Some(s) => s,
        None => return,
    };
    let root = data.scoping.root_scope_id();

    // Single pass: known_values + props_id detection
    for decl in &script.declarations {
        if decl.is_rune == Some(RuneKind::PropsId) && data.props_id.is_none() {
            data.props_id = Some(decl.name.to_string());
        }

        let Some(ref lit) = decl.init_literal else {
            continue;
        };

        let sym_id = data.scoping.find_binding(root, &decl.name);
        let rune_kind = sym_id.and_then(|id| data.scoping.rune_kind(id));
        let is_mutated = sym_id.map_or(false, |id| data.scoping.is_mutated(id));

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

    analyze_props_declaration(data);
}

fn analyze_props_declaration(data: &mut AnalysisData) {
    let decl = match data.script.as_ref().and_then(|s| s.props_declaration.as_ref()) {
        Some(d) => d,
        None => return,
    };

    let root = data.scoping.root_scope_id();

    let props = decl
        .props
        .iter()
        .map(|p| {
            let sym_id = data.scoping.find_binding(root, p.local_name.as_str());
            let is_mutated = data.custom_element
                || sym_id.is_some_and(|id| data.scoping.is_mutated(id));
            let is_prop_source =
                data.custom_element || p.default_span.is_some() || is_mutated;

            if !p.is_rest {
                if let Some(sym_id) = sym_id {
                    if is_prop_source {
                        data.scoping.mark_prop_source(sym_id);
                    } else {
                        data.scoping.mark_prop_non_source(sym_id, p.prop_name.to_string());
                    }
                }
            }

            let is_lazy_default = p.default_span.is_some() && !p.is_simple_default;

            PropAnalysis {
                local_name: p.local_name.to_string(),
                prop_name: p.prop_name.to_string(),
                default_span: p.default_span,
                default_text: p.default_text.clone(),
                is_bindable: p.is_bindable,
                is_rest: p.is_rest,
                is_lazy_default,
                is_prop_source,
                is_mutated,
                is_reserved: p.prop_name.starts_with("$$"),
            }
        })
        .collect::<Vec<PropAnalysis>>();

    let has_bindable = decl.props.iter().any(|p| p.is_bindable);

    data.props = Some(PropsAnalysis { props, has_bindable });
}

/// Detect each-block index variable usage via resolved SymbolIds.
/// Index variables are only in scope inside the body, so any resolved reference
/// to the index SymbolId must be within the body.
fn detect_each_index_usage(data: &mut AnalysisData) {
    let syms: Vec<_> = data
        .each_blocks
        .index_syms
        .iter()
        .map(|(id, sym)| (id, *sym))
        .collect();
    for (block_id, idx_sym) in syms {
        let body_uses = data
            .expressions
            .values()
            .chain(data.attr_expressions.values())
            .any(|info| {
                info.references
                    .iter()
                    .any(|r| r.symbol_id == Some(idx_sym))
            });
        if body_uses {
            data.each_blocks.body_uses_index.insert(block_id);
        }

        if let Some(key_info) = data.each_blocks.key_infos.get(block_id) {
            if key_info
                .references
                .iter()
                .any(|r| r.symbol_id == Some(idx_sym))
            {
                data.each_blocks.key_uses_index.insert(block_id);
            }
        }
    }
}

/// $.store_mutate needs component context ($.push/$.pop) — detect deep store mutations.
fn aggregate_store_needs_context(data: &mut AnalysisData) {
    if data.needs_context {
        return;
    }

    let has_deep = data.expressions.values().any(|i| i.has_store_member_mutation)
        || data.attr_expressions.values().any(|i| i.has_store_member_mutation)
        || data.has_store_member_mutations;

    if has_deep {
        data.needs_context = true;
    }
}
