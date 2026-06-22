use std::{iter, mem};

use oxc_allocator::Vec as OxcVec;
use rustc_hash::{FxHashMap, FxHashSet};

use oxc_ast::ast::{
    Argument, BindingPattern, Class, ClassElement, Declaration, Expression, Program, Statement,
};
use oxc_semantic::SymbolId;

use super::location::sanitize_location;
use super::model::AsyncDerivedMode;
use super::model::IgnoreQuery;
use svelte_ast_builder::{Arg, Builder};

pub(crate) struct DevContext<'a> {
    pub(crate) component_line_index: &'a svelte_span::LineIndex,
    pub(crate) filename: &'a str,

    pub(crate) ignore_query: IgnoreQuery<'a, 'a>,
}

impl DevContext<'_> {
    fn locate(&self, offset: u32) -> String {
        let (line, col) = self.component_line_index.line_col(offset);
        format!("{}:{}:{}", sanitize_location(self.filename), line, col)
    }
}

pub(crate) fn wrap_derived_thunks<'a>(
    b: &Builder<'a>,
    program: &mut Program<'a>,
    pending: &FxHashSet<SymbolId>,
    async_pending: &FxHashMap<SymbolId, AsyncDerivedMode>,
    dev_ctx: Option<&DevContext<'_>>,
) {
    wrap_derived_thunks_in_stmts(b, &mut program.body, pending, async_pending, dev_ctx);
}

fn wrap_derived_thunks_in_stmts<'a>(
    b: &Builder<'a>,
    stmts: &mut OxcVec<'a, Statement<'a>>,
    pending: &FxHashSet<SymbolId>,
    async_pending: &FxHashMap<SymbolId, AsyncDerivedMode>,
    dev_ctx: Option<&DevContext<'_>>,
) {
    for stmt in stmts.iter_mut() {
        match stmt {
            Statement::VariableDeclaration(decl) => {
                let decl_start = decl.span.start;
                for declarator in decl.declarations.iter_mut() {
                    recurse_into_function_init(
                        b,
                        declarator.init.as_mut(),
                        pending,
                        async_pending,
                        dev_ctx,
                    );
                    let sym_id = match &declarator.id {
                        BindingPattern::BindingIdentifier(id) => match id.symbol_id.get() {
                            Some(s) => s,
                            None => continue,
                        },
                        _ => continue,
                    };
                    if !pending.contains(&sym_id) {
                        continue;
                    }
                    if let Some(Expression::CallExpression(call)) = &mut declarator.init
                        && !call.arguments.is_empty()
                    {
                        let mut dummy = Argument::from(b.cheap_expr());
                        mem::swap(&mut call.arguments[0], &mut dummy);
                        let arg_expr = dummy.into_expression().into_inner_expression();

                        let async_mode = async_pending.get(&sym_id).copied();
                        let is_async = matches!(arg_expr, Expression::AwaitExpression(_))
                            || async_mode.is_some();

                        let var_name = match &declarator.id {
                            BindingPattern::BindingIdentifier(id) => id.name.to_string(),
                            _ => String::new(),
                        };

                        if is_async {
                            let init_span_start = call.span.start;

                            let thunk = if let Expression::AwaitExpression(await_expr) = arg_expr {
                                let inner = await_expr.unbox().argument;
                                b.thunk(inner)
                            } else {
                                b.async_arrow_expr_body(arg_expr)
                            };

                            let mut extra_args: Vec<Argument<'a>> = Vec::new();
                            if let Some(ctx) = dev_ctx {
                                extra_args.push(Argument::from(b.str_expr(&var_name)));

                                if !ctx
                                    .ignore_query
                                    .is_ignored_at_span(decl_start, "await_waterfall")
                                {
                                    let loc = ctx.locate(init_span_start);
                                    extra_args.push(Argument::from(b.str_expr(&loc)));
                                }
                            }

                            call.arguments[0] = Argument::from(thunk);
                            for arg in extra_args {
                                call.arguments.push(arg);
                            }
                            call.callee = b.rid_expr("$.async_derived");
                            let call_expr = b.move_expr(
                                declarator
                                    .init
                                    .as_mut()
                                    .expect("$derived declarator always has an initializer"),
                            );
                            declarator.init =
                                Some(match async_mode.unwrap_or(AsyncDerivedMode::Await) {
                                    AsyncDerivedMode::Await => b.await_expr(call_expr),
                                    AsyncDerivedMode::Save => {
                                        let saved = b.call_expr("$.save", [Arg::Expr(call_expr)]);
                                        b.call_expr_callee(
                                            b.await_expr(saved),
                                            iter::empty::<Arg<'a, '_>>(),
                                        )
                                    }
                                });
                        } else {
                            let thunk = b.thunk(arg_expr);
                            call.arguments[0] = Argument::from(thunk);

                            if dev_ctx.is_some() {
                                let derived_expr = b.move_expr(
                                    declarator
                                        .init
                                        .as_mut()
                                        .expect("$derived declarator always has an initializer"),
                                );
                                declarator.init = Some(b.call_expr(
                                    "$.tag",
                                    [Arg::Expr(derived_expr), Arg::Str(var_name)],
                                ));
                            }
                        }
                    }
                }
            }
            Statement::FunctionDeclaration(func) => {
                if let Some(body) = &mut func.body {
                    wrap_derived_thunks_in_stmts(
                        b,
                        &mut body.statements,
                        pending,
                        async_pending,
                        dev_ctx,
                    );
                }
            }
            Statement::ClassDeclaration(class) => {
                wrap_derived_thunks_in_class(b, class, pending, async_pending, dev_ctx);
            }
            Statement::ExportNamedDeclaration(export) => match &mut export.declaration {
                Some(Declaration::FunctionDeclaration(func)) => {
                    if let Some(body) = &mut func.body {
                        wrap_derived_thunks_in_stmts(
                            b,
                            &mut body.statements,
                            pending,
                            async_pending,
                            dev_ctx,
                        );
                    }
                }
                Some(Declaration::ClassDeclaration(class)) => {
                    wrap_derived_thunks_in_class(b, class, pending, async_pending, dev_ctx);
                }
                Some(Declaration::VariableDeclaration(decl)) => {
                    for declarator in decl.declarations.iter_mut() {
                        recurse_into_function_init(
                            b,
                            declarator.init.as_mut(),
                            pending,
                            async_pending,
                            dev_ctx,
                        );
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}

fn wrap_derived_thunks_in_class<'a>(
    b: &Builder<'a>,
    class: &mut Class<'a>,
    pending: &FxHashSet<SymbolId>,
    async_pending: &FxHashMap<SymbolId, AsyncDerivedMode>,
    dev_ctx: Option<&DevContext<'_>>,
) {
    for element in class.body.body.iter_mut() {
        if let ClassElement::MethodDefinition(method) = element
            && let Some(body) = &mut method.value.body
        {
            wrap_derived_thunks_in_stmts(b, &mut body.statements, pending, async_pending, dev_ctx);
        }
    }
}

fn recurse_into_function_init<'a>(
    b: &Builder<'a>,
    init: Option<&mut Expression<'a>>,
    pending: &FxHashSet<SymbolId>,
    async_pending: &FxHashMap<SymbolId, AsyncDerivedMode>,
    dev_ctx: Option<&DevContext<'_>>,
) {
    let Some(expr) = init else {
        return;
    };
    match expr {
        Expression::ArrowFunctionExpression(arrow) => {
            wrap_derived_thunks_in_stmts(
                b,
                &mut arrow.body.statements,
                pending,
                async_pending,
                dev_ctx,
            );
        }
        Expression::FunctionExpression(func) => {
            if let Some(body) = &mut func.body {
                wrap_derived_thunks_in_stmts(
                    b,
                    &mut body.statements,
                    pending,
                    async_pending,
                    dev_ctx,
                );
            }
        }
        Expression::CallExpression(call) => {
            for arg in call.arguments.iter_mut() {
                if let Some(arg_expr) = arg.as_expression_mut() {
                    recurse_into_function_init(b, Some(arg_expr), pending, async_pending, dev_ctx);
                }
            }
        }
        _ => {}
    }
}

pub(crate) fn wrap_lazy<'a>(b: &Builder<'a>, expr: Expression<'a>) -> Expression<'a> {
    if let Expression::CallExpression(call) = &expr
        && call.arguments.is_empty()
        && let Expression::Identifier(_) = &call.callee
    {
        return b.clone_expr(&call.callee);
    }
    b.arrow_expr(b.no_params(), [b.expr_stmt(expr)])
}
