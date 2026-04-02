use oxc_ast::ast::{ArrowFunctionExpression, Expression, Statement, VariableDeclarator};
use oxc_span::GetSpan;

use super::super::ScriptTransformer;

impl<'a> ScriptTransformer<'_, 'a> {
    pub(super) fn strip_ts_class_members(&self, node: &mut oxc_ast::ast::ClassBody<'a>) {
        if self.is_ts {
            node.body.retain(|member| match member {
                oxc_ast::ast::ClassElement::PropertyDefinition(prop) => {
                    !prop.declare
                        && prop.r#type
                            != oxc_ast::ast::PropertyDefinitionType::TSAbstractPropertyDefinition
                }
                oxc_ast::ast::ClassElement::MethodDefinition(method) => {
                    method.r#type != oxc_ast::ast::MethodDefinitionType::TSAbstractMethodDefinition
                }
                oxc_ast::ast::ClassElement::TSIndexSignature(_) => false,
                _ => true,
            });
        }
    }

    pub(super) fn strip_ts_function_bits(&self, node: &mut oxc_ast::ast::Function<'a>) {
        if self.is_ts {
            node.type_parameters = None;
            node.return_type = None;
            node.this_param = None;
        }
    }

    pub(super) fn strip_ts_arrow_bits(&self, node: &mut ArrowFunctionExpression<'a>) {
        if self.is_ts {
            node.type_parameters = None;
            node.return_type = None;
        }
    }

    pub(super) fn strip_ts_formal_parameter(&self, node: &mut oxc_ast::ast::FormalParameter<'a>) {
        if self.is_ts {
            node.type_annotation = None;
            node.accessibility = None;
            node.readonly = false;
            node.r#override = false;
        }
    }

    pub(super) fn strip_ts_catch_parameter(&self, node: &mut oxc_ast::ast::CatchParameter<'a>) {
        if self.is_ts {
            node.type_annotation = None;
        }
    }

    pub(super) fn strip_ts_call_bits(&self, node: &mut oxc_ast::ast::CallExpression<'a>) {
        if self.is_ts {
            node.type_arguments = None;
        }
    }

    pub(super) fn capture_call_label_name(&mut self, node: &oxc_ast::ast::CallExpression<'a>) {
        let has_fn_arg = node.arguments.iter().any(|arg| {
            matches!(
                arg,
                oxc_ast::ast::Argument::ArrowFunctionExpression(_)
                    | oxc_ast::ast::Argument::FunctionExpression(_)
            )
        });
        if has_fn_arg {
            let start = (self.script_content_start + node.callee.span().start) as usize;
            let end = (self.script_content_start + node.callee.span().end) as usize;
            if end <= self.component_source.len() {
                let callee_text = &self.component_source[start..end];
                self.next_arrow_name = Some(format!("{callee_text}(...)"));
            }
        }
    }

    pub(super) fn strip_ts_new_bits(&self, node: &mut oxc_ast::ast::NewExpression<'a>) {
        if self.is_ts {
            node.type_arguments = None;
        }
    }

    pub(super) fn strip_ts_tagged_template_bits(
        &self,
        node: &mut oxc_ast::ast::TaggedTemplateExpression<'a>,
    ) {
        if self.is_ts {
            node.type_arguments = None;
        }
    }

    pub(super) fn strip_ts_class_bits(&self, node: &mut oxc_ast::ast::Class<'a>) {
        if self.is_ts {
            node.type_parameters = None;
            node.super_type_arguments = None;
            node.implements.clear();
            node.r#abstract = false;
        }
    }

    pub(super) fn strip_ts_property_definition_bits(
        &self,
        node: &mut oxc_ast::ast::PropertyDefinition<'a>,
    ) {
        if self.is_ts {
            node.type_annotation = None;
            node.accessibility = None;
            node.readonly = false;
            node.r#override = false;
            node.optional = false;
            node.definite = false;
        }
    }

    pub(super) fn strip_ts_accessor_property_bits(
        &self,
        node: &mut oxc_ast::ast::AccessorProperty<'a>,
    ) {
        if self.is_ts {
            node.type_annotation = None;
            node.accessibility = None;
            node.r#override = false;
            node.definite = false;
        }
    }

    pub(super) fn capture_object_property_label_name(
        &mut self,
        node: &oxc_ast::ast::ObjectProperty<'a>,
    ) {
        if !node.computed {
            let is_fn_value = matches!(
                &node.value,
                Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_)
            );
            if is_fn_value || node.method {
                if let oxc_ast::ast::PropertyKey::StaticIdentifier(id) = &node.key {
                    self.next_arrow_name = Some(id.name.to_string());
                }
            }
        }
    }

    pub(super) fn strip_ts_method_definition_bits(
        &self,
        node: &mut oxc_ast::ast::MethodDefinition<'a>,
    ) {
        if self.is_ts {
            node.accessibility = None;
            node.r#override = false;
            node.optional = false;
        }
    }

    pub(super) fn strip_ts_variable_declarator_bits(&self, node: &mut VariableDeclarator<'a>) {
        if self.is_ts {
            node.type_annotation = None;
            node.definite = false;
        }
    }

    pub(super) fn capture_variable_arrow_name(&mut self, node: &VariableDeclarator<'a>) {
        if let Some(Expression::ArrowFunctionExpression(_)) = &node.init {
            if let oxc_ast::ast::BindingPattern::BindingIdentifier(id) = &node.id {
                self.next_arrow_name = Some(id.name.to_string());
            }
        }
    }

    pub(super) fn strip_ts_expression_wrappers(&self, node: &mut Expression<'a>) {
        if self.is_ts {
            loop {
                match node {
                    Expression::TSAsExpression(_)
                    | Expression::TSSatisfiesExpression(_)
                    | Expression::TSNonNullExpression(_)
                    | Expression::TSTypeAssertion(_)
                    | Expression::TSInstantiationExpression(_) => {
                        let inner = match self.b.move_expr(node) {
                            Expression::TSAsExpression(ts) => ts.unbox().expression,
                            Expression::TSSatisfiesExpression(ts) => ts.unbox().expression,
                            Expression::TSNonNullExpression(ts) => ts.unbox().expression,
                            Expression::TSTypeAssertion(ts) => ts.unbox().expression,
                            Expression::TSInstantiationExpression(ts) => ts.unbox().expression,
                            _ => unreachable!(),
                        };
                        *node = inner;
                    }
                    _ => break,
                }
            }
        }
    }

    pub(super) fn strip_ts_specifiers_and_statements(
        &self,
        stmts: &mut oxc_allocator::Vec<'a, Statement<'a>>,
    ) {
        if !self.is_ts {
            return;
        }

        for stmt in stmts.iter_mut() {
            match stmt {
                Statement::ImportDeclaration(import) => {
                    if let Some(specs) = &mut import.specifiers {
                        specs.retain(|spec| {
                            !matches!(spec, oxc_ast::ast::ImportDeclarationSpecifier::ImportSpecifier(s) if s.import_kind.is_type())
                        });
                    }
                }
                Statement::ExportNamedDeclaration(export) if export.declaration.is_none() => {
                    export.specifiers.retain(|spec| !spec.export_kind.is_type());
                }
                _ => {}
            }
        }

        stmts.retain(|stmt| match stmt {
            Statement::TSTypeAliasDeclaration(_)
            | Statement::TSInterfaceDeclaration(_)
            | Statement::TSModuleDeclaration(_)
            | Statement::TSEnumDeclaration(_) => false,
            Statement::VariableDeclaration(decl) if decl.declare => false,
            Statement::FunctionDeclaration(func) if func.declare => false,
            Statement::ClassDeclaration(class) if class.declare => false,
            Statement::ImportDeclaration(import) if import.import_kind.is_type() => false,
            Statement::ExportNamedDeclaration(export) if export.export_kind.is_type() => false,
            Statement::ExportAllDeclaration(export) if export.export_kind.is_type() => false,
            Statement::ImportDeclaration(import) => {
                import.specifiers.as_ref().is_none_or(|s| !s.is_empty())
            }
            Statement::ExportNamedDeclaration(export) => {
                export.declaration.is_some() || !export.specifiers.is_empty()
            }
            _ => true,
        });
    }
}
