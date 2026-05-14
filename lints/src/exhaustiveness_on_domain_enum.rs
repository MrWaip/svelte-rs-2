use rustc_errors::{Diag, DiagDecorator};
use rustc_hir::{self as hir, ExprKind, PatKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_middle::ty;
use rustc_session::{declare_lint, declare_lint_pass};
use rustc_span::ExpnKind;

declare_lint! {
    pub EXHAUSTIVENESS_ON_DOMAIN_ENUM,
    Allow,
    "wildcard or large or-pattern arm on whitelisted domain enum; pick option 1, option 2, or coordinate with the enum owner"
}

const OR_THRESHOLD: usize = 3;

declare_lint_pass!(ExhaustivenessOnDomainEnum => [EXHAUSTIVENESS_ON_DOMAIN_ENUM]);

const DOMAIN_ENUMS: &[(&str, &str)] = &[
    ("svelte_analyze", "BlockSemantics"),
    ("svelte_analyze", "AttributeSemantics"),
    ("svelte_analyze", "ExpressionSemantics"),
    ("svelte_analyze", "ReferenceSemantics"),
    ("svelte_analyze", "DeclaratorSemantics"),
    ("svelte_analyze", "BindingSemantics"),
    ("svelte_analyze", "ContextualBindingSemantics"),
    ("svelte_analyze", "ConstBindingSemantics"),
    ("svelte_analyze", "StateBindingSemantics"),
    ("svelte_analyze", "PropReferenceSemantics"),
    ("svelte_analyze", "ComponentPropSemantics"),
    ("svelte_analyze", "RuneKind"),
    ("svelte_analyze", "RuntimeRuneKind"),
];

const DIVERGING_MACROS: &[&str] = &["unreachable", "panic", "todo", "unimplemented"];

fn matches_domain_enum(cx: &LateContext<'_>, ty: ty::Ty<'_>) -> Option<&'static str> {
    let ty = ty.peel_refs();
    let ty::Adt(adt, _) = ty.kind() else { return None };
    let did = adt.did();
    let krate = cx.tcx.crate_name(did.krate);
    let name = cx.tcx.item_name(did);
    for (k, n) in DOMAIN_ENUMS {
        if krate.as_str() == *k && name.as_str() == *n {
            return Some(*n);
        }
    }
    None
}

fn body_is_diverging_macro(body: &hir::Expr<'_>) -> bool {
    body.span.macro_backtrace().any(|expn| match expn.kind {
        ExpnKind::Macro(_, name) => {
            let s = name.as_str();
            DIVERGING_MACROS.iter().any(|m| s == *m)
        }
        _ => false,
    })
}

fn or_pattern_leaf_count(pat: &hir::Pat<'_>) -> usize {
    match pat.kind {
        PatKind::Or(pats) => pats.iter().map(or_pattern_leaf_count).sum(),
        _ => 1,
    }
}

impl<'tcx> LateLintPass<'tcx> for ExhaustivenessOnDomainEnum {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx hir::Expr<'tcx>) {
        let ExprKind::Match(scrutinee, arms, _) = &expr.kind else { return };
        let ty = cx.typeck_results().expr_ty(scrutinee);
        let Some(enum_name) = matches_domain_enum(cx, ty) else { return };
        for arm in arms.iter() {
            if body_is_diverging_macro(arm.body) {
                continue;
            }
            if matches!(arm.pat.kind, PatKind::Wild) {
                let type_name = enum_name;
                cx.opt_span_lint(
                    EXHAUSTIVENESS_ON_DOMAIN_ENUM,
                    Some(arm.span),
                    DiagDecorator(move |diag: &mut Diag<'_, ()>| {
                        diag.primary_message(format!(
                            "silent `_ =>` arm on domain enum `{type_name}`. Do one of:\n  1. list each handled variant explicitly so the compiler flags future additions of `{type_name}`\n  2. use `unreachable!(\"why\")` if this branch is genuinely unreachable\nIf neither fits, coordinate with the owner of `{type_name}` before silencing this lint via `#[allow(exhaustiveness_on_domain_enum, reason = \"…\")]`"
                        ));
                    }),
                );
                continue;
            }
            if matches!(arm.pat.kind, PatKind::Or(_)) {
                let n = or_pattern_leaf_count(arm.pat);
                if n >= OR_THRESHOLD {
                    let type_name = enum_name;
                    cx.opt_span_lint(
                        EXHAUSTIVENESS_ON_DOMAIN_ENUM,
                        Some(arm.span),
                        DiagDecorator(move |diag: &mut Diag<'_, ()>| {
                            diag.primary_message(format!(
                                "or-pattern arm groups {n} variants of domain enum `{type_name}` under one body. Do one of:\n  1. split into per-variant arms so each branch is reviewed independently\n  2. use `unreachable!(\"why\")` if these variants are genuinely unreachable\nIf neither fits, coordinate with the owner of `{type_name}` before silencing this lint via `#[allow(exhaustiveness_on_domain_enum, reason = \"…\")]`"
                            ));
                        }),
                    );
                }
            }
        }
    }
}
