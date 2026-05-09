use oxc_allocator::Vec as OxcVec;
use oxc_ast::ast::{
    AccessorProperty, Argument, ArrowFunctionExpression, AssignmentTarget, BindingPattern,
    CallExpression, CatchParameter, ChainElement, Class, ClassBody, ClassElement,
    Expression, FormalParameter, FormalParameterRest, Function, ImportDeclarationSpecifier,
    MethodDefinition, MethodDefinitionType, NewExpression, ObjectProperty, PropertyDefinition,
    PropertyDefinitionType, PropertyKey, PropertyKind, SimpleAssignmentTarget, Statement,
    TaggedTemplateExpression, VariableDeclarator, match_member_expression,
};
use oxc_span::GetSpan;

use super::model::ComponentTransformer;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(crate) fn strip_ts_class_members(&self, node: &mut ClassBody<'a>) {
        node.body.retain(|member| match member {
            ClassElement::PropertyDefinition(prop) => {
                !prop.declare
                    && prop.r#type != PropertyDefinitionType::TSAbstractPropertyDefinition
            }
            ClassElement::MethodDefinition(method) => {
                method.r#type != MethodDefinitionType::TSAbstractMethodDefinition
            }
            ClassElement::TSIndexSignature(_) => false,
            _ => true,
        });
    }

    pub(crate) fn strip_ts_function_bits(&self, node: &mut Function<'a>) {
        node.type_parameters = None;
        node.return_type = None;
        node.this_param = None;
    }

    pub(crate) fn strip_ts_arrow_bits(&self, node: &mut ArrowFunctionExpression<'a>) {
        node.type_parameters = None;
        node.return_type = None;
    }

    pub(crate) fn strip_ts_formal_parameter(&self, node: &mut FormalParameter<'a>) {
        node.type_annotation = None;
        node.accessibility = None;
        node.readonly = false;
        node.r#override = false;
        node.optional = false;
    }

    pub(crate) fn strip_ts_catch_parameter(&self, node: &mut CatchParameter<'a>) {
        node.type_annotation = None;
    }

    pub(crate) fn strip_ts_formal_parameter_rest(&self, node: &mut FormalParameterRest<'a>) {
        node.type_annotation = None;
    }

    pub(crate) fn strip_ts_call_bits(&self, node: &mut CallExpression<'a>) {
        node.type_arguments = None;
    }

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

    pub(crate) fn strip_ts_new_bits(&self, node: &mut NewExpression<'a>) {
        node.type_arguments = None;
    }

    pub(crate) fn strip_ts_tagged_template_bits(&self, node: &mut TaggedTemplateExpression<'a>) {
        node.type_arguments = None;
    }

    pub(crate) fn strip_ts_class_bits(&self, node: &mut Class<'a>) {
        node.type_parameters = None;
        node.super_type_arguments = None;
        node.implements.clear();
        node.r#abstract = false;
    }

    pub(crate) fn strip_ts_property_definition_bits(&self, node: &mut PropertyDefinition<'a>) {
        node.type_annotation = None;
        node.accessibility = None;
        node.readonly = false;
        node.r#override = false;
        node.optional = false;
        node.definite = false;
    }

    pub(crate) fn strip_ts_accessor_property_bits(&self, node: &mut AccessorProperty<'a>) {
        node.type_annotation = None;
        node.accessibility = None;
        node.r#override = false;
        node.definite = false;
    }

    pub(crate) fn normalize_object_property_method_shorthand(
        &self,
        node: &mut ObjectProperty<'a>,
    ) {
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

    pub(crate) fn strip_ts_method_definition_bits(&self, node: &mut MethodDefinition<'a>) {
        node.accessibility = None;
        node.r#override = false;
        node.optional = false;
    }

    pub(crate) fn strip_ts_variable_declarator_bits(&self, node: &mut VariableDeclarator<'a>) {
        node.type_annotation = None;
        node.definite = false;
    }

    pub(crate) fn capture_variable_arrow_name(&mut self, node: &VariableDeclarator<'a>) {
        if let Some(Expression::ArrowFunctionExpression(_)) = &node.init
            && let BindingPattern::BindingIdentifier(id) = &node.id
        {
            self.next_arrow_name = Some(id.name.to_string());
        }
    }

    pub(crate) fn replace_ts_only_body_with_empty(&self, stmt: &mut Statement<'a>) {
        if stmt.is_typescript_syntax() {
            let span = stmt.span();
            *stmt = self.b.ast.statement_empty(span);
        }
    }

    pub(crate) fn strip_ts_only_alternate(&self, alternate: &mut Option<Statement<'a>>) {
        if let Some(stmt) = alternate.as_mut() {
            self.replace_ts_only_body_with_empty(stmt);
        }
    }

    pub(crate) fn strip_ts_simple_assignment_target(
        &self,
        node: &mut SimpleAssignmentTarget<'a>,
    ) {
        let Some(expr) = node.get_expression_mut() else {
            return;
        };
        let inner = self.b.move_expr(expr.get_inner_expression_mut());
        match inner {
            Expression::Identifier(id) => {
                *node = SimpleAssignmentTarget::AssignmentTargetIdentifier(id);
            }
            expr @ match_member_expression!(Expression) => {
                *node = SimpleAssignmentTarget::from(expr.into_member_expression());
            }
            _ => {}
        }
    }

    pub(crate) fn strip_ts_assignment_target(&self, node: &mut AssignmentTarget<'a>) {
        let Some(expr) = node.get_expression_mut() else {
            return;
        };
        let inner = self.b.move_expr(expr.get_inner_expression_mut());
        match inner {
            Expression::Identifier(id) => {
                *node = AssignmentTarget::AssignmentTargetIdentifier(id);
            }
            expr @ match_member_expression!(Expression) => {
                *node = AssignmentTarget::from(expr.into_member_expression());
            }
            _ => {}
        }
    }

    pub(crate) fn strip_ts_expression_wrappers(&self, node: &mut Expression<'a>) {
        if node.is_typescript_syntax() {
            let inner = self.b.move_expr(node.get_inner_expression_mut());
            *node = inner;
        }
    }

    pub(crate) fn strip_ts_chain_element_wrappers(&self, node: &mut ChainElement<'a>) {
        if let ChainElement::TSNonNullExpression(ts) = node {
            let inner = self.b.move_expr(ts.expression.get_inner_expression_mut());
            *node = match inner {
                Expression::CallExpression(c) => ChainElement::CallExpression(c),
                expr @ match_member_expression!(Expression) => {
                    ChainElement::from(expr.into_member_expression())
                }
                _ => unreachable!("TSNonNullExpression inside ChainExpression must wrap member/call"),
            };
        }
    }

    pub(crate) fn strip_ts_specifiers_and_statements(
        &self,
        stmts: &mut OxcVec<'a, Statement<'a>>,
    ) {
        for stmt in stmts.iter_mut() {
            match stmt {
                Statement::ImportDeclaration(import) => {
                    if let Some(specs) = &mut import.specifiers {
                        specs.retain(|spec| {
                            !matches!(spec, ImportDeclarationSpecifier::ImportSpecifier(s) if s.import_kind.is_type())
                        });
                    }
                }
                Statement::ExportNamedDeclaration(export) if export.declaration.is_none() => {
                    export.specifiers.retain(|spec| !spec.export_kind.is_type());
                }
                _ => {}
            }
        }

        stmts.retain(|stmt| {
            if stmt.is_typescript_syntax() {
                return false;
            }
            match stmt {
                Statement::ImportDeclaration(import) => {
                    if import.import_kind.is_type() {
                        return false;
                    }
                    import.specifiers.as_ref().is_none_or(|s| !s.is_empty())
                }
                Statement::ExportNamedDeclaration(export) => {
                    export.declaration.is_some() || !export.specifiers.is_empty()
                }
                _ => true,
            }
        });
    }
}
