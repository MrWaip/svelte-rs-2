use rustc_errors::{Diag, DiagDecorator};
use rustc_hir::{self as hir, ImplItemKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_middle::ty;
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    pub FROZEN_API_SURFACE,
    Warn,
    "new public method on frozen type; coordinate via the cluster root PRD in docs/ and update allow-list"
}

declare_lint_pass!(FrozenApiSurface => [FROZEN_API_SURFACE]);

const FROZEN_APIS: &[(&str, &str, &[&str])] = &[
    (
        "svelte_analyze",
        "ReactivitySemantics",
        &[
            "binding_semantics",
            "declarator_semantics",
            "reference_semantics",
            "class_field_semantics",
            "summary",
            "iter_runes_prop_symbols",
            "prop_default_span",
            "legacy_bindable_prop_symbols",
            "legacy_bindable_prop_alias",
            "iter_store_bindings",
            "legacy_reactive",
            "uses_runes",
            "runes_mode",
        ],
    ),
    (
        "svelte_analyze",
        "BlockSemanticsStore",
        &["get", "block_for_each_index_sym", "is_each_index_sym"],
    ),
    ("svelte_analyze", "AttributeSemanticsStore", &["get"]),
    (
        "svelte_analyze",
        "ExpressionSemanticsStore",
        &["get", "is_context_required", "get_by_oxc"],
    ),
    (
        "svelte_analyze",
        "ElementSemanticsStore",
        &["query"],
    ),
    (
        "svelte_analyze",
        "AnalysisData",
        &[
            "ancestors",
            "attr_expression_blockers",
            "attr_index",
            "attribute",
            "bind_directive",
            "bind_target_symbol",
            "binding_origin_key",
            "binding_origin_key_for_identifier_reference",
            "binding_origin_key_for_reference",
            "binding_semantics",
            "block_semantics",
            "blocker_data",
            "class_field_semantics",
            "component_attr_needs_memo",
            "component_name",
            "console_call_contains_state",
            "creation_namespace",
            "css_hash",
            "custom_element_slot_names",
            "declarator_semantics",
            "each_block_for_index_sym",
            "each_index_sym",
            "each_is_destructured",
            "each_item_indirect_sources",
            "effective_fragment_scope",
            "element_facts",
            "expr_ancestors",
            "expr_has_blockers",
            "expr_parent",
            "expression_attribute",
            "expression_blockers",
            "expression_data",
            "expression_data_by_oxc",
            "expression_data_for",
            "fragment_child_count_by_id",
            "fragment_facts_by_id",
            "fragment_has_children_by_id",
            "fragment_has_direct_animate_child_by_id",
            "fragment_has_direct_snippet_child_by_id",
            "fragment_has_expression_child_by_id",
            "fragment_has_non_trivial_children_by_id",
            "fragment_has_rich_content_by_id",
            "fragment_non_trivial_child_count_by_id",
            "fragment_references_any_symbol",
            "fragment_single_child_by_id",
            "fragment_single_expression_child_by_id",
            "fragment_single_non_trivial_child_by_id",
            "has_attribute",
            "has_component_css_props",
            "has_runtime_attrs",
            "has_spread",
            "has_true_boolean_attribute",
            "inject_styles",
            "is_css_scoped",
            "is_custom_element",
            "is_input",
            "is_void",
            "iter_store_bindings",
            "legacy_indirect_bindings",
            "namespace",
            "nearest_element",
            "nearest_element_for_expr",
            "node_ref_symbols",
            "parent",
            "reference_semantics",
            "shorthand_symbol",
            "static_text_attribute_value",
            "string_attribute",
            "svelte_element_tag",
            "symbol_for_identifier_reference",
            "symbol_for_reference",
            "template_element",
            "template_element_has_static_class",
            "template_element_may_match_class",
            "template_element_may_match_id",
            "template_element_next_sibling",
            "template_element_parent",
            "template_element_previous_sibling",
            "template_element_previous_siblings",
            "template_element_static_id",
            "template_element_tag_name",
            "template_elements",
            "template_elements_for_class",
            "template_elements_for_id",
            "template_elements_with_tag",
            "title_elements_for_fragment_by_id",
            "uses_runes",
        ],
    ),
];

fn self_ty_def_id(cx: &LateContext<'_>, hir_id: hir::HirId) -> Option<hir::def_id::DefId> {
    let ty = cx.tcx.type_of(hir_id.owner.def_id).instantiate_identity();
    if let ty::Adt(adt, _) = ty.kind() {
        Some(adt.did())
    } else {
        None
    }
}

fn frozen_allow_list(
    cx: &LateContext<'_>,
    def_id: hir::def_id::DefId,
) -> Option<(&'static str, &'static [&'static str])> {
    let krate = cx.tcx.crate_name(def_id.krate);
    let name = cx.tcx.item_name(def_id);
    for (k, n, allowed) in FROZEN_APIS {
        if krate.as_str() == *k && name.as_str() == *n {
            return Some((n, allowed));
        }
    }
    None
}

fn is_exposed(vis: ty::Visibility<hir::def_id::DefId>) -> bool {
    matches!(vis, ty::Visibility::Public)
}

impl<'tcx> LateLintPass<'tcx> for FrozenApiSurface {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx hir::Item<'tcx>) {
        let hir::ItemKind::Impl(impl_) = &item.kind else {
            return;
        };
        if impl_.of_trait.is_some() {
            return;
        }
        let Some(def_id) = self_ty_def_id(cx, item.hir_id()) else {
            return;
        };
        let Some((type_name, allowed)) = frozen_allow_list(cx, def_id) else {
            return;
        };
        for assoc_id in impl_.items {
            let impl_item = cx.tcx.hir_impl_item(*assoc_id);
            let ImplItemKind::Fn(_, _) = impl_item.kind else {
                continue;
            };
            let assoc_def_id = impl_item.owner_id.def_id;
            let vis = cx.tcx.visibility(assoc_def_id);
            if !is_exposed(vis) {
                continue;
            }
            let name = impl_item.ident.as_str().to_owned();
            if allowed.iter().any(|a| *a == name.as_str()) {
                continue;
            }
            let owner = type_name;
            cx.opt_span_lint(
                FROZEN_API_SURFACE,
                Some(impl_item.span),
                DiagDecorator(move |diag: &mut Diag<'_, ()>| {
                    diag.primary_message(format!(
                        "method `{name}` is not in the allow-list of `{owner}`; if intentional, add it to FROZEN_APIS in lints/src/frozen_api_surface.rs and document in the cluster root PRD under docs/"
                    ));
                }),
            );
        }
    }
}
