use oxc_ast::Visit;
use oxc_index::{index_vec, IndexVec};
use oxc_semantic::{ScopeTree, SemanticBuilder, SymbolId, SymbolTable};

use crate::ast::Ast;

pub struct Analyzer {}

pub struct AnalyzeResult {
    pub script: Option<ScriptResult>,
    pub runes: IndexVec<SymbolId, Rune>,
}

pub struct ScriptResult {
    pub symbols: SymbolTable,
    pub scopes: ScopeTree,
}

pub struct Rune {}

impl Analyzer {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn analyze<'a, 'link>(&self, ast: &'link Ast<'a>) -> AnalyzeResult {
        let mut result = AnalyzeResult {
            runes: index_vec!(),
            script: None,
        };

        if let Some(script) = &ast.script {
            let ret = SemanticBuilder::new().build(&script.program);

            if !ret.errors.is_empty() {
                todo!();
            }

            let (symbols, scopes) = ret.semantic.into_symbol_table_and_scope_tree();

            result.script = Some(ScriptResult { scopes, symbols })
        }

        return result;
    }
}

pub struct Visitor {}

impl<'a> Visit<'a> for Visitor {}
