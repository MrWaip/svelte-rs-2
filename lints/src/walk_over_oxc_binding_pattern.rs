use rustc_errors::{Diag, DiagDecorator};
use rustc_hir::{self as hir, ExprKind, Node, PatKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_middle::ty;
use rustc_session::{declare_lint, declare_lint_pass};
use rustc_span::Span;

declare_lint! {
    pub WALK_OVER_OXC_BINDING_PATTERN,
    Allow,
    "hand-written walk over oxc `BindingPattern`/`BindingPatternKind`; use `oxc_ast_visit::Visit::visit_binding_pattern` or `walk_binding_pattern` for deep traversal"
}

declare_lint_pass!(WalkOverOxcBindingPattern => [WALK_OVER_OXC_BINDING_PATTERN]);

const SCRUTINEE_TYPES: &[(&str, &str)] = &[
    ("oxc_ast", "BindingPattern"),
    ("oxc_ast", "BindingPatternKind"),
];

const VISITOR_TRAITS: &[(&str, &str)] = &[
    ("oxc_ast_visit", "Visit"),
    ("oxc_ast_visit", "VisitMut"),
    ("oxc_traverse", "Traverse"),
];

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

fn emit(cx: &LateContext<'_>, span: Span, type_name: &'static str, form: &'static str) {
    cx.opt_span_lint(
        WALK_OVER_OXC_BINDING_PATTERN,
        Some(span),
        DiagDecorator(move |diag: &mut Diag<'_, ()>| {
            diag.primary_message(format!(
                "{form} on `oxc_ast::{type_name}` outside `impl Visit`. Deep walks usually belong in `oxc_ast_visit::Visit::visit_binding_pattern` (or call `oxc_ast_visit::walk::walk_binding_pattern`). If this match is intentional and does not need to recurse into `ObjectPattern`/`ArrayPattern`/`AssignmentPattern`, coordinate before silencing via `#[allow(walk_over_oxc_binding_pattern, reason = \"…\")]`"
            ));
        }),
    );
}

impl<'tcx> LateLintPass<'tcx> for WalkOverOxcBindingPattern {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx hir::Expr<'tcx>) {
        match &expr.kind {
            ExprKind::Match(scrutinee, arms, _) => {
                let ty = cx.typeck_results().expr_ty(scrutinee);
                let Some(enum_name) = matches_scrutinee(cx, ty) else { return };
                let has_real_arm = arms
                    .iter()
                    .any(|a| !matches!(a.pat.kind, PatKind::Wild));
                if !has_real_arm {
                    return;
                }
                if in_visitor_impl(cx, expr.hir_id) {
                    return;
                }
                emit(cx, expr.span, enum_name, "match");
            }
            ExprKind::Let(let_expr) => {
                let ty = cx.typeck_results().expr_ty(let_expr.init);
                let Some(enum_name) = matches_scrutinee(cx, ty) else { return };
                if in_visitor_impl(cx, expr.hir_id) {
                    return;
                }
                emit(cx, expr.span, enum_name, "`if let` / `while let`");
            }
            _ => {}
        }
    }

    fn check_local(&mut self, cx: &LateContext<'tcx>, local: &'tcx hir::LetStmt<'tcx>) {
        if local.els.is_none() {
            return;
        }
        let Some(init) = local.init else { return };
        let ty = cx.typeck_results().expr_ty(init);
        let Some(enum_name) = matches_scrutinee(cx, ty) else { return };
        if in_visitor_impl(cx, local.hir_id) {
            return;
        }
        emit(cx, local.span, enum_name, "`let … else`");
    }
}
