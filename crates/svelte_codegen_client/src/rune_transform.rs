//! Shared rune transformation helpers for $.get / $.set / $.update calls.

use oxc_ast::ast::Expression;

use crate::builder::{Arg, Builder};

/// Transform an identifier reference to a mutable rune into `$.get(name)`.
pub fn transform_rune_get<'a>(b: &Builder<'a>, name: &str) -> Expression<'a> {
    b.call_expr("$.get", [Arg::Ident(name)])
}

/// Transform an assignment to a mutable rune into `$.set(name, value)`.
/// If `proxy` is true, adds a third `true` argument.
pub fn transform_rune_set<'a>(
    b: &Builder<'a>,
    name: &str,
    right: Expression<'a>,
    proxy: bool,
) -> Expression<'a> {
    let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident(name), Arg::Expr(right)];
    if proxy {
        args.push(Arg::Bool(true));
    }
    let call = b.call("$.set", args);
    Expression::CallExpression(b.alloc(call))
}

/// Transform an update expression (++/--) on a mutable rune into `$.update` / `$.update_pre`.
pub fn transform_rune_update<'a>(
    b: &Builder<'a>,
    name: &str,
    is_prefix: bool,
    is_increment: bool,
) -> Expression<'a> {
    let (fn_name, delta): (&str, Option<f64>) = match (is_prefix, is_increment) {
        (true, true) => ("$.update_pre", None),
        (true, false) => ("$.update_pre", Some(-1.0)),
        (false, true) => ("$.update", None),
        (false, false) => ("$.update", Some(-1.0)),
    };

    let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident(name)];
    if let Some(d) = delta {
        args.push(Arg::Num(d));
    }

    b.call_expr(fn_name, args)
}
