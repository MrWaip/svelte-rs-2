use oxc_allocator::Allocator;
use oxc_ast::ast::{self, CallExpression, Program, Statement};
use oxc_semantic::{ScopeTree, SymbolTable};
use oxc_traverse::{traverse_mut, Traverse, TraverseCtx};

use crate::ast::ScriptTag;

use super::{builder::Builder, scope};

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
    ) -> TransformResult<'a> {
        let mut transformer = TransformerImpl {};

        let (symbols, scopes) = traverse_mut(
            &mut transformer,
            &self.b.ast.allocator,
            &mut script.program,
            symbols,
            scopes,
        );

        for i in scopes.iter_bindings_in(scopes.root_scope_id()) {
            let refs = symbols.get_resolved_references(i);

            for reff in refs {
                dbg!(reff);
            }
        }
        // scopes.root_scope_id()
        // symbols.get_declaration(symbol_id)

        // symbols.symbol_ids()[]

        return TransformResult {
            body: vec![],
            program: script.program,
            hoisted: vec![],
            symbols,
            scopes,
        };
    }
}

struct TransformerImpl {}

impl<'a> Traverse<'a> for TransformerImpl {
    fn enter_call_expression(&mut self, node: &mut CallExpression<'a>, ctx: &mut TraverseCtx<'a>) {

        // ctx.scopes()

        // 1) oxc уже собрал все скоупы, собрал символы и референсы
        // 2) следующим этапом мы пробегаемся по аст еще раз, и собираем пул SymbolId которые относятся к рунам
        // 3) на этапе трансформирования мы проверяем, обращается ли этот идентификатор к руне, если да конвертим это во что-нибудь

        // dbg!(node);
    }
}
