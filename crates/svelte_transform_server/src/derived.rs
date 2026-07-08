use std::mem;

use oxc_ast::ast::{
    Argument, AssignmentOperator, AssignmentTarget, BindingPattern, CallExpression, Expression,
    IdentifierReference, PropertyKey, Statement, VariableDeclarator,
};
use oxc_span::SPAN;
use svelte_analyze::{
    AnalysisData, BindingSemantics, DeclaratorSemantics, DerivedAsyncKind, DerivedKind,
    DerivedSource, IdentGen,
};
use svelte_ast_builder::{Arg, Builder};
use svelte_component_semantics::{Access, SymbolId, walk_bindings};

use crate::model::ServerTransform;

impl<'a> ServerTransform<'_, 'a> {
    pub(crate) fn rewrite_derived(
        &self,
        declarator: &mut VariableDeclarator<'a>,
        kind: DerivedKind,
        async_kind: DerivedAsyncKind,
        source: DerivedSource,
    ) {
        if !matches!(&declarator.id, BindingPattern::BindingIdentifier(_)) {
            return;
        }
        let Some(Expression::CallExpression(call)) = declarator.init.as_mut() else {
            return;
        };
        let value = self.take_first_arg(call);
        if matches!(kind, DerivedKind::Derived)
            && matches!(async_kind, DerivedAsyncKind::Sync)
            && let Expression::Identifier(id) = value.get_inner_expression()
            && self.reads_runtime_derived(id)
        {
            let getter = self.b.rid_expr(id.name.as_str());
            declarator.init = Some(self.b.call_expr("$.derived", [Arg::Expr(getter)]));
            return;
        }
        let _ = source;
        declarator.init = Some(build_derived_init(self.b, kind, async_kind, value));
    }

    fn reads_runtime_derived(&self, id: &IdentifierReference<'a>) -> bool {
        id.reference_id
            .get()
            .and_then(|ref_id| self.analysis.symbol_for_reference(ref_id))
            .is_some_and(|sym| {
                matches!(
                    self.analysis.binding_semantics(sym),
                    BindingSemantics::Derived(_) | BindingSemantics::OptimizedDerived(_)
                )
            })
    }

    fn take_first_arg(&self, call: &mut CallExpression<'a>) -> Expression<'a> {
        let Some(first) = call.arguments.first_mut() else {
            return self.b.void_zero_expr();
        };
        if matches!(first, Argument::SpreadElement(_)) {
            return self.b.void_zero_expr();
        }
        let mut taken = Argument::from(self.b.cheap_expr());
        mem::swap(first, &mut taken);
        taken.into_expression()
    }
}

pub(crate) fn build_derived_init<'a>(
    b: &Builder<'a>,
    kind: DerivedKind,
    async_kind: DerivedAsyncKind,
    value: Expression<'a>,
) -> Expression<'a> {
    match kind {
        DerivedKind::DerivedBy => b.call_expr("$.derived", [Arg::Expr(value)]),
        DerivedKind::Derived => match async_kind {
            DerivedAsyncKind::Async => {
                let thunk = b.async_thunk(unwrap_wrappers(value));
                b.await_expr(b.call_expr("$.async_derived", [Arg::Expr(thunk)]))
            }
            DerivedAsyncKind::Sync => b.call_expr("$.derived", [Arg::Expr(b.thunk(value))]),
        },
    }
}

fn unwrap_wrappers(expr: Expression<'_>) -> Expression<'_> {
    match expr {
        Expression::ParenthesizedExpression(p) => unwrap_wrappers(p.unbox().expression),
        Expression::TSAsExpression(t) => unwrap_wrappers(t.unbox().expression),
        Expression::TSSatisfiesExpression(t) => unwrap_wrappers(t.unbox().expression),
        Expression::TSNonNullExpression(t) => unwrap_wrappers(t.unbox().expression),
        other => other,
    }
}

struct Expansion<'a> {
    dd: &'a str,
    init: Expression<'a>,
    entries: Vec<(&'a str, Expression<'a>)>,
}

enum PathAccess<'a> {
    Key(&'a str),
    Index(u32),
}

fn prepare<'a>(
    b: &Builder<'a>,
    analysis: &AnalysisData<'a>,
    ident_gen: &mut IdentGen,
    declarator: &mut VariableDeclarator<'a>,
) -> Option<Expansion<'a>> {
    let (kind, async_kind) = match analysis.declarator_semantics(declarator.node_id()) {
        DeclaratorSemantics::RuneDerived {
            kind, async_kind, ..
        } => (kind, async_kind),
        _ => return None,
    };
    if matches!(&declarator.id, BindingPattern::BindingIdentifier(_)) {
        return None;
    }

    let targets = collect_destructure_paths(b, &declarator.id)?;

    let Some(Expression::CallExpression(call)) = declarator.init.as_mut() else {
        return None;
    };
    let first = call.arguments.first_mut()?;
    if matches!(first, Argument::SpreadElement(_)) {
        return None;
    }
    let mut taken = Argument::from(b.cheap_expr());
    mem::swap(first, &mut taken);
    let value = taken.into_expression();

    let dd: &str = b.alloc_str(&ident_gen.generate("$$d"));
    let init = build_derived_init(b, kind, async_kind, value);

    let entries = targets
        .into_iter()
        .map(|(symbol, path)| {
            let name: &str = b.alloc_str(analysis.scoping.symbol_name(symbol));
            let root = b.call_expr_callee(b.rid_expr(dd), []);
            let member = build_access(b, root, &path);
            (name, b.call_expr("$.derived", [Arg::Expr(b.thunk(member))]))
        })
        .collect();

    Some(Expansion { dd, init, entries })
}

fn collect_destructure_paths<'a>(
    b: &Builder<'a>,
    pattern: &BindingPattern<'a>,
) -> Option<Vec<(SymbolId, Vec<PathAccess<'a>>)>> {
    let mut targets: Vec<(SymbolId, Vec<PathAccess<'a>>)> = Vec::new();
    let mut supported = true;
    walk_bindings(pattern, |v| {
        if v.is_rest {
            supported = false;
            return;
        }
        let mut path = Vec::with_capacity(v.path.len());
        for step in v.path {
            if step.default.is_some() {
                supported = false;
                return;
            }
            match step.access {
                Access::Key {
                    key,
                    computed: false,
                } => match static_key_name(key) {
                    Some(name) => path.push(PathAccess::Key(b.alloc_str(name))),
                    None => supported = false,
                },
                Access::Index { index, .. } => path.push(PathAccess::Index(index)),
                _ => supported = false,
            }
        }
        targets.push((v.symbol, path));
    });
    if !supported || targets.is_empty() {
        return None;
    }
    Some(targets)
}

fn build_access<'a>(
    b: &Builder<'a>,
    root: Expression<'a>,
    path: &[PathAccess<'a>],
) -> Expression<'a> {
    let mut access = root;
    for step in path {
        access = match step {
            PathAccess::Key(key) => b.static_member_expr(access, key),
            PathAccess::Index(index) => b.computed_member_expr(access, b.num_expr(*index as f64)),
        };
    }
    access
}

pub(crate) fn expand_derived_destructure_statements<'a>(
    b: &Builder<'a>,
    analysis: &AnalysisData<'a>,
    ident_gen: &mut IdentGen,
    mut declarator: VariableDeclarator<'a>,
) -> Result<Vec<Statement<'a>>, VariableDeclarator<'a>> {
    let Some(expansion) = prepare(b, analysis, ident_gen, &mut declarator) else {
        return Err(declarator);
    };
    let mut stmts = vec![b.var_stmt(expansion.dd, expansion.init)];
    for (name, derived) in expansion.entries {
        let target = AssignmentTarget::AssignmentTargetIdentifier(
            b.alloc(b.ast.identifier_reference(SPAN, name)),
        );
        let assign = b
            .ast
            .expression_assignment(SPAN, AssignmentOperator::Assign, target, derived);
        stmts.push(b.expr_stmt(assign));
    }
    Ok(stmts)
}

fn static_key_name<'a>(key: &PropertyKey<'a>) -> Option<&'a str> {
    match key {
        PropertyKey::StaticIdentifier(id) => Some(id.name.as_str()),
        PropertyKey::StringLiteral(s) => Some(s.value.as_str()),
        _ => None,
    }
}
