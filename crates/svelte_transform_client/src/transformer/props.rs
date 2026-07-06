use oxc_ast::ast::{BindingPattern, Expression, VariableDeclaration};

use super::model::ComponentTransformer;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(crate) fn is_props_id_declaration(decl: &VariableDeclaration<'a>) -> bool {
        decl.declarations.iter().any(|d| {
            if let BindingPattern::BindingIdentifier(_) = &d.id
                && let Some(Expression::CallExpression(call)) = &d.init
                && let Expression::StaticMemberExpression(member) = &call.callee
                && let Expression::Identifier(obj) = &member.object
            {
                return obj.name.as_str() == "$props" && member.property.name.as_str() == "id";
            }
            false
        })
    }
}
