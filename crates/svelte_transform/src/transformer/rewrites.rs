use std::iter;
use std::mem;

use oxc_ast::ast::{
    Argument, AssignmentOperator, AssignmentTarget, Expression, IdentifierReference,
    SimpleAssignmentTarget, UpdateOperator,
};
use oxc_span::SPAN;
use oxc_traverse::TraverseCtx;
use svelte_ast_builder::Arg;

use svelte_analyze::reactivity_semantics::legacy_reactive::legacy_reactive_import_wrapper_name;
use svelte_analyze::{
    AnalysisData, BindingSemantics, CarrierMemberReadSemantics, ContextualReadKind,
    ContextualReadSemantics, PropReferenceSemantics, ReferenceSemantics, StateKind,
};
use svelte_component_semantics::SymbolId;

use svelte_emit_builders::store::{
    build_store_base_read, make_store_mutate, make_store_set, make_store_update,
};

use super::model::ComponentTransformer;
use crate::rune_refs;

fn store_base_symbol(analysis: &AnalysisData<'_>, store_sym: SymbolId) -> SymbolId {
    match analysis.binding_semantics(store_sym) {
        BindingSemantics::Store(facts) => facts.base_symbol,
        BindingSemantics::NonReactive
        | BindingSemantics::MaybeReactive
        | BindingSemantics::State(_)
        | BindingSemantics::Derived(_)
        | BindingSemantics::OptimizedDerived(_)
        | BindingSemantics::OptimizedRune(_)
        | BindingSemantics::Prop(_)
        | BindingSemantics::LegacyBindableProp(_)
        | BindingSemantics::LegacyState(_)
        | BindingSemantics::Const(_)
        | BindingSemantics::Contextual(_)
        | BindingSemantics::RuntimeRune { .. }
        | BindingSemantics::Unresolved
        | BindingSemantics::LegacyApiExport => unreachable!(
            "store_sym from ReferenceSemantics::Store* must classify as BindingSemantics::Store (set together by analyze store builder)"
        ),
    }
}

impl<'a> ComponentTransformer<'_, 'a> {
    pub(crate) fn identifier_is_store_read(&self, ident: &IdentifierReference<'a>) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let Some(ref_id) = ident.reference_id.get() else {
            return false;
        };
        matches!(
            analysis.reference_semantics(ref_id),
            ReferenceSemantics::StoreRead { .. }
        )
    }

    pub(crate) fn dispatch_identifier_read(&self, expr: &mut Expression<'a>) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let (name, ref_id) = {
            let Expression::Identifier(id) = &*expr else {
                return false;
            };
            let Some(ref_id) = id.reference_id.get() else {
                return false;
            };
            (id.name, ref_id)
        };
        let sem = analysis.reference_semantics(ref_id);

        match sem {
            ReferenceSemantics::StoreRead { .. } => {
                *expr = self.make_thunk_call(name.as_str());
                true
            }
            ReferenceSemantics::SignalRead { safe: false, .. }
            | ReferenceSemantics::SignalWrite { .. }
            | ReferenceSemantics::SignalUpdate { safe: false, .. }
            | ReferenceSemantics::LegacyStateWrite
            | ReferenceSemantics::LegacyStateUpdate { safe: false }
            | ReferenceSemantics::LegacyStateSubscribedWrite { .. }
            | ReferenceSemantics::LegacyStateSubscribedUpdate { safe: false, .. } => {
                *expr = self.make_rune_get(name.as_str());
                true
            }
            ReferenceSemantics::SignalRead { safe: true, .. }
            | ReferenceSemantics::SignalUpdate { safe: true, .. }
            | ReferenceSemantics::LegacyStateUpdate { safe: true }
            | ReferenceSemantics::LegacyStateSubscribedUpdate { safe: true, .. } => {
                *expr = self.make_rune_safe_get(name.as_str());
                true
            }
            ReferenceSemantics::PropRead(PropReferenceSemantics::Source { .. }) => {
                *expr = self.make_thunk_call(name.as_str());
                true
            }
            ReferenceSemantics::PropSourceMemberMutationRoot { bindable: true, .. }
                if !self.in_bind_setter_traverse =>
            {
                *expr = self.make_thunk_call(name.as_str());
                true
            }
            ReferenceSemantics::EachItemMemberMutationStoreInvalidate { .. }
            | ReferenceSemantics::LegacyEachItemMemberMutationRoot { .. }
                if !self.in_bind_setter_traverse =>
            {
                *expr = self.make_rune_get(name.as_str());
                true
            }
            ReferenceSemantics::PropRead(PropReferenceSemantics::NonSourceStatic { symbol }) => {
                let (prop_name, _origin_kind) =
                    analysis.binding_origin_key(symbol).unwrap_or_else(|| {
                        panic!(
                            "NonSourceStatic prop read missing binding origin key for ref {:?}",
                            ref_id
                        )
                    });
                *expr = self.make_props_access(prop_name.as_ref());
                true
            }
            ReferenceSemantics::PropRead(PropReferenceSemantics::NonSourceComputed { symbol }) => {
                let (prop_name, _origin_kind) =
                    analysis.binding_origin_key(symbol).unwrap_or_else(|| {
                        panic!(
                            "NonSourceComputed prop read missing binding origin key for ref {:?}",
                            ref_id
                        )
                    });
                *expr = self.make_props_computed_access(prop_name.as_ref());
                true
            }
            ReferenceSemantics::ConstAliasRead { owner_node } => {
                if let Some(tmp) = self.transform_data.const_tag_tmp_names.get(&owner_node) {
                    *expr = self.make_member_get(tmp.as_str(), name.as_str());
                }
                true
            }
            ReferenceSemantics::CarrierMemberRead(CarrierMemberReadSemantics {
                carrier_symbol,
                ..
            }) => {
                let carrier_name = analysis.scoping.symbol_name(carrier_symbol);
                *expr = self.make_member_get(carrier_name, name.as_str());
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
                *expr = self.make_rune_get(name.as_str());
                true
            }
            ReferenceSemantics::LegacyStateRead { safe: true } => {
                *expr = self.make_rune_safe_get(name.as_str());
                true
            }
            ReferenceSemantics::LegacyStateMemberMutationRoot { symbol } => {
                let safe = matches!(
                    analysis.binding_semantics(symbol),
                    BindingSemantics::LegacyState(s) if s.var_declared
                );
                *expr = if safe {
                    self.make_rune_safe_get(name.as_str())
                } else {
                    self.make_rune_get(name.as_str())
                };
                true
            }
            ReferenceSemantics::LegacyStateSubscribedRead { safe, .. } => {
                *expr = if safe {
                    self.make_rune_safe_get(name.as_str())
                } else {
                    self.make_rune_get(name.as_str())
                };
                true
            }
            ReferenceSemantics::LegacyReactiveImportRead => {
                let import_name: &str = self
                    .b
                    .alloc_str(&legacy_reactive_import_wrapper_name(name.as_str()));
                *expr = self.b.call_expr_callee(self.b.rid_expr(import_name), []);
                true
            }
            ReferenceSemantics::ContextualRead(ContextualReadSemantics {
                kind,
                in_key_expression,
                ..
            }) => {
                if in_key_expression {
                    return true;
                }
                match kind {
                    ContextualReadKind::EachItem { accessor: true, .. }
                    | ContextualReadKind::SnippetParam { accessor: true, .. } => {
                        *expr = self.make_thunk_call(name.as_str());
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
                        *expr = self.make_rune_get(name.as_str());
                    }
                    ContextualReadKind::EachItem {
                        accessor: false,
                        signal: false,
                    }
                    | ContextualReadKind::EachIndex { signal: false }
                    | ContextualReadKind::SnippetParam {
                        accessor: false,
                        signal: false,
                    }
                    | ContextualReadKind::LetDirectiveDirect => {}
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
            | ReferenceSemantics::LegacyReactiveImportMemberMutationRoot { .. }
            | ReferenceSemantics::LegacyEachItemMemberMutationRoot { .. }
            | ReferenceSemantics::EachItemMemberMutationStoreInvalidate { .. }
            | ReferenceSemantics::ImportSubscribedRead { .. }
            | ReferenceSemantics::DerivedWrite
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
        let AssignmentTarget::AssignmentTargetIdentifier(id) = &assign.left else {
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
            ReferenceSemantics::DerivedWrite => self.rewrite_derived_identifier_assignment(node),
            ReferenceSemantics::LegacyStateSubscribedWrite { store_symbol }
            | ReferenceSemantics::LegacyStateSubscribedUpdate { store_symbol, .. } => {
                if !self.rewrite_signal_or_store_identifier_assignment(node, suppress_proxy) {
                    return false;
                }
                let dollar_name = analysis.scoping.symbol_name(store_symbol).to_string();
                let inner = self.b.move_expr(node);
                *node = self.make_store_unsub(inner, &dollar_name);
                true
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
            | ReferenceSemantics::LegacyStateSubscribedRead { .. }
            | ReferenceSemantics::LegacyStateMemberMutationRoot { .. }
            | ReferenceSemantics::LegacyReactiveImportRead
            | ReferenceSemantics::ImportSubscribedRead { .. }
            | ReferenceSemantics::LegacyReactiveImportMemberMutationRoot { .. }
            | ReferenceSemantics::LegacyEachItemMemberMutationRoot { .. }
            | ReferenceSemantics::EachItemMemberMutationStoreInvalidate { .. }
            | ReferenceSemantics::IllegalWrite
            | ReferenceSemantics::Unresolved => false,
        }
    }

    fn rewrite_derived_identifier_assignment(&self, node: &mut Expression<'a>) -> bool {
        let (name, operator) = {
            let Expression::AssignmentExpression(assign) = &*node else {
                return false;
            };
            let AssignmentTarget::AssignmentTargetIdentifier(id) = &assign.left else {
                return false;
            };
            (id.name, assign.operator)
        };
        if !matches!(operator, AssignmentOperator::Assign) {
            return false;
        }
        let Expression::AssignmentExpression(assign) = &mut *node else {
            unreachable!()
        };
        let right = mem::replace(&mut assign.right, self.make_rune_get(""));
        *node = self.make_rune_set(name.as_str(), right, false);
        true
    }

    pub(crate) fn dispatch_identifier_update(&self, node: &mut Expression<'a>) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let Expression::UpdateExpression(upd) = node else {
            return false;
        };
        let SimpleAssignmentTarget::AssignmentTargetIdentifier(id) = &upd.argument else {
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
            ReferenceSemantics::LegacyStateSubscribedUpdate { store_symbol, .. } => {
                if !self.rewrite_signal_or_store_identifier_update(node) {
                    return false;
                }
                let dollar_name = analysis.scoping.symbol_name(store_symbol).to_string();
                let inner = self.b.move_expr(node);
                *node = self.make_store_unsub(inner, &dollar_name);
                true
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
            | ReferenceSemantics::LegacyStateSubscribedRead { .. }
            | ReferenceSemantics::LegacyStateSubscribedWrite { .. }
            | ReferenceSemantics::LegacyStateMemberMutationRoot { .. }
            | ReferenceSemantics::LegacyReactiveImportRead
            | ReferenceSemantics::ImportSubscribedRead { .. }
            | ReferenceSemantics::LegacyReactiveImportMemberMutationRoot { .. }
            | ReferenceSemantics::LegacyEachItemMemberMutationRoot { .. }
            | ReferenceSemantics::EachItemMemberMutationStoreInvalidate { .. }
            | ReferenceSemantics::DerivedWrite
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
                self.rewrite_prop_member_assignment(node, is_expr_stmt, ctx)
            }
            ReferenceSemantics::LegacyEachItemMemberMutationRoot { item_sym } => {
                self.rewrite_legacy_each_item_member_assignment(node, item_sym, ctx)
            }
            ReferenceSemantics::EachItemMemberMutationStoreInvalidate {
                collection_store, ..
            } => self.rewrite_each_item_member_store_invalidate_assignment(node, collection_store),
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
            | ReferenceSemantics::LegacyStateSubscribedRead { .. }
            | ReferenceSemantics::LegacyStateSubscribedWrite { .. }
            | ReferenceSemantics::LegacyStateSubscribedUpdate { .. }
            | ReferenceSemantics::LegacyReactiveImportRead
            | ReferenceSemantics::ImportSubscribedRead { .. }
            | ReferenceSemantics::DerivedWrite
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
            ReferenceSemantics::EachItemMemberMutationStoreInvalidate {
                collection_store, ..
            } => self.rewrite_each_item_member_store_invalidate_update(node, collection_store),
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
            | ReferenceSemantics::LegacyStateSubscribedRead { .. }
            | ReferenceSemantics::LegacyStateSubscribedWrite { .. }
            | ReferenceSemantics::LegacyStateSubscribedUpdate { .. }
            | ReferenceSemantics::LegacyReactiveImportRead
            | ReferenceSemantics::ImportSubscribedRead { .. }
            | ReferenceSemantics::DerivedWrite
            | ReferenceSemantics::IllegalWrite
            | ReferenceSemantics::Unresolved => false,
        }
    }

    fn rewrite_each_item_member_store_invalidate_assignment(
        &self,
        node: &mut Expression<'a>,
        collection_store: SymbolId,
    ) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let item_name = {
            let Expression::AssignmentExpression(assign) = &*node else {
                return false;
            };
            let Some(member) = assign.left.as_member_expression() else {
                return false;
            };
            let Some(root) = rune_refs::find_expr_root_identifier(member.object()) else {
                return false;
            };
            root.name
        };
        let Expression::AssignmentExpression(assign) = &mut *node else {
            unreachable!()
        };
        rune_refs::replace_expr_root_in_assign_target(
            &mut assign.left,
            self.make_rune_get(item_name.as_str()),
        );
        let dollar_name = analysis.scoping.symbol_name(collection_store).to_string();
        let placeholder = self.make_rune_get("");
        let mutation = mem::replace(node, placeholder);
        *node = self.make_invalidate_store_seq(mutation, &dollar_name);
        true
    }

    fn rewrite_each_item_member_store_invalidate_update(
        &self,
        node: &mut Expression<'a>,
        collection_store: SymbolId,
    ) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let item_name = {
            let Expression::UpdateExpression(upd) = &*node else {
                return false;
            };
            let Some(member) = upd.argument.as_member_expression() else {
                return false;
            };
            let Some(root) = rune_refs::find_expr_root_identifier(member.object()) else {
                return false;
            };
            root.name
        };
        let Expression::UpdateExpression(upd) = &mut *node else {
            unreachable!()
        };
        rune_refs::replace_expr_root_in_simple_target(
            &mut upd.argument,
            self.make_rune_get(item_name.as_str()),
        );
        let dollar_name = analysis.scoping.symbol_name(collection_store).to_string();
        let placeholder = self.make_rune_get("");
        let mutation = mem::replace(node, placeholder);
        *node = self.make_invalidate_store_seq(mutation, &dollar_name);
        true
    }

    pub(crate) fn rewrite_signal_or_store_identifier_assignment(
        &self,
        node: &mut Expression<'a>,
        suppress_proxy: bool,
    ) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let (name, ref_id, operator) = {
            let Expression::AssignmentExpression(assign) = &*node else {
                return false;
            };
            let AssignmentTarget::AssignmentTargetIdentifier(id) = &assign.left else {
                return false;
            };
            let Some(ref_id) = id.reference_id.get() else {
                return false;
            };
            (id.name, ref_id, assign.operator)
        };
        let semantics = analysis.reference_semantics(ref_id);

        match semantics {
            ReferenceSemantics::StoreWrite { symbol }
            | ReferenceSemantics::StoreUpdate { symbol } => {
                let base_sym = store_base_symbol(analysis, symbol);
                let base_expr = build_store_base_read(self.b, analysis, base_sym);
                let Expression::AssignmentExpression(assign) = &mut *node else {
                    unreachable!()
                };
                let right = mem::replace(&mut assign.right, self.make_rune_get(""));

                let left_read = self.make_thunk_call(name.as_str());
                let value = self.build_compound_value(operator, left_read, right);
                *node = make_store_set(self.b, base_expr, value);
                true
            }
            ReferenceSemantics::SignalWrite { kind } => {
                let Expression::AssignmentExpression(assign) = &mut *node else {
                    unreachable!()
                };
                let right = mem::replace(&mut assign.right, self.make_rune_get(""));
                let needs_proxy = !suppress_proxy
                    && kind == StateKind::State
                    && rune_refs::is_non_coercive_operator(operator)
                    && rune_refs::should_proxy(&right);
                let left_read = self.make_rune_get(name.as_str());
                let value = self.build_compound_value(operator, left_read, right);
                *node = self.make_rune_set(name.as_str(), value, needs_proxy);
                true
            }
            ReferenceSemantics::SignalUpdate { kind, safe } => {
                let Expression::AssignmentExpression(assign) = &mut *node else {
                    unreachable!()
                };
                let right = mem::replace(&mut assign.right, self.make_rune_get(""));
                let needs_proxy = !suppress_proxy
                    && kind == StateKind::State
                    && rune_refs::is_non_coercive_operator(operator)
                    && rune_refs::should_proxy(&right);
                let left_read = if safe {
                    self.make_rune_safe_get(name.as_str())
                } else {
                    self.make_rune_get(name.as_str())
                };
                let value = self.build_compound_value(operator, left_read, right);
                *node = self.make_rune_set(name.as_str(), value, needs_proxy);
                true
            }

            ReferenceSemantics::LegacyStateWrite => {
                let Expression::AssignmentExpression(assign) = &mut *node else {
                    unreachable!()
                };
                let right = mem::replace(&mut assign.right, self.make_rune_get(""));
                let left_read = self.make_rune_get(name.as_str());
                let value = self.build_compound_value(operator, left_read, right);
                *node = self.make_rune_set(name.as_str(), value, false);
                true
            }

            ReferenceSemantics::LegacyStateUpdate { safe } => {
                let Expression::AssignmentExpression(assign) = &mut *node else {
                    unreachable!()
                };
                let right = mem::replace(&mut assign.right, self.make_rune_get(""));
                let left_read = if safe {
                    self.make_rune_safe_get(name.as_str())
                } else {
                    self.make_rune_get(name.as_str())
                };
                let value = self.build_compound_value(operator, left_read, right);
                *node = self.make_rune_set(name.as_str(), value, false);
                true
            }
            ReferenceSemantics::LegacyStateSubscribedWrite { .. } => {
                let Expression::AssignmentExpression(assign) = &mut *node else {
                    unreachable!()
                };
                let right = mem::replace(&mut assign.right, self.make_rune_get(""));
                let left_read = self.make_rune_get(name.as_str());
                let value = self.build_compound_value(operator, left_read, right);
                *node = self.make_rune_set(name.as_str(), value, false);
                true
            }
            ReferenceSemantics::LegacyStateSubscribedUpdate { safe, .. } => {
                let Expression::AssignmentExpression(assign) = &mut *node else {
                    unreachable!()
                };
                let right = mem::replace(&mut assign.right, self.make_rune_get(""));
                let left_read = if safe {
                    self.make_rune_safe_get(name.as_str())
                } else {
                    self.make_rune_get(name.as_str())
                };
                let value = self.build_compound_value(operator, left_read, right);
                *node = self.make_rune_set(name.as_str(), value, false);
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
        let (name, ref_id, is_increment, is_prefix) = {
            let Expression::UpdateExpression(upd) = &*node else {
                return false;
            };
            let SimpleAssignmentTarget::AssignmentTargetIdentifier(id) = &upd.argument else {
                return false;
            };
            let Some(ref_id) = id.reference_id.get() else {
                return false;
            };
            (
                id.name,
                ref_id,
                upd.operator == UpdateOperator::Increment,
                upd.prefix,
            )
        };

        match analysis.reference_semantics(ref_id) {
            ReferenceSemantics::StoreUpdate { symbol } => {
                let base_sym = store_base_symbol(analysis, symbol);
                let base_name = analysis.scoping.symbol_name(base_sym);
                *node =
                    make_store_update(self.b, base_name, name.as_str(), is_prefix, is_increment);
                true
            }
            ReferenceSemantics::SignalUpdate {
                kind: StateKind::State | StateKind::StateRaw,
                ..
            } => {
                *node = self.make_rune_update(name.as_str(), is_prefix, is_increment);
                true
            }

            ReferenceSemantics::LegacyStateUpdate { .. } => {
                *node = self.make_rune_update(name.as_str(), is_prefix, is_increment);
                true
            }
            ReferenceSemantics::LegacyStateSubscribedUpdate { .. } => {
                *node = self.make_rune_update(name.as_str(), is_prefix, is_increment);
                true
            }
            _ => false,
        }
    }

    pub(crate) fn rewrite_prop_identifier_assignment(&self, node: &mut Expression<'a>) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let (name, ref_id, operator) = {
            let Expression::AssignmentExpression(assign) = &*node else {
                return false;
            };
            let AssignmentTarget::AssignmentTargetIdentifier(id) = &assign.left else {
                return false;
            };
            let Some(ref_id) = id.reference_id.get() else {
                return false;
            };
            (id.name, ref_id, assign.operator)
        };
        if !matches!(
            analysis.reference_semantics(ref_id),
            ReferenceSemantics::PropMutation { .. }
        ) {
            return false;
        }
        let Expression::AssignmentExpression(assign) = &mut *node else {
            unreachable!()
        };
        let right = self.b.move_expr(&mut assign.right);
        let value = if operator.is_assign() {
            right
        } else {
            let left_read = self
                .b
                .call_expr(name.as_str(), iter::empty::<Arg<'a, '_>>());
            self.build_compound_value(operator, left_read, right)
        };
        *node = self.b.call_expr(name.as_str(), [Arg::Expr(value)]);
        true
    }

    pub(crate) fn rewrite_prop_identifier_update(&self, node: &mut Expression<'a>) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let (name, ref_id, is_prefix, is_decrement) = {
            let Expression::UpdateExpression(upd) = &*node else {
                return false;
            };
            let SimpleAssignmentTarget::AssignmentTargetIdentifier(id) = &upd.argument else {
                return false;
            };
            let Some(ref_id) = id.reference_id.get() else {
                return false;
            };
            (
                id.name,
                ref_id,
                upd.prefix,
                upd.operator == UpdateOperator::Decrement,
            )
        };
        if !matches!(
            analysis.reference_semantics(ref_id),
            ReferenceSemantics::PropMutation { .. }
        ) {
            return false;
        }
        let fn_name = if is_prefix {
            "$.update_pre_prop"
        } else {
            "$.update_prop"
        };
        let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident(name.as_str())];
        if is_decrement {
            args.push(Arg::Num(-1.0));
        }
        *node = self.b.call_expr(fn_name, args);
        true
    }

    pub(crate) fn rewrite_deep_store_member_assignment(&self, node: &mut Expression<'a>) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let (root_name, ref_id) = {
            let Expression::AssignmentExpression(assign) = &*node else {
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
            (root.name, ref_id)
        };
        let ReferenceSemantics::StoreRead { symbol } = analysis.reference_semantics(ref_id) else {
            return false;
        };
        let base_sym = store_base_symbol(analysis, symbol);
        let base = build_store_base_read(self.b, analysis, base_sym);
        let Expression::AssignmentExpression(assign) = &mut *node else {
            unreachable!()
        };
        rune_refs::replace_expr_root_in_assign_target(
            &mut assign.left,
            self.make_untrack(root_name.as_str()),
        );
        let placeholder = self.make_rune_get("");
        let mutation = mem::replace(node, placeholder);
        let untracked = self.make_untrack(root_name.as_str());
        *node = make_store_mutate(self.b, base, mutation, untracked);
        true
    }

    pub(crate) fn rewrite_legacy_reactive_import_member_assignment(
        &self,
        node: &mut Expression<'a>,
    ) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let (root_name, ref_id) = {
            let Expression::AssignmentExpression(assign) = &*node else {
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
            (root.name, ref_id)
        };
        if !matches!(
            analysis.reference_semantics(ref_id),
            ReferenceSemantics::LegacyReactiveImportMemberMutationRoot { .. }
        ) {
            return false;
        }
        let import_name: &'a str = self
            .b
            .alloc_str(&legacy_reactive_import_wrapper_name(root_name.as_str()));
        let import_call = self.b.call_expr_callee(self.b.rid_expr(import_name), []);
        let Expression::AssignmentExpression(assign) = &mut *node else {
            unreachable!()
        };
        rune_refs::replace_expr_root_in_assign_target(&mut assign.left, import_call);
        let placeholder = self.b.cheap_expr();
        let mutation = mem::replace(node, placeholder);
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
        let (root_name, ref_id) = {
            let Expression::UpdateExpression(upd) = &*node else {
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
            (root.name, ref_id)
        };
        if !matches!(
            analysis.reference_semantics(ref_id),
            ReferenceSemantics::LegacyReactiveImportMemberMutationRoot { .. }
        ) {
            return false;
        }
        let import_name: &'a str = self
            .b
            .alloc_str(&legacy_reactive_import_wrapper_name(root_name.as_str()));
        let import_call = self.b.call_expr_callee(self.b.rid_expr(import_name), []);
        let Expression::UpdateExpression(upd) = &mut *node else {
            unreachable!()
        };
        rune_refs::replace_expr_root_in_simple_target(&mut upd.argument, import_call);
        let placeholder = self.b.cheap_expr();
        let mutation = mem::replace(node, placeholder);
        *node = self.b.call_expr(import_name, [Arg::Expr(mutation)]);
        true
    }

    pub(crate) fn rewrite_legacy_state_member_assignment(&self, node: &mut Expression<'a>) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let (root_name, ref_id) = {
            let Expression::AssignmentExpression(assign) = &*node else {
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
            (root.name, ref_id)
        };
        if !matches!(
            analysis.reference_semantics(ref_id),
            ReferenceSemantics::LegacyStateMemberMutationRoot { .. }
        ) {
            return false;
        }
        let Expression::AssignmentExpression(assign) = &mut *node else {
            unreachable!()
        };
        rune_refs::replace_expr_root_in_assign_target(
            &mut assign.left,
            self.make_rune_get(root_name.as_str()),
        );
        let placeholder = self.make_rune_get("");
        let mutation = mem::replace(node, placeholder);
        *node = self.make_legacy_state_mutate(root_name.as_str(), mutation);
        true
    }

    pub(crate) fn rewrite_legacy_state_member_update(&self, node: &mut Expression<'a>) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let (root_name, ref_id) = {
            let Expression::UpdateExpression(upd) = &*node else {
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
            (root.name, ref_id)
        };
        if !matches!(
            analysis.reference_semantics(ref_id),
            ReferenceSemantics::LegacyStateMemberMutationRoot { .. }
        ) {
            return false;
        }
        let Expression::UpdateExpression(upd) = &mut *node else {
            unreachable!()
        };
        rune_refs::replace_expr_root_in_simple_target(
            &mut upd.argument,
            self.make_rune_get(root_name.as_str()),
        );
        let placeholder = self.make_rune_get("");
        let mutation = mem::replace(node, placeholder);
        *node = self.make_legacy_state_mutate(root_name.as_str(), mutation);
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
        let item_name = {
            let Expression::AssignmentExpression(assign) = &*node else {
                return false;
            };
            let Some(member) = assign.left.as_member_expression() else {
                return false;
            };
            let Some(root) = rune_refs::find_expr_root_identifier(member.object()) else {
                return false;
            };
            root.name
        };
        let Some(source_syms) = analysis.each_item_indirect_sources(item_sym) else {
            return false;
        };
        if source_syms.is_empty() {
            return false;
        }
        let Expression::AssignmentExpression(assign) = &mut *node else {
            unreachable!()
        };
        rune_refs::replace_expr_root_in_assign_target(
            &mut assign.left,
            self.make_rune_get(item_name.as_str()),
        );
        let placeholder = self.make_rune_get("");
        let mutation = mem::replace(node, placeholder);
        *node = self.make_each_item_invalidate_seq(analysis, mutation, source_syms, ctx);
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
        let item_name = {
            let Expression::UpdateExpression(upd) = &*node else {
                return false;
            };
            let Some(member) = upd.argument.as_member_expression() else {
                return false;
            };
            let Some(root) = rune_refs::find_expr_root_identifier(member.object()) else {
                return false;
            };
            root.name
        };
        let Some(source_syms) = analysis.each_item_indirect_sources(item_sym) else {
            return false;
        };
        if source_syms.is_empty() {
            return false;
        }
        let Expression::UpdateExpression(upd) = &mut *node else {
            unreachable!()
        };
        rune_refs::replace_expr_root_in_simple_target(
            &mut upd.argument,
            self.make_rune_get(item_name.as_str()),
        );
        let placeholder = self.make_rune_get("");
        let mutation = mem::replace(node, placeholder);
        *node = self.make_each_item_invalidate_seq(analysis, mutation, source_syms, ctx);
        true
    }

    pub(crate) fn rewrite_deep_store_member_update(&self, node: &mut Expression<'a>) -> bool {
        let Some(analysis) = self.analysis else {
            return false;
        };
        let (root_name, ref_id) = {
            let Expression::UpdateExpression(upd) = &*node else {
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
            (root.name, ref_id)
        };
        let ReferenceSemantics::StoreRead { symbol } = analysis.reference_semantics(ref_id) else {
            return false;
        };
        let base_sym = store_base_symbol(analysis, symbol);
        let base = build_store_base_read(self.b, analysis, base_sym);
        let Expression::UpdateExpression(upd) = &mut *node else {
            unreachable!()
        };
        rune_refs::replace_expr_root_in_simple_target(
            &mut upd.argument,
            self.make_untrack(root_name.as_str()),
        );
        let placeholder = self.make_rune_get("");
        let mutation = mem::replace(node, placeholder);
        let untracked = self.make_untrack(root_name.as_str());
        *node = make_store_mutate(self.b, base, mutation, untracked);
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
        let Expression::StaticMemberExpression(member) = call.callee.get_inner_expression() else {
            return false;
        };
        let Expression::Identifier(obj) = member.object.get_inner_expression() else {
            return false;
        };
        if self.identifier_is_store_read(obj) {
            return false;
        }
        match (obj.name.as_str(), member.property.name.as_str()) {
            ("$state", "eager") => {
                if let Expression::CallExpression(call) =
                    mem::replace(expr, self.make_eager_pending())
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
                    call.arguments.push(Argument::from(
                        self.b.ast.expression_boolean_literal(SPAN, true),
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
        let Expression::Identifier(id) = member.object.get_inner_expression() else {
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
            .expression_identifier(SPAN, self.b.ast.atom("$$props"));
        true
    }
}
