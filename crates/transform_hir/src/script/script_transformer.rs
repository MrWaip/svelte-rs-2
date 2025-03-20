use std::path::Path;

use analyze_hir::HirAnalyses;
use ast_builder::Builder;
use hir::HirStore;
use oxc_ast::ast::{Language, Statement};
use oxc_semantic::{ScopeTree, SymbolTable};
use oxc_transformer::{TransformOptions, Transformer as OxcTransformer};
use oxc_traverse::{Traverse, traverse_mut};

pub struct ScriptTransformer<'hir> {
    pub(crate) analyses: &'hir HirAnalyses,
    pub(crate) b: &'hir Builder<'hir>,
    pub(crate) store: &'hir HirStore<'hir>,
    pub(crate) imports: Vec<Statement<'hir>>,
}

impl<'hir> ScriptTransformer<'hir> {
    pub fn new(
        analyses: &'hir HirAnalyses,
        builder: &'hir Builder<'hir>,
        store: &'hir HirStore<'hir>,
    ) -> Self {
        Self {
            analyses,
            b: builder,
            store,
            imports: vec![],
        }
    }

    pub fn transform(&mut self) -> Vec<Statement<'hir>> {
        let mut res = vec![];
        {
            let program = &self.store.program;
            let mut oxc_program = program.program.borrow_mut();

            if program.language == Language::TypeScript {
                let mut opts = TransformOptions::default();

                opts.typescript.only_remove_type_imports = true;

                let ts_transformer =
                    OxcTransformer::new(self.b.ast.allocator, Path::new("some.ts"), &opts);

                let (symbols, scopes) = self.analyses.take_scoping();
                let ret =
                    ts_transformer.build_with_symbols_and_scopes(symbols, scopes, &mut oxc_program);

                self.analyses.set_scoping(ret.symbols, ret.scopes);
            }

            traverse_mut(
                self,
                &self.b.ast.allocator,
                &mut oxc_program,
                SymbolTable::default(),
                ScopeTree::default(),
            );
        }

        let program = self.store.program.program.replace(self.b.program(vec![]));

        for stmt in program.body {
            if matches!(&stmt, Statement::ImportDeclaration(_)) {
                self.imports.push(stmt);
            } else {
                res.push(stmt);
            }
        }

        return res;
    }
}

impl<'hir> Traverse<'hir> for ScriptTransformer<'hir> {}
