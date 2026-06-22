use oxc_ast::ast::{
    Argument, BindingPattern, CallExpression, Expression, ObjectProperty, PropertyKey,
    PropertyKind, VariableDeclarator,
};
use oxc_span::GetSpan;

use super::model::ComponentTransformer;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(crate) fn capture_call_label_name(&mut self, node: &CallExpression<'a>) {
        let has_fn_arg = node.arguments.iter().any(|arg| {
            matches!(
                arg,
                Argument::ArrowFunctionExpression(_) | Argument::FunctionExpression(_)
            )
        });
        if has_fn_arg {
            let start = (node.callee.span().start) as usize;
            let end = (node.callee.span().end) as usize;
            if end <= self.component_source.len() {
                let callee_text = &self.component_source[start..end];
                self.next_arrow_name = Some(format!("{callee_text}(...)"));
            }
        }
    }

    pub(crate) fn normalize_object_property_method_shorthand(&self, node: &mut ObjectProperty<'a>) {
        if node.method || node.kind != PropertyKind::Init {
            return;
        }
        if matches!(&node.value, Expression::FunctionExpression(_)) {
            node.method = true;
        }
    }

    pub(crate) fn capture_object_property_label_name(&mut self, node: &ObjectProperty<'a>) {
        if !node.computed {
            let is_fn_value = matches!(
                &node.value,
                Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_)
            );
            if (is_fn_value || node.method)
                && let PropertyKey::StaticIdentifier(id) = &node.key
            {
                self.next_arrow_name = Some(id.name.to_string());
            }
        }
    }

    pub(crate) fn capture_variable_arrow_name(&mut self, node: &VariableDeclarator<'a>) {
        if let Some(Expression::ArrowFunctionExpression(_)) = &node.init
            && let BindingPattern::BindingIdentifier(id) = &node.id
        {
            self.next_arrow_name = Some(id.name.to_string());
        }
    }
}
