use rustc_ast as ast;
use rustc_errors::{Diag, DiagDecorator};
use rustc_lint::{EarlyContext, EarlyLintPass, LintContext};
use rustc_session::{declare_lint, declare_lint_pass};
use rustc_span::Span;

declare_lint! {
    pub FORBIDDEN_IDENTIFIER,
    Warn,
    "identifier segment is on the project banlist (CLAUDE.md ## Never use)"
}

declare_lint_pass!(ForbiddenIdentifier => [FORBIDDEN_IDENTIFIER]);

const BANLIST: &[&str] = &[
    "leaf", "leaves", "plan", "shape", "shadow", "lower", "lowering", "tpl", "dyn", "sym",
];

fn banned_segment(name: &str) -> Option<&'static str> {
    let mut segments: Vec<String> = Vec::new();
    let mut current = String::new();
    for c in name.chars() {
        if c == '_' {
            if !current.is_empty() {
                segments.push(std::mem::take(&mut current));
            }
        } else if c.is_ascii_uppercase() {
            if !current.is_empty() {
                segments.push(std::mem::take(&mut current));
            }
            current.push(c.to_ascii_lowercase());
        } else {
            current.push(c);
        }
    }
    if !current.is_empty() {
        segments.push(current);
    }
    for seg in &segments {
        for banned in BANLIST {
            if seg == banned {
                return Some(banned);
            }
        }
    }
    None
}

fn report(cx: &EarlyContext<'_>, name: &str, span: Span) {
    if let Some(banned) = banned_segment(name) {
        cx.opt_span_lint(
            FORBIDDEN_IDENTIFIER,
            Some(span),
            DiagDecorator(|diag: &mut Diag<'_, ()>| {
                diag.primary_message(format!(
                    "identifier `{name}` contains banned segment `{banned}`; see CLAUDE.md ## Never use"
                ));
            }),
        );
    }
}

impl EarlyLintPass for ForbiddenIdentifier {
    fn check_item(&mut self, cx: &EarlyContext<'_>, item: &ast::Item) {
        if let Some(ident) = item.kind.ident() {
            report(cx, ident.as_str(), ident.span);
        }
    }

    fn check_variant(&mut self, cx: &EarlyContext<'_>, variant: &ast::Variant) {
        report(cx, variant.ident.as_str(), variant.ident.span);
    }
}
