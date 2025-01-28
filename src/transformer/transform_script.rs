use std::collections::HashMap;

use oxc_allocator::{Box as ArenaBox, Vec as ArenaVec};
use oxc_ast::ast::{
    self, AssignmentExpression, AssignmentTarget, BindingPatternKind, Expression,
    ExpressionStatement, Program, SimpleAssignmentTarget, Statement, UpdateOperator,
};
use oxc_semantic::{ScopeTree, SymbolId, SymbolTable};
use oxc_traverse::{traverse_mut, Ancestor, Traverse, TraverseCtx};

use crate::analyze::Rune;

use super::builder::{Builder, BuilderExpression, BuilderFunctionArgument};

pub struct TransformScript<'a> {
    b: &'a Builder<'a>,
}

#[derive(Debug)]
pub struct TransformResult {
    pub symbols: SymbolTable,
    pub scopes: ScopeTree,
}

#[derive(Debug)]
pub struct TransformExpressionResult<'a> {
    pub symbols: SymbolTable,
    pub scopes: ScopeTree,
    pub expression: Expression<'a>,
}

impl<'a> TransformScript<'a> {
    pub fn new(builder: &'a Builder<'a>) -> Self {
        return Self { b: builder };
    }

    pub fn transform(
        &self,
        program: &mut Program<'a>,
        symbols: SymbolTable,
        scopes: ScopeTree,
        runes: &HashMap<SymbolId, Rune>,
    ) -> TransformResult {
        let mut transformer = TransformerImpl {
            runes,
            builder: self.b,
        };

        let (symbols, scopes) = traverse_mut(
            &mut transformer,
            &self.b.ast.allocator,
            program,
            symbols,
            scopes,
        );

        return TransformResult { symbols, scopes };
    }

    pub fn transform_expression(
        &self,
        expression: Expression<'a>,
        symbols: SymbolTable,
        scopes: ScopeTree,
        runes: &HashMap<SymbolId, Rune>,
    ) -> TransformExpressionResult<'a> {
        let mut transformer = TransformerImpl {
            runes,
            builder: self.b,
        };

        let mut program = self.b.program(vec![self
            .b
            .stmt(super::builder::BuilderStatement::Expr(expression))]);

        program.set_scope_id(scopes.root_scope_id());

        let (symbols, scopes) = traverse_mut(
            &mut transformer,
            &self.b.ast.allocator,
            &mut program,
            symbols,
            scopes,
        );

        let stmt = program.body.remove(0);

        let expression = if let Statement::ExpressionStatement(mut stmt) = stmt {
            self.b.ast.move_expression(&mut stmt.expression)
        } else {
            unreachable!()
        };

        return TransformExpressionResult {
            scopes,
            symbols,
            expression,
        };
    }
}

struct TransformerImpl<'link, 'a> {
    runes: &'link HashMap<SymbolId, Rune>,
    builder: &'link Builder<'a>,
}

impl<'link, 'a> TransformerImpl<'link, 'a> {
    fn transform_rune_reference(&mut self, node: &mut Expression<'a>, ctx: &mut TraverseCtx<'a>) {
        let Expression::Identifier(ident) = node else {
            unreachable!()
        };

        if let Some(rune) = self.get_rune_by_reference(ident, ctx) {
            if !rune.mutated {
                return;
            }

            let call = self
                .builder
                .call("$.get", [BuilderFunctionArgument::Ident(&ident.name)]);

            *node = Expression::CallExpression(self.builder.alloc(call))
        }
    }

    fn transform_rune_update(&mut self, node: &mut Expression<'a>, ctx: &mut TraverseCtx<'a>) {
        let Expression::UpdateExpression(update) = node else {
            unreachable!();
        };

        let ident =
            if let SimpleAssignmentTarget::AssignmentTargetIdentifier(ident) = &update.argument {
                if self.is_rune_reference(ident, ctx) {
                    Some(ident.name.as_str())
                } else {
                    None
                }
            } else {
                None
            };

        if let Some(name) = ident {
            let callee = if update.prefix {
                "$.update_pre"
            } else {
                "$.update"
            };

            let mut args = vec![BuilderFunctionArgument::Ident(name)];

            if update.operator == UpdateOperator::Decrement {
                args.push(BuilderFunctionArgument::Num(-1.0));
            }

            let call = self.builder.call(&callee, args);

            *node = Expression::CallExpression(self.builder.alloc(call))
        }
    }

    fn transform_rune_assignment(&mut self, node: &mut Expression<'a>, ctx: &mut TraverseCtx<'a>) {
        let Expression::AssignmentExpression(assign) = node else {
            unreachable!();
        };

        let ident = if let AssignmentTarget::AssignmentTargetIdentifier(ident) = &assign.left {
            if self.is_rune_reference(ident, ctx) {
                Some(ident.name.as_str())
            } else {
                None
            }
        } else {
            None
        };

        if let Some(name) = ident {
            let mut right = self.builder.ast.move_expression(&mut assign.right);

            if !right.is_literal() {
                right = self
                    .builder
                    .call_expr("$.proxy", [BuilderFunctionArgument::Expr(right)]);
            }

            let call = self.builder.call(
                "$.set",
                [
                    BuilderFunctionArgument::Ident(name),
                    BuilderFunctionArgument::Expr(right),
                ],
            );

            *node = Expression::CallExpression(self.builder.alloc(call));
        }
    }

    fn is_rune_reference(
        &self,
        ident: &ast::IdentifierReference<'a>,
        ctx: &mut TraverseCtx<'a>,
    ) -> bool {
        return self.get_rune_by_reference(ident, ctx).is_some();
    }

    fn get_rune_by_reference(
        &self,
        ident: &ast::IdentifierReference<'a>,
        ctx: &mut TraverseCtx<'a>,
    ) -> Option<&Rune> {
        let reference_id = ident.reference_id.get();

        if reference_id.is_none() {
            return None;
        }

        let reference_id = reference_id.unwrap();
        let reference = ctx.symbols().get_reference(reference_id);
        let symbol_id = reference.symbol_id();

        if symbol_id.is_none() {
            return None;
        }

        return self.runes.get(&symbol_id.unwrap());
    }
}

impl<'a, 'link> Traverse<'a> for TransformerImpl<'link, 'a> {
    fn enter_variable_declarator(
        &mut self,
        node: &mut ast::VariableDeclarator<'a>,
        _ctx: &mut TraverseCtx<'a>,
    ) {
        if let BindingPatternKind::BindingIdentifier(id) = &node.id.kind {
            if let Some(rune) = self.runes.get(&id.symbol_id()) {
                if let Some(expr) = node.init.as_mut() {
                    let expr = self.builder.ast.move_expression(expr);

                    if let Expression::CallExpression(mut call) = expr {
                        if rune.mutated {
                            call.callee = self
                                .builder
                                .expr(BuilderExpression::Ident(self.builder.rid("$.state")));

                            if call.arguments.is_empty() {
                                call.arguments.push(
                                    self.builder
                                        .arg(BuilderFunctionArgument::Ident("undefined")),
                                );
                            }

                            node.init = Some(Expression::CallExpression(call))
                        } else {
                            let expr: Expression<'a> = if call.arguments.is_empty() {
                                let undef = self.builder.rid("undefined");
                                Expression::Identifier(self.builder.alloc(undef))
                            } else {
                                call.arguments.remove(0).into_expression()
                            };

                            node.init = Some(expr);
                        }
                    }
                }
            }
        }
    }

    fn enter_expression(&mut self, node: &mut Expression<'a>, ctx: &mut TraverseCtx<'a>) {
        match node {
            Expression::Identifier(_) => {
                self.transform_rune_reference(node, ctx);
            }
            Expression::AssignmentExpression(_) => {
                self.transform_rune_assignment(node, ctx);
            }
            Expression::UpdateExpression(_) => {
                self.transform_rune_update(node, ctx);
            }
            _ => return,
        }
    }
}
