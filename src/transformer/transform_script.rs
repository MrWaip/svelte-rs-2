use std::collections::HashMap;

use oxc_ast::ast::{self, Argument, BindingPatternKind, Expression, Program, Statement};
use oxc_semantic::{ScopeTree, SymbolId, SymbolTable};
use oxc_traverse::{traverse_mut, Traverse, TraverseCtx};

use crate::{analyze::Rune, ast::ScriptTag};

use super::builder::{Builder, BuilderExpression, BuilderFunctionArgument};

pub struct TransformScript<'a> {
    b: &'a Builder<'a>,
    hoisted: Vec<Statement<'a>>,
}

#[derive(Debug)]
pub struct TransformResult<'a> {
    pub body: Vec<Statement<'a>>,
    pub hoisted: Vec<Statement<'a>>,
    pub symbols: SymbolTable,
    pub scopes: ScopeTree,
    pub program: Program<'a>,
}

impl<'a> TransformScript<'a> {
    pub fn new(builder: &'a Builder<'a>) -> Self {
        return Self {
            b: builder,
            hoisted: vec![],
        };
    }

    pub fn transform(
        self,
        mut script: ScriptTag<'a>,
        symbols: SymbolTable,
        scopes: ScopeTree,
        runes: &HashMap<SymbolId, Rune>,
    ) -> TransformResult<'a> {
        let mut transformer = TransformerImpl {
            runes,
            builder: self.b,
        };

        let (symbols, scopes) = traverse_mut(
            &mut transformer,
            &self.b.ast.allocator,
            &mut script.program,
            symbols,
            scopes,
        );

        return TransformResult {
            body: vec![],
            program: script.program,
            hoisted: vec![],
            symbols,
            scopes,
        };
    }
}

struct TransformerImpl<'link, 'a> {
    runes: &'link HashMap<SymbolId, Rune>,
    builder: &'link Builder<'a>,
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
        if let Expression::Identifier(ident) = node {
            let reference_id = ident.reference_id.get();

            if reference_id.is_none() {
                return;
            }

            let reference_id = reference_id.unwrap();
            let reference = ctx.symbols().get_reference(reference_id);
            let symbol_id = reference.symbol_id();

            if symbol_id.is_none() {
                return;
            }

            if let Some(rune) = self.runes.get(&symbol_id.unwrap()) {
                if !rune.mutated {
                    return;
                }

                let call = self
                    .builder
                    .call("$.get", [BuilderFunctionArgument::Ident(&ident.name)]);

                *node = Expression::CallExpression(self.builder.alloc(call))
            }
        }
    }
}
