use svelte_emit_builders::legacy_wrap;
use svelte_emit_builders::binding::{LegacyStateSafety, read_binding};
use svelte_emit_builders::runes::rune_get;
use oxc_ast::ast::Expression;
use svelte_analyze::scope::SymbolId;
use svelte_analyze::{Evaluation, KnownValue};
use svelte_ast::{ExprRef, Node, NodeId};

use crate::context::Ctx;
use super::{Codegen, CodegenError, Result};

pub(crate) fn evaluation_is_defined(eval: &Evaluation) -> bool {
    match eval {
        Evaluation::Known(KnownValue::Null | KnownValue::Undefined) => false,
        Evaluation::Known(_) | Evaluation::Defined { .. } => true,
        Evaluation::MaybeNullish { .. } => false,
    }
}

fn expr_ref_for_node(node: &Node) -> Option<&ExprRef> {
    match node {
        Node::ExpressionTag(t) => Some(&t.expression),
        Node::HtmlTag(t) => Some(&t.expression),
        Node::RenderTag(t) => Some(&t.expression),
        Node::IfBlock(b) => Some(&b.test),
        Node::EachBlock(b) => Some(&b.expression),
        Node::KeyBlock(b) => Some(&b.expression),
        Node::AwaitBlock(b) => Some(&b.expression),
        Node::SvelteElement(el) => el.this_expr(),
        _ => None,
    }
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn take_node_expr(&mut self, id: NodeId) -> Result<Expression<'a>> {
        let node = self.ctx.query.component.store.get(id);
        let Some(expr_ref) = expr_ref_for_node(node) else {
            return CodegenError::missing_expression(id);
        };
        let expr = match self.ctx.state.parsed.take_expr(expr_ref.id()) {
            Some(expr) => expr,
            None => return CodegenError::missing_expression(id),
        };
        Ok(self.maybe_wrap_legacy_slots_read(expr))
    }

    pub(super) fn take_attr_expr(
        &mut self,
        attr_id: NodeId,
        expr_ref: &ExprRef,
    ) -> Result<Expression<'a>> {
        let expr = match self.ctx.state.parsed.take_expr(expr_ref.id()) {
            Some(expr) => expr,
            None => return CodegenError::missing_expression(attr_id),
        };
        Ok(self.maybe_wrap_legacy_slots_read(expr))
    }

    pub(in crate::codegen) fn maybe_wrap_legacy_slots_read(
        &self,
        expr: Expression<'a>,
    ) -> Expression<'a> {
        if !self.ctx.query.needs_sanitized_legacy_slots() {
            return expr;
        }
        if !expr_roots_in_legacy_slots(&expr) {
            return expr;
        }
        use svelte_ast_builder::Arg;
        self.ctx
            .b
            .call_expr("$.untrack", [Arg::Expr(self.ctx.b.thunk(expr))])
    }

    pub(super) fn take_expr_by_ref(&mut self, expr_ref: &ExprRef) -> Option<Expression<'a>> {
        self.ctx.state.parsed.take_expr(expr_ref.id())
    }
}

pub(in crate::codegen) fn coarse_wrap<'a>(
    ctx: &Ctx<'a>,
    expr: Expression<'a>,
    data: Option<&svelte_analyze::ExpressionData>,
) -> Expression<'a> {
    legacy_wrap::maybe(&ctx.b, expr, data, |sym| legacy_dep_expr(ctx, sym))
}

pub(in crate::codegen) fn legacy_dep_expr<'a>(
    ctx: &Ctx<'a>,
    sym: SymbolId,
) -> Option<Expression<'a>> {
    use svelte_ast_builder::Arg;
    let getter = build_reactive_dep_expr_legacy(ctx, sym)?;
    Some(if uses_deep_read_state(ctx, sym) {
        ctx.b.call_expr("$.deep_read_state", [Arg::Expr(getter)])
    } else {
        getter
    })
}

fn uses_deep_read_state(ctx: &Ctx<'_>, sym: SymbolId) -> bool {
    use svelte_analyze::{
        BindingSemantics, ConstBindingSemantics, ContextualBindingSemantics, PropBindingKind,
        PropBindingSemantics,
    };
    let decl = ctx.query.view.binding_semantics(sym);
    matches!(
        decl,
        BindingSemantics::Prop(PropBindingSemantics {
            kind: PropBindingKind::NonSource | PropBindingKind::Rest,
            ..
        }) | BindingSemantics::LegacyBindableProp(_)
            | BindingSemantics::Contextual(
                ContextualBindingSemantics::LetDirective
                    | ContextualBindingSemantics::LetDirectiveCarrierMember { .. }
                    | ContextualBindingSemantics::AwaitValue
                    | ContextualBindingSemantics::AwaitError
            )
            | BindingSemantics::Const(ConstBindingSemantics::ConstTag { .. })
    ) || ctx.query.scoping().is_import(sym)
}

pub(in crate::codegen) fn build_reactive_dep_expr_legacy<'a>(
    ctx: &Ctx<'a>,
    sym: SymbolId,
) -> Option<Expression<'a>> {
    use svelte_analyze::{BindingSemantics, ConstBindingSemantics};
    if let BindingSemantics::Const(ConstBindingSemantics::ConstTag {
        destructured: true,
        owner_node,
        ..
    }) = ctx.query.view.binding_semantics(sym)
    {
        let tmp = ctx.transform_data.const_tag_tmp_names.get(&owner_node)?;
        let tmp_ref: &str = ctx.b.alloc_str(tmp);
        let field = ctx.query.symbol_name(sym);
        return Some(ctx.b.static_member_expr(
            rune_get(&ctx.b, tmp_ref),
            field,
        ));
    }
    if matches!(
        ctx.query.view.binding_semantics(sym),
        BindingSemantics::State(_)
            | BindingSemantics::Derived(_)
            | BindingSemantics::OptimizedRune(_)
    ) {
        return None;
    }
    read_binding(
        &ctx.b,
        ctx.query.analysis,
        sym,
        LegacyStateSafety::FromVarDeclared,
    )
}

fn expr_roots_in_legacy_slots(expr: &Expression<'_>) -> bool {
    match expr.get_inner_expression() {
        Expression::Identifier(ident) => ident.name.as_str() == "$$slots",
        Expression::StaticMemberExpression(member) => expr_roots_in_legacy_slots(&member.object),
        Expression::ComputedMemberExpression(member) => expr_roots_in_legacy_slots(&member.object),
        _ => false,
    }
}
