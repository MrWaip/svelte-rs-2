use super::super::{
    BlockSemantics, EachAsyncKind, EachBlockSemantics, EachCollection, EachCollectionSource,
    EachFlags, EachFlavor, EachIndexKind, EachItemKind, EachKeyKind, store_root_is_reactive,
};
use super::common::{binding_ident_of, binding_pattern_node_id, declarator_from_stmt};
use super::walker::Ctx;
use crate::expression_semantics::{ExprKind, ExpressionData, ExpressionSemantics};
use crate::reactivity_semantics::data::{
    BindingSemantics, PropBindingKind, PropBindingSemantics, PropReferenceSemantics,
    ReferenceSemantics,
};
use oxc_ast::ast::{
    AwaitExpression, BindingPattern, CallExpression, ChainElement, Expression, IdentifierReference,
    NewExpression,
};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{walk_await_expression, walk_call_expression, walk_new_expression};
use oxc_syntax::scope::ScopeId;
use svelte_ast::{Attribute, EachBlock, Node, NodeId};
use svelte_component_semantics::{ComponentSemantics, OxcNodeId, ReferenceId, SymbolId};

pub(super) fn populate(ctx: &mut Ctx<'_, '_>, block: &EachBlock) {
    let context_declarator = block
        .context
        .as_ref()
        .and_then(|r| ctx.parsed.stmt(r.id()))
        .and_then(declarator_from_stmt);

    let index_declarator = block
        .index
        .as_ref()
        .and_then(|r| ctx.parsed.stmt(r.id()))
        .and_then(declarator_from_stmt);

    let key_expr = block.key.as_ref().and_then(|r| ctx.parsed.expr(r.id()));

    let collection_expr = ctx.parsed.expr(block.expression.id());

    let (item, item_sym) = match context_declarator {
        None => (EachItemKind::NoBinding, None),
        Some(d) => match &d.id {
            BindingPattern::BindingIdentifier(ident) => {
                let sym = ctx
                    .semantics
                    .fragment_scope_by_id(block.body)
                    .and_then(|scope| ctx.semantics.find_binding(scope, ident.name.as_str()));
                match sym {
                    Some(sym) => (EachItemKind::Identifier(sym), Some(sym)),
                    None => (EachItemKind::NoBinding, None),
                }
            }
            _ => (EachItemKind::Pattern(binding_pattern_node_id(&d.id)), None),
        },
    };

    let body_scope = ctx.semantics.fragment_scope_by_id(block.body);

    let index_sym = index_declarator
        .and_then(|d| binding_ident_of(&d.id))
        .and_then(|ident| {
            body_scope.and_then(|scope| ctx.semantics.find_binding(scope, ident.name.as_str()))
        });

    let key = match key_expr {
        None => EachKeyKind::Unkeyed,
        Some(expr) => {
            if index_sym.is_some_and(|sym| expression_is_identifier_of(expr, sym, ctx.semantics)) {
                EachKeyKind::KeyedByIndex
            } else if let Some(sym) = item_sym {
                if expression_is_identifier_of(expr, sym, ctx.semantics) {
                    EachKeyKind::KeyedByItem
                } else {
                    EachKeyKind::KeyedByExpr(expression_node_id(expr))
                }
            } else {
                EachKeyKind::KeyedByExpr(expression_node_id(expr))
            }
        }
    };

    let pattern_fallback = matches!(item, EachItemKind::Pattern(_));
    let introduced =
        ctx.collect_each_introduced_symbols(block, item_sym, pattern_fallback, index_sym);

    let index = match index_sym {
        Some(sym) => {
            ctx.store.record_each_index_sym(sym, block.id);
            let all_refs = ctx.semantics.get_resolved_reference_ids(sym);
            let used_in_key = match key_expr {
                Some(expr) => expression_contains_reference_to(expr, sym, ctx.semantics),
                None => false,
            };
            let used_in_body = if !used_in_key {
                !all_refs.is_empty()
            } else {
                let key_ref_count = key_expr
                    .map(|e| count_references_to_in_expr(e, sym, ctx.semantics))
                    .unwrap_or(0);
                all_refs.len() > key_ref_count
            };
            EachIndexKind::Declared {
                sym,
                used_in_body,
                used_in_key,
            }
        }
        None => EachIndexKind::Absent,
    };

    let body_nodes = ctx.component.fragment_nodes(block.body).to_vec();
    let has_animate = body_has_direct_animate(ctx, &body_nodes);

    let shadows_outer = body_scope
        .and_then(|child| {
            ctx.semantics
                .scope_parent_id(child)
                .map(|parent| (child, parent))
        })
        .is_some_and(|(child, parent)| {
            ctx.semantics
                .own_binding_names(child)
                .filter(|name| !name.starts_with("$$"))
                .any(|name| ctx.semantics.find_binding(parent, name).is_some())
        });

    let collection_facts = match (collection_expr, body_scope) {
        (Some(expr), Some(scope)) => collection_expression_facts(ctx, expr, scope),
        _ => CollectionExprFacts::default(),
    };
    if let Some(e) = collection_expr
        && derive_forces_runtime_context(ctx, e, &collection_facts)
    {
        ctx.store.mark_legacy_each_forces_runtime_context();
    }

    let expression_data = match ctx.expressions.get(block.id) {
        ExpressionSemantics::Expression(d) => Some(d),
        ExpressionSemantics::NonSpecial => None,
    };

    let async_kind = derive_async_kind(expression_data);

    let source = derive_collection_source(ctx, collection_expr, expression_data);

    let has_external = collection_facts.has_external;
    let uses_store = collection_facts.uses_store;
    let collection_store = collection_facts.collection_store;

    let has_key = !matches!(key, EachKeyKind::Unkeyed | EachKeyKind::KeyedByIndex);
    let has_index = matches!(index, EachIndexKind::Declared { .. });
    let key_is_item = matches!(key, EachKeyKind::KeyedByItem);
    let runes = ctx.reactivity.uses_runes();

    let mut each_flags = EachFlags::empty();
    if has_key && has_index {
        each_flags |= EachFlags::INDEX_REACTIVE;
    }
    if has_external && (!runes || !key_is_item || uses_store) {
        each_flags |= EachFlags::ITEM_REACTIVE;
    }
    if runes && !uses_store {
        each_flags |= EachFlags::ITEM_IMMUTABLE;
    }
    if has_key && has_animate {
        each_flags |= EachFlags::ANIMATED;
    }

    ctx.push_each_frame(block.id, introduced);
    ctx.visit_fragment(block.body);
    if let Some(fb) = block.fallback {
        ctx.visit_fragment(fb);
    }
    ctx.pop_each_frame();

    let flavor = if ctx.each_has_group_binding(block.id) {
        EachFlavor::BindGroup
    } else {
        EachFlavor::Regular
    };

    ctx.store.set(
        block.id,
        BlockSemantics::Each(EachBlockSemantics {
            item,
            index,
            key,
            flavor,
            each_flags,
            shadows_outer,
            async_kind,
            collection: EachCollection { source },
            collection_store,
        }),
    );
}

fn derive_async_kind(data: Option<&ExpressionData>) -> EachAsyncKind {
    match data {
        Some(d) => {
            let has_await = matches!(d.kind, ExprKind::Async { has_await: true });
            if has_await || !d.blockers.is_empty() {
                EachAsyncKind::Async {
                    has_await,
                    blockers: d.blockers.clone(),
                }
            } else {
                EachAsyncKind::Sync
            }
        }
        None => EachAsyncKind::Sync,
    }
}

fn derive_collection_source<'a>(
    ctx: &Ctx<'_, 'a>,
    expr: Option<&Expression<'a>>,
    data: Option<&ExpressionData>,
) -> EachCollectionSource {
    let Some(d) = data else {
        return EachCollectionSource::Local;
    };
    if !matches!(d.kind, ExprKind::SimpleRead { .. }) {
        return EachCollectionSource::Local;
    }
    if d.references.len() != 1 {
        return EachCollectionSource::Local;
    }
    let Some(expr) = expr else {
        return EachCollectionSource::Local;
    };
    if !matches!(expr.get_inner_expression(), Expression::Identifier(_)) {
        return EachCollectionSource::Local;
    }
    let sym = d.references[0];
    if !matches!(
        ctx.reactivity.binding_semantics(sym),
        BindingSemantics::Prop(PropBindingSemantics {
            kind: PropBindingKind::Source { .. },
            ..
        }) | BindingSemantics::LegacyBindableProp(_)
    ) {
        return EachCollectionSource::Local;
    }
    EachCollectionSource::Prop { sym }
}

fn derive_forces_runtime_context<'a>(
    ctx: &Ctx<'_, 'a>,
    expr: &Expression<'a>,
    facts: &CollectionExprFacts,
) -> bool {
    if !ctx.reactivity.runes_mode().is_hard_legacy() {
        return false;
    }
    let mut current = expr.get_inner_expression();
    let mut peeled_member = false;
    loop {
        match current {
            Expression::StaticMemberExpression(m) => {
                peeled_member = true;
                current = m.object.get_inner_expression();
            }
            Expression::ComputedMemberExpression(m) => {
                peeled_member = true;
                current = m.object.get_inner_expression();
            }
            Expression::ChainExpression(chain) => match &chain.expression {
                ChainElement::CallExpression(_) => break,
                ChainElement::TSNonNullExpression(_) => unreachable!("TS stripped at parse"),
                ChainElement::StaticMemberExpression(m) => {
                    peeled_member = true;
                    current = m.object.get_inner_expression();
                }
                ChainElement::ComputedMemberExpression(m) => {
                    peeled_member = true;
                    current = m.object.get_inner_expression();
                }
                ChainElement::PrivateFieldExpression(m) => {
                    peeled_member = true;
                    current = m.object.get_inner_expression();
                }
            },
            Expression::Identifier(id) => {
                if !peeled_member || facts.has_call {
                    return false;
                }
                let Some(ref_id) = id.reference_id.get() else {
                    return false;
                };
                let sem = ctx.reactivity.reference_semantics(ref_id);
                if matches!(
                    sem,
                    ReferenceSemantics::PropRead(PropReferenceSemantics::Source { .. })
                ) {
                    return true;
                }
                if let ReferenceSemantics::StoreRead { symbol } = sem {
                    return store_root_is_reactive(ctx.reactivity, symbol);
                }
                return false;
            }
            _ => break,
        }
    }
    if facts.has_new_expression {
        return true;
    }
    if !facts.has_call {
        return false;
    }
    let callee_sym = bare_call_identifier_callee(ctx, expr);
    let has_state_dep = facts.refs.iter().any(|&ref_id| {
        matches!(
            ctx.reactivity.reference_semantics(ref_id),
            ReferenceSemantics::PropRead(PropReferenceSemantics::Source { .. })
                | ReferenceSemantics::StoreRead { .. }
                | ReferenceSemantics::StoreWrite { .. }
                | ReferenceSemantics::StoreUpdate { .. }
        )
    });
    if has_state_dep {
        return callee_sym
            .map(|s| {
                matches!(
                    ctx.reactivity.binding_semantics(s),
                    BindingSemantics::MaybeReactive
                )
            })
            .unwrap_or(false);
    }
    if let Some(sym) = callee_sym
        && matches!(
            ctx.reactivity.binding_semantics(sym),
            BindingSemantics::MaybeReactive
        )
    {
        return true;
    }
    false
}

fn bare_call_identifier_callee<'a>(
    ctx: &Ctx<'_, 'a>,
    expr: &Expression<'a>,
) -> Option<SymbolId> {
    let Expression::CallExpression(call) = expr.get_inner_expression() else {
        return None;
    };
    let Expression::Identifier(ident) = call.callee.get_inner_expression() else {
        return None;
    };
    let ref_id = ident.reference_id.get()?;
    ctx.semantics.get_reference(ref_id).symbol_id()
}

fn collection_expression_facts<'a>(
    ctx: &Ctx<'_, 'a>,
    expr: &Expression<'a>,
    body_scope: ScopeId,
) -> CollectionExprFacts {
    let each_depth = ctx.semantics.function_depth(body_scope) + 1;
    let mut collector = CollectionExprCollector {
        refs: Vec::new(),
        has_call: false,
        has_new_expression: false,
        has_await: false,
    };
    collector.visit_expression(expr);

    let mut has_external = false;
    let mut uses_store = false;
    let mut collection_store: Option<SymbolId> = None;
    for ref_id in &collector.refs {
        let sem = ctx.reactivity.reference_semantics(*ref_id);

        let effective_sym = match sem {
            ReferenceSemantics::StoreRead { symbol }
            | ReferenceSemantics::StoreWrite { symbol }
            | ReferenceSemantics::StoreUpdate { symbol } => Some(symbol),
            _ => ctx.semantics.get_reference(*ref_id).symbol_id(),
        };
        if matches!(
            sem,
            ReferenceSemantics::StoreRead { .. }
                | ReferenceSemantics::StoreWrite { .. }
                | ReferenceSemantics::StoreUpdate { .. }
        ) {
            uses_store = true;
            if collection_store.is_none() {
                collection_store = effective_sym;
            }
        }
        if let Some(sym) = effective_sym
            && !has_external
        {
            let decl_scope = ctx.semantics.symbol_scope_id(sym);
            if ctx.semantics.function_depth(decl_scope) < each_depth {
                has_external = true;
            }
        }
    }
    CollectionExprFacts {
        has_external,
        uses_store,
        collection_store,
        has_call: collector.has_call,
        has_new_expression: collector.has_new_expression,
        refs: collector.refs,
    }
}

fn body_has_direct_animate(ctx: &Ctx<'_, '_>, nodes: &[NodeId]) -> bool {
    nodes.iter().any(|&id| {
        let node = ctx.component.store.get(id);
        match node {
            Node::Element(el) => el
                .attributes
                .iter()
                .any(|a| matches!(a, Attribute::AnimateDirective(_))),
            Node::SvelteElement(el) => el
                .attributes
                .iter()
                .any(|a| matches!(a, Attribute::AnimateDirective(_))),
            _ => false,
        }
    })
}

#[derive(Default)]
struct CollectionExprFacts {
    has_external: bool,
    uses_store: bool,
    collection_store: Option<SymbolId>,
    has_call: bool,
    has_new_expression: bool,
    refs: Vec<ReferenceId>,
}

struct CollectionExprCollector {
    refs: Vec<ReferenceId>,
    has_call: bool,
    has_new_expression: bool,
    has_await: bool,
}

impl<'a> Visit<'a> for CollectionExprCollector {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        if let Some(ref_id) = ident.reference_id.get() {
            self.refs.push(ref_id);
        }
    }
    fn visit_call_expression(&mut self, expr: &CallExpression<'a>) {
        self.has_call = true;
        walk_call_expression(self, expr);
    }
    fn visit_new_expression(&mut self, expr: &NewExpression<'a>) {
        self.has_new_expression = true;
        walk_new_expression(self, expr);
    }
    fn visit_await_expression(&mut self, expr: &AwaitExpression<'a>) {
        self.has_await = true;
        walk_await_expression(self, expr);
    }
}

fn expression_node_id(expr: &Expression<'_>) -> OxcNodeId {
    match expr {
        Expression::Identifier(e) => e.node_id(),
        Expression::StringLiteral(e) => e.node_id(),
        Expression::NumericLiteral(e) => e.node_id(),
        Expression::BooleanLiteral(e) => e.node_id(),
        Expression::NullLiteral(e) => e.node_id(),
        Expression::TemplateLiteral(e) => e.node_id(),
        Expression::BigIntLiteral(e) => e.node_id(),
        Expression::RegExpLiteral(e) => e.node_id(),
        Expression::ArrayExpression(e) => e.node_id(),
        Expression::ObjectExpression(e) => e.node_id(),
        Expression::ArrowFunctionExpression(e) => e.node_id(),
        Expression::FunctionExpression(e) => e.node_id(),
        Expression::AssignmentExpression(e) => e.node_id(),
        Expression::AwaitExpression(e) => e.node_id(),
        Expression::BinaryExpression(e) => e.node_id(),
        Expression::CallExpression(e) => e.node_id(),
        Expression::ChainExpression(e) => e.node_id(),
        Expression::ClassExpression(e) => e.node_id(),
        Expression::ConditionalExpression(e) => e.node_id(),
        Expression::LogicalExpression(e) => e.node_id(),
        Expression::NewExpression(e) => e.node_id(),
        Expression::ParenthesizedExpression(e) => e.node_id(),
        Expression::SequenceExpression(e) => e.node_id(),
        Expression::TaggedTemplateExpression(e) => e.node_id(),
        Expression::ThisExpression(e) => e.node_id(),
        Expression::UnaryExpression(e) => e.node_id(),
        Expression::UpdateExpression(e) => e.node_id(),
        Expression::YieldExpression(e) => e.node_id(),
        Expression::PrivateInExpression(e) => e.node_id(),
        Expression::JSXElement(e) => e.node_id(),
        Expression::JSXFragment(e) => e.node_id(),
        Expression::ImportExpression(e) => e.node_id(),
        Expression::MetaProperty(e) => e.node_id(),
        Expression::Super(e) => e.node_id(),
        Expression::V8IntrinsicExpression(e) => e.node_id(),
        Expression::ComputedMemberExpression(e) => e.node_id(),
        Expression::StaticMemberExpression(e) => e.node_id(),
        Expression::PrivateFieldExpression(e) => e.node_id(),
        Expression::TSAsExpression(_)
        | Expression::TSSatisfiesExpression(_)
        | Expression::TSTypeAssertion(_)
        | Expression::TSNonNullExpression(_)
        | Expression::TSInstantiationExpression(_) => unreachable!("TS stripped at parse"),
    }
}

fn expression_is_identifier_of(
    expr: &Expression<'_>,
    target: SymbolId,
    semantics: &ComponentSemantics<'_>,
) -> bool {
    let Expression::Identifier(ident) = expr.get_inner_expression() else {
        return false;
    };
    let Some(ref_id) = ident.reference_id.get() else {
        return false;
    };
    semantics.get_reference(ref_id).symbol_id() == Some(target)
}

struct IdentRefCounter<'s, 'a> {
    target: SymbolId,
    semantics: &'s ComponentSemantics<'a>,
    count: usize,
    early_exit: bool,
}

impl<'a> Visit<'a> for IdentRefCounter<'_, 'a> {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        if self.early_exit && self.count > 0 {
            return;
        }
        if let Some(ref_id) = ident.reference_id.get()
            && self.semantics.get_reference(ref_id).symbol_id() == Some(self.target)
        {
            self.count += 1;
        }
    }
}

fn expression_contains_reference_to(
    expr: &Expression<'_>,
    target: SymbolId,
    semantics: &ComponentSemantics<'_>,
) -> bool {
    let mut counter = IdentRefCounter {
        target,
        semantics,
        count: 0,
        early_exit: true,
    };
    counter.visit_expression(expr);
    counter.count > 0
}

fn count_references_to_in_expr(
    expr: &Expression<'_>,
    target: SymbolId,
    semantics: &ComponentSemantics<'_>,
) -> usize {
    let mut counter = IdentRefCounter {
        target,
        semantics,
        count: 0,
        early_exit: false,
    };
    counter.visit_expression(expr);
    counter.count
}
