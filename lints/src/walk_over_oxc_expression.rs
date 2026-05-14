use rustc_errors::{Diag, DiagDecorator};
use rustc_hir::{self as hir, ExprKind, Node, PatKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_middle::ty;
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    pub WALK_OVER_OXC_EXPRESSION,
    Allow,
    "hand-written walk over oxc AST; use oxc_ast_visit::Visit instead"
}

declare_lint_pass!(WalkOverOxcExpression => [WALK_OVER_OXC_EXPRESSION]);

const SCRUTINEE_TYPES: &[(&str, &str)] = &[
    ("oxc_ast", "Expression"),
    ("oxc_ast", "Statement"),
    ("oxc_ast", "BindingPattern"),
    ("oxc_ast", "BindingPatternKind"),
];

const VISITOR_TRAITS: &[(&str, &str)] = &[
    ("oxc_ast_visit", "Visit"),
    ("oxc_ast_visit", "VisitMut"),
    ("oxc_traverse", "Traverse"),
];

const THRESHOLD: usize = 5;

fn matches_scrutinee(cx: &LateContext<'_>, ty: ty::Ty<'_>) -> Option<&'static str> {
    let ty = ty.peel_refs();
    let ty::Adt(adt, _) = ty.kind() else { return None };
    let did = adt.did();
    let krate = cx.tcx.crate_name(did.krate);
    let name = cx.tcx.item_name(did);
    for (k, n) in SCRUTINEE_TYPES {
        if krate.as_str() == *k && name.as_str() == *n {
            return Some(*n);
        }
    }
    None
}

fn trait_matches_visitor(cx: &LateContext<'_>, def_id: hir::def_id::DefId) -> bool {
    let krate = cx.tcx.crate_name(def_id.krate);
    let name = cx.tcx.item_name(def_id);
    VISITOR_TRAITS
        .iter()
        .any(|(k, n)| krate.as_str() == *k && name.as_str() == *n)
}

fn in_visitor_impl(cx: &LateContext<'_>, hir_id: hir::HirId) -> bool {
    for (_, node) in cx.tcx.hir_parent_iter(hir_id) {
        if let Node::Item(item) = node
            && let hir::ItemKind::Impl(impl_) = &item.kind
            && let Some(header) = impl_.of_trait
            && let Some(def_id) = header.trait_ref.path.res.opt_def_id()
            && trait_matches_visitor(cx, def_id)
        {
            return true;
        }
    }
    false
}

impl<'tcx> LateLintPass<'tcx> for WalkOverOxcExpression {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx hir::Expr<'tcx>) {
        let ExprKind::Match(scrutinee, arms, _) = &expr.kind else { return };
        let ty = cx.typeck_results().expr_ty(scrutinee);
        let Some(enum_name) = matches_scrutinee(cx, ty) else { return };
        let non_wildcard = arms
            .iter()
            .filter(|a| !matches!(a.pat.kind, PatKind::Wild))
            .count();
        if non_wildcard < THRESHOLD {
            return;
        }
        if in_visitor_impl(cx, expr.hir_id) {
            return;
        }
        let count = non_wildcard;
        let type_name = enum_name;
        cx.opt_span_lint(
            WALK_OVER_OXC_EXPRESSION,
            Some(expr.span),
            DiagDecorator(move |diag: &mut Diag<'_, ()>| {
                diag.primary_message(format!(
                    "match walks {count} variants of `oxc_ast::{type_name}` outside `impl Visit`; use `oxc_ast_visit::Visit` or `VisitMut` instead"
                ));
            }),
        );
    }
}
