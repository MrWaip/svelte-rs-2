use rustc_hash::FxHashSet;

use oxc_ast::ast::{Expression, Statement};

use crate::builder::Builder;

/// Post-traverse: wrap `$.derived(expr)` → `$.derived(() => expr)` for $derived runes.
pub(crate) fn wrap_derived_thunks<'a>(
    b: &Builder<'a>,
    program: &mut oxc_ast::ast::Program<'a>,
    pending: &FxHashSet<oxc_semantic::SymbolId>,
) {
    wrap_derived_thunks_in_stmts(b, &mut program.body, pending);
}

fn wrap_derived_thunks_in_stmts<'a>(
    b: &Builder<'a>,
    stmts: &mut oxc_allocator::Vec<'a, Statement<'a>>,
    pending: &FxHashSet<oxc_semantic::SymbolId>,
) {
    for stmt in stmts.iter_mut() {
        match stmt {
            Statement::VariableDeclaration(decl) => {
                for declarator in decl.declarations.iter_mut() {
                    let sym_id = match &declarator.id {
                        oxc_ast::ast::BindingPattern::BindingIdentifier(id) => match id.symbol_id.get()
                        {
                            Some(s) => s,
                            None => continue,
                        },
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

                            if let Expression::AwaitExpression(await_expr) = arg_expr {
                                let inner = await_expr.unbox().argument;
                                let thunk = b.thunk(inner);
                                call.arguments[0] = oxc_ast::ast::Argument::from(thunk);
                                call.callee = b.rid_expr("$.async_derived");
                                let call_expr = b.move_expr(declarator.init.as_mut().unwrap());
                                declarator.init = Some(b.await_expr(call_expr));
                            } else {
                                let thunk = b.thunk(arg_expr);
                                call.arguments[0] = oxc_ast::ast::Argument::from(thunk);
                            }
                        }
                    }
                }
            }
            Statement::FunctionDeclaration(func) => {
                if let Some(body) = &mut func.body {
                    wrap_derived_thunks_in_stmts(b, &mut body.statements, pending);
                }
            }
            Statement::ExportNamedDeclaration(export) => {
                if let Some(oxc_ast::ast::Declaration::FunctionDeclaration(func)) =
                    &mut export.declaration
                {
                    if let Some(body) = &mut func.body {
                        wrap_derived_thunks_in_stmts(b, &mut body.statements, pending);
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
