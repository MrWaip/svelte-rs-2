
use oxc_ast::ast::{
    self, AssignmentTarget, BindingPatternKind, Expression, Program, SimpleAssignmentTarget,
    Statement, UpdateOperator,
};
use oxc_semantic::{ScopeTree, SymbolTable};
use oxc_traverse::{traverse_mut, Traverse, TraverseCtx};

use analyzer::svelte_table::{Rune, SvelteTable};

use ast_builder::{Builder, BuilderExpression, BuilderFunctionArgument};

pub struct TransformScript<'a, 'link> {
    b: &'a Builder<'a>,
    svelte_table: &'link SvelteTable,
}

#[derive(Debug)]
pub struct TransformResult<'a> {
    pub imports: Vec<Statement<'a>>,
}

#[derive(Debug)]
pub struct TransformExpressionResult<'a> {
    pub expression: Expression<'a>,
}

impl<'a, 'link> TransformScript<'a, 'link> {
    pub fn new(
        builder: &'a Builder<'a>,
        svelte_table: &'link SvelteTable,
    ) -> TransformScript<'a, 'link> {
        Self {
            b: builder,
            svelte_table,
        }
    }

    pub fn transform(&self, program: &mut Program<'a>) -> TransformResult {
        let mut transformer = TransformerImpl {
            svelte_table: self.svelte_table,
            builder: self.b,
            imports: vec![],
            proxy_rune: true,
        };

        traverse_mut(
            &mut transformer,
            self.b.ast.allocator,
            program,
            SymbolTable::default(),
            ScopeTree::default(),
        );

        let imports = std::mem::take(&mut transformer.imports);

        TransformResult { imports }
    }

    pub fn transform_expression(
        &self,
        expression: Expression<'a>,
        proxy_rune: bool,
    ) -> TransformExpressionResult<'a> {
        let mut program = self.b.program(vec![self
            .b
            .stmt(ast_builder::BuilderStatement::Expr(expression))]);

        program.set_scope_id(self.svelte_table.root_scope_id());

        let mut transformer = TransformerImpl {
            svelte_table: self.svelte_table,
            builder: self.b,
            imports: vec![],
            proxy_rune,
        };

        traverse_mut(
            &mut transformer,
            self.b.ast.allocator,
            &mut program,
            SymbolTable::default(),
            ScopeTree::default(),
        );

        let stmt = program.body.remove(0);

        let expression = if let Statement::ExpressionStatement(mut stmt) = stmt {
            self.b.ast.move_expression(&mut stmt.expression)
        } else {
            unreachable!()
        };

        TransformExpressionResult { expression }
    }
}

struct TransformerImpl<'link, 'a> {
    svelte_table: &'link SvelteTable,
    builder: &'link Builder<'a>,
    imports: Vec<Statement<'a>>,
    proxy_rune: bool,
}

impl<'a> TransformerImpl<'_, 'a> {
    fn transform_rune_reference(&mut self, node: &mut Expression<'a>, _ctx: &mut TraverseCtx<'a>) {
        let Expression::Identifier(ident) = node else {
            unreachable!()
        };

        if let Some(rune) = self.get_rune_by_reference(ident) {
            if !rune.mutated {
                return;
            }

            let call = self
                .builder
                .call("$.get", [BuilderFunctionArgument::Ident(&ident.name)]);

            *node = Expression::CallExpression(self.builder.alloc(call))
        }
    }

    fn transform_rune_update(&mut self, node: &mut Expression<'a>, _ctx: &mut TraverseCtx<'a>) {
        let Expression::UpdateExpression(update) = node else {
            unreachable!();
        };

        let ident =
            if let SimpleAssignmentTarget::AssignmentTargetIdentifier(ident) = &update.argument {
                if self.is_rune_reference(ident) {
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

            let call = self.builder.call(callee, args);

            *node = Expression::CallExpression(self.builder.alloc(call))
        }
    }

    fn transform_rune_assignment(&mut self, node: &mut Expression<'a>, _ctx: &mut TraverseCtx<'a>) {
        let Expression::AssignmentExpression(assign) = node else {
            unreachable!();
        };

        let ident = if let AssignmentTarget::AssignmentTargetIdentifier(ident) = &assign.left {
            if self.is_rune_reference(ident) {
                Some(ident.name.as_str())
            } else {
                None
            }
        } else {
            None
        };

        if let Some(name) = ident {
            let mut right = self.builder.ast.move_expression(&mut assign.right);

            if !right.is_literal() && self.proxy_rune {
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

    fn is_rune_reference(&self, ident: &ast::IdentifierReference<'a>) -> bool {
        self.get_rune_by_reference(ident).is_some()
    }

    fn get_rune_by_reference(&self, ident: &ast::IdentifierReference<'a>) -> Option<&Rune> {
        let reference_id = ident.reference_id.get();

        reference_id?;

        let reference_id = reference_id.unwrap();

        self.svelte_table.get_rune_by_reference(reference_id)
    }
}

impl<'a> Traverse<'a> for TransformerImpl<'_, 'a> {
    fn enter_variable_declarator(
        &mut self,
        node: &mut ast::VariableDeclarator<'a>,
        _ctx: &mut TraverseCtx<'a>,
    ) {
        if let BindingPatternKind::BindingIdentifier(id) = &node.id.kind {
            if let Some(rune) = self.svelte_table.get_rune_by_symbol_id(id.symbol_id()) {
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

    fn enter_program(&mut self, node: &mut Program<'a>, _ctx: &mut TraverseCtx<'a>) {
        node.body.retain_mut(|stmt| {
            if matches!(stmt, Statement::ImportDeclaration(_)) {
                self.imports.push(self.builder.ast.move_statement(stmt));
                false
            } else {
                true
            }
        });
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
            _ => (),
        }
    }
}
