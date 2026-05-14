#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_lint;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

mod bare_allow_on_domain_lints;
mod exhaustiveness_on_domain_enum;
mod forbidden_identifier;
mod frozen_api_surface;
mod walk_over_oxc_binding_pattern;
mod walk_over_oxc_expression;

dylint_linting::dylint_library!();

#[unsafe(no_mangle)]
pub fn register_lints(_sess: &rustc_session::Session, lint_store: &mut rustc_lint::LintStore) {
    lint_store.register_lints(&[
        forbidden_identifier::FORBIDDEN_IDENTIFIER,
        walk_over_oxc_expression::WALK_OVER_OXC_EXPRESSION,
        walk_over_oxc_binding_pattern::WALK_OVER_OXC_BINDING_PATTERN,
        exhaustiveness_on_domain_enum::EXHAUSTIVENESS_ON_DOMAIN_ENUM,
        frozen_api_surface::FROZEN_API_SURFACE,
        bare_allow_on_domain_lints::BARE_ALLOW_ON_DOMAIN_LINTS,
    ]);
    lint_store.register_early_pass(|| Box::new(forbidden_identifier::ForbiddenIdentifier));
    lint_store.register_early_pass(|| Box::new(bare_allow_on_domain_lints::BareAllowOnDomainLints));
    lint_store.register_late_pass(|_| Box::new(walk_over_oxc_expression::WalkOverOxcExpression));
    lint_store
        .register_late_pass(|_| Box::new(walk_over_oxc_binding_pattern::WalkOverOxcBindingPattern));
    lint_store
        .register_late_pass(|_| Box::new(exhaustiveness_on_domain_enum::ExhaustivenessOnDomainEnum));
    lint_store.register_late_pass(|_| Box::new(frozen_api_surface::FrozenApiSurface));
}
