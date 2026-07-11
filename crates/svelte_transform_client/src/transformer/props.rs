use oxc_ast::ast::{BindingPattern, VariableDeclaration};
use svelte_analyze::RuntimeRuneKind;

use super::model::ComponentTransformer;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(crate) fn is_props_id_declaration(&self, decl: &VariableDeclaration<'a>) -> bool {
        let Some(analysis) = self.analysis.as_ref() else {
            return false;
        };
        decl.declarations.iter().any(|d| {
            let BindingPattern::BindingIdentifier(ident) = &d.id else {
                return false;
            };
            let Some(sym) = ident.symbol_id.get() else {
                return false;
            };
            analysis.binding_semantics(sym).runtime_rune() == Some(RuntimeRuneKind::PropsId)
        })
    }
}
