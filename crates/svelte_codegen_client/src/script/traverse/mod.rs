mod assignments;
mod derived;
mod inspect;
mod runes;
mod statement_passes;
mod ts_cleanup;

use oxc_ast::ast::{ArrowFunctionExpression, Expression, FunctionBody, Statement, VariableDeclarator};
use oxc_traverse::{Traverse, TraverseCtx};

use super::{FunctionInfo, ScriptTransformer};

pub(super) use derived::{wrap_derived_thunks, wrap_lazy, DevContext};

impl<'a> Traverse<'a, ()> for ScriptTransformer<'_, 'a> {
    fn enter_class_body(
        &mut self,
        node: &mut oxc_ast::ast::ClassBody<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        let info = self.scan_class_state_fields(node);
        self.class_state_stack.push(info);
    }

    fn exit_class_body(
        &mut self,
        node: &mut oxc_ast::ast::ClassBody<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_class_members(node);

        let Some(info) = self.class_state_stack.pop() else {
            return;
        };
        if info.fields.is_empty() {
            return;
        }
        self.rewrite_class_body(node, &info);
    }

    fn enter_function(
        &mut self,
        node: &mut oxc_ast::ast::Function<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_function_bits(node);
        let name = node
            .id
            .as_ref()
            .map(|id| id.name.to_string())
            .or_else(|| self.next_arrow_name.take());
        self.function_info_stack.push(FunctionInfo {
            is_async: node.r#async,
            name,
            span_start: node.span.start,
        });
    }

    fn exit_function(
        &mut self,
        _node: &mut oxc_ast::ast::Function<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.function_info_stack.pop();
    }

    fn enter_arrow_function_expression(
        &mut self,
        node: &mut ArrowFunctionExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_arrow_bits(node);
        let name = self.next_arrow_name.take();
        self.function_info_stack.push(FunctionInfo {
            is_async: node.r#async,
            name,
            span_start: node.span.start,
        });
    }

    fn exit_arrow_function_expression(
        &mut self,
        _node: &mut ArrowFunctionExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.function_info_stack.pop();
    }

    fn exit_function_body(
        &mut self,
        body: &mut FunctionBody<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.rewrite_trace_function_body(body);
    }

    fn exit_statements(
        &mut self,
        stmts: &mut oxc_allocator::Vec<'a, Statement<'a>>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.process_statement_block(stmts);
    }

    fn enter_formal_parameter(
        &mut self,
        node: &mut oxc_ast::ast::FormalParameter<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_formal_parameter(node);
    }

    fn enter_catch_parameter(
        &mut self,
        node: &mut oxc_ast::ast::CatchParameter<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_catch_parameter(node);
    }

    fn enter_call_expression(
        &mut self,
        node: &mut oxc_ast::ast::CallExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_call_bits(node);
        self.capture_call_label_name(node);
    }

    fn enter_new_expression(
        &mut self,
        node: &mut oxc_ast::ast::NewExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_new_bits(node);
    }

    fn enter_tagged_template_expression(
        &mut self,
        node: &mut oxc_ast::ast::TaggedTemplateExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_tagged_template_bits(node);
    }

    fn enter_class(
        &mut self,
        node: &mut oxc_ast::ast::Class<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_class_bits(node);
    }

    fn enter_property_definition(
        &mut self,
        node: &mut oxc_ast::ast::PropertyDefinition<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_property_definition_bits(node);
    }

    fn enter_accessor_property(
        &mut self,
        node: &mut oxc_ast::ast::AccessorProperty<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_accessor_property_bits(node);
    }

    fn enter_object_property(
        &mut self,
        node: &mut oxc_ast::ast::ObjectProperty<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.capture_object_property_label_name(node);
    }

    fn enter_method_definition(
        &mut self,
        node: &mut oxc_ast::ast::MethodDefinition<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_method_definition_bits(node);
    }

    fn enter_variable_declarator(
        &mut self,
        node: &mut VariableDeclarator<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_variable_declarator_bits(node);
        self.capture_variable_arrow_name(node);
        self.rewrite_variable_rune_init(node);
    }

    fn enter_for_of_statement(
        &mut self,
        node: &mut oxc_ast::ast::ForOfStatement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if node.r#await && self.dev && self.experimental_async {
            use crate::builder::Arg;
            let right = self.b.move_expr(&mut node.right);
            node.right = self.b.call_expr("$.for_await_track_reactivity_loss", [Arg::Expr(right)]);
        }
    }

    fn enter_expression(&mut self, node: &mut Expression<'a>, ctx: &mut TraverseCtx<'a, ()>) {
        self.strip_ts_expression_wrappers(node);
        match node {
            Expression::AssignmentExpression(_) => self.transform_assignment(node, ctx),
            Expression::UpdateExpression(_) => self.transform_update(node, ctx),
            Expression::CallExpression(_) => self.rewrite_call_expression(node),
            Expression::StaticMemberExpression(_) => self.rewrite_static_member_expression(node, ctx),
            Expression::Identifier(_) => self.rewrite_identifier_expression(node),
            _ => {}
        }
    }

    fn exit_expression(&mut self, node: &mut Expression<'a>, _ctx: &mut TraverseCtx<'a, ()>) {
        if self.rewrite_private_assignment_exit(node) {
            return;
        }
        if self.rewrite_private_read_exit(node) {
            return;
        }
        if self.dev {
            if let Some(replacement) = self.transform_inspect(node) {
                *node = replacement;
                return;
            }
            self.rewrite_dev_await_tracking(node);
        }
    }
}
