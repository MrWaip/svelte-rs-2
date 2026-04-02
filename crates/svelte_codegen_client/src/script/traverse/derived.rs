use rustc_hash::FxHashSet;

use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::IgnoreData;

use crate::builder::{Arg, Builder};
use crate::script::{compute_line_col, sanitize_location};

/// Dev-mode context for adding label/location args to `$.async_derived`.
pub(crate) struct DevContext<'a> {
    pub(crate) component_source: &'a str,
    pub(crate) script_content_start: u32,
    pub(crate) filename: &'a str,
    /// Symbols whose `$derived` init was `$derived(await expr)` — tracked before
    /// `rewrite_dev_await_tracking` can transform the `await` into a different form.
    pub(crate) async_derived_pending: FxHashSet<oxc_semantic::SymbolId>,
    /// Svelte-ignore directives (populated in analyze, includes span-based JS comment lookups).
    pub(crate) ignore_data: &'a IgnoreData,
}

impl DevContext<'_> {
    /// `script_offset` is relative to the script block start, not the full component source.
    fn locate(&self, script_offset: u32) -> String {
        let full_offset = self.script_content_start + script_offset;
        let (line, col) = compute_line_col(self.component_source, full_offset);
        format!("{}:{}:{}", sanitize_location(self.filename), line, col)
    }
}

/// Post-traverse: wrap `$.derived(expr)` → `$.derived(() => expr)` for $derived runes.
pub(crate) fn wrap_derived_thunks<'a>(
    b: &Builder<'a>,
    program: &mut oxc_ast::ast::Program<'a>,
    pending: &FxHashSet<oxc_semantic::SymbolId>,
    dev_ctx: Option<&DevContext<'_>>,
) {
    wrap_derived_thunks_in_stmts(b, &mut program.body, pending, dev_ctx);
}

fn wrap_derived_thunks_in_stmts<'a>(
    b: &Builder<'a>,
    stmts: &mut oxc_allocator::Vec<'a, Statement<'a>>,
    pending: &FxHashSet<oxc_semantic::SymbolId>,
    dev_ctx: Option<&DevContext<'_>>,
) {
    for stmt in stmts.iter_mut() {
        match stmt {
            Statement::VariableDeclaration(decl) => {
                let decl_start = decl.span.start;
                for declarator in decl.declarations.iter_mut() {
                    let sym_id = match &declarator.id {
                        oxc_ast::ast::BindingPattern::BindingIdentifier(id) => {
                            match id.symbol_id.get() {
                                Some(s) => s,
                                None => continue,
                            }
                        }
                        _ => continue,
                    };
                    if !pending.contains(&sym_id) {
                        continue;
                    }
                    if let Some(Expression::CallExpression(call)) = &mut declarator.init {
                        if !call.arguments.is_empty() {
                            let mut dummy = oxc_ast::ast::Argument::from(b.cheap_expr());
                            std::mem::swap(&mut call.arguments[0], &mut dummy);
                            let arg_expr = dummy.into_expression();

                            // Determine whether this is an async derived.
                            // We rely on `async_derived_pending` because in dev mode the
                            // `await` inside `$derived(await expr)` has already been
                            // transformed to `(await $.track_reactivity_loss(expr))()` by
                            // `rewrite_dev_await_tracking`, so we can't detect it by
                            // checking for `AwaitExpression` alone.
                            let is_async = matches!(arg_expr, Expression::AwaitExpression(_))
                                || dev_ctx
                                    .as_ref()
                                    .is_some_and(|ctx| ctx.async_derived_pending.contains(&sym_id));

                            let var_name = match &declarator.id {
                                oxc_ast::ast::BindingPattern::BindingIdentifier(id) => {
                                    id.name.to_string()
                                }
                                _ => String::new(),
                            };

                            if is_async {
                                // Must read span before the mem::swap below invalidates arg positions.
                                let init_span_start = call.span.start;

                                // Thunk form depends on whether arg is still AwaitExpression
                                // (non-dev mode) or already transformed (dev mode).
                                let thunk =
                                    if let Expression::AwaitExpression(await_expr) = arg_expr {
                                        // Non-dev: strip the await, create non-async thunk.
                                        // `async () => await fetch(x)` → optimized to `() => fetch(x)`
                                        // by the reference's `unthunk`. We match that behavior directly.
                                        let inner = await_expr.unbox().argument;
                                        b.thunk(inner)
                                    } else {
                                        // Dev mode: arg was already transformed to
                                        // `(await $.track_reactivity_loss(expr))()` by
                                        // `rewrite_dev_await_tracking`. Use async_arrow_expr_body
                                        // to avoid adding an extra `await`.
                                        b.async_arrow_expr_body(arg_expr)
                                    };

                                let mut extra_args: Vec<oxc_ast::ast::Argument<'a>> = Vec::new();
                                if let Some(ctx) = dev_ctx {
                                    extra_args
                                        .push(oxc_ast::ast::Argument::from(b.str_expr(&var_name)));
                                    // Only pass location if not suppressed by svelte-ignore await_waterfall
                                    if !ctx
                                        .ignore_data
                                        .is_ignored_at_span(decl_start, "await_waterfall")
                                    {
                                        let loc = ctx.locate(init_span_start);
                                        extra_args
                                            .push(oxc_ast::ast::Argument::from(b.str_expr(&loc)));
                                    }
                                }

                                call.arguments[0] = oxc_ast::ast::Argument::from(thunk);
                                for arg in extra_args {
                                    call.arguments.push(arg);
                                }
                                call.callee = b.rid_expr("$.async_derived");
                                let call_expr = b.move_expr(declarator.init.as_mut().unwrap());
                                declarator.init = Some(b.await_expr(call_expr));
                            } else {
                                let thunk = b.thunk(arg_expr);
                                call.arguments[0] = oxc_ast::ast::Argument::from(thunk);

                                if dev_ctx.is_some() {
                                    let derived_expr =
                                        b.move_expr(declarator.init.as_mut().unwrap());
                                    declarator.init = Some(b.call_expr(
                                        "$.tag",
                                        [Arg::Expr(derived_expr), Arg::Str(var_name)],
                                    ));
                                }
                            }
                        }
                    }
                }
            }
            Statement::FunctionDeclaration(func) => {
                if let Some(body) = &mut func.body {
                    wrap_derived_thunks_in_stmts(b, &mut body.statements, pending, dev_ctx);
                }
            }
            Statement::ExportNamedDeclaration(export) => {
                if let Some(oxc_ast::ast::Declaration::FunctionDeclaration(func)) =
                    &mut export.declaration
                {
                    if let Some(body) = &mut func.body {
                        wrap_derived_thunks_in_stmts(b, &mut body.statements, pending, dev_ctx);
                    }
                }
            }
            _ => {}
        }
    }
}

/// Wrap a non-simple default expression for lazy evaluation.
pub(crate) fn wrap_lazy<'a>(b: &Builder<'a>, expr: Expression<'a>) -> Expression<'a> {
    if let Expression::CallExpression(call) = &expr {
        if call.arguments.is_empty() {
            if let Expression::Identifier(_) = &call.callee {
                return b.clone_expr(&call.callee);
            }
        }
    }
    b.arrow_expr(b.no_params(), [b.expr_stmt(expr)])
}
