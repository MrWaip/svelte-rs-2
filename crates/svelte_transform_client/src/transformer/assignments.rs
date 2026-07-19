use std::iter;
use std::mem;

use oxc_ast::ast::{
    AssignmentOperator, AssignmentTarget, Expression, IdentifierReference, MemberExpression,
    SimpleAssignmentTarget, UpdateOperator,
};
use oxc_span::GetSpan;
use oxc_traverse::{Ancestor, TraverseCtx};
use svelte_analyze::{ClassFieldSemantics, ReferenceSemantics, WarningCode};
use svelte_component_semantics::ReferenceId;

use svelte_ast_builder::Arg;

use super::async_check::is_expression_async;
use super::model::PendingPropMutationValidation;

use super::model::ComponentTransformer;
use crate::rune_refs::{replace_expr_root_in_assign_target, replace_expr_root_in_simple_target};

fn root_object_span_start(member: &MemberExpression<'_>) -> u32 {
    let mut cur: &Expression<'_> = match member {
        MemberExpression::ComputedMemberExpression(m) => &m.object,
        MemberExpression::StaticMemberExpression(m) => &m.object,
        MemberExpression::PrivateFieldExpression(m) => &m.object,
    };
    loop {
        cur = match cur {
            Expression::ComputedMemberExpression(m) => &m.object,
            Expression::StaticMemberExpression(m) => &m.object,
            Expression::PrivateFieldExpression(m) => &m.object,
            other => return other.span().start,
        };
    }
}

fn assign_op_literal(op: AssignmentOperator) -> Option<&'static str> {
    match op {
        AssignmentOperator::Assign => Some("="),
        AssignmentOperator::LogicalAnd => Some("&&="),
        AssignmentOperator::LogicalOr => Some("||="),
        AssignmentOperator::LogicalNullish => Some("??="),
        _ => None,
    }
}

impl<'a> ComponentTransformer<'_, 'a> {
    fn member_root_identifier<'b>(
        &self,
        target: &'b MemberExpression<'a>,
    ) -> Option<&'b IdentifierReference<'a>> {
        let mut root = target.object().get_inner_expression();
        while let Some(member) = root.as_member_expression() {
            root = member.object().get_inner_expression();
        }

        let Expression::Identifier(root_id) = root else {
            return None;
        };
        Some(root_id)
    }

    fn dev_assign_root_is_plain_identifier(&self, target: &AssignmentTarget<'a>) -> bool {
        let Some(member) = target.as_member_expression() else {
            return false;
        };
        let Some(root) = self.member_root_identifier(member) else {
            return false;
        };
        let Some(analysis) = self.analysis else {
            return true;
        };
        let Some(ref_id) = root.reference_id.get() else {
            return false;
        };
        analysis.scoping.get_reference(ref_id).symbol_id().is_some()
    }

    fn is_excluded_owner_arrow_body(&self, ctx: &TraverseCtx<'a, ()>) -> bool {
        let Some(owner) = self.template_owner_node else {
            return false;
        };
        let Some(analysis) = self.analysis else {
            return false;
        };
        if !analysis.attributes.get(owner).is_event_or_binding() {
            return false;
        }
        for ancestor in ctx.ancestors() {
            match ancestor {
                Ancestor::ParenthesizedExpressionExpression(_)
                | Ancestor::ExpressionStatementExpression(_)
                | Ancestor::FunctionBodyStatements(_) => continue,
                Ancestor::ArrowFunctionExpressionBody(_) => return true,
                _ => return false,
            }
        }
        false
    }

    fn prop_mutation_segments_from_member(
        &self,
        target: &MemberExpression<'a>,
    ) -> Option<Vec<Expression<'a>>> {
        let mut root = target.object().get_inner_expression();
        let mut segments_rev: Vec<Expression<'a>> = vec![match target {
            MemberExpression::StaticMemberExpression(member) => {
                self.b.str_expr(member.property.name.as_str())
            }
            MemberExpression::ComputedMemberExpression(member) => {
                self.prop_mutation_path_segment_expr(&member.expression)?
            }
            MemberExpression::PrivateFieldExpression(_) => return None,
        }];
        loop {
            match root {
                Expression::StaticMemberExpression(member) => {
                    segments_rev.push(self.b.str_expr(member.property.name.as_str()));
                    root = member.object.get_inner_expression();
                }
                Expression::ComputedMemberExpression(member) => {
                    segments_rev.push(self.prop_mutation_path_segment_expr(&member.expression)?);
                    root = member.object.get_inner_expression();
                }
                _ => break,
            }
        }
        segments_rev.reverse();
        Some(segments_rev)
    }

    fn prop_mutation_path_segment_expr(&self, expr: &Expression<'a>) -> Option<Expression<'a>> {
        match expr.get_inner_expression() {
            Expression::StringLiteral(lit) => Some(self.b.str_expr(lit.value.as_str())),
            Expression::NumericLiteral(lit) => Some(self.b.num_expr(lit.value)),
            Expression::Identifier(id) => {
                let idref = self.b.rid_at(id.name.as_str(), id.span);
                idref.reference_id.set(id.reference_id.get());
                Some(Expression::Identifier(self.b.ast.alloc(idref)))
            }
            _ => None,
        }
    }

    fn rewrite_prop_source_member_assignment_target(
        &mut self,
        target: &mut AssignmentTarget<'a>,
        root_name: &str,
    ) {
        replace_expr_root_in_assign_target(
            target,
            self.b.call_expr(root_name, iter::empty::<Arg<'a, '_>>()),
        );
    }

    fn rewrite_prop_source_member_update_target(
        &mut self,
        target: &mut SimpleAssignmentTarget<'a>,
        root_name: &str,
    ) {
        replace_expr_root_in_simple_target(
            target,
            self.b.call_expr(root_name, iter::empty::<Arg<'a, '_>>()),
        );
    }

    fn wrap_bindable_prop_source_mutation(&mut self, node: &mut Expression<'a>, root_name: &str) {
        let expr = self.b.move_expr(node);
        *node = self
            .b
            .call_expr(root_name, [Arg::Expr(expr), Arg::Bool(true)]);
    }

    fn wrap_prop_mutation_validation(
        &mut self,
        node: &mut Expression<'a>,
        prop_alias: Option<String>,
        root_name: String,
        segments: Vec<Expression<'a>>,
        span_start: u32,
    ) {
        if !self.dev || self.is_in_ignored_stmt(WarningCode::OwnershipInvalidMutation) {
            return;
        }
        self.needs_ownership_validator = true;

        let offset = span_start;
        let (line, col) = self.component_line_index.line_col(offset);

        let mut path: Vec<Expression<'a>> = Vec::with_capacity(1 + segments.len());
        path.push(self.b.str_expr(&root_name));
        path.extend(segments);

        let expr = self.b.move_expr(node);
        let alias_arg = match prop_alias {
            Some(s) => Arg::Str(s),
            None => Arg::Expr(self.b.null_expr()),
        };
        let wrapped = self.b.call_expr(
            "$$ownership_validator.mutation",
            [
                alias_arg,
                Arg::Expr(self.b.array_expr(path)),
                Arg::Expr(expr),
                Arg::Num(line as f64),
                Arg::Num(col as f64),
            ],
        );
        *node = wrapped;
    }

    fn wrap_pending_prop_mutation_validation(
        &mut self,
        node: &mut Expression<'a>,
        mutation_info: PendingPropMutationValidation<'a>,
        _span_start: u32,
    ) {
        if !self.dev || self.is_in_ignored_stmt(WarningCode::OwnershipInvalidMutation) {
            return;
        }
        self.needs_ownership_validator = true;

        let offset = mutation_info.loc_span_start;
        let (line, col) = self.component_line_index.line_col(offset);

        let mut path: Vec<Expression<'a>> = Vec::with_capacity(1 + mutation_info.segments.len());
        path.push(self.b.str_expr(&mutation_info.root_name));
        path.extend(mutation_info.segments);

        let expr = self.b.move_expr(node);
        let alias_arg = match mutation_info.prop_alias {
            Some(s) => Arg::Str(s),
            None => Arg::Expr(self.b.null_expr()),
        };
        let wrapped = self.b.call_expr(
            "$$ownership_validator.mutation",
            [
                alias_arg,
                Arg::Expr(self.b.array_expr(path)),
                Arg::Expr(expr),
                Arg::Num(line as f64),
                Arg::Num(col as f64),
            ],
        );
        *node = wrapped;
    }

    fn apply_indirect_invalidate(
        &self,
        node: &mut Expression<'a>,
        root_ref_id: Option<ReferenceId>,
        ctx: &mut TraverseCtx<'a, ()>,
    ) {
        let (Some(analysis), Some(ref_id)) = (self.analysis, root_ref_id) else {
            return;
        };
        let Some(root_sym) = analysis.symbol_for_reference(ref_id) else {
            return;
        };
        if analysis.legacy_indirect_bindings(root_sym).is_none() {
            return;
        }
        let placeholder = self.make_rune_get("");
        let built = mem::replace(node, placeholder);
        *node = self.maybe_wrap_legacy_indirect_invalidate(analysis, built, ref_id, ctx);
    }

    fn finish_semantic_prop_member_assignment(
        &mut self,
        node: &mut Expression<'a>,
        is_expr_stmt: bool,
        prop_alias: Option<String>,
        root_name: String,
        bindable: bool,
        source_root_name: Option<String>,
        segments: Option<Vec<Expression<'a>>>,
        root_ref_id: Option<ReferenceId>,
        ctx: &mut TraverseCtx<'a, ()>,
    ) {
        let Expression::AssignmentExpression(assign) = node else {
            unreachable!();
        };

        let bindable_prop_source_root_name =
            source_root_name.as_ref().filter(|_| bindable).cloned();
        let left_span_start = assign
            .left
            .as_member_expression()
            .map(root_object_span_start)
            .unwrap_or(assign.span.start);

        if let Some(source_root_name) = &source_root_name {
            self.rewrite_prop_source_member_assignment_target(&mut assign.left, source_root_name);
        }

        if !self.dev {
            if let Some(source_root_name) = bindable_prop_source_root_name {
                self.wrap_bindable_prop_source_mutation(node, &source_root_name);
            }
            self.apply_indirect_invalidate(node, root_ref_id, ctx);
            return;
        }

        if !self.is_in_ignored_stmt(WarningCode::OwnershipInvalidMutation) {
            self.needs_ownership_validator = true;
        }

        if is_expr_stmt {
            if let Some(source_root_name) = bindable_prop_source_root_name {
                self.wrap_bindable_prop_source_mutation(node, &source_root_name);
            }
            self.apply_indirect_invalidate(node, root_ref_id, ctx);
            if let Some(segments) = segments {
                self.wrap_prop_mutation_validation(
                    node,
                    prop_alias,
                    root_name,
                    segments,
                    left_span_start,
                );
            }
            return;
        }

        let op_literal = assign_op_literal(assign.operator);
        let is_static = matches!(&assign.left, AssignmentTarget::StaticMemberExpression(_));
        let is_computed = matches!(&assign.left, AssignmentTarget::ComputedMemberExpression(_));
        let should_rewrite_assign = op_literal.is_some()
            && (is_static || is_computed)
            && segments.is_none()
            && self.runes
            && !self.in_bind_setter_traverse;
        if !should_rewrite_assign {
            if let Some(source_root_name) = bindable_prop_source_root_name {
                self.wrap_bindable_prop_source_mutation(node, &source_root_name);
            }
            self.apply_indirect_invalidate(node, root_ref_id, ctx);
            if let Some(segments) = segments {
                self.wrap_prop_mutation_validation(
                    node,
                    prop_alias,
                    root_name,
                    segments,
                    left_span_start,
                );
            }
            return;
        }

        let op_literal = op_literal.unwrap_or_else(|| unreachable!());

        let offset = left_span_start;
        let (line, col) = self.component_line_index.line_col(offset);
        let loc = format!(
            "{}:{}:{}",
            super::location::sanitize_location(self.filename),
            line,
            col
        );

        let whole = self.b.move_expr(node);
        let Expression::AssignmentExpression(assign_box) = whole else {
            unreachable!();
        };
        let assign = assign_box.unbox();

        let needs_lazy_getter = op_literal != "=";
        let needs_async = needs_lazy_getter && is_expression_async(&assign.right);
        let fn_name = if needs_async {
            "$.assign_async"
        } else {
            "$.assign"
        };
        let scope = ctx.current_scope_id();
        let value_expr = if needs_lazy_getter {
            if needs_async {
                self.b.async_arrow_body_in_scope(assign.right, scope)
            } else {
                self.b.arrow_in_scope_expr(
                    self.b.no_params(),
                    [self.b.expr_stmt(assign.right)],
                    scope,
                )
            }
        } else {
            assign.right
        };

        let (object, key) = if is_static {
            let AssignmentTarget::StaticMemberExpression(m) = assign.left else {
                unreachable!();
            };
            let m = m.unbox();
            (m.object, self.b.str_expr(m.property.name.as_str()))
        } else {
            let AssignmentTarget::ComputedMemberExpression(m) = assign.left else {
                unreachable!();
            };
            let m = m.unbox();
            (m.object, m.expression)
        };

        let mut call = self.b.call_expr(
            fn_name,
            [
                Arg::Expr(object),
                Arg::Expr(key),
                Arg::Str(op_literal.to_string()),
                Arg::Expr(value_expr),
                Arg::Str(loc),
            ],
        );
        if needs_async {
            call = self.b.await_expr(call);
        }
        *node = call;
        if let Some(source_root_name) = bindable_prop_source_root_name {
            self.wrap_bindable_prop_source_mutation(node, &source_root_name);
        }
        self.apply_indirect_invalidate(node, root_ref_id, ctx);
        if let Some(segments) = segments {
            self.wrap_prop_mutation_validation(
                node,
                prop_alias,
                root_name,
                segments,
                left_span_start,
            );
        }
    }

    fn finish_semantic_prop_member_update(
        &mut self,
        node: &mut Expression<'a>,
        prop_alias: Option<String>,
        root_name: String,
        bindable: bool,
        source_root_name: Option<String>,
        segments: Vec<Expression<'a>>,
    ) {
        let Expression::UpdateExpression(upd) = node else {
            unreachable!();
        };
        let span_start = upd.span.start;
        let loc_span_start = upd
            .argument
            .as_member_expression()
            .map(root_object_span_start)
            .unwrap_or(span_start);

        let bindable_prop_source_root_name =
            source_root_name.as_ref().filter(|_| bindable).cloned();

        if let Some(source_root_name) = &source_root_name {
            self.rewrite_prop_source_member_update_target(&mut upd.argument, source_root_name);
        }

        if !self.dev {
            if let Some(source_root_name) = bindable_prop_source_root_name {
                self.wrap_bindable_prop_source_mutation(node, &source_root_name);
            }
            return;
        }

        if !self.is_in_ignored_stmt(WarningCode::OwnershipInvalidMutation) {
            self.needs_ownership_validator = true;
        }

        self.pending_prop_update_validations.insert(
            span_start,
            PendingPropMutationValidation {
                prop_alias,
                root_name,
                segments,
                loc_span_start,
                bindable_source_root_name: bindable_prop_source_root_name,
            },
        );
    }

    pub(crate) fn rewrite_prop_member_assignment(
        &mut self,
        node: &mut Expression<'a>,
        is_expr_stmt: bool,
        ctx: &mut TraverseCtx<'a, ()>,
    ) -> bool {
        let Expression::AssignmentExpression(assign) = node else {
            return false;
        };
        let mut semantic_prop_alias = None;
        let mut semantic_root_name = None;
        let mut semantic_bindable = false;
        let mut semantic_source_root_name = None;
        let mut semantic_segments = None;
        let mut semantic_ref_id = None;
        if let Some(analysis) = self.analysis
            && let Some(member) = assign.left.as_member_expression()
            && let Some(root_id) = self.member_root_identifier(member)
            && let Some(ref_id) = root_id.reference_id.get()
        {
            semantic_ref_id = Some(ref_id);
            match analysis.reference_semantics(ref_id) {
                ReferenceSemantics::PropSourceMemberMutationRoot { bindable, symbol } => {
                    if let Some((key, _origin_kind)) = analysis.binding_origin_key(symbol) {
                        let is_runes = analysis.binding_semantics(symbol).is_runes_prop();
                        let root_name = analysis.scoping.symbol_name(symbol).to_string();
                        semantic_prop_alias = Some(is_runes.then(|| key.into_owned()));
                        semantic_root_name = Some(root_name.clone());
                        semantic_bindable = bindable;
                        semantic_source_root_name = Some(root_name);
                        semantic_segments = self.prop_mutation_segments_from_member(member);
                    }
                }
                ReferenceSemantics::PropNonSourceMemberMutationRoot { symbol } => {
                    if let Some((key, _origin_kind)) = analysis.binding_origin_key(symbol) {
                        let is_runes = analysis.binding_semantics(symbol).is_runes_prop();
                        semantic_prop_alias = Some(is_runes.then(|| key.into_owned()));
                        semantic_root_name = Some(analysis.scoping.symbol_name(symbol).to_string());
                        semantic_segments = self.prop_mutation_segments_from_member(member);
                    }
                }
                _ => {}
            }
        }
        let (Some(prop_alias), Some(root_name)) = (semantic_prop_alias, semantic_root_name) else {
            return false;
        };
        self.finish_semantic_prop_member_assignment(
            node,
            is_expr_stmt,
            prop_alias,
            root_name,
            semantic_bindable,
            semantic_source_root_name,
            semantic_segments,
            semantic_ref_id,
            ctx,
        );
        true
    }

    pub(crate) fn rewrite_prop_member_update(&mut self, node: &mut Expression<'a>) -> bool {
        let Expression::UpdateExpression(upd) = node else {
            return false;
        };
        let mut semantic_prop_alias = None;
        let mut semantic_root_name = None;
        let mut semantic_bindable = false;
        let mut semantic_source_root_name = None;
        let mut semantic_segments = None;
        if let Some(analysis) = self.analysis
            && let Some(member) = upd.argument.as_member_expression()
            && let Some(root_id) = self.member_root_identifier(member)
            && let Some(ref_id) = root_id.reference_id.get()
        {
            match analysis.reference_semantics(ref_id) {
                ReferenceSemantics::PropSourceMemberMutationRoot { bindable, symbol } => {
                    if let (Some((key, _origin_kind)), Some(segments)) = (
                        analysis.binding_origin_key(symbol),
                        self.prop_mutation_segments_from_member(member),
                    ) {
                        let is_runes = analysis.binding_semantics(symbol).is_runes_prop();
                        let root_name = analysis.scoping.symbol_name(symbol).to_string();
                        semantic_prop_alias = Some(is_runes.then(|| key.into_owned()));
                        semantic_root_name = Some(root_name.clone());
                        semantic_bindable = bindable;
                        semantic_source_root_name = Some(root_name);
                        semantic_segments = Some(segments);
                    }
                }
                ReferenceSemantics::PropNonSourceMemberMutationRoot { symbol } => {
                    if let (Some((key, _origin_kind)), Some(segments)) = (
                        analysis.binding_origin_key(symbol),
                        self.prop_mutation_segments_from_member(member),
                    ) {
                        let is_runes = analysis.binding_semantics(symbol).is_runes_prop();
                        semantic_prop_alias = Some(is_runes.then(|| key.into_owned()));
                        semantic_root_name = Some(analysis.scoping.symbol_name(symbol).to_string());
                        semantic_segments = Some(segments);
                    }
                }
                _ => {}
            }
        }
        let (Some(prop_alias), Some(root_name), Some(segments)) =
            (semantic_prop_alias, semantic_root_name, semantic_segments)
        else {
            return false;
        };
        self.finish_semantic_prop_member_update(
            node,
            prop_alias,
            root_name,
            semantic_bindable,
            semantic_source_root_name,
            segments,
        );
        true
    }

    pub(crate) fn transform_assignment(
        &mut self,
        node: &mut Expression<'a>,
        ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if !matches!(node, Expression::AssignmentExpression(_)) {
            return;
        }

        if super::state_legacy::is_destructure_assignment_lhs(node) {
            self.destructure_lhs_depth += 1;
        }

        let is_identifier_target = matches!(
            &*node,
            Expression::AssignmentExpression(assign)
                if matches!(&assign.left, AssignmentTarget::AssignmentTargetIdentifier(_))
        );

        if is_identifier_target && self.dispatch_identifier_assignment(node, ctx) {
            return;
        }

        let assign_span_start = {
            let Expression::AssignmentExpression(assign) = &*node else {
                return;
            };
            assign.span.start
        };
        if assign_span_start != 0
            && self
                .dispatched_member_assignments
                .contains(&assign_span_start)
        {
            return;
        }

        let is_expr_stmt = assignment_parent_is_expression_statement(ctx);
        if self.dispatch_member_assignment(node, is_expr_stmt, ctx) {
            if assign_span_start != 0 {
                self.dispatched_member_assignments.insert(assign_span_start);
            }
            return;
        }

        self.apply_dev_member_assign_wrap(node, is_expr_stmt, ctx);
    }

    pub(crate) fn apply_dev_member_assign_wrap(
        &mut self,
        node: &mut Expression<'a>,
        is_expr_stmt: bool,
        ctx: &mut TraverseCtx<'a, ()>,
    ) {
        let Expression::AssignmentExpression(assign) = node else {
            return;
        };
        let left_span_start = assign.span.start;

        if !self.dev {
            return;
        }

        if is_expr_stmt {
            return;
        }

        let Expression::AssignmentExpression(assign) = node else {
            return;
        };
        let op_literal = assign_op_literal(assign.operator);
        let is_static = matches!(&assign.left, AssignmentTarget::StaticMemberExpression(_));
        let is_computed = matches!(&assign.left, AssignmentTarget::ComputedMemberExpression(_));
        let rhs_non_primitive = self
            .analysis
            .is_some_and(|analysis| analysis.assignment_value_is_non_primitive(assign.node_id()));
        let should_rewrite_assign = op_literal.is_some()
            && (is_static || is_computed)
            && rhs_non_primitive
            && self.dev_assign_root_is_plain_identifier(&assign.left)
            && !self.in_bind_setter_traverse
            && !self.is_excluded_owner_arrow_body(ctx);
        if !should_rewrite_assign {
            return;
        }

        let op_literal = op_literal.unwrap_or_else(|| unreachable!());

        let offset = left_span_start;
        let (line, col) = self.component_line_index.line_col(offset);
        let loc = format!(
            "{}:{}:{}",
            super::location::sanitize_location(self.filename),
            line,
            col
        );

        let whole = self.b.move_expr(node);
        let Expression::AssignmentExpression(assign_box) = whole else {
            unreachable!();
        };
        let assign = assign_box.unbox();

        let needs_lazy_getter = op_literal != "=";
        let needs_async = needs_lazy_getter && is_expression_async(&assign.right);
        let fn_name = if needs_async {
            "$.assign_async"
        } else {
            "$.assign"
        };
        let scope = ctx.current_scope_id();
        let value_expr = if needs_lazy_getter {
            if needs_async {
                self.b.async_arrow_body_in_scope(assign.right, scope)
            } else {
                self.b.arrow_in_scope_expr(
                    self.b.no_params(),
                    [self.b.expr_stmt(assign.right)],
                    scope,
                )
            }
        } else {
            assign.right
        };

        let (object, key) = if is_static {
            let AssignmentTarget::StaticMemberExpression(m) = assign.left else {
                unreachable!();
            };
            let m = m.unbox();
            (m.object, self.b.str_expr(m.property.name.as_str()))
        } else {
            let AssignmentTarget::ComputedMemberExpression(m) = assign.left else {
                unreachable!();
            };
            let m = m.unbox();
            (m.object, m.expression)
        };

        let mut call = self.b.call_expr(
            fn_name,
            [
                Arg::Expr(object),
                Arg::Expr(key),
                Arg::Str(op_literal.to_string()),
                Arg::Expr(value_expr),
                Arg::Str(loc),
            ],
        );
        if needs_async {
            call = self.b.await_expr(call);
        }
        *node = call;
    }

    pub(crate) fn transform_update(
        &mut self,
        node: &mut Expression<'a>,
        ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if !matches!(node, Expression::UpdateExpression(_)) {
            return;
        }

        let is_identifier_target = {
            let Expression::UpdateExpression(upd) = &*node else {
                unreachable!();
            };
            matches!(
                &upd.argument,
                SimpleAssignmentTarget::AssignmentTargetIdentifier(_)
            )
        };

        if is_identifier_target && self.dispatch_identifier_update(node, ctx) {
            return;
        }

        let Expression::UpdateExpression(upd) = node else {
            return;
        };

        if let SimpleAssignmentTarget::PrivateFieldExpression(pfe) = &upd.argument
            && self
                .analysis
                .is_some_and(|analysis| analysis.class_field_semantics(pfe.node_id()).is_field())
        {
            let field_name = pfe.field.name.as_str();
            let fn_name = if upd.prefix {
                "$.update_pre"
            } else {
                "$.update"
            };
            let field_expr = self
                .b
                .private_member(self.b.clone_expr(&pfe.object), field_name);
            let mut args: Vec<Arg<'a, '_>> = vec![Arg::Expr(field_expr)];
            if upd.operator == UpdateOperator::Decrement {
                args.push(Arg::Num(-1.0));
            }
            *node = self.b.call_expr(fn_name, args);
            return;
        }

        self.dispatch_member_update(node, ctx);
    }

    pub(crate) fn rewrite_prop_update_ownership_exit(&mut self, node: &mut Expression<'a>) {
        let Expression::UpdateExpression(upd) = node else {
            return;
        };
        let span_start = upd.span.start;
        let Some(mutation_info) = self.pending_prop_update_validations.remove(&span_start) else {
            return;
        };
        if let Some(source_root_name) = mutation_info.bindable_source_root_name.clone() {
            self.wrap_bindable_prop_source_mutation(node, &source_root_name);
        }
        self.wrap_pending_prop_mutation_validation(node, mutation_info, span_start);
    }

    pub(crate) fn rewrite_private_assignment_exit(&self, node: &mut Expression<'a>) -> bool {
        let Expression::AssignmentExpression(assign) = node else {
            return false;
        };
        let AssignmentTarget::PrivateFieldExpression(pfe) = &assign.left else {
            return false;
        };
        let Some(analysis) = self.analysis else {
            return false;
        };
        let (proxy, tracked) = match analysis.class_field_semantics(pfe.node_id()) {
            ClassFieldSemantics::None => return false,
            ClassFieldSemantics::State { proxy, tracked, .. } => (proxy, tracked),
            ClassFieldSemantics::Derived { .. } => (false, true),
        };
        let field_name = pfe.field.name.as_str();
        let left_expr = self
            .b
            .private_member(self.b.clone_expr(&pfe.object), field_name);
        let read_expr = self
            .b
            .private_member(self.b.clone_expr(&pfe.object), field_name);
        let right = self.b.move_expr(&mut assign.right);
        let left_read = if tracked {
            self.b.call_expr("$.get", [Arg::Expr(read_expr)])
        } else {
            self.b.static_member_expr(read_expr, "v")
        };
        let value = self.build_compound_value(assign.operator, left_read, right);

        *node = if proxy {
            self.b.call_expr(
                "$.set",
                [Arg::Expr(left_expr), Arg::Expr(value), Arg::Bool(true)],
            )
        } else {
            self.b
                .call_expr("$.set", [Arg::Expr(left_expr), Arg::Expr(value)])
        };
        true
    }

    pub(crate) fn rewrite_private_read_exit(&self, node: &mut Expression<'a>) -> bool {
        let Expression::PrivateFieldExpression(pfe) = &*node else {
            return false;
        };
        let Some(analysis) = self.analysis else {
            return false;
        };
        match analysis.class_field_semantics(pfe.node_id()) {
            ClassFieldSemantics::None => false,
            ClassFieldSemantics::State { tracked: false, .. } => {
                let field_expr = self.b.move_expr(node);
                *node = self.b.static_member_expr(field_expr, "v");
                true
            }
            ClassFieldSemantics::State { .. } | ClassFieldSemantics::Derived { .. } => {
                let field_expr = self.b.move_expr(node);
                *node = self.b.call_expr("$.get", [Arg::Expr(field_expr)]);
                true
            }
        }
    }

    pub(crate) fn rewrite_dev_await_tracking(&self, node: &mut Expression<'a>) {
        if let Expression::AwaitExpression(await_expr) = node {
            if self.is_in_ignored_stmt(WarningCode::AwaitReactivityLoss) {
                return;
            }
            if is_internal_async_await(&await_expr.argument) {
                return;
            }
            let arg = self.b.move_expr(&mut await_expr.argument);
            let track_call = self
                .b
                .call_expr("$.track_reactivity_loss", [Arg::Expr(arg)]);
            let awaited = self.b.await_expr(track_call);
            *node = self
                .b
                .call_expr_callee(awaited, iter::empty::<Arg<'a, '_>>());
        }
    }
}

fn is_internal_async_await(arg: &Expression<'_>) -> bool {
    let Expression::CallExpression(call) = arg else {
        return false;
    };
    let Expression::Identifier(id) = &call.callee else {
        return false;
    };
    matches!(id.name.as_str(), "$.async_derived" | "$.save")
}

pub(crate) fn assignment_parent_is_expression_statement(ctx: &TraverseCtx<'_, ()>) -> bool {
    let mut ancestors = ctx
        .ancestors()
        .filter(|a| !matches!(a, Ancestor::ParenthesizedExpressionExpression(_)));
    let Some(first) = ancestors.next() else {
        return false;
    };
    if !first.is_expression_statement() {
        return false;
    }
    match ancestors.next() {
        Some(Ancestor::FunctionBodyStatements(_)) => match ancestors.next() {
            Some(Ancestor::ArrowFunctionExpressionBody(arrow)) => !*arrow.expression(),
            _ => true,
        },
        _ => true,
    }
}
