use std::{collections::HashMap, mem::replace};

use oxc_ast::{
    ast::{BindingPatternKind, Expression, VariableDeclarator},
    Visit,
};
use oxc_semantic::{ScopeTree, SemanticBuilder, SymbolId, SymbolTable};

use crate::ast::Ast;

pub struct Analyzer {}

pub struct AnalyzeResult {
    pub runes: HashMap<SymbolId, Rune>,
    pub symbols: SymbolTable,
    pub scopes: ScopeTree,
}

#[derive(Debug)]
pub struct Rune {
    pub mutated: bool,
    pub kind: RuneKind,
}

#[derive(Debug)]
pub enum RuneKind {
    State,
}

impl Analyzer {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn analyze<'a, 'link>(&self, ast: &'link Ast<'a>) -> AnalyzeResult {
        let (runes, symbols, scopes) = if let Some(script) = &ast.script {
            let ret = SemanticBuilder::new().build(&script.program);

            if !ret.errors.is_empty() {
                todo!();
            }

            let (symbols, scopes) = ret.semantic.into_symbol_table_and_scope_tree();

            let mut visitor = Visitor {
                runes: HashMap::default(),
                scopes: &scopes,
                symbols: &symbols,
            };

            visitor.visit_program(&script.program);

            (
                replace(&mut visitor.runes, HashMap::default()),
                symbols,
                scopes,
            )
        } else {
            (
                HashMap::default(),
                SymbolTable::default(),
                ScopeTree::default(),
            )
        };

        return AnalyzeResult {
            runes,
            scopes,
            symbols,
        };
    }
}

pub struct Visitor<'link> {
    pub runes: HashMap<SymbolId, Rune>,
    pub symbols: &'link SymbolTable,
    pub scopes: &'link ScopeTree,
}

impl<'a, 'link> Visit<'a> for Visitor<'link> {
    fn visit_variable_declarator(&mut self, declarator: &VariableDeclarator<'a>) {
        if let Some(Expression::CallExpression(call)) = &declarator.init {
            if call.callee_name() == Some("$state") {
                if let BindingPatternKind::BindingIdentifier(id) = &declarator.id.kind {
                    let symbol_id = id.symbol_id();

                    self.runes.insert(
                        symbol_id,
                        Rune {
                            kind: RuneKind::State,
                            mutated: self.symbols.symbol_is_mutated(symbol_id.clone()),
                        },
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use oxc_allocator::Allocator;

    use crate::parser::Parser;

    use super::*;

    #[test]
    fn analyze_smoke() {
        let allocator = Allocator::default();
        let analyzer = Analyzer::new();
        let mut parser = Parser::new(
            "<script>let rune_var = $state(10); onMount(() => rune_var = 0);</script>",
            &allocator,
        );
        let ast = parser.parse().unwrap();
        let result = analyzer.analyze(&ast);

        assert!(!result.runes.is_empty());

        for (id, _rune) in result.runes.iter() {
            assert_eq!(result.symbols.get_name(id.clone()), "rune_var");
            dbg!(_rune);
        }
    }
}
