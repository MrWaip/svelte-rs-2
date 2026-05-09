mod assignments;
mod async_check;
mod builders;
mod derived;
mod entry;
mod equals;
mod inspect;
pub(crate) mod legacy_reactive;
mod location;
pub(crate) mod model;
mod props;

mod props_legacy;
mod rewrites;
mod runes;
mod state;

mod state_legacy;
mod statement_passes;
pub(crate) mod template_entry;
mod template_rewrites;
mod ts_cleanup;

pub use entry::{TransformScriptOutput, transform_script};
pub use location::sanitize_location;
pub use model::IgnoreQuery;

pub(crate) use svelte_analyze::{
    PROPS_IS_BINDABLE, PROPS_IS_IMMUTABLE, PROPS_IS_LAZY_INITIAL, PROPS_IS_RUNES, PROPS_IS_UPDATED,
};

use oxc_allocator::Vec as OxcVec;
use oxc_ast::ast::{
    AccessorProperty, ArrowFunctionExpression, AssignmentTarget, CallExpression, CatchParameter,
    ChainElement, Class, ClassBody, DoWhileStatement, ForInStatement, ForOfStatement, ForStatement,
    FormalParameter, FormalParameterRest, Expression, FunctionBody, Function, IfStatement,
    MethodDefinition, MethodDefinitionKind, NewExpression, ObjectProperty, PropertyDefinition,
    SimpleAssignmentTarget, Statement, TaggedTemplateExpression, VariableDeclarator,
    WhileStatement,
};
use oxc_span::GetSpan;
use oxc_traverse::{Ancestor, Traverse, TraverseCtx};

use model::{ComponentTransformer, FunctionInfo};

impl<'a> Traverse<'a, ()> for ComponentTransformer<'_, 'a> {
    fn enter_class_body(
        &mut self,
        node: &mut ClassBody<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.mode == model::TransformMode::Template {
            return;
        }
        let info = self.scan_class_state_fields(node);
        self.class_state_stack.push(info);
    }

    fn exit_class_body(
        &mut self,
        node: &mut ClassBody<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_class_members(node);
        if self.mode == model::TransformMode::Template {
            return;
        }

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
        node: &mut Function<'a>,
        ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_function_bits(node);
        if self.mode == model::TransformMode::Template {
            return;
        }
        let name = node
            .id
            .as_ref()
            .map(|id| id.name.to_string())
            .or_else(|| self.next_arrow_name.take());
        let in_constructor = matches!(
            ctx.parent(),
            Ancestor::MethodDefinitionValue(md)
                if *md.kind() == MethodDefinitionKind::Constructor
        );
        self.function_info_stack.push(FunctionInfo {
            is_async: node.r#async,
            name,
            span_start: node.span.start,
            in_constructor,
        });
    }

    fn exit_function(
        &mut self,
        _node: &mut Function<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.mode == model::TransformMode::Template {
            return;
        }
        self.function_info_stack.pop();
    }

    fn enter_arrow_function_expression(
        &mut self,
        node: &mut ArrowFunctionExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_arrow_bits(node);
        if self.mode == model::TransformMode::Template {
            return;
        }
        let name = self.next_arrow_name.take();
        self.function_info_stack.push(FunctionInfo {
            is_async: node.r#async,
            name,
            span_start: node.span.start,
            in_constructor: false,
        });
    }

    fn exit_arrow_function_expression(
        &mut self,
        _node: &mut ArrowFunctionExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.mode == model::TransformMode::Template {
            return;
        }
        self.function_info_stack.pop();
    }

    fn exit_function_body(&mut self, body: &mut FunctionBody<'a>, _ctx: &mut TraverseCtx<'a, ()>) {
        if self.mode == model::TransformMode::Template {
            return;
        }
        self.rewrite_trace_function_body(body);
    }

    fn exit_statements(
        &mut self,
        stmts: &mut OxcVec<'a, Statement<'a>>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.mode == model::TransformMode::Template {
            return;
        }
        self.process_statement_block(stmts);
    }

    fn enter_formal_parameter(
        &mut self,
        node: &mut FormalParameter<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_formal_parameter(node);
    }

    fn enter_catch_parameter(
        &mut self,
        node: &mut CatchParameter<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_catch_parameter(node);
    }

    fn enter_formal_parameter_rest(
        &mut self,
        node: &mut FormalParameterRest<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_formal_parameter_rest(node);
    }

    fn enter_simple_assignment_target(
        &mut self,
        node: &mut SimpleAssignmentTarget<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_simple_assignment_target(node);
    }

    fn enter_assignment_target(
        &mut self,
        node: &mut AssignmentTarget<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_assignment_target(node);
    }

    fn enter_if_statement(
        &mut self,
        node: &mut IfStatement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.replace_ts_only_body_with_empty(&mut node.consequent);
        self.strip_ts_only_alternate(&mut node.alternate);
    }

    fn enter_for_statement(
        &mut self,
        node: &mut ForStatement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.replace_ts_only_body_with_empty(&mut node.body);
    }

    fn enter_for_in_statement(
        &mut self,
        node: &mut ForInStatement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.replace_ts_only_body_with_empty(&mut node.body);
    }

    fn enter_while_statement(
        &mut self,
        node: &mut WhileStatement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.replace_ts_only_body_with_empty(&mut node.body);
    }

    fn enter_do_while_statement(
        &mut self,
        node: &mut DoWhileStatement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.replace_ts_only_body_with_empty(&mut node.body);
    }

    fn enter_call_expression(
        &mut self,
        node: &mut CallExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_call_bits(node);
        if self.mode == model::TransformMode::Template {
            return;
        }
        self.capture_call_label_name(node);
    }

    fn enter_new_expression(
        &mut self,
        node: &mut NewExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_new_bits(node);
    }

    fn enter_tagged_template_expression(
        &mut self,
        node: &mut TaggedTemplateExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_tagged_template_bits(node);
    }

    fn enter_class(&mut self, node: &mut Class<'a>, _ctx: &mut TraverseCtx<'a, ()>) {
        self.strip_ts_class_bits(node);
        if self.mode == model::TransformMode::Template {
            return;
        }
        self.class_name_stack
            .push(node.id.as_ref().map(|id| id.name.to_string()));
    }

    fn exit_class(&mut self, _node: &mut Class<'a>, _ctx: &mut TraverseCtx<'a, ()>) {
        if self.mode == model::TransformMode::Template {
            return;
        }
        self.class_name_stack.pop();
    }

    fn enter_property_definition(
        &mut self,
        node: &mut PropertyDefinition<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_property_definition_bits(node);
    }

    fn enter_accessor_property(
        &mut self,
        node: &mut AccessorProperty<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_accessor_property_bits(node);
    }

    fn enter_object_property(
        &mut self,
        node: &mut ObjectProperty<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.mode == model::TransformMode::Template {
            return;
        }
        self.capture_object_property_label_name(node);
        self.normalize_object_property_method_shorthand(node);
    }

    fn enter_method_definition(
        &mut self,
        node: &mut MethodDefinition<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_method_definition_bits(node);
    }

    fn enter_statement(&mut self, node: &mut Statement<'a>, _ctx: &mut TraverseCtx<'a, ()>) {
        if self.mode == model::TransformMode::Template {
            return;
        }
        self.enclosing_stmt_start.push(node.span().start);
    }

    fn exit_statement(&mut self, _node: &mut Statement<'a>, _ctx: &mut TraverseCtx<'a, ()>) {
        if self.mode == model::TransformMode::Template {
            return;
        }
        self.enclosing_stmt_start.pop();
    }

    fn enter_variable_declarator(
        &mut self,
        node: &mut VariableDeclarator<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_variable_declarator_bits(node);
        if let Some(init) = node.init.as_mut() {
            self.strip_ts_expression_wrappers(init);
        }
        if self.mode == model::TransformMode::Template {
            return;
        }
        self.capture_variable_arrow_name(node);
        self.rewrite_variable_rune_init(node);
    }

    fn enter_for_of_statement(
        &mut self,
        node: &mut ForOfStatement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.replace_ts_only_body_with_empty(&mut node.body);
        if self.mode == model::TransformMode::Template {
            return;
        }
        if node.r#await
            && self.dev
            && self.experimental_async
            && !self.is_in_ignored_stmt("await_reactivity_loss")
        {
            use svelte_ast_builder::Arg;
            let right = self.b.move_expr(&mut node.right);
            node.right = self
                .b
                .call_expr("$.for_await_track_reactivity_loss", [Arg::Expr(right)]);
        }
    }

    fn enter_chain_element(
        &mut self,
        node: &mut ChainElement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.strip_ts_chain_element_wrappers(node);
    }

    fn enter_expression(&mut self, node: &mut Expression<'a>, ctx: &mut TraverseCtx<'a, ()>) {
        self.strip_ts_expression_wrappers(node);

        if self.mode == model::TransformMode::Template {
            let is_lhs = matches!(
                ctx.parent(),
                Ancestor::AssignmentExpressionLeft(_)
                    | Ancestor::UpdateExpressionArgument(_)
            );
            template_rewrites::rewrite_template_enter(self, node, is_lhs, ctx);
            return;
        }

        match node {
            Expression::AssignmentExpression(_) => self.transform_assignment(node, ctx),
            Expression::UpdateExpression(_) => self.transform_update(node, ctx),
            Expression::CallExpression(_) => self.rewrite_call_expression(node),
            Expression::StaticMemberExpression(_) => {
                self.rewrite_static_member_expression(node, ctx)
            }
            Expression::Identifier(_) => self.rewrite_identifier_expression(node),
            _ => {}
        }
    }

    fn exit_expression(&mut self, node: &mut Expression<'a>, _ctx: &mut TraverseCtx<'a, ()>) {
        if self.mode == model::TransformMode::Template {
            template_rewrites::rewrite_template_exit(self, node);
            return;
        }

        if self.rewrite_legacy_state_destructure_assignment_exit(node) {
            return;
        }
        self.rewrite_prop_update_ownership_exit(node);
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
            if let Some(replacement) = self.transform_console_log(node) {
                *node = replacement;
                return;
            }
            self.rewrite_dev_await_tracking(node);
            equals::wrap_binary_equals_dev(self.b, node);
        }
    }
}
