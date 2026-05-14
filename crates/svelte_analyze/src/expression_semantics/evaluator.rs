use super::data::{Evaluation, KnownValue, ValueClass};
use crate::reactivity_semantics::data::ReactivitySemantics;
use crate::scope::ComponentScoping;
use crate::types::data::{BindingSemantics, SnippetData};
use compact_str::CompactString;
use oxc_ast::ast::{
    Argument, BinaryExpression, CallExpression, ConditionalExpression, Expression,
    IdentifierReference, LogicalExpression, NewExpression, Statement, StaticMemberExpression,
    TemplateLiteral, UnaryExpression,
};
use oxc_syntax::node::NodeId as OxcNodeId;
use oxc_syntax::operator::{BinaryOperator, LogicalOperator, UnaryOperator};
use rustc_hash::{FxHashMap, FxHashSet};
use crate::types::script::RuneKind;
use crate::utils::script_info::detect_rune_from_call;
use std::f64::consts;
use smallvec::{SmallVec, smallvec};
use svelte_component_semantics::{ComponentSemantics, SymbolId};

pub struct EvalCtx<'c, 'a> {
    pub scoping: &'c ComponentScoping<'a>,
    pub semantics: &'c ComponentSemantics<'a>,
    pub reactivity: &'c ReactivitySemantics,
    pub snippets: &'c SnippetData,
    pub bindings_init: &'c FxHashMap<SymbolId, &'c Expression<'a>>,
    pub function_decls: &'c FxHashSet<SymbolId>,
    pub dev: bool,
}

pub fn evaluate(expr: &Expression<'_>, ctx: &EvalCtx<'_, '_>) -> Evaluation {
    let mut guard: FxHashSet<OxcNodeId> = FxHashSet::default();
    set_to_evaluation(eval_set(expr, ctx, &mut guard))
}

#[derive(Clone, Debug)]
enum EvalAtom {
    Known(KnownValue),
    Class(ValueClass),
    Unknown,
}

type EvalSet = SmallVec<[EvalAtom; 1]>;

fn eval_set(
    expr: &Expression<'_>,
    ctx: &EvalCtx<'_, '_>,
    guard: &mut FxHashSet<OxcNodeId>,
) -> EvalSet {
    match expr {
        Expression::NullLiteral(_) => smallvec![EvalAtom::Known(KnownValue::Null)],
        Expression::BooleanLiteral(b) => smallvec![EvalAtom::Known(KnownValue::Bool(b.value))],
        Expression::NumericLiteral(n) => smallvec![EvalAtom::Known(KnownValue::Num(n.value))],
        Expression::StringLiteral(s) => smallvec![EvalAtom::Known(KnownValue::Str(
            CompactString::from(s.value.as_str())
        ))],
        Expression::BigIntLiteral(_) => smallvec![EvalAtom::Known(KnownValue::BigInt)],
        Expression::TemplateLiteral(t) => eval_template_literal(t, ctx, guard),
        Expression::Identifier(ident) => eval_identifier(ident, ctx, guard),
        Expression::BinaryExpression(bin) => eval_binary(bin, ctx, guard),
        Expression::LogicalExpression(le) => eval_logical(le, ctx, guard),
        Expression::ConditionalExpression(c) => eval_conditional(c, ctx, guard),
        Expression::UnaryExpression(u) => eval_unary(u, ctx, guard),
        Expression::ArrayExpression(_) | Expression::ObjectExpression(_) => {
            smallvec![EvalAtom::Class(ValueClass::Object)]
        }
        Expression::FunctionExpression(_)
        | Expression::ArrowFunctionExpression(_)
        | Expression::ClassExpression(_) => {
            smallvec![EvalAtom::Class(ValueClass::Function)]
        }
        Expression::StaticMemberExpression(m) => eval_static_member(m, ctx, guard),
        Expression::CallExpression(c) => eval_call(c, ctx, guard),
        Expression::NewExpression(n) => eval_new(n, ctx),
        Expression::SequenceExpression(s) => match s.expressions.last() {
            Some(last) => eval_set(last, ctx, guard),
            None => smallvec![EvalAtom::Unknown],
        },
        Expression::ParenthesizedExpression(p) => eval_set(&p.expression, ctx, guard),
        _ => smallvec![EvalAtom::Unknown],
    }
}

fn expression_node_id(expr: &Expression<'_>) -> OxcNodeId {
    use oxc_ast::ast::Expression::*;
    match expr {
        BooleanLiteral(e) => e.node_id(),
        NullLiteral(e) => e.node_id(),
        NumericLiteral(e) => e.node_id(),
        BigIntLiteral(e) => e.node_id(),
        RegExpLiteral(e) => e.node_id(),
        StringLiteral(e) => e.node_id(),
        TemplateLiteral(e) => e.node_id(),
        Identifier(e) => e.node_id(),
        MetaProperty(e) => e.node_id(),
        Super(e) => e.node_id(),
        ArrayExpression(e) => e.node_id(),
        ArrowFunctionExpression(e) => e.node_id(),
        AssignmentExpression(e) => e.node_id(),
        AwaitExpression(e) => e.node_id(),
        BinaryExpression(e) => e.node_id(),
        CallExpression(e) => e.node_id(),
        ChainExpression(e) => e.node_id(),
        ClassExpression(e) => e.node_id(),
        ConditionalExpression(e) => e.node_id(),
        FunctionExpression(e) => e.node_id(),
        ImportExpression(e) => e.node_id(),
        LogicalExpression(e) => e.node_id(),
        NewExpression(e) => e.node_id(),
        ObjectExpression(e) => e.node_id(),
        ParenthesizedExpression(e) => e.node_id(),
        SequenceExpression(e) => e.node_id(),
        TaggedTemplateExpression(e) => e.node_id(),
        ThisExpression(e) => e.node_id(),
        UnaryExpression(e) => e.node_id(),
        UpdateExpression(e) => e.node_id(),
        YieldExpression(e) => e.node_id(),
        PrivateInExpression(e) => e.node_id(),
        JSXElement(e) => e.node_id(),
        JSXFragment(e) => e.node_id(),
        TSAsExpression(e) => e.node_id(),
        TSSatisfiesExpression(e) => e.node_id(),
        TSTypeAssertion(e) => e.node_id(),
        TSNonNullExpression(e) => e.node_id(),
        TSInstantiationExpression(e) => e.node_id(),
        V8IntrinsicExpression(e) => e.node_id(),
        ComputedMemberExpression(e) => e.node_id(),
        StaticMemberExpression(e) => e.node_id(),
        PrivateFieldExpression(e) => e.node_id(),
    }
}

fn eval_call(
    c: &CallExpression<'_>,
    ctx: &EvalCtx<'_, '_>,
    guard: &mut FxHashSet<OxcNodeId>,
) -> EvalSet {
    if let Some(rune) = detect_rune_from_call(c) {
        return eval_rune_call(rune, c, ctx, guard);
    }
    let Some(keypath) = call_global_keypath(&c.callee, ctx) else {
        return smallvec![EvalAtom::Unknown];
    };
    if c.arguments.iter().any(|a| matches!(a, Argument::SpreadElement(_))) {
        if let Some(class) = global_call_return_class(&keypath) {
            return smallvec![EvalAtom::Class(class)];
        }
        return smallvec![EvalAtom::Unknown];
    }
    let mut known_args: Vec<KnownValue> = Vec::with_capacity(c.arguments.len());
    let mut all_known = true;
    for a in &c.arguments {
        let Some(expr) = a.as_expression() else {
            all_known = false;
            break;
        };
        let part = eval_set(expr, ctx, guard);
        match single_known(&part) {
            Some(v) => known_args.push(v.clone()),
            None => {
                all_known = false;
                break;
            }
        }
    }
    if all_known
        && let Some(folded) = fold_global_call(&keypath, &known_args)
    {
        return smallvec![EvalAtom::Known(folded)];
    }
    if let Some(class) = global_call_return_class(&keypath) {
        return smallvec![EvalAtom::Class(class)];
    }
    smallvec![EvalAtom::Unknown]
}

fn fold_global_call(keypath: &str, args: &[KnownValue]) -> Option<KnownValue> {
    let nums = || -> Option<Vec<f64>> { args.iter().map(known_to_number).collect() };
    match keypath {
        "Math.min" => {
            let xs = nums()?;
            Some(KnownValue::Num(xs.into_iter().fold(f64::INFINITY, f64::min)))
        }
        "Math.max" => {
            let xs = nums()?;
            Some(KnownValue::Num(
                xs.into_iter().fold(f64::NEG_INFINITY, f64::max),
            ))
        }
        "Math.abs" => Some(KnownValue::Num(known_to_number(args.first()?)?.abs())),
        "Math.floor" => Some(KnownValue::Num(known_to_number(args.first()?)?.floor())),
        "Math.ceil" => Some(KnownValue::Num(known_to_number(args.first()?)?.ceil())),
        "Math.round" => Some(KnownValue::Num(known_to_number(args.first()?)?.round())),
        "Math.trunc" => Some(KnownValue::Num(known_to_number(args.first()?)?.trunc())),
        "Math.sqrt" => Some(KnownValue::Num(known_to_number(args.first()?)?.sqrt())),
        "Math.cbrt" => Some(KnownValue::Num(known_to_number(args.first()?)?.cbrt())),
        "Math.sign" => Some(KnownValue::Num(known_to_number(args.first()?)?.signum())),
        "Math.cos" => Some(KnownValue::Num(known_to_number(args.first()?)?.cos())),
        "Math.sin" => Some(KnownValue::Num(known_to_number(args.first()?)?.sin())),
        "Math.tan" => Some(KnownValue::Num(known_to_number(args.first()?)?.tan())),
        "Math.acos" => Some(KnownValue::Num(known_to_number(args.first()?)?.acos())),
        "Math.asin" => Some(KnownValue::Num(known_to_number(args.first()?)?.asin())),
        "Math.atan" => Some(KnownValue::Num(known_to_number(args.first()?)?.atan())),
        "Math.atan2" => Some(KnownValue::Num(
            known_to_number(args.first()?)?.atan2(known_to_number(args.get(1)?)?),
        )),
        "Math.exp" => Some(KnownValue::Num(known_to_number(args.first()?)?.exp())),
        "Math.log" => Some(KnownValue::Num(known_to_number(args.first()?)?.ln())),
        "Math.log2" => Some(KnownValue::Num(known_to_number(args.first()?)?.log2())),
        "Math.log10" => Some(KnownValue::Num(known_to_number(args.first()?)?.log10())),
        "Math.log1p" => Some(KnownValue::Num(known_to_number(args.first()?)?.ln_1p())),
        "Math.expm1" => Some(KnownValue::Num(known_to_number(args.first()?)?.exp_m1())),
        "Math.pow" => Some(KnownValue::Num(
            known_to_number(args.first()?)?.powf(known_to_number(args.get(1)?)?),
        )),
        "Math.fround" => Some(KnownValue::Num(known_to_number(args.first()?)? as f32 as f64)),
        "Math.imul" => {
            let a = known_to_number(args.first()?)? as i32;
            let b = known_to_number(args.get(1)?)? as i32;
            Some(KnownValue::Num(a.wrapping_mul(b) as f64))
        }
        "Math.clz32" => {
            let x = known_to_number(args.first()?)? as u32;
            Some(KnownValue::Num(x.leading_zeros() as f64))
        }
        "Math.cosh" => Some(KnownValue::Num(known_to_number(args.first()?)?.cosh())),
        "Math.sinh" => Some(KnownValue::Num(known_to_number(args.first()?)?.sinh())),
        "Math.tanh" => Some(KnownValue::Num(known_to_number(args.first()?)?.tanh())),
        "Math.acosh" => Some(KnownValue::Num(known_to_number(args.first()?)?.acosh())),
        "Math.asinh" => Some(KnownValue::Num(known_to_number(args.first()?)?.asinh())),
        "Math.atanh" => Some(KnownValue::Num(known_to_number(args.first()?)?.atanh())),
        "Number" => Some(KnownValue::Num(known_to_number(args.first()?)?)),
        "Number.isInteger" => {
            let x = known_to_number(args.first()?)?;
            Some(KnownValue::Bool(x.is_finite() && x.fract() == 0.0))
        }
        "Number.isFinite" => Some(KnownValue::Bool(known_to_number(args.first()?)?.is_finite())),
        "Number.isNaN" => Some(KnownValue::Bool(known_to_number(args.first()?)?.is_nan())),
        "Number.isSafeInteger" => {
            let x = known_to_number(args.first()?)?;
            Some(KnownValue::Bool(
                x.is_finite() && x.fract() == 0.0 && x.abs() <= 9_007_199_254_740_991.0,
            ))
        }
        "Number.parseFloat" => match args.first()? {
            KnownValue::Str(s) => s.trim().parse::<f64>().ok().map(KnownValue::Num),
            _ => None,
        },
        "Number.parseInt" => match args.first()? {
            KnownValue::Str(s) => s.trim().parse::<i64>().ok().map(|n| KnownValue::Num(n as f64)),
            _ => None,
        },
        "String" => Some(KnownValue::Str(CompactString::from(known_to_string(
            args.first()?,
        )?))),
        "String.fromCharCode" => {
            let mut s = String::new();
            for a in args {
                let n = known_to_number(a)? as u32;
                let ch = char::from_u32(n)?;
                s.push(ch);
            }
            Some(KnownValue::Str(CompactString::from(s)))
        }
        "String.fromCodePoint" => {
            let mut s = String::new();
            for a in args {
                let n = known_to_number(a)? as u32;
                let ch = char::from_u32(n)?;
                s.push(ch);
            }
            Some(KnownValue::Str(CompactString::from(s)))
        }
        _ => None,
    }
}

fn eval_rune_call(
    rune: RuneKind,
    c: &CallExpression<'_>,
    ctx: &EvalCtx<'_, '_>,
    guard: &mut FxHashSet<OxcNodeId>,
) -> EvalSet {
    use RuneKind::*;
    let arg0 = c.arguments.first().and_then(|a| a.as_expression());
    match rune {
        State | StateRaw | Derived => match arg0 {
            Some(arg) => eval_set(arg, ctx, guard),
            None => smallvec![EvalAtom::Known(KnownValue::Undefined)],
        },
        PropsId => smallvec![EvalAtom::Class(ValueClass::String)],
        EffectTracking => smallvec![EvalAtom::Class(ValueClass::Boolean)],
        DerivedBy => match arg0 {
            Some(Expression::ArrowFunctionExpression(arrow)) if !arrow.body.is_empty() => {
                if let Some(stmt) = arrow.body.statements.first()
                    && let Statement::ExpressionStatement(es) = stmt
                    && arrow.expression
                {
                    return eval_set(&es.expression, ctx, guard);
                }
                smallvec![EvalAtom::Unknown]
            }
            _ => smallvec![EvalAtom::Unknown],
        },
        _ => smallvec![EvalAtom::Unknown],
    }
}

fn call_global_keypath(callee: &Expression<'_>, ctx: &EvalCtx<'_, '_>) -> Option<String> {
    match callee {
        Expression::Identifier(id) => {
            if ctx.semantics.symbol_for_identifier_reference(id).is_some() {
                return None;
            }
            Some(id.name.to_string())
        }
        Expression::StaticMemberExpression(m) => {
            let Expression::Identifier(obj) = &m.object else {
                return None;
            };
            if ctx.semantics.symbol_for_identifier_reference(obj).is_some() {
                return None;
            }
            Some(format!("{}.{}", obj.name, m.property.name))
        }
        _ => None,
    }
}

fn global_call_return_class(keypath: &str) -> Option<ValueClass> {
    match keypath {
        "BigInt" | "Number" => Some(ValueClass::Number),
        "String" => Some(ValueClass::String),
        kp if kp.starts_with("Math.") || kp.starts_with("Number.") => Some(ValueClass::Number),
        kp if kp.starts_with("String.") => Some(ValueClass::String),
        _ => None,
    }
}

fn eval_new(
    n: &NewExpression<'_>,
    ctx: &EvalCtx<'_, '_>,
) -> EvalSet {
    if let Expression::Identifier(callee) = &n.callee {
        let is_global = ctx
            .semantics
            .symbol_for_identifier_reference(callee)
            .is_none();
        if is_global && callee.name.as_str() == "Date" {
            return smallvec![EvalAtom::Class(ValueClass::Object)];
        }
    }
    smallvec![EvalAtom::Unknown]
}

fn eval_static_member(
    m: &StaticMemberExpression<'_>,
    ctx: &EvalCtx<'_, '_>,
    _guard: &mut FxHashSet<OxcNodeId>,
) -> EvalSet {
    if let Expression::Identifier(obj) = &m.object {
        let prop = m.property.name.as_str();
        let obj_name = obj.name.as_str();
        let is_global = ctx
            .semantics
            .symbol_for_identifier_reference(obj)
            .is_none();
        if is_global
            && let Some(known) = global_keypath(obj_name, prop)
        {
            return smallvec![EvalAtom::Known(known)];
        }
    }
    smallvec![EvalAtom::Unknown]
}

fn global_keypath(obj: &str, prop: &str) -> Option<KnownValue> {
    match (obj, prop) {
        ("Number", "POSITIVE_INFINITY") => Some(KnownValue::Num(f64::INFINITY)),
        ("Number", "NEGATIVE_INFINITY") => Some(KnownValue::Num(f64::NEG_INFINITY)),
        ("Number", "MAX_VALUE") => Some(KnownValue::Num(f64::MAX)),
        ("Number", "MIN_VALUE") => Some(KnownValue::Num(f64::MIN_POSITIVE)),
        ("Number", "MAX_SAFE_INTEGER") => Some(KnownValue::Num(9_007_199_254_740_991.0)),
        ("Number", "MIN_SAFE_INTEGER") => Some(KnownValue::Num(-9_007_199_254_740_991.0)),
        ("Number", "EPSILON") => Some(KnownValue::Num(2.220446049250313e-16)),
        ("Number", "NaN") => Some(KnownValue::Num(f64::NAN)),
        ("Math", "PI") => Some(KnownValue::Num(consts::PI)),
        ("Math", "E") => Some(KnownValue::Num(consts::E)),
        ("Math", "LN2") => Some(KnownValue::Num(consts::LN_2)),
        ("Math", "LN10") => Some(KnownValue::Num(consts::LN_10)),
        ("Math", "LOG2E") => Some(KnownValue::Num(consts::LOG2_E)),
        ("Math", "LOG10E") => Some(KnownValue::Num(consts::LOG10_E)),
        ("Math", "SQRT2") => Some(KnownValue::Num(consts::SQRT_2)),
        _ => None,
    }
}

fn eval_template_literal(
    t: &TemplateLiteral<'_>,
    ctx: &EvalCtx<'_, '_>,
    guard: &mut FxHashSet<OxcNodeId>,
) -> EvalSet {
    let mut result = String::new();
    let mut quasis = t.quasis.iter();
    let Some(first) = quasis.next() else {
        return smallvec![EvalAtom::Class(ValueClass::String)];
    };
    result.push_str(first.value.cooked.as_deref().unwrap_or(""));
    for (expr, quasi) in t.expressions.iter().zip(quasis) {
        let part = eval_set(expr, ctx, guard);
        let Some(known) = single_known(&part) else {
            return smallvec![EvalAtom::Class(ValueClass::String)];
        };
        let Some(s) = known_to_string(known) else {
            return smallvec![EvalAtom::Class(ValueClass::String)];
        };
        result.push_str(&s);
        result.push_str(quasi.value.cooked.as_deref().unwrap_or(""));
    }
    smallvec![EvalAtom::Known(KnownValue::Str(CompactString::from(result)))]
}

fn known_to_string(v: &KnownValue) -> Option<String> {
    match v {
        KnownValue::Str(s) => Some(s.to_string()),
        KnownValue::Num(n) => Some(format_js_number(*n)),
        KnownValue::Bool(b) => Some(b.to_string()),
        KnownValue::Null => Some("null".to_string()),
        KnownValue::Undefined => Some("undefined".to_string()),
        KnownValue::BigInt => None,
    }
}

fn format_js_number(n: f64) -> String {
    if n.is_nan() {
        "NaN".to_string()
    } else if n.is_infinite() {
        if n > 0.0 {
            "Infinity".to_string()
        } else {
            "-Infinity".to_string()
        }
    } else if n == n.trunc() && n.abs() < 1e21 {
        format!("{}", n as i64)
    } else {
        format!("{n}")
    }
}

fn eval_unary(
    u: &UnaryExpression<'_>,
    ctx: &EvalCtx<'_, '_>,
    guard: &mut FxHashSet<OxcNodeId>,
) -> EvalSet {
    use UnaryOperator::*;
    match u.operator {
        LogicalNot | Delete => smallvec![EvalAtom::Class(ValueClass::Boolean)],
        Void => smallvec![EvalAtom::Known(KnownValue::Undefined)],
        Typeof => smallvec![EvalAtom::Class(ValueClass::String)],
        UnaryNegation | UnaryPlus | BitwiseNot => {
            let arg = eval_set(&u.argument, ctx, guard);
            if let Some(v) = single_known(&arg)
                && let Some(folded) = fold_unary_numeric(u.operator, v)
            {
                return smallvec![EvalAtom::Known(folded)];
            }
            smallvec![EvalAtom::Class(ValueClass::Number)]
        }
    }
}

fn fold_unary_numeric(
    op: UnaryOperator,
    v: &KnownValue,
) -> Option<KnownValue> {
    use UnaryOperator::*;
    let n = known_to_number(v)?;
    let result = match op {
        UnaryNegation => -n,
        UnaryPlus => n,
        BitwiseNot => !(n as i32) as f64,
        _ => return None,
    };
    Some(KnownValue::Num(result))
}

fn known_to_number(v: &KnownValue) -> Option<f64> {
    match v {
        KnownValue::Num(n) => Some(*n),
        KnownValue::Bool(true) => Some(1.0),
        KnownValue::Bool(false) => Some(0.0),
        KnownValue::Null => Some(0.0),
        KnownValue::Undefined => Some(f64::NAN),
        KnownValue::Str(s) => s.trim().parse::<f64>().ok().or({
            if s.trim().is_empty() {
                Some(0.0)
            } else {
                None
            }
        }),
        KnownValue::BigInt => None,
    }
}

fn eval_conditional(
    ce: &ConditionalExpression<'_>,
    ctx: &EvalCtx<'_, '_>,
    guard: &mut FxHashSet<OxcNodeId>,
) -> EvalSet {
    let test = eval_set(&ce.test, ctx, guard);
    let consequent = eval_set(&ce.consequent, ctx, guard);
    let alternate = eval_set(&ce.alternate, ctx, guard);

    if let Some(test_value) = single_known(&test) {
        if is_falsy(test_value) {
            return alternate;
        } else {
            return consequent;
        }
    }

    let mut values: EvalSet = SmallVec::new();
    for v in &consequent {
        push_unique(&mut values, v);
    }
    for v in &alternate {
        push_unique(&mut values, v);
    }
    if values.is_empty() {
        smallvec![EvalAtom::Unknown]
    } else {
        values
    }
}

fn eval_identifier(
    ident: &IdentifierReference<'_>,
    ctx: &EvalCtx<'_, '_>,
    guard: &mut FxHashSet<OxcNodeId>,
) -> EvalSet {
    let Some(sym) = ctx.semantics.symbol_for_identifier_reference(ident) else {
        if ident.name.as_str() == "undefined" {
            return smallvec![EvalAtom::Known(KnownValue::Undefined)];
        }
        return smallvec![EvalAtom::Unknown];
    };
    if ctx.scoping.is_each_index_non_dynamic(sym) {
        return smallvec![EvalAtom::Class(ValueClass::Number)];
    }
    if ctx.snippets.snippet_by_symbol(sym).is_some() {
        return smallvec![EvalAtom::Class(ValueClass::Function)];
    }
    if ctx.function_decls.contains(&sym) {
        return smallvec![EvalAtom::Class(ValueClass::Function)];
    }
    if ctx.semantics.is_mutated(sym) {
        return smallvec![EvalAtom::Unknown];
    }
    if is_post_transform_unknown(ctx.reactivity, sym) {
        return smallvec![EvalAtom::Unknown];
    }
    let Some(&init_expr) = ctx.bindings_init.get(&sym) else {
        return smallvec![EvalAtom::Unknown];
    };
    if let Expression::CallExpression(call) = init_expr
        && detect_rune_from_call(call)
            == Some(RuneKind::PropsId)
    {
        return smallvec![EvalAtom::Class(ValueClass::String)];
    }
    let init_node_id = expression_node_id(init_expr);
    if !guard.insert(init_node_id) {
        return smallvec![EvalAtom::Unknown];
    }
    let result = eval_set(init_expr, ctx, guard);
    guard.remove(&init_node_id);
    if matches!(
        ctx.reactivity.binding_semantics(sym),
        BindingSemantics::OptimizedRune(_) | BindingSemantics::RuntimeRune { .. },
    ) && result
        .iter()
        .all(|a| matches!(a, EvalAtom::Known(KnownValue::Undefined)))
    {
        return smallvec![EvalAtom::Unknown];
    }
    result
}

fn eval_binary(
    bin: &BinaryExpression<'_>,
    ctx: &EvalCtx<'_, '_>,
    guard: &mut FxHashSet<OxcNodeId>,
) -> EvalSet {
    use BinaryOperator::*;
    if matches!(
        bin.operator,
        Equality | StrictEquality | Inequality | StrictInequality
    ) && ctx.dev
    {
        return smallvec![EvalAtom::Unknown];
    }

    let left = eval_set(&bin.left, ctx, guard);
    let right = eval_set(&bin.right, ctx, guard);
    if let (Some(a), Some(b)) = (single_known(&left), single_known(&right))
        && let Some(folded) = fold_binary(bin.operator, a, b)
    {
        return smallvec![EvalAtom::Known(folded)];
    }

    match bin.operator {
        Equality
        | StrictEquality
        | Inequality
        | StrictInequality
        | LessThan
        | LessEqualThan
        | GreaterThan
        | GreaterEqualThan
        | Instanceof
        | In => smallvec![EvalAtom::Class(ValueClass::Boolean)],
        Subtraction
        | Multiplication
        | Division
        | Remainder
        | Exponential
        | BitwiseOR
        | BitwiseAnd
        | BitwiseXOR
        | ShiftLeft
        | ShiftRight
        | ShiftRightZeroFill => smallvec![EvalAtom::Class(ValueClass::Number)],
        Addition => {
            let left_str = matches!(single_known(&left), Some(KnownValue::Str(_)))
                || matches!(left.as_slice(), [EvalAtom::Class(ValueClass::String)]);
            let right_str = matches!(single_known(&right), Some(KnownValue::Str(_)))
                || matches!(right.as_slice(), [EvalAtom::Class(ValueClass::String)]);
            if left_str || right_str {
                smallvec![EvalAtom::Class(ValueClass::String)]
            } else {
                smallvec![
                    EvalAtom::Class(ValueClass::Number),
                    EvalAtom::Class(ValueClass::String)
                ]
            }
        }
    }
}

fn fold_binary(
    op: BinaryOperator,
    a: &KnownValue,
    b: &KnownValue,
) -> Option<KnownValue> {
    use BinaryOperator::*;
    match op {
        Addition => match (a, b) {
            (KnownValue::Str(x), KnownValue::Str(y)) => {
                Some(KnownValue::Str(CompactString::from(format!("{x}{y}"))))
            }
            (KnownValue::Str(x), other) => {
                let s = known_to_string(other)?;
                Some(KnownValue::Str(CompactString::from(format!("{x}{s}"))))
            }
            (other, KnownValue::Str(y)) => {
                let s = known_to_string(other)?;
                Some(KnownValue::Str(CompactString::from(format!("{s}{y}"))))
            }
            _ => {
                let l = known_to_number(a)?;
                let r = known_to_number(b)?;
                Some(KnownValue::Num(l + r))
            }
        },
        Subtraction | Multiplication | Division | Remainder | Exponential | ShiftLeft
        | ShiftRight | ShiftRightZeroFill | BitwiseOR | BitwiseAnd | BitwiseXOR => {
            let l = known_to_number(a)?;
            let r = known_to_number(b)?;
            let result = match op {
                Subtraction => l - r,
                Multiplication => l * r,
                Division => l / r,
                Remainder => l % r,
                Exponential => l.powf(r),
                ShiftLeft => ((l as i32).wrapping_shl(r as u32)) as f64,
                ShiftRight => ((l as i32).wrapping_shr(r as u32)) as f64,
                ShiftRightZeroFill => ((l as u32).wrapping_shr(r as u32)) as f64,
                BitwiseOR => ((l as i32) | (r as i32)) as f64,
                BitwiseAnd => ((l as i32) & (r as i32)) as f64,
                BitwiseXOR => ((l as i32) ^ (r as i32)) as f64,
                _ => unreachable!(),
            };
            Some(KnownValue::Num(result))
        }
        Equality => Some(KnownValue::Bool(loose_equal(a, b))),
        Inequality => Some(KnownValue::Bool(!loose_equal(a, b))),
        StrictEquality => Some(KnownValue::Bool(strict_equal(a, b))),
        StrictInequality => Some(KnownValue::Bool(!strict_equal(a, b))),
        LessThan => {
            let l = known_to_number(a)?;
            let r = known_to_number(b)?;
            Some(KnownValue::Bool(l < r))
        }
        LessEqualThan => {
            let l = known_to_number(a)?;
            let r = known_to_number(b)?;
            Some(KnownValue::Bool(l <= r))
        }
        GreaterThan => {
            let l = known_to_number(a)?;
            let r = known_to_number(b)?;
            Some(KnownValue::Bool(l > r))
        }
        GreaterEqualThan => {
            let l = known_to_number(a)?;
            let r = known_to_number(b)?;
            Some(KnownValue::Bool(l >= r))
        }
        Instanceof | In => None,
    }
}

fn strict_equal(a: &KnownValue, b: &KnownValue) -> bool {
    match (a, b) {
        (KnownValue::Null, KnownValue::Null) => true,
        (KnownValue::Undefined, KnownValue::Undefined) => true,
        (KnownValue::Bool(x), KnownValue::Bool(y)) => x == y,
        (KnownValue::Num(x), KnownValue::Num(y)) => x == y,
        (KnownValue::Str(x), KnownValue::Str(y)) => x == y,
        _ => false,
    }
}

fn loose_equal(a: &KnownValue, b: &KnownValue) -> bool {
    match (a, b) {
        (KnownValue::Null, KnownValue::Undefined) | (KnownValue::Undefined, KnownValue::Null) => {
            true
        }
        _ => strict_equal(a, b),
    }
}

fn eval_logical(
    le: &LogicalExpression<'_>,
    ctx: &EvalCtx<'_, '_>,
    guard: &mut FxHashSet<OxcNodeId>,
) -> EvalSet {
    use LogicalOperator::*;
    let a = eval_set(&le.left, ctx, guard);
    let b = eval_set(&le.right, ctx, guard);

    let mut values: EvalSet = SmallVec::new();

    if let Some(a_value) = single_known(&a) {
        let short_circuit_to_a = match le.operator {
            And => is_falsy(a_value),
            Or => !is_falsy(a_value),
            Coalesce => !is_nullish_known(a_value),
        };
        if short_circuit_to_a {
            push_unique(&mut values, &EvalAtom::Known(a_value.clone()));
        } else {
            for v in &b {
                push_unique(&mut values, v);
            }
        }
    } else {
        for v in &a {
            push_unique(&mut values, v);
        }
        for v in &b {
            push_unique(&mut values, v);
        }
    }

    if values.is_empty() {
        smallvec![EvalAtom::Unknown]
    } else {
        values
    }
}

fn single_known(set: &EvalSet) -> Option<&KnownValue> {
    if set.len() != 1 {
        return None;
    }
    if let EvalAtom::Known(v) = &set[0] {
        Some(v)
    } else {
        None
    }
}

fn push_unique(dst: &mut EvalSet, v: &EvalAtom) {
    if !dst.iter().any(|x| atoms_equal(x, v)) {
        dst.push(v.clone());
    }
}

fn atoms_equal(a: &EvalAtom, b: &EvalAtom) -> bool {
    match (a, b) {
        (EvalAtom::Known(x), EvalAtom::Known(y)) => x == y,
        (EvalAtom::Class(x), EvalAtom::Class(y)) => x == y,
        (EvalAtom::Unknown, EvalAtom::Unknown) => true,
        _ => false,
    }
}

fn is_post_transform_unknown(reactivity: &ReactivitySemantics, sym: SymbolId) -> bool {
    matches!(
        reactivity.binding_semantics(sym),
        BindingSemantics::Prop(_)
            | BindingSemantics::LegacyBindableProp(_)
            | BindingSemantics::State(_)
            | BindingSemantics::Derived(_)
            | BindingSemantics::LegacyState(_)
            | BindingSemantics::Store(_)
            | BindingSemantics::Contextual(_)
    )
}

fn is_falsy(v: &KnownValue) -> bool {
    match v {
        KnownValue::Null | KnownValue::Undefined => true,
        KnownValue::Bool(false) => true,
        KnownValue::Num(n) => *n == 0.0 || n.is_nan(),
        KnownValue::Str(s) => s.is_empty(),
        KnownValue::Bool(true) | KnownValue::BigInt => false,
    }
}

fn is_nullish_known(v: &KnownValue) -> bool {
    matches!(v, KnownValue::Null | KnownValue::Undefined)
}

fn set_to_evaluation(set: EvalSet) -> Evaluation {
    if set.len() == 1
        && let EvalAtom::Known(v) = &set[0]
    {
        return Evaluation::Known(v.clone());
    }

    let mut nullish = false;
    let mut has_unknown = false;
    let mut single_class: Option<ValueClass> = None;
    let mut multi_class = false;

    for atom in &set {
        match atom {
            EvalAtom::Unknown => {
                has_unknown = true;
                nullish = true;
            }
            EvalAtom::Known(KnownValue::Null | KnownValue::Undefined) => {
                nullish = true;
            }
            EvalAtom::Known(v) => {
                let cls = match v {
                    KnownValue::Bool(_) => ValueClass::Boolean,
                    KnownValue::Num(_) => ValueClass::Number,
                    KnownValue::Str(_) => ValueClass::String,
                    KnownValue::BigInt => ValueClass::BigInt,
                    KnownValue::Null | KnownValue::Undefined => unreachable!(),
                };
                merge_class(&mut single_class, &mut multi_class, cls);
            }
            EvalAtom::Class(cls) => {
                merge_class(&mut single_class, &mut multi_class, *cls);
            }
        }
    }

    if nullish {
        return Evaluation::MaybeNullish { has_unknown };
    }
    let class = if multi_class { None } else { single_class };
    Evaluation::Defined { class }
}

fn merge_class(
    single: &mut Option<ValueClass>,
    multi: &mut bool,
    cls: ValueClass,
) {
    if *multi {
        return;
    }
    match *single {
        None => *single = Some(cls),
        Some(existing) if existing == cls => {}
        Some(_) => {
            *multi = true;
            *single = None;
        }
    }
}
