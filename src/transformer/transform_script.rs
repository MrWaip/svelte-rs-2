use std::collections::HashMap;

use oxc_ast::ast::{self, BindingPatternKind, Expression, Program, Statement};
use oxc_semantic::{ScopeTree, SymbolId, SymbolTable};
use oxc_traverse::{traverse_mut, Traverse, TraverseCtx};

use crate::{analyze::Rune, ast::ScriptTag};

use super::builder::Builder;

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
                if rune.mutated {
                    todo!()
                } else {
                    if let Some(expr) = node.init.as_mut() {
                        let expr = self.builder.ast.move_expression(expr);

                        if let Expression::CallExpression(mut call) = expr {
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
}
