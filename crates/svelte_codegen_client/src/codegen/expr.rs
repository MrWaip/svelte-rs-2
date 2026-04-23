use oxc_ast::ast::Expression;
use svelte_ast::{ExprRef, Node, NodeId};

use super::{Codegen, CodegenError, Result};

/// Resolve the primary `ExprRef` carried by a template node id.
fn expr_ref_for_node(node: &Node) -> Option<&ExprRef> {
    match node {
        Node::ExpressionTag(t) => Some(&t.expression),
        Node::HtmlTag(t) => Some(&t.expression),
        Node::RenderTag(t) => Some(&t.expression),
        Node::IfBlock(b) => Some(&b.test),
        Node::EachBlock(b) => Some(&b.expression),
        Node::KeyBlock(b) => Some(&b.expression),
        Node::AwaitBlock(b) => Some(&b.expression),
        Node::SvelteElement(el) => el.tag.as_ref(),
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

    pub(super) fn maybe_wrap_legacy_coarse_expr(
        &self,
        expr: Expression<'a>,
        info: Option<&svelte_analyze::ExpressionInfo>,
    ) -> Expression<'a> {
        use svelte_ast_builder::Arg;
        let Some(info) = info else { return expr };
        if self.ctx.query.runes() {
            return expr;
        }
        if !info.needs_legacy_coarse_wrap() {
            return expr;
        }
        let mut seq_parts: Vec<Expression<'a>> = Vec::new();
        for &sym in info.ref_symbols() {
            let Some(getter) = build_reactive_dep_expr_legacy(self.ctx, sym) else {
                continue;
            };
            let getter = self
                .ctx
                .b
                .call_expr("$.deep_read_state", [Arg::Expr(getter)]);
            seq_parts.push(getter);
        }
        if seq_parts.is_empty() {
            return expr;
        }
        let mut iter = seq_parts.into_iter();
        let Some(first) = iter.next() else {
            return expr;
        };
        let mut sequence = first;
        for next in iter.chain(std::iter::once(
            self.ctx
                .b
                .call_expr("$.untrack", [Arg::Expr(self.ctx.b.thunk(expr))]),
        )) {
            sequence = self.ctx.b.seq_expr([sequence, next]);
        }
        sequence
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

fn build_reactive_dep_expr_legacy<'a>(
    ctx: &crate::context::Ctx<'a>,
    sym: svelte_analyze::scope::SymbolId,
) -> Option<Expression<'a>> {
    use svelte_analyze::{
        ConstDeclarationSemantics, ContextualDeclarationSemantics as Ck, DeclarationSemantics,
        EachIndexStrategy, EachItemStrategy, PropDeclarationKind, PropDeclarationSemantics,
        SnippetParamStrategy,
    };
    use svelte_ast_builder::Arg;
    let node_id = ctx.query.scoping().symbol_declaration(sym);
    match ctx.query.view.declaration_semantics(node_id) {
        DeclarationSemantics::Prop(PropDeclarationSemantics {
            kind: PropDeclarationKind::NonSource,
            ..
        }) => {
            let prop_name = ctx.query.view.binding_origin_key(sym)?;
            Some(
                ctx.b
                    .static_member_expr(ctx.b.rid_expr("$$props"), prop_name),
            )
        }
        DeclarationSemantics::Prop(PropDeclarationSemantics {
            kind: PropDeclarationKind::Source { .. },
            ..
        }) => Some(ctx.b.call_expr(
            ctx.query.symbol_name(sym),
            std::iter::empty::<Arg<'a, '_>>(),
        )),
        DeclarationSemantics::Prop(PropDeclarationSemantics {
            kind: PropDeclarationKind::Rest,
            ..
        }) => Some(ctx.b.rid_expr(ctx.query.symbol_name(sym))),
        DeclarationSemantics::Const(ConstDeclarationSemantics::ConstTag {
            destructured, ..
        }) => {
            let helper = if destructured { "$.safe_get" } else { "$.get" };
            Some(ctx.b.call_expr(
                helper,
                [Arg::Expr(ctx.b.rid_expr(ctx.query.symbol_name(sym)))],
            ))
        }
        DeclarationSemantics::Contextual(kind) => {
            let name = ctx.query.symbol_name(sym);
            match kind {
                Ck::EachItem(EachItemStrategy::Accessor)
                | Ck::SnippetParam(SnippetParamStrategy::Accessor) => {
                    Some(ctx.b.call_expr(name, std::iter::empty::<Arg<'a, '_>>()))
                }
                Ck::EachItem(EachItemStrategy::Direct)
                | Ck::EachIndex(EachIndexStrategy::Direct) => Some(ctx.b.rid_expr(name)),
                Ck::EachItem(EachItemStrategy::Signal)
                | Ck::EachIndex(EachIndexStrategy::Signal)
                | Ck::SnippetParam(SnippetParamStrategy::Signal)
                | Ck::AwaitValue
                | Ck::AwaitError
                | Ck::LetDirective => {
                    Some(ctx.b.call_expr("$.get", [Arg::Expr(ctx.b.rid_expr(name))]))
                }
            }
        }
        DeclarationSemantics::NonReactive if ctx.query.scoping().is_import(sym) => {
            Some(ctx.b.rid_expr(ctx.query.symbol_name(sym)))
        }
        _ => None,
    }
}

fn expr_roots_in_legacy_slots(expr: &Expression<'_>) -> bool {
    match expr {
        Expression::Identifier(ident) => ident.name.as_str() == "$$slots",
        Expression::StaticMemberExpression(member) => expr_roots_in_legacy_slots(&member.object),
        Expression::ComputedMemberExpression(member) => expr_roots_in_legacy_slots(&member.object),
        _ => false,
    }
}
