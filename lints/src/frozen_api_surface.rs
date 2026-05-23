use rustc_errors::{Diag, DiagDecorator};
use rustc_hir::{self as hir, ImplItemKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_middle::ty;
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    pub FROZEN_API_SURFACE,
    Warn,
    "new public method on frozen type; coordinate via ARCHITECTURE.md and update allow-list"
}

declare_lint_pass!(FrozenApiSurface => [FROZEN_API_SURFACE]);

const FROZEN_APIS: &[(&str, &str, &[&str])] = &[
    (
        "svelte_analyze",
        "ReactivitySemantics",
        &[
            "declaration_semantics",
            "reference_semantics",
            "binding_semantics",
            "declarator_semantics",
            "iter_store_declarations",
            "iter_store_bindings",
            "has_store_bindings",
            "legacy_bindable_prop_symbols",
            "has_legacy_bindable_prop",
            "legacy_uses_props",
            "legacy_uses_rest_props",
            "legacy_has_member_mutated",
            "legacy_reactive",
            "uses_runes",
            "runes_mode",
            "uses_props_rune",
            "uses_rest_props",
            "uses_dollar_dollar_props",
            "uses_dollar_dollar_rest_props",
        ],
    ),
    (
        "svelte_analyze",
        "BlockSemanticsStore",
        &["get", "block_for_each_index_sym", "is_each_index_sym"],
    ),
    (
        "svelte_analyze",
        "AttributeSemanticsStore",
        &["get"],
    ),
    (
        "svelte_analyze",
        "ExpressionSemanticsStore",
        &["get", "is_context_required"],
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
        let hir::ItemKind::Impl(impl_) = &item.kind else { return };
        if impl_.of_trait.is_some() {
            return;
        }
        let Some(def_id) = self_ty_def_id(cx, item.hir_id()) else { return };
        let Some((type_name, allowed)) = frozen_allow_list(cx, def_id) else { return };
        for assoc_id in impl_.items {
            let impl_item = cx.tcx.hir_impl_item(*assoc_id);
            let ImplItemKind::Fn(_, _) = impl_item.kind else { continue };
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
                        "method `{name}` is not in the allow-list of `{owner}`; if intentional, add it to FROZEN_APIS in lints/src/frozen_api_surface.rs and document in ARCHITECTURE.md"
                    ));
                }),
            );
        }
    }
}
