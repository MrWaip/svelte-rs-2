use oxc_ast::ast::Expression;
use oxc_span::SPAN;
use oxc_syntax::scope::ScopeId;
use svelte_analyze::{LegacyBindablePropSemantics, PropDefaultKind, PropsFlags};
use svelte_ast_builder::{Arg, Builder};

pub fn props_member<'a>(b: &Builder<'a>, prop_name: &str) -> Expression<'a> {
    let ast = b.ast;
    let object = ast.expression_identifier(SPAN, ast.atom("$$props"));
    let property = ast.identifier_name(SPAN, ast.atom(prop_name));
    Expression::StaticMemberExpression(
        ast.alloc(ast.static_member_expression(SPAN, object, property, false)),
    )
}

pub fn props_computed_access<'a>(b: &Builder<'a>, prop_name: &str) -> Expression<'a> {
    let ast = b.ast;
    let object = ast.expression_identifier(SPAN, ast.atom("$$props"));
    let property = b.str_expr(prop_name);
    Expression::ComputedMemberExpression(
        ast.alloc(ast.computed_member_expression(SPAN, object, property, false)),
    )
}

pub fn build_legacy_prop_call<'a>(
    b: &Builder<'a>,
    gen_arrow_scope: Option<ScopeId>,
    local: &'a str,
    alias: Option<&str>,
    legacy: LegacyBindablePropSemantics,
    default_init: Option<Expression<'a>>,
) -> Expression<'a> {
    let prop_key = alias.unwrap_or(local).to_string();
    let mut runtime_flags = legacy.flags;
    if matches!(
        legacy.default_kind,
        PropDefaultKind::Lazy | PropDefaultKind::LazyAccessor
    ) {
        runtime_flags |= PropsFlags::LAZY_INITIAL;
    }
    let flags_bits = runtime_flags.bits();
    let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident("$$props"), Arg::Str(prop_key)];
    let default_init = default_init.map(unwrap_paren_and_ts);
    match legacy.default_kind {
        PropDefaultKind::None => {
            if !runtime_flags.is_empty() {
                args.push(Arg::Num(flags_bits as f64));
            }
        }
        PropDefaultKind::Eager => {
            args.push(Arg::Num(flags_bits as f64));
            let default_expr = default_init
                .unwrap_or_else(|| panic!("eager default missing for legacy prop {local}"));
            args.push(Arg::Expr(default_expr));
        }
        PropDefaultKind::Lazy => {
            args.push(Arg::Num(flags_bits as f64));
            let default_expr = default_init
                .unwrap_or_else(|| panic!("lazy default missing for legacy prop {local}"));
            let lazy = wrap_lazy(b, default_expr);
            b.seed_arrow_scope(&lazy, gen_arrow_scope);
            args.push(Arg::Expr(lazy));
        }
        PropDefaultKind::LazyAccessor => {
            args.push(Arg::Num(flags_bits as f64));
            let default_expr = default_init
                .unwrap_or_else(|| panic!("lazy accessor default missing for legacy prop {local}"));
            let accessor_name = match &default_expr {
                Expression::Identifier(id) => id.name.as_str().to_string(),
                Expression::CallExpression(call) if call.arguments.is_empty() => match &call.callee
                {
                    Expression::Identifier(callee) => callee.name.as_str().to_string(),
                    _ => panic!(
                        "lazy accessor default must be a bare-identifier call for legacy prop {local}"
                    ),
                },
                _ => panic!(
                    "lazy accessor default must be an identifier or bare-call for legacy prop {local}"
                ),
            };
            args.push(Arg::Expr(b.rid_expr(&accessor_name)));
        }
    }
    b.call_expr("$.prop", args)
}

fn wrap_lazy<'a>(b: &Builder<'a>, expr: Expression<'a>) -> Expression<'a> {
    if let Expression::CallExpression(call) = &expr
        && call.arguments.is_empty()
        && let Expression::Identifier(_) = &call.callee
    {
        return b.clone_expr(&call.callee);
    }
    b.arrow_expr(b.no_params(), [b.expr_stmt(expr)])
}

fn unwrap_paren_and_ts<'a>(expr: Expression<'a>) -> Expression<'a> {
    let mut inner = expr.into_inner_expression();
    match &mut inner {
        Expression::ArrowFunctionExpression(arrow) => arrow.pife = false,
        Expression::FunctionExpression(func) => func.pife = false,
        _ => {}
    }
    inner
}
