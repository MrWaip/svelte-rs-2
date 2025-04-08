use std::path::Path;

use analyze_hir::{HirAnalyses, SvelteRune};
use ast_builder::{Builder, BuilderStatement};
use hir::{HirStore, OwnerId};
use oxc_ast::ast::{Expression, IdentifierReference, Language, Statement};
use oxc_semantic::{ScopeTree, SymbolTable};
use oxc_transformer::{TransformOptions, Transformer as OxcTransformer};
use oxc_traverse::{Traverse, traverse_mut};

pub struct ScriptTransformer<'hir> {
    pub(crate) analyses: &'hir HirAnalyses,
    pub(crate) b: &'hir Builder<'hir>,
    pub(crate) store: &'hir HirStore<'hir>,
    pub(crate) imports: Vec<Statement<'hir>>,
    pub(crate) _owner_id: OwnerId,
}

impl<'hir> ScriptTransformer<'hir> {
    pub fn new(
        analyses: &'hir HirAnalyses,
        builder: &'hir Builder<'hir>,
        store: &'hir HirStore<'hir>,
        owner_id: OwnerId,
    ) -> Self {
        Self {
            analyses,
            b: builder,
            store,
            imports: vec![],
            _owner_id: owner_id,
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
                self.b.ast.allocator,
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

        res
    }

    pub(crate) fn transform_expression(
        &mut self,
        expression: Expression<'hir>,
    ) -> Expression<'hir> {
        let mut program = self
            .b
            .program(vec![self.b.stmt(BuilderStatement::Expr(expression))]);

        program.set_scope_id(self.analyses.root_scope_id());

        traverse_mut(
            self,
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

        expression
    }

    pub(crate) fn get_rune_by_reference(
        &self,
        ident: &IdentifierReference<'hir>,
    ) -> Option<&SvelteRune> {
        let reference_id = ident.reference_id.get();

        reference_id?;

        let reference_id = reference_id.unwrap();

        self.analyses.get_rune_by_reference(reference_id)
    }

    pub(crate) fn should_proxy_rune_init(&self, e: &Expression) -> bool {
        if e.is_literal() {
            return false;
        }

        if matches!(
            e,
            Expression::TemplateLiteral(_)
                | Expression::ArrowFunctionExpression(_)
                | Expression::FunctionExpression(_)
                | Expression::UnaryExpression(_)
                | Expression::BinaryExpression(_)
        ) {
            return false;
        }

        if let Expression::Identifier(id) = e {
            if id.name == "undefined" {
                return false;
            }

            // todo!();
        }

        // if (node.type === 'Identifier' && scope !== null) {
        // 	const binding = scope.get(node.name);
        // 	// Let's see if the reference is something that can be proxied
        // 	if (
        // 		binding !== null &&
        // 		!binding.reassigned &&
        // 		binding.initial !== null &&
        // 		binding.initial.type !== 'FunctionDeclaration' &&
        // 		binding.initial.type !== 'ClassDeclaration' &&
        // 		binding.initial.type !== 'ImportDeclaration' &&
        // 		binding.initial.type !== 'EachBlock' &&
        // 		binding.initial.type !== 'SnippetBlock'
        // 	) {
        // 		return should_proxy(binding.initial, null);
        // 	}
        // }

        true
    }
}

impl<'hir> Traverse<'hir> for ScriptTransformer<'hir> {
    fn enter_variable_declarator(
        &mut self,
        node: &mut oxc_ast::ast::VariableDeclarator<'hir>,
        _ctx: &mut oxc_traverse::TraverseCtx<'hir>,
    ) {
        self.transform_rune_declaration(node);
    }

    fn enter_expression(
        &mut self,
        node: &mut Expression<'hir>,
        ctx: &mut oxc_traverse::TraverseCtx<'hir>,
    ) {
        match node {
            Expression::Identifier(_) => {
                self.transform_rune_reference(node);
            }
            Expression::AssignmentExpression(_) => {
                self.transform_rune_assignment(node, ctx);
            }
            _ => (),
        }
    }
}
