use rustc_ast as ast;
use rustc_errors::{Diag, DiagDecorator};
use rustc_lint::{EarlyContext, EarlyLintPass, LintContext};
use rustc_session::{declare_lint, declare_lint_pass};
use rustc_span::sym;

declare_lint! {
    pub BARE_ALLOW_ON_DOMAIN_LINTS,
    Warn,
    "`#[allow(...)]` of our domain lints must include `reason = \"...\"` documenting intent"
}

declare_lint_pass!(BareAllowOnDomainLints => [BARE_ALLOW_ON_DOMAIN_LINTS]);

const DOMAIN_LINTS: &[&str] = &[
    "walk_over_oxc_expression",
    "exhaustiveness_on_domain_enum",
    "frozen_api_surface",
    "forbidden_identifier",
];

impl EarlyLintPass for BareAllowOnDomainLints {
    fn check_attribute(&mut self, cx: &EarlyContext<'_>, attr: &ast::Attribute) {
        if !attr.has_name(sym::allow) {
            return;
        }
        let Some(meta_list) = attr.meta_item_list() else { return };
        let mut targeted: Option<String> = None;
        let mut has_reason = false;
        for item in &meta_list {
            if let Some(mi) = item.meta_item() {
                if mi.has_name(sym::reason) {
                    has_reason = true;
                    continue;
                }
            }
            if let Some(ident) = item.ident() {
                let s = ident.as_str();
                if DOMAIN_LINTS.iter().any(|l| s == *l) {
                    targeted = Some(s.to_owned());
                }
            }
        }
        let Some(lint_name) = targeted else { return };
        if has_reason {
            return;
        }
        cx.opt_span_lint(
            BARE_ALLOW_ON_DOMAIN_LINTS,
            Some(attr.span),
            DiagDecorator(move |diag: &mut Diag<'_, ()>| {
                diag.primary_message(format!(
                    "`#[allow({lint_name})]` must include `reason = \"…\"` explaining the escape hatch"
                ));
            }),
        );
    }
}
