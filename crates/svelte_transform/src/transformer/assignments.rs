use oxc_ast::ast::Expression;
use oxc_traverse::{Ancestor, TraverseCtx};
use svelte_analyze::{ReferenceSemantics, RuneKind};

use svelte_ast_builder::Arg;

use super::model::PendingPropMutationValidation;

use super::model::ComponentTransformer;

impl<'a> ComponentTransformer<'_, 'a> {
    fn member_root_identifier<'b>(
        &self,
        target: &'b oxc_ast::ast::MemberExpression<'a>,
    ) -> Option<&'b oxc_ast::ast::IdentifierReference<'a>> {
        let mut root = target.object();
        while let Some(member) = root.as_member_expression() {
            root = member.object();
        }

        let Expression::Identifier(root_id) = root else {
            return None;
        };
        Some(root_id)
    }

    fn prop_mutation_segments_from_member(
        &self,
        target: &oxc_ast::ast::MemberExpression<'a>,
    ) -> Option<Vec<Expression<'a>>> {
        let mut root = target.object();
        let mut segments_rev: Vec<Expression<'a>> = vec![match target {
            oxc_ast::ast::MemberExpression::StaticMemberExpression(member) => {
                self.b.str_expr(member.property.name.as_str())
            }
            oxc_ast::ast::MemberExpression::ComputedMemberExpression(member) => {
                self.prop_mutation_path_segment_expr(&member.expression)?
            }
            oxc_ast::ast::MemberExpression::PrivateFieldExpression(_) => return None,
        }];
        loop {
            match root {
                Expression::StaticMemberExpression(member) => {
                    segments_rev.push(self.b.str_expr(member.property.name.as_str()));
                    root = &member.object;
                }
                Expression::ComputedMemberExpression(member) => {
                    segments_rev.push(self.prop_mutation_path_segment_expr(&member.expression)?);
                    root = &member.object;
                }
                _ => break,
            }
        }
        segments_rev.reverse();
        Some(segments_rev)
    }

    fn prop_mutation_path_segment_expr(&self, expr: &Expression<'a>) -> Option<Expression<'a>> {
        match expr {
            Expression::StringLiteral(lit) => Some(self.b.str_expr(lit.value.as_str())),
            Expression::Identifier(_) => Some(self.b.clone_expr(expr)),
            _ => None,
        }
    }

    fn rewrite_prop_source_member_assignment_target(
        &mut self,
        target: &mut oxc_ast::ast::AssignmentTarget<'a>,
        root_name: &str,
    ) {
        crate::rune_refs::replace_expr_root_in_assign_target(
            target,
            self.b
                .call_expr(root_name, std::iter::empty::<Arg<'a, '_>>()),
        );
    }

    fn rewrite_prop_source_member_update_target(
        &mut self,
        target: &mut oxc_ast::ast::SimpleAssignmentTarget<'a>,
        root_name: &str,
    ) {
        crate::rune_refs::replace_expr_root_in_simple_target(
            target,
            self.b
                .call_expr(root_name, std::iter::empty::<Arg<'a, '_>>()),
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
        prop_alias: String,
        root_name: String,
        segments: Vec<Expression<'a>>,
        span_start: u32,
    ) {
        if !self.dev || self.is_in_ignored_stmt("ownership_invalid_mutation") {
            return;
        }
        self.needs_ownership_validator = true;

        let offset = self.script_content_start + span_start;
        let (line, col) = super::location::compute_line_col(self.component_source, offset);

        let mut path: Vec<Expression<'a>> = Vec::with_capacity(1 + segments.len());
        path.push(self.b.str_expr(&root_name));
        path.extend(segments);

        let expr = self.b.move_expr(node);
        let wrapped = self.b.call_expr(
            "$$ownership_validator.mutation",
            [
                Arg::Str(prop_alias),
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
        span_start: u32,
    ) {
        if !self.dev || self.is_in_ignored_stmt("ownership_invalid_mutation") {
            return;
        }
        self.needs_ownership_validator = true;

        let offset = self.script_content_start + span_start;
        let (line, col) = super::location::compute_line_col(self.component_source, offset);

        let mut path: Vec<Expression<'a>> = Vec::with_capacity(1 + mutation_info.segments.len());
        path.push(self.b.str_expr(&mutation_info.root_name));
        path.extend(mutation_info.segments);

        let expr = self.b.move_expr(node);
        let wrapped = self.b.call_expr(
            "$$ownership_validator.mutation",
            [
                Arg::Str(mutation_info.prop_alias),
                Arg::Expr(self.b.array_expr(path)),
                Arg::Expr(expr),
                Arg::Num(line as f64),
                Arg::Num(col as f64),
            ],
        );
        *node = wrapped;
    }

    fn finish_semantic_prop_member_assignment(
        &mut self,
        node: &mut Expression<'a>,
        ctx: &mut TraverseCtx<'a, ()>,
        prop_alias: String,
        root_name: String,
        bindable: bool,
        source_root_name: Option<String>,
        segments: Vec<Expression<'a>>,
    ) {
        let Expression::AssignmentExpression(assign) = node else {
            unreachable!();
        };

        let bindable_prop_source_root_name =
            source_root_name.as_ref().filter(|_| bindable).cloned();
        let left_span_start = assign.span.start;

        if let Some(source_root_name) = &source_root_name {
            self.rewrite_prop_source_member_assignment_target(&mut assign.left, source_root_name);
        }

        if !self.dev {
            if let Some(source_root_name) = bindable_prop_source_root_name {
                self.wrap_bindable_prop_source_mutation(node, &source_root_name);
            }
            return;
        }

        let is_expr_stmt = matches!(ctx.parent(), Ancestor::ExpressionStatementExpression(_));
        if is_expr_stmt {
            if let Some(source_root_name) = bindable_prop_source_root_name {
                self.wrap_bindable_prop_source_mutation(node, &source_root_name);
            }
            self.wrap_prop_mutation_validation(
                node,
                prop_alias,
                root_name,
                segments,
                left_span_start,
            );
            return;
        }

        let fn_name = match assign.operator {
            oxc_ast::ast::AssignmentOperator::Assign => Some("$.assign"),
            oxc_ast::ast::AssignmentOperator::LogicalAnd => Some("$.assign_and"),
            oxc_ast::ast::AssignmentOperator::LogicalOr => Some("$.assign_or"),
            oxc_ast::ast::AssignmentOperator::LogicalNullish => Some("$.assign_nullish"),
            _ => None,
        };
        let is_static = matches!(
            &assign.left,
            oxc_ast::ast::AssignmentTarget::StaticMemberExpression(_)
        );
        let is_computed = matches!(
            &assign.left,
            oxc_ast::ast::AssignmentTarget::ComputedMemberExpression(_)
        );
        let should_rewrite_assign = fn_name.is_some() && (is_static || is_computed);
        if !should_rewrite_assign {
            if let Some(source_root_name) = bindable_prop_source_root_name {
                self.wrap_bindable_prop_source_mutation(node, &source_root_name);
            }
            self.wrap_prop_mutation_validation(
                node,
                prop_alias,
                root_name,
                segments,
                left_span_start,
            );
            return;
        }

        let fn_name = fn_name.unwrap_or_else(|| unreachable!());

        let offset = self.script_content_start + left_span_start;
        let (line, col) = super::location::compute_line_col(self.component_source, offset);
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

        if is_static {
            let oxc_ast::ast::AssignmentTarget::StaticMemberExpression(m) = assign.left else {
                unreachable!();
            };
            let m = m.unbox();
            let key = self.b.str_expr(m.property.name.as_str());
            *node = self.b.call_expr(
                fn_name,
                [
                    Arg::Expr(m.object),
                    Arg::Expr(key),
                    Arg::Expr(assign.right),
                    Arg::Str(loc),
                ],
            );
        } else {
            let oxc_ast::ast::AssignmentTarget::ComputedMemberExpression(m) = assign.left else {
                unreachable!();
            };
            let m = m.unbox();
            *node = self.b.call_expr(
                fn_name,
                [
                    Arg::Expr(m.object),
                    Arg::Expr(m.expression),
                    Arg::Expr(assign.right),
                    Arg::Str(loc),
                ],
            );
        }
        if let Some(source_root_name) = bindable_prop_source_root_name {
            self.wrap_bindable_prop_source_mutation(node, &source_root_name);
        }
        self.wrap_prop_mutation_validation(node, prop_alias, root_name, segments, left_span_start);
    }

    fn finish_semantic_prop_member_update(
        &mut self,
        node: &mut Expression<'a>,
        prop_alias: String,
        root_name: String,
        source_root_name: Option<String>,
        segments: Vec<Expression<'a>>,
    ) {
        let Expression::UpdateExpression(upd) = node else {
            unreachable!();
        };
        let span_start = upd.span.start;

        if let Some(source_root_name) = source_root_name {
            self.rewrite_prop_source_member_update_target(&mut upd.argument, &source_root_name);
            self.pending_prop_update_validations.insert(
                span_start,
                PendingPropMutationValidation {
                    prop_alias,
                    root_name,
                    segments,
                },
            );
            return;
        }

        self.pending_prop_update_validations.insert(
            span_start,
            PendingPropMutationValidation {
                prop_alias,
                root_name,
                segments,
            },
        );
    }

    pub(crate) fn transform_assignment(
        &mut self,
        node: &mut Expression<'a>,
        ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if !matches!(node, Expression::AssignmentExpression(_)) {
            return;
        }

        // Script-only identifier branches: prop mutation, then shared signal/store
        // rewrite (used identically by template exit), then legacy v1 SignalSet
        // fallback. Each block re-borrows `node` because the previous step may
        // have replaced it.
        let is_identifier_target = {
            let Expression::AssignmentExpression(assign) = &*node else {
                unreachable!();
            };
            matches!(
                &assign.left,
                oxc_ast::ast::AssignmentTarget::AssignmentTargetIdentifier(_)
            )
        };

        if is_identifier_target {
            if let Some(analysis) = self.analysis {
                // Prop identifier mutation: `prop = val` → `prop(val)`. Script-only —
                // template never writes a prop identifier directly. Must run before
                // the shared helper so a prop-classified identifier takes this branch.
                let prop_rewrite = {
                    let Expression::AssignmentExpression(assign) = node else {
                        unreachable!();
                    };
                    let oxc_ast::ast::AssignmentTarget::AssignmentTargetIdentifier(id) =
                        &assign.left
                    else {
                        unreachable!();
                    };
                    id.reference_id.get().is_some_and(|ref_id| {
                        matches!(
                            analysis.reference_semantics(ref_id),
                            ReferenceSemantics::PropMutation { .. }
                        )
                    })
                };
                if prop_rewrite {
                    let Expression::AssignmentExpression(assign) = node else {
                        unreachable!();
                    };
                    let oxc_ast::ast::AssignmentTarget::AssignmentTargetIdentifier(id) =
                        &assign.left
                    else {
                        unreachable!();
                    };
                    let name = id.name.as_str().to_string();
                    let right = self.b.move_expr(&mut assign.right);
                    *node = self.b.call_expr(&name, [Arg::Expr(right)]);
                    return;
                }

                // Signal / store identifier assignment — shared with Template.
                if super::rewrites::rewrite_signal_or_store_identifier_assignment(
                    analysis,
                    self.b.ast.allocator,
                    node,
                    false,
                ) {
                    return;
                }
            }

        }

        // Member-target branches: prop member dev-validation + deep store
        // member mutation. Re-bind `assign` because the identifier-target
        // block above may have consumed it.
        let Expression::AssignmentExpression(assign) = node else {
            return;
        };
        // Prop member mutation (`foo.x = val` where foo is a `$props()` binding):
        // `ReferenceSemantics::Prop*MemberMutationRoot` is the analyzer's
        // operation-oriented answer for the root identifier. No AST
        // reconstruction here; the semantic variant already tells us this
        // reference is exactly "root of an LHS member on an assignment".
        let mut semantic_prop_alias = None;
        let mut semantic_root_name = None;
        let mut semantic_bindable = false;
        let mut semantic_source_root_name = None;
        let mut semantic_segments = None;
        if let Some(analysis) = self.analysis {
            if let Some(member) = assign.left.as_member_expression() {
                if let Some(root_id) = self.member_root_identifier(member) {
                    if let Some(ref_id) = root_id.reference_id.get() {
                        match analysis.reference_semantics(ref_id) {
                            ReferenceSemantics::PropSourceMemberMutationRoot {
                                bindable,
                                symbol,
                            } => {
                                if let (Some(prop_alias), Some(segments)) = (
                                    analysis.binding_origin_key(symbol),
                                    self.prop_mutation_segments_from_member(member),
                                ) {
                                    let root_name = analysis.scoping.symbol_name(symbol).to_string();
                                    semantic_prop_alias = Some(prop_alias.to_string());
                                    semantic_root_name = Some(root_name.clone());
                                    semantic_bindable = bindable;
                                    semantic_source_root_name = Some(root_name);
                                    semantic_segments = Some(segments);
                                }
                            }
                            ReferenceSemantics::PropNonSourceMemberMutationRoot { symbol } => {
                                if let (Some(prop_alias), Some(segments)) = (
                                    analysis.binding_origin_key(symbol),
                                    self.prop_mutation_segments_from_member(member),
                                ) {
                                    semantic_prop_alias = Some(prop_alias.to_string());
                                    semantic_root_name = Some(analysis.scoping.symbol_name(symbol).to_string());
                                    semantic_segments = Some(segments);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        if let (Some(prop_alias), Some(root_name), Some(segments)) =
            (semantic_prop_alias, semantic_root_name, semantic_segments)
        {
            self.finish_semantic_prop_member_assignment(
                node,
                ctx,
                prop_alias,
                root_name,
                semantic_bindable,
                semantic_source_root_name,
                segments,
            );
            return;
        }
        let left_span_start = assign.span.start;

        // Deep store member mutation — shared with Template (same helper).
        if let Some(analysis) = self.analysis {
            if super::rewrites::rewrite_deep_store_member_assignment(
                analysis,
                self.b.ast.allocator,
                node,
            ) {
                return;
            }
        }

        if !self.dev {
            return;
        }

        let is_expr_stmt = matches!(ctx.parent(), Ancestor::ExpressionStatementExpression(_));
        if is_expr_stmt {
            return;
        }

        // Re-bind after the shared helper call above; it may have mutated `node`.
        let Expression::AssignmentExpression(assign) = node else {
            return;
        };
        let fn_name = match assign.operator {
            oxc_ast::ast::AssignmentOperator::Assign => Some("$.assign"),
            oxc_ast::ast::AssignmentOperator::LogicalAnd => Some("$.assign_and"),
            oxc_ast::ast::AssignmentOperator::LogicalOr => Some("$.assign_or"),
            oxc_ast::ast::AssignmentOperator::LogicalNullish => Some("$.assign_nullish"),
            _ => None,
        };
        let is_static = matches!(
            &assign.left,
            oxc_ast::ast::AssignmentTarget::StaticMemberExpression(_)
        );
        let is_computed = matches!(
            &assign.left,
            oxc_ast::ast::AssignmentTarget::ComputedMemberExpression(_)
        );
        let should_rewrite_assign = fn_name.is_some()
            && (is_static || is_computed)
            && crate::rune_refs::should_proxy(&assign.right);
        if !should_rewrite_assign {
            return;
        }

        let fn_name = fn_name.unwrap_or_else(|| unreachable!());

        let offset = self.script_content_start + left_span_start;
        let (line, col) = super::location::compute_line_col(self.component_source, offset);
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

        if is_static {
            let oxc_ast::ast::AssignmentTarget::StaticMemberExpression(m) = assign.left else {
                unreachable!();
            };
            let m = m.unbox();
            let key = self.b.str_expr(m.property.name.as_str());
            *node = self.b.call_expr(
                fn_name,
                [
                    Arg::Expr(m.object),
                    Arg::Expr(key),
                    Arg::Expr(assign.right),
                    Arg::Str(loc),
                ],
            );
        } else {
            let oxc_ast::ast::AssignmentTarget::ComputedMemberExpression(m) = assign.left else {
                unreachable!();
            };
            let m = m.unbox();
            *node = self.b.call_expr(
                fn_name,
                [
                    Arg::Expr(m.object),
                    Arg::Expr(m.expression),
                    Arg::Expr(assign.right),
                    Arg::Str(loc),
                ],
            );
        }
    }

    pub(crate) fn transform_update(
        &mut self,
        node: &mut Expression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
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
                oxc_ast::ast::SimpleAssignmentTarget::AssignmentTargetIdentifier(_)
            )
        };

        if is_identifier_target {
            if let Some(analysis) = self.analysis {
                // Prop identifier update — script-only.
                let prop_rewrite = {
                    let Expression::UpdateExpression(upd) = node else {
                        unreachable!();
                    };
                    let oxc_ast::ast::SimpleAssignmentTarget::AssignmentTargetIdentifier(id) =
                        &upd.argument
                    else {
                        unreachable!();
                    };
                    id.reference_id.get().is_some_and(|ref_id| {
                        matches!(
                            analysis.reference_semantics(ref_id),
                            ReferenceSemantics::PropMutation { .. }
                        )
                    })
                };
                if prop_rewrite {
                    let Expression::UpdateExpression(upd) = node else {
                        unreachable!();
                    };
                    let oxc_ast::ast::SimpleAssignmentTarget::AssignmentTargetIdentifier(id) =
                        &upd.argument
                    else {
                        unreachable!();
                    };
                    let name = id.name.as_str().to_string();
                    let fn_name = if upd.prefix {
                        "$.update_pre_prop"
                    } else {
                        "$.update_prop"
                    };
                    let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident(&name)];
                    if upd.operator == oxc_ast::ast::UpdateOperator::Decrement {
                        args.push(Arg::Num(-1.0));
                    }
                    *node = self.b.call_expr(fn_name, args);
                    return;
                }

                // Signal / store identifier update — shared with Template.
                if super::rewrites::rewrite_signal_or_store_identifier_update(
                    analysis,
                    self.b.ast.allocator,
                    node,
                ) {
                    return;
                }
            }

        }

        // Re-bind `upd`; the identifier-target block above may have consumed it.
        let Expression::UpdateExpression(upd) = node else {
            return;
        };

        if let oxc_ast::ast::SimpleAssignmentTarget::PrivateFieldExpression(pfe) = &upd.argument {
            if matches!(&pfe.object, Expression::ThisExpression(_))
                && self.is_private_state_field(pfe.field.name.as_str())
            {
                let field_name = pfe.field.name.as_str();
                let fn_name = if upd.prefix {
                    "$.update_pre"
                } else {
                    "$.update"
                };
                let field_expr = self.b.this_private_member(field_name);
                let mut args: Vec<Arg<'a, '_>> = vec![Arg::Expr(field_expr)];
                if upd.operator == oxc_ast::ast::UpdateOperator::Decrement {
                    args.push(Arg::Num(-1.0));
                }
                *node = self.b.call_expr(fn_name, args);
                return;
            }
        }

        // Deep store member update — shared with Template (same helper).
        if let Some(analysis) = self.analysis {
            if super::rewrites::rewrite_deep_store_member_update(
                analysis,
                self.b.ast.allocator,
                node,
            ) {
                return;
            }
        }

        // Re-bind again for prop member dev-validation bookkeeping.
        let Expression::UpdateExpression(upd) = node else {
            return;
        };
        // Mirror of the assignment enter-path: `PropSourceMemberMutationRoot` /
        // `PropNonSourceMemberMutationRoot` carry the operation directly,
        // no need to inspect AST to decide "is this a prop member mutation".
        let mut semantic_prop_alias = None;
        let mut semantic_root_name = None;
        let mut semantic_source_root_name = None;
        let mut semantic_segments = None;
        if let Some(analysis) = self.analysis {
            if let Some(member) = upd.argument.as_member_expression() {
                if let Some(root_id) = self.member_root_identifier(member) {
                    if let Some(ref_id) = root_id.reference_id.get() {
                        match analysis.reference_semantics(ref_id) {
                            ReferenceSemantics::PropSourceMemberMutationRoot {
                                symbol, ..
                            } => {
                                if let (Some(prop_alias), Some(segments)) = (
                                    analysis.binding_origin_key(symbol),
                                    self.prop_mutation_segments_from_member(member),
                                ) {
                                    let root_name = analysis.scoping.symbol_name(symbol).to_string();
                                    semantic_prop_alias = Some(prop_alias.to_string());
                                    semantic_root_name = Some(root_name.clone());
                                    semantic_source_root_name = Some(root_name);
                                    semantic_segments = Some(segments);
                                }
                            }
                            ReferenceSemantics::PropNonSourceMemberMutationRoot { symbol } => {
                                if let (Some(prop_alias), Some(segments)) = (
                                    analysis.binding_origin_key(symbol),
                                    self.prop_mutation_segments_from_member(member),
                                ) {
                                    semantic_prop_alias = Some(prop_alias.to_string());
                                    semantic_root_name = Some(analysis.scoping.symbol_name(symbol).to_string());
                                    semantic_segments = Some(segments);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        if let (Some(prop_alias), Some(root_name), Some(segments)) =
            (semantic_prop_alias, semantic_root_name, semantic_segments)
        {
            self.finish_semantic_prop_member_update(
                node,
                prop_alias,
                root_name,
                semantic_source_root_name,
                segments,
            );
            return;
        }
    }

    pub(crate) fn rewrite_prop_update_ownership_exit(&mut self, node: &mut Expression<'a>) {
        let Expression::UpdateExpression(upd) = node else {
            return;
        };
        let span_start = upd.span.start;
        let Some(mutation_info) = self.pending_prop_update_validations.remove(&span_start) else {
            return;
        };
        self.wrap_pending_prop_mutation_validation(node, mutation_info, span_start);
    }

    pub(crate) fn rewrite_private_assignment_exit(&self, node: &mut Expression<'a>) -> bool {
        if let Expression::AssignmentExpression(assign) = node {
            if let oxc_ast::ast::AssignmentTarget::PrivateFieldExpression(pfe) = &assign.left {
                if matches!(&pfe.object, Expression::ThisExpression(_)) {
                    let field_name = pfe.field.name.as_str();
                    if self.is_private_state_field(field_name) {
                        let left_expr = self.b.this_private_member(field_name);
                        let right = self.b.move_expr(&mut assign.right);
                        let get_expr = self.b.this_private_member(field_name);
                        let left_read = self.b.call_expr("$.get", [Arg::Expr(get_expr)]);
                        let value = crate::rune_refs::build_compound_value(
                            self.b.ast.allocator,
                            assign.operator,
                            left_read,
                            right,
                        );

                        *node = self
                            .b
                            .call_expr("$.set", [Arg::Expr(left_expr), Arg::Expr(value)]);
                        return true;
                    }
                }
            }
        }
        false
    }

    pub(crate) fn rewrite_private_read_exit(&self, node: &mut Expression<'a>) -> bool {
        if let Expression::PrivateFieldExpression(pfe) = node {
            if matches!(&pfe.object, Expression::ThisExpression(_)) {
                let rune_kind = self.private_state_field_rune_kind(pfe.field.name.as_str());
                if let Some(kind) = rune_kind {
                    if self.in_constructor() && matches!(kind, RuneKind::State | RuneKind::StateRaw)
                    {
                        // Inside constructor, $state/$state.raw: this.#field → this.#field.v
                        let field_expr = self.b.move_expr(node);
                        *node = self.b.static_member_expr(field_expr, "v");
                    } else {
                        // Outside constructor or $derived: this.#field → $.get(this.#field)
                        let field_expr = self.b.move_expr(node);
                        *node = self.b.call_expr("$.get", [Arg::Expr(field_expr)]);
                    }
                    return true;
                }
            }
        }
        false
    }

    pub(crate) fn rewrite_dev_await_tracking(&self, node: &mut Expression<'a>) {
        if let Expression::AwaitExpression(await_expr) = node {
            if self.is_in_ignored_stmt("await_reactivity_loss") {
                return;
            }
            let arg = self.b.move_expr(&mut await_expr.argument);
            let track_call = self
                .b
                .call_expr("$.track_reactivity_loss", [Arg::Expr(arg)]);
            let awaited = self.b.await_expr(track_call);
            *node = self
                .b
                .call_expr_callee(awaited, std::iter::empty::<Arg<'a, '_>>());
        }
    }
}
