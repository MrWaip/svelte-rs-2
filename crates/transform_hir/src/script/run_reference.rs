use analyze_hir::SvelteRune;
use ast_builder::{BuilderExpression, BuilderFunctionArgument};
use oxc_ast::ast::{BindingPatternKind, Expression, IdentifierReference};

use super::script_transformer::ScriptTransformer;

impl<'hir> ScriptTransformer<'hir> {
    pub(crate) fn transform_rune_reference(&mut self, node: &mut Expression<'hir>) {
        let Expression::Identifier(ident) = node else {
            unreachable!()
        };

        let Some(rune) = self.get_rune_by_reference(ident) else {
            return;
        };

        if !rune.mutated {
            return;
        }

        let call = self
            .b
            .call_expr("$.get", [BuilderFunctionArgument::Ident(&ident.name)]);

        *node = call;
    }
}
