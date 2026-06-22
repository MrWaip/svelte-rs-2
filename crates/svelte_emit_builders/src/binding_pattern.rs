use oxc_allocator::CloneIn;
use oxc_ast::ast::{Expression, PropertyKey};
use oxc_span::GetSpan;
use oxc_syntax::scope::ScopeId;
use svelte_ast_builder::{Arg, Builder};
use svelte_component_semantics::{Access, Step};

pub fn member_access<'a>(
    b: &Builder<'a>,
    object: Expression<'a>,
    key: &PropertyKey<'_>,
    computed: bool,
) -> Expression<'a> {
    if !computed {
        match key {
            PropertyKey::StaticIdentifier(id) => {
                return b.static_member_expr(object, id.name.as_str());
            }
            PropertyKey::StringLiteral(s) => {
                return b.computed_member_expr(object, b.str_expr(s.value.as_str()));
            }
            PropertyKey::NumericLiteral(n) => {
                return b.computed_member_expr(object, b.num_expr(n.value));
            }
            _ => {}
        }
    }
    let key_expr = key
        .as_expression()
        .map(|e| e.clone_in(b.ast.allocator))
        .unwrap_or_else(|| b.void_zero_expr());
    b.computed_member_expr(object, key_expr)
}

pub fn to_array_derived<'a>(
    b: &Builder<'a>,
    source: Expression<'a>,
    count: Option<u32>,
    scope: Option<ScopeId>,
) -> Expression<'a> {
    let to_array = match count {
        Some(count) => b.call_expr("$.to_array", [Arg::Expr(source), Arg::Num(count as f64)]),
        None => b.call_expr("$.to_array", [Arg::Expr(source)]),
    };
    let thunk = b.thunk(to_array);
    b.seed_arrow_scope(&thunk, scope);
    b.call_expr("$.derived", [Arg::Expr(thunk)])
}

pub fn exclude_from_object<'a>(
    b: &Builder<'a>,
    object: Expression<'a>,
    excluded: &[&PropertyKey<'_>],
) -> Expression<'a> {
    let keys: Vec<Expression<'a>> = excluded
        .iter()
        .filter_map(|key| match key {
            PropertyKey::StaticIdentifier(id) => Some(b.str_expr(id.name.as_str())),
            PropertyKey::StringLiteral(s) => Some(b.str_expr(s.value.as_str())),
            PropertyKey::NumericLiteral(n) => Some(b.str_expr(&format_numeric_key(n.value))),
            _ => key.as_expression().map(|e| {
                let cloned = e.clone_in(b.ast.allocator);
                b.call_expr("String", [Arg::Expr(cloned)])
            }),
        })
        .collect();
    let excluded_array = b.array_expr(keys);
    b.call_expr(
        "$.exclude_from_object",
        [Arg::Expr(object), Arg::Expr(excluded_array)],
    )
}

pub fn fallback<'a>(
    b: &Builder<'a>,
    expr: Expression<'a>,
    default: &Expression<'_>,
    scope: Option<ScopeId>,
) -> Expression<'a> {
    let simple = is_simple_expression(default);
    fallback_with_simple(b, expr, default, scope, simple)
}

pub fn fallback_with_simple<'a>(
    b: &Builder<'a>,
    expr: Expression<'a>,
    default: &Expression<'_>,
    scope: Option<ScopeId>,
    simple: bool,
) -> Expression<'a> {
    let default = default.clone_in(b.ast.allocator);
    if simple {
        b.call_expr("$.fallback", [Arg::Expr(expr), Arg::Expr(default)])
    } else {
        let thunk = b.thunk(default);
        b.seed_arrow_scope(&thunk, scope);
        b.call_expr(
            "$.fallback",
            [Arg::Expr(expr), Arg::Expr(thunk), Arg::Bool(true)],
        )
    }
}

pub fn serialize_prefix(prefix: &[Step<'_>]) -> String {
    let mut out = String::new();
    for step in prefix {
        out.push_str(&serialize_access(&step.access));
        out.push('/');
    }
    out
}

fn serialize_access(access: &Access<'_>) -> String {
    match access {
        Access::Key { key, computed } => serialize_key(key, *computed),
        Access::Index { index, .. } => format!("[{index}]"),
        Access::Slice { from } => format!("[s{from}]"),
    }
}

fn serialize_key(key: &PropertyKey<'_>, computed: bool) -> String {
    match key {
        PropertyKey::StaticIdentifier(id) if !computed => format!(".{}", id.name),
        PropertyKey::StringLiteral(s) => format!(".{:?}", s.value),
        PropertyKey::NumericLiteral(n) => format!(".#{}", n.value),
        _ => {
            let span = key.span();
            format!(".[{}:{}]", span.start, span.end)
        }
    }
}

fn format_numeric_key(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{}", value as i64)
    } else {
        format!("{value}")
    }
}

fn is_simple_expression(expr: &Expression<'_>) -> bool {
    match expr {
        Expression::NumericLiteral(_)
        | Expression::StringLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::NullLiteral(_)
        | Expression::BigIntLiteral(_)
        | Expression::RegExpLiteral(_)
        | Expression::Identifier(_)
        | Expression::ArrowFunctionExpression(_)
        | Expression::FunctionExpression(_) => true,
        Expression::ParenthesizedExpression(inner) => is_simple_expression(&inner.expression),
        Expression::ConditionalExpression(cond) => {
            is_simple_expression(&cond.test)
                && is_simple_expression(&cond.consequent)
                && is_simple_expression(&cond.alternate)
        }
        Expression::BinaryExpression(bin) => {
            is_simple_expression(&bin.left) && is_simple_expression(&bin.right)
        }
        Expression::LogicalExpression(log) => {
            is_simple_expression(&log.left) && is_simple_expression(&log.right)
        }
        _ => false,
    }
}
