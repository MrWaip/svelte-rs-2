use oxc_ast::ast::Expression;
use oxc_traverse::TraverseCtx;
use svelte_ast_builder::Arg;

use svelte_analyze::reactivity_semantics::legacy_reactive::legacy_reactive_import_wrapper_name;
use svelte_analyze::{
    CarrierMemberReadSemantics, ContextualReadKind, ContextualReadSemantics,
    PropReferenceSemantics, ReferenceSemantics, StateKind,
};
use svelte_component_semantics::SymbolId;

use super::model::ComponentTransformer;
use crate::rune_refs;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(crate) fn dispatch_identifier_read(&self, expr: &mut Expression<'a>) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let Expression::Identifier(id) = &*expr else {
            return false;
        };
        let Some(ref_id) = id.reference_id.get() else {
            return false;
        };
        let name = id.name.as_str().to_string();
        let sem = analysis.reference_semantics(ref_id);

        match sem {
            ReferenceSemantics::StoreRead { .. } => {
                *expr = self.make_thunk_call(&name);
                true
            }
            ReferenceSemantics::SignalRead { safe: false, .. }
            | ReferenceSemantics::SignalWrite { .. }
            | ReferenceSemantics::SignalUpdate { safe: false, .. }
            | ReferenceSemantics::LegacyStateWrite
            | ReferenceSemantics::LegacyStateUpdate { safe: false } => {
                *expr = self.make_rune_get(&name);
                true
            }
            ReferenceSemantics::SignalRead { safe: true, .. }
            | ReferenceSemantics::SignalUpdate { safe: true, .. }
            | ReferenceSemantics::LegacyStateUpdate { safe: true } => {
                *expr = self.make_rune_safe_get(&name);
                true
            }
            ReferenceSemantics::PropRead(PropReferenceSemantics::Source { .. }) => {
                *expr = self.make_thunk_call(&name);
                true
            }
            ReferenceSemantics::PropRead(PropReferenceSemantics::NonSource { symbol }) => {
                let prop_name = analysis.binding_origin_key(symbol).unwrap_or_else(|| {
                    panic!(
                        "NonSource prop read missing binding origin key for ref {:?}",
                        ref_id
                    )
                });
                *expr = self.make_props_access(prop_name);
                true
            }
            ReferenceSemantics::ConstAliasRead { owner_node } => {
                if let Some(tmp) = self.transform_data.const_tag_tmp_names.get(&owner_node) {
                    let tmp_name = tmp.clone();
                    *expr = self.make_member_get(&tmp_name, &name);
                }
                true
            }
            ReferenceSemantics::CarrierMemberRead(CarrierMemberReadSemantics {
                carrier_symbol,
                ..
            }) => {
                let carrier_name = analysis.scoping.symbol_name(carrier_symbol).to_string();
                *expr = self.make_member_get(&carrier_name, &name);
                true
            }
            ReferenceSemantics::LegacyPropsIdentifierRead => {
                *expr = self.b.rid_expr("$$sanitized_props");
                true
            }
            ReferenceSemantics::LegacyRestPropsIdentifierRead => {
                *expr = self.b.rid_expr("$$restProps");
                true
            }
            ReferenceSemantics::LegacyStateRead { safe: false } => {
                *expr = self.make_rune_get(&name);
                true
            }
            ReferenceSemantics::LegacyStateRead { safe: true } => {
                *expr = self.make_rune_safe_get(&name);
                true
            }
            ReferenceSemantics::LegacyReactiveImportRead => {
                let import_name: &str = self
                    .b
                    .alloc_str(&legacy_reactive_import_wrapper_name(&name));
                *expr = self.b.call_expr_callee(self.b.rid_expr(import_name), []);
                true
            }
            ReferenceSemantics::ContextualRead(ContextualReadSemantics { kind, .. }) => {
                match kind {
                    ContextualReadKind::EachItem { accessor: true, .. }
                    | ContextualReadKind::SnippetParam { accessor: true, .. } => {
                        *expr = self.make_thunk_call(&name);
                    }
                    ContextualReadKind::EachItem {
                        signal: true,
                        accessor: false,
                    }
                    | ContextualReadKind::EachIndex { signal: true }
                    | ContextualReadKind::SnippetParam {
                        signal: true,
                        accessor: false,
                    }
                    | ContextualReadKind::AwaitValue
                    | ContextualReadKind::AwaitError
                    | ContextualReadKind::LetDirective => {
                        *expr = self.make_rune_get(&name);
                    }
                    ContextualReadKind::EachItem {
                        accessor: false,
                        signal: false,
                    }
                    | ContextualReadKind::EachIndex { signal: false }
                    | ContextualReadKind::SnippetParam {
                        accessor: false,
                        signal: false,
                    } => {}
                }
                true
            }
            ReferenceSemantics::NonReactive
            | ReferenceSemantics::Proxy
            | ReferenceSemantics::StoreWrite { .. }
            | ReferenceSemantics::StoreUpdate { .. }
            | ReferenceSemantics::PropMutation { .. }
            | ReferenceSemantics::PropSourceMemberMutationRoot { .. }
            | ReferenceSemantics::PropNonSourceMemberMutationRoot { .. }
            | ReferenceSemantics::RestPropMemberRewrite
            | ReferenceSemantics::LegacyStateMemberMutationRoot { .. }
            | ReferenceSemantics::LegacyReactiveImportMemberMutationRoot { .. }
            | ReferenceSemantics::LegacyEachItemMemberMutationRoot { .. }
            | ReferenceSemantics::IllegalWrite
            | ReferenceSemantics::Unresolved => false,
        }
    }

    pub(crate) fn dispatch_identifier_assignment(
        &self,
        node: &mut Expression<'a>,
        suppress_proxy: bool,
    ) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let Expression::AssignmentExpression(assign) = node else {
            return false;
        };
        let oxc_ast::ast::AssignmentTarget::AssignmentTargetIdentifier(id) = &assign.left else {
            return false;
        };
        let Some(ref_id) = id.reference_id.get() else {
            return false;
        };
        let sem = analysis.reference_semantics(ref_id);

        match sem {
            ReferenceSemantics::SignalWrite { .. }
            | ReferenceSemantics::SignalUpdate { .. }
            | ReferenceSemantics::StoreWrite { .. }
            | ReferenceSemantics::StoreUpdate { .. }
            | ReferenceSemantics::LegacyStateWrite
            | ReferenceSemantics::LegacyStateUpdate { .. } => {
                self.rewrite_signal_or_store_identifier_assignment(node, suppress_proxy)
            }
            ReferenceSemantics::PropMutation { .. } => {
                self.rewrite_prop_identifier_assignment(node)
            }
            ReferenceSemantics::NonReactive
            | ReferenceSemantics::Proxy
            | ReferenceSemantics::SignalRead { .. }
            | ReferenceSemantics::StoreRead { .. }
            | ReferenceSemantics::PropRead(_)
            | ReferenceSemantics::PropSourceMemberMutationRoot { .. }
            | ReferenceSemantics::PropNonSourceMemberMutationRoot { .. }
            | ReferenceSemantics::ConstAliasRead { .. }
            | ReferenceSemantics::ContextualRead(_)
            | ReferenceSemantics::CarrierMemberRead(_)
            | ReferenceSemantics::RestPropMemberRewrite
            | ReferenceSemantics::LegacyPropsIdentifierRead
            | ReferenceSemantics::LegacyRestPropsIdentifierRead
            | ReferenceSemantics::LegacyStateRead { .. }
            | ReferenceSemantics::LegacyStateMemberMutationRoot { .. }
            | ReferenceSemantics::LegacyReactiveImportRead
            | ReferenceSemantics::LegacyReactiveImportMemberMutationRoot { .. }
            | ReferenceSemantics::LegacyEachItemMemberMutationRoot { .. }
            | ReferenceSemantics::IllegalWrite
            | ReferenceSemantics::Unresolved => false,
        }
    }

    pub(crate) fn dispatch_identifier_update(&self, node: &mut Expression<'a>) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let Expression::UpdateExpression(upd) = node else {
            return false;
        };
        let oxc_ast::ast::SimpleAssignmentTarget::AssignmentTargetIdentifier(id) = &upd.argument
        else {
            return false;
        };
        let Some(ref_id) = id.reference_id.get() else {
            return false;
        };
        let sem = analysis.reference_semantics(ref_id);

        match sem {
            ReferenceSemantics::SignalUpdate { .. }
            | ReferenceSemantics::StoreUpdate { .. }
            | ReferenceSemantics::LegacyStateUpdate { .. } => {
                self.rewrite_signal_or_store_identifier_update(node)
            }
            ReferenceSemantics::PropMutation { .. } => self.rewrite_prop_identifier_update(node),
            ReferenceSemantics::NonReactive
            | ReferenceSemantics::Proxy
            | ReferenceSemantics::SignalRead { .. }
            | ReferenceSemantics::SignalWrite { .. }
            | ReferenceSemantics::StoreRead { .. }
            | ReferenceSemantics::StoreWrite { .. }
            | ReferenceSemantics::PropRead(_)
            | ReferenceSemantics::PropSourceMemberMutationRoot { .. }
            | ReferenceSemantics::PropNonSourceMemberMutationRoot { .. }
            | ReferenceSemantics::ConstAliasRead { .. }
            | ReferenceSemantics::ContextualRead(_)
            | ReferenceSemantics::CarrierMemberRead(_)
            | ReferenceSemantics::RestPropMemberRewrite
            | ReferenceSemantics::LegacyPropsIdentifierRead
            | ReferenceSemantics::LegacyRestPropsIdentifierRead
            | ReferenceSemantics::LegacyStateRead { .. }
            | ReferenceSemantics::LegacyStateWrite
            | ReferenceSemantics::LegacyStateMemberMutationRoot { .. }
            | ReferenceSemantics::LegacyReactiveImportRead
            | ReferenceSemantics::LegacyReactiveImportMemberMutationRoot { .. }
            | ReferenceSemantics::LegacyEachItemMemberMutationRoot { .. }
            | ReferenceSemantics::IllegalWrite
            | ReferenceSemantics::Unresolved => false,
        }
    }

    pub(crate) fn dispatch_member_assignment(
        &mut self,
        node: &mut Expression<'a>,
        is_expr_stmt: bool,
        ctx: &mut TraverseCtx<'a, ()>,
    ) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let Expression::AssignmentExpression(assign) = node else {
            return false;
        };
        let Some(member) = assign.left.as_member_expression() else {
            return false;
        };
        let Some(root) = rune_refs::find_expr_root_identifier(member.object()) else {
            return false;
        };
        let Some(ref_id) = root.reference_id.get() else {
            return false;
        };
        let sem = analysis.reference_semantics(ref_id);

        match sem {
            ReferenceSemantics::StoreRead { .. } => self.rewrite_deep_store_member_assignment(node),
            ReferenceSemantics::LegacyReactiveImportMemberMutationRoot { .. } => {
                self.rewrite_legacy_reactive_import_member_assignment(node)
            }
            ReferenceSemantics::LegacyStateMemberMutationRoot { .. } => {
                self.rewrite_legacy_state_member_assignment(node)
            }
            ReferenceSemantics::PropSourceMemberMutationRoot { .. }
            | ReferenceSemantics::PropNonSourceMemberMutationRoot { .. } => {
                self.rewrite_prop_member_assignment(node, is_expr_stmt)
            }
            ReferenceSemantics::LegacyEachItemMemberMutationRoot { item_sym } => {
                self.rewrite_legacy_each_item_member_assignment(node, item_sym, ctx)
            }
            ReferenceSemantics::NonReactive
            | ReferenceSemantics::Proxy
            | ReferenceSemantics::SignalRead { .. }
            | ReferenceSemantics::SignalWrite { .. }
            | ReferenceSemantics::SignalUpdate { .. }
            | ReferenceSemantics::StoreWrite { .. }
            | ReferenceSemantics::StoreUpdate { .. }
            | ReferenceSemantics::PropRead(_)
            | ReferenceSemantics::PropMutation { .. }
            | ReferenceSemantics::ConstAliasRead { .. }
            | ReferenceSemantics::ContextualRead(_)
            | ReferenceSemantics::CarrierMemberRead(_)
            | ReferenceSemantics::RestPropMemberRewrite
            | ReferenceSemantics::LegacyPropsIdentifierRead
            | ReferenceSemantics::LegacyRestPropsIdentifierRead
            | ReferenceSemantics::LegacyStateRead { .. }
            | ReferenceSemantics::LegacyStateWrite
            | ReferenceSemantics::LegacyStateUpdate { .. }
            | ReferenceSemantics::LegacyReactiveImportRead
            | ReferenceSemantics::IllegalWrite
            | ReferenceSemantics::Unresolved => false,
        }
    }

    pub(crate) fn dispatch_member_update(
        &mut self,
        node: &mut Expression<'a>,
        ctx: &mut TraverseCtx<'a, ()>,
    ) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let Expression::UpdateExpression(upd) = node else {
            return false;
        };
        let Some(member) = upd.argument.as_member_expression() else {
            return false;
        };
        let Some(root) = rune_refs::find_expr_root_identifier(member.object()) else {
            return false;
        };
        let Some(ref_id) = root.reference_id.get() else {
            return false;
        };
        let sem = analysis.reference_semantics(ref_id);

        match sem {
            ReferenceSemantics::StoreRead { .. } => self.rewrite_deep_store_member_update(node),
            ReferenceSemantics::LegacyReactiveImportMemberMutationRoot { .. } => {
                self.rewrite_legacy_reactive_import_member_update(node)
            }
            ReferenceSemantics::LegacyStateMemberMutationRoot { .. } => {
                self.rewrite_legacy_state_member_update(node)
            }
            ReferenceSemantics::PropSourceMemberMutationRoot { .. }
            | ReferenceSemantics::PropNonSourceMemberMutationRoot { .. } => {
                self.rewrite_prop_member_update(node)
            }
            ReferenceSemantics::LegacyEachItemMemberMutationRoot { item_sym } => {
                self.rewrite_legacy_each_item_member_update(node, item_sym, ctx)
            }
            ReferenceSemantics::NonReactive
            | ReferenceSemantics::Proxy
            | ReferenceSemantics::SignalRead { .. }
            | ReferenceSemantics::SignalWrite { .. }
            | ReferenceSemantics::SignalUpdate { .. }
            | ReferenceSemantics::StoreWrite { .. }
            | ReferenceSemantics::StoreUpdate { .. }
            | ReferenceSemantics::PropRead(_)
            | ReferenceSemantics::PropMutation { .. }
            | ReferenceSemantics::ConstAliasRead { .. }
            | ReferenceSemantics::ContextualRead(_)
            | ReferenceSemantics::CarrierMemberRead(_)
            | ReferenceSemantics::RestPropMemberRewrite
            | ReferenceSemantics::LegacyPropsIdentifierRead
            | ReferenceSemantics::LegacyRestPropsIdentifierRead
            | ReferenceSemantics::LegacyStateRead { .. }
            | ReferenceSemantics::LegacyStateWrite
            | ReferenceSemantics::LegacyStateUpdate { .. }
            | ReferenceSemantics::LegacyReactiveImportRead
            | ReferenceSemantics::IllegalWrite
            | ReferenceSemantics::Unresolved => false,
        }
    }

    pub(crate) fn rewrite_signal_or_store_identifier_assignment(
        &self,
        node: &mut Expression<'a>,
        suppress_proxy: bool,
    ) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let Expression::AssignmentExpression(assign) = node else {
            return false;
        };
        let oxc_ast::ast::AssignmentTarget::AssignmentTargetIdentifier(id) = &assign.left else {
            return false;
        };
        let Some(ref_id) = id.reference_id.get() else {
            return false;
        };
        let name = id.name.as_str().to_string();
        let operator = assign.operator;
        let semantics = analysis.reference_semantics(ref_id);

        match semantics {
            ReferenceSemantics::StoreWrite { symbol }
            | ReferenceSemantics::StoreUpdate { symbol } => {
                let base_name = analysis.scoping.symbol_name(symbol).to_string();
                let right = std::mem::replace(&mut assign.right, self.make_rune_get(""));

                let left_read = self.make_thunk_call(&name);
                let value = self.build_compound_value(operator, left_read, right);
                *node = self.make_store_set(&base_name, value);
                true
            }
            ReferenceSemantics::SignalWrite { kind } => {
                let right = std::mem::replace(&mut assign.right, self.make_rune_get(""));
                let needs_proxy = !suppress_proxy
                    && kind == StateKind::State
                    && rune_refs::is_non_coercive_operator(operator)
                    && rune_refs::should_proxy(&right);
                let left_read = self.make_rune_get(&name);
                let value = self.build_compound_value(operator, left_read, right);
                *node = self.make_rune_set(&name, value, needs_proxy);
                true
            }
            ReferenceSemantics::SignalUpdate { kind, safe } => {
                let right = std::mem::replace(&mut assign.right, self.make_rune_get(""));
                let needs_proxy = !suppress_proxy
                    && kind == StateKind::State
                    && rune_refs::is_non_coercive_operator(operator)
                    && rune_refs::should_proxy(&right);
                let left_read = if safe {
                    self.make_rune_safe_get(&name)
                } else {
                    self.make_rune_get(&name)
                };
                let value = self.build_compound_value(operator, left_read, right);
                *node = self.make_rune_set(&name, value, needs_proxy);
                true
            }

            ReferenceSemantics::LegacyStateWrite => {
                let right = std::mem::replace(&mut assign.right, self.make_rune_get(""));
                let left_read = self.make_rune_get(&name);
                let value = self.build_compound_value(operator, left_read, right);
                *node = self.make_rune_set(&name, value, false);
                true
            }

            ReferenceSemantics::LegacyStateUpdate { safe } => {
                let right = std::mem::replace(&mut assign.right, self.make_rune_get(""));
                let left_read = if safe {
                    self.make_rune_safe_get(&name)
                } else {
                    self.make_rune_get(&name)
                };
                let value = self.build_compound_value(operator, left_read, right);
                *node = self.make_rune_set(&name, value, false);
                true
            }
            _ => false,
        }
    }

    pub(crate) fn rewrite_signal_or_store_identifier_update(
        &self,
        node: &mut Expression<'a>,
    ) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let Expression::UpdateExpression(upd) = node else {
            return false;
        };
        let oxc_ast::ast::SimpleAssignmentTarget::AssignmentTargetIdentifier(id) = &upd.argument
        else {
            return false;
        };
        let Some(ref_id) = id.reference_id.get() else {
            return false;
        };
        let is_increment = upd.operator == oxc_syntax::operator::UpdateOperator::Increment;
        let name = id.name.as_str().to_string();
        let is_prefix = upd.prefix;

        match analysis.reference_semantics(ref_id) {
            ReferenceSemantics::StoreUpdate { symbol } => {
                let base_name = analysis.scoping.symbol_name(symbol).to_string();
                *node = self.make_store_update(&base_name, &name, is_prefix, is_increment);
                true
            }
            ReferenceSemantics::SignalUpdate {
                kind: StateKind::State | StateKind::StateRaw,
                ..
            } => {
                *node = self.make_rune_update(&name, is_prefix, is_increment);
                true
            }

            ReferenceSemantics::LegacyStateUpdate { .. } => {
                *node = self.make_rune_update(&name, is_prefix, is_increment);
                true
            }
            _ => false,
        }
    }

    pub(crate) fn rewrite_prop_identifier_assignment(&self, node: &mut Expression<'a>) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let Expression::AssignmentExpression(assign) = node else {
            return false;
        };
        let oxc_ast::ast::AssignmentTarget::AssignmentTargetIdentifier(id) = &assign.left else {
            return false;
        };
        let Some(ref_id) = id.reference_id.get() else {
            return false;
        };
        if !matches!(
            analysis.reference_semantics(ref_id),
            ReferenceSemantics::PropMutation { .. }
        ) {
            return false;
        }
        let name = id.name.as_str().to_string();
        let operator = assign.operator;
        let right = self.b.move_expr(&mut assign.right);
        let value = if operator.is_assign() {
            right
        } else {
            let left_read = self.b.call_expr(&name, std::iter::empty::<Arg<'a, '_>>());
            self.build_compound_value(operator, left_read, right)
        };
        *node = self.b.call_expr(&name, [Arg::Expr(value)]);
        true
    }

    pub(crate) fn rewrite_prop_identifier_update(&self, node: &mut Expression<'a>) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let Expression::UpdateExpression(upd) = node else {
            return false;
        };
        let oxc_ast::ast::SimpleAssignmentTarget::AssignmentTargetIdentifier(id) = &upd.argument
        else {
            return false;
        };
        let Some(ref_id) = id.reference_id.get() else {
            return false;
        };
        if !matches!(
            analysis.reference_semantics(ref_id),
            ReferenceSemantics::PropMutation { .. }
        ) {
            return false;
        }
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
        true
    }

    pub(crate) fn rewrite_deep_store_member_assignment(&self, node: &mut Expression<'a>) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let Expression::AssignmentExpression(assign) = node else {
            return false;
        };
        let Some(member) = assign.left.as_member_expression() else {
            return false;
        };
        let Some(root) = rune_refs::find_expr_root_identifier(member.object()) else {
            return false;
        };
        let Some(ref_id) = root.reference_id.get() else {
            return false;
        };
        let ReferenceSemantics::StoreRead { symbol } = analysis.reference_semantics(ref_id) else {
            return false;
        };
        let root_name = root.name.as_str().to_string();
        let base_name = analysis.scoping.symbol_name(symbol).to_string();
        rune_refs::replace_expr_root_in_assign_target(
            &mut assign.left,
            self.make_untrack(&root_name),
        );
        let placeholder = self.make_rune_get("");
        let mutation = std::mem::replace(node, placeholder);
        let untracked = self.make_untrack(&root_name);
        *node = self.make_store_mutate(&base_name, mutation, untracked);
        true
    }

    pub(crate) fn rewrite_legacy_reactive_import_member_assignment(
        &self,
        node: &mut Expression<'a>,
    ) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let Expression::AssignmentExpression(assign) = node else {
            return false;
        };
        let Some(member) = assign.left.as_member_expression() else {
            return false;
        };
        let Some(root) = rune_refs::find_expr_root_identifier(member.object()) else {
            return false;
        };
        let Some(ref_id) = root.reference_id.get() else {
            return false;
        };
        if !matches!(
            analysis.reference_semantics(ref_id),
            ReferenceSemantics::LegacyReactiveImportMemberMutationRoot { .. }
        ) {
            return false;
        }
        let root_name = root.name.as_str().to_string();
        let import_name: &'a str = self
            .b
            .alloc_str(&legacy_reactive_import_wrapper_name(&root_name));
        let import_call = self.b.call_expr_callee(self.b.rid_expr(import_name), []);
        rune_refs::replace_expr_root_in_assign_target(&mut assign.left, import_call);
        let placeholder = self.b.cheap_expr();
        let mutation = std::mem::replace(node, placeholder);
        *node = self.b.call_expr(import_name, [Arg::Expr(mutation)]);
        true
    }

    pub(crate) fn rewrite_legacy_reactive_import_member_update(
        &self,
        node: &mut Expression<'a>,
    ) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let Expression::UpdateExpression(upd) = node else {
            return false;
        };
        let Some(member) = upd.argument.as_member_expression() else {
            return false;
        };
        let Some(root) = rune_refs::find_expr_root_identifier(member.object()) else {
            return false;
        };
        let Some(ref_id) = root.reference_id.get() else {
            return false;
        };
        if !matches!(
            analysis.reference_semantics(ref_id),
            ReferenceSemantics::LegacyReactiveImportMemberMutationRoot { .. }
        ) {
            return false;
        }
        let root_name = root.name.as_str().to_string();
        let import_name: &'a str = self
            .b
            .alloc_str(&legacy_reactive_import_wrapper_name(&root_name));
        let import_call = self.b.call_expr_callee(self.b.rid_expr(import_name), []);
        rune_refs::replace_expr_root_in_simple_target(&mut upd.argument, import_call);
        let placeholder = self.b.cheap_expr();
        let mutation = std::mem::replace(node, placeholder);
        *node = self.b.call_expr(import_name, [Arg::Expr(mutation)]);
        true
    }

    pub(crate) fn rewrite_legacy_state_member_assignment(&self, node: &mut Expression<'a>) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let Expression::AssignmentExpression(assign) = node else {
            return false;
        };
        let Some(member) = assign.left.as_member_expression() else {
            return false;
        };
        let Some(root) = rune_refs::find_expr_root_identifier(member.object()) else {
            return false;
        };
        let Some(ref_id) = root.reference_id.get() else {
            return false;
        };
        if !matches!(
            analysis.reference_semantics(ref_id),
            ReferenceSemantics::LegacyStateMemberMutationRoot { .. }
        ) {
            return false;
        }
        let root_name = root.name.as_str().to_string();
        rune_refs::replace_expr_root_in_assign_target(
            &mut assign.left,
            self.make_rune_get(&root_name),
        );
        let placeholder = self.make_rune_get("");
        let mutation = std::mem::replace(node, placeholder);
        *node = self.make_legacy_state_mutate(&root_name, mutation);
        true
    }

    pub(crate) fn rewrite_legacy_state_member_update(&self, node: &mut Expression<'a>) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let Expression::UpdateExpression(upd) = node else {
            return false;
        };
        let Some(member) = upd.argument.as_member_expression() else {
            return false;
        };
        let Some(root) = rune_refs::find_expr_root_identifier(member.object()) else {
            return false;
        };
        let Some(ref_id) = root.reference_id.get() else {
            return false;
        };
        if !matches!(
            analysis.reference_semantics(ref_id),
            ReferenceSemantics::LegacyStateMemberMutationRoot { .. }
        ) {
            return false;
        }
        let root_name = root.name.as_str().to_string();
        rune_refs::replace_expr_root_in_simple_target(
            &mut upd.argument,
            self.make_rune_get(&root_name),
        );
        let placeholder = self.make_rune_get("");
        let mutation = std::mem::replace(node, placeholder);
        *node = self.make_legacy_state_mutate(&root_name, mutation);
        true
    }

    pub(crate) fn rewrite_legacy_each_item_member_assignment(
        &self,
        node: &mut Expression<'a>,
        item_sym: SymbolId,
        ctx: &mut TraverseCtx<'a, ()>,
    ) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let Expression::AssignmentExpression(assign) = node else {
            return false;
        };
        let Some(member) = assign.left.as_member_expression() else {
            return false;
        };
        let Some(root) = rune_refs::find_expr_root_identifier(member.object()) else {
            return false;
        };
        let item_name = root.name.as_str().to_string();
        let Some(source_syms) = analysis.each_item_indirect_sources(item_sym) else {
            return false;
        };
        if source_syms.is_empty() {
            return false;
        }
        rune_refs::replace_expr_root_in_assign_target(
            &mut assign.left,
            self.make_rune_get(&item_name),
        );
        let placeholder = self.make_rune_get("");
        let mutation = std::mem::replace(node, placeholder);
        *node = self.make_each_item_invalidate_seq(mutation, source_syms, ctx);
        true
    }

    pub(crate) fn rewrite_legacy_each_item_member_update(
        &self,
        node: &mut Expression<'a>,
        item_sym: SymbolId,
        ctx: &mut TraverseCtx<'a, ()>,
    ) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let Expression::UpdateExpression(upd) = node else {
            return false;
        };
        let Some(member) = upd.argument.as_member_expression() else {
            return false;
        };
        let Some(root) = rune_refs::find_expr_root_identifier(member.object()) else {
            return false;
        };
        let item_name = root.name.as_str().to_string();
        let Some(source_syms) = analysis.each_item_indirect_sources(item_sym) else {
            return false;
        };
        if source_syms.is_empty() {
            return false;
        }
        rune_refs::replace_expr_root_in_simple_target(
            &mut upd.argument,
            self.make_rune_get(&item_name),
        );
        let placeholder = self.make_rune_get("");
        let mutation = std::mem::replace(node, placeholder);
        *node = self.make_each_item_invalidate_seq(mutation, source_syms, ctx);
        true
    }

    pub(crate) fn rewrite_deep_store_member_update(&self, node: &mut Expression<'a>) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let Expression::UpdateExpression(upd) = node else {
            return false;
        };
        let Some(member) = upd.argument.as_member_expression() else {
            return false;
        };
        let Some(root) = rune_refs::find_expr_root_identifier(member.object()) else {
            return false;
        };
        let Some(ref_id) = root.reference_id.get() else {
            return false;
        };
        let ReferenceSemantics::StoreRead { symbol } = analysis.reference_semantics(ref_id) else {
            return false;
        };
        let root_name = root.name.as_str().to_string();
        let base_name = analysis.scoping.symbol_name(symbol).to_string();
        rune_refs::replace_expr_root_in_simple_target(
            &mut upd.argument,
            self.make_untrack(&root_name),
        );
        let placeholder = self.make_rune_get("");
        let mutation = std::mem::replace(node, placeholder);
        let untracked = self.make_untrack(&root_name);
        *node = self.make_store_mutate(&base_name, mutation, untracked);
        true
    }

    pub(crate) fn rewrite_shared_call(
        &self,
        expr: &mut Expression<'a>,
        dev_snapshot_uncloneable_ignored: bool,
    ) -> bool {
        let Expression::CallExpression(call) = expr else {
            return false;
        };
        let Expression::StaticMemberExpression(member) = &call.callee else {
            return false;
        };
        let Expression::Identifier(obj) = &member.object else {
            return false;
        };
        match (obj.name.as_str(), member.property.name.as_str()) {
            ("$state", "eager") => {
                if let Expression::CallExpression(call) =
                    std::mem::replace(expr, self.make_eager_pending())
                {
                    let mut call = call.unbox();
                    if !call.arguments.is_empty() {
                        let arg = call.arguments.remove(0).into_expression();
                        *expr = self.make_eager_thunk(arg);
                    }
                }
                true
            }
            ("$state", "snapshot") => {
                let Expression::CallExpression(call) = expr else {
                    unreachable!()
                };
                call.callee = self.make_dollar_member("snapshot");
                if dev_snapshot_uncloneable_ignored {
                    call.arguments.push(oxc_ast::ast::Argument::from(
                        self.b.ast.expression_boolean_literal(oxc_span::SPAN, true),
                    ));
                }
                true
            }
            ("$effect", "pending") => {
                *expr = self.make_eager_pending();
                true
            }
            _ => false,
        }
    }

    pub(crate) fn rewrite_rest_prop_member(&self, expr: &mut Expression<'a>, is_lhs: bool) -> bool {
        if is_lhs {
            return false;
        }
        let Some(analysis) = self.analysis else {
            return false;
        };
        let Expression::StaticMemberExpression(member) = expr else {
            return false;
        };
        let Expression::Identifier(id) = &member.object else {
            return false;
        };
        let Some(ref_id) = id.reference_id.get() else {
            return false;
        };
        if !matches!(
            analysis.reference_semantics(ref_id),
            ReferenceSemantics::RestPropMemberRewrite
        ) {
            return false;
        }
        member.object = self
            .b
            .ast
            .expression_identifier(oxc_span::SPAN, self.b.ast.atom("$$props"));
        true
    }
}
