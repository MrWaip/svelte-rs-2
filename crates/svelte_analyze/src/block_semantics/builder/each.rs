use super::super::{
    BlockSemantics, EachAsyncKind, EachBlockSemantics, EachCollection, EachCollectionSource,
    EachFlags, EachFlavor, EachIndexKind, EachItemKind, EachKeyKind,
};
use super::common::{binding_ident_of, binding_pattern_node_id, declarator_from_stmt};
use super::walker::Ctx;
use crate::expression_semantics::{ExpressionData, ExpressionSemantics, Volatility};
use crate::reactivity_semantics::data::{BindingSemantics, PropBindingKind};
use crate::utils::node_id_utils::expression_node_id;
use oxc_ast::ast::{BindingPattern, Expression, IdentifierReference};
use oxc_ast_visit::Visit;
use oxc_syntax::scope::ScopeId;
use svelte_ast::{Attribute, EachBlock, Node, NodeId};
use svelte_component_semantics::{ComponentSemantics, SymbolId};

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
            let item_is_indexed_legacy = item_sym.is_some_and(|item| {
                ctx.reactivity
                    .binding_semantics(item)
                    .is_each_item_indexed_legacy()
            });
            let used_in_body = item_is_indexed_legacy
                || if !used_in_key {
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

    let has_animate = body_has_direct_animate(ctx, ctx.component.fragment_nodes(block.body));

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

    let expression_data = match ctx.expressions.get(block.id) {
        ExpressionSemantics::Expression(d) => Some(d),
        ExpressionSemantics::NonSpecial => None,
    };

    let async_kind = derive_async_kind(expression_data);

    let declaration_blockers = super::declaration_group::declaration_blockers_of(ctx, block.id);

    let source = derive_collection_source(ctx, collection_expr, expression_data);

    let runes = ctx.reactivity.uses_runes();

    let references = expression_data
        .map(|d| d.references.as_slice())
        .unwrap_or(&[]);
    let each_flags = compute_each_flags(
        ctx,
        references,
        body_scope,
        &key,
        &index,
        has_animate,
        runes,
    );

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

    let render_index_required = requires_render_index(ctx, index, item_sym, shadows_outer, flavor);

    ctx.store.set(
        block.id,
        BlockSemantics::Each(EachBlockSemantics {
            item,
            index,
            key,
            flavor,
            each_flags,
            shadows_outer,
            render_index_required,
            async_kind,
            declaration_blockers,
            collection: EachCollection { source },
        }),
    );
}

fn requires_render_index(
    ctx: &Ctx<'_, '_>,
    index: EachIndexKind,
    item_sym: Option<SymbolId>,
    shadows_outer: bool,
    flavor: EachFlavor,
) -> bool {
    let index_used_by_body = match index {
        EachIndexKind::Declared { used_in_body, .. } => used_in_body,
        EachIndexKind::Absent => false,
    };
    if index_used_by_body {
        return true;
    }

    let legacy_writeback_no_user_index = match index {
        EachIndexKind::Absent => item_sym.is_some_and(|sym| {
            ctx.reactivity
                .binding_semantics(sym)
                .is_each_item_indexed_legacy()
        }),
        EachIndexKind::Declared { .. } => false,
    };
    if legacy_writeback_no_user_index {
        return true;
    }

    if item_sym.is_some_and(|sym| ctx.semantics.is_member_mutated(sym)) {
        return true;
    }

    if shadows_outer {
        return true;
    }

    match flavor {
        EachFlavor::BindGroup => true,
        EachFlavor::Regular => false,
    }
}

fn compute_each_flags(
    ctx: &Ctx<'_, '_>,
    references: &[SymbolId],
    body_scope: Option<ScopeId>,
    key: &EachKeyKind,
    index: &EachIndexKind,
    has_animate: bool,
    runes: bool,
) -> EachFlags {
    let has_key = match key {
        EachKeyKind::Unkeyed | EachKeyKind::KeyedByIndex => false,
        EachKeyKind::KeyedByItem | EachKeyKind::KeyedByExpr(_) => true,
    };
    let has_index = match index {
        EachIndexKind::Declared { .. } => true,
        EachIndexKind::Absent => false,
    };
    let key_is_item = match key {
        EachKeyKind::KeyedByItem => true,
        EachKeyKind::Unkeyed | EachKeyKind::KeyedByIndex | EachKeyKind::KeyedByExpr(_) => false,
    };

    let each_depth = body_scope.map(|scope| ctx.semantics.function_depth(scope) + 1);
    let mut uses_store = false;
    let mut has_external = false;
    for &sym in references {
        if symbol_is_store(ctx.reactivity.binding_semantics(sym)) {
            uses_store = true;
        }
        if let Some(depth) = each_depth
            && ctx
                .semantics
                .function_depth(ctx.semantics.symbol_scope_id(sym))
                < depth
        {
            has_external = true;
        }
    }

    let mut each_flags = EachFlags::empty();
    each_flags.set(EachFlags::INDEX_REACTIVE, has_key && has_index);
    each_flags.set(
        EachFlags::ITEM_REACTIVE,
        has_external && (!runes || !key_is_item || uses_store),
    );
    each_flags.set(EachFlags::ITEM_IMMUTABLE, runes && !uses_store);
    each_flags.set(EachFlags::ANIMATED, has_key && has_animate);
    each_flags
}

fn symbol_is_store(binding: BindingSemantics) -> bool {
    match binding {
        BindingSemantics::Store(_) => true,
        BindingSemantics::State(_)
        | BindingSemantics::Derived(_)
        | BindingSemantics::OptimizedDerived(_)
        | BindingSemantics::LegacyState(_)
        | BindingSemantics::Prop(_)
        | BindingSemantics::LegacyBindableProp(_)
        | BindingSemantics::Contextual(_)
        | BindingSemantics::OptimizedRune(_)
        | BindingSemantics::Const(_)
        | BindingSemantics::OptimizedConst(_)
        | BindingSemantics::DeclarationTag
        | BindingSemantics::OptimizedDeclarationTag
        | BindingSemantics::RuntimeRune { .. }
        | BindingSemantics::LegacyApiExport
        | BindingSemantics::MaybeReactive
        | BindingSemantics::NonReactive
        | BindingSemantics::LegacyPropsObject
        | BindingSemantics::Unresolved => false,
    }
}

fn derive_async_kind(data: Option<&ExpressionData>) -> EachAsyncKind {
    match data {
        Some(d) => match d.volatility {
            Volatility::Asynchronous => EachAsyncKind::Awaited {
                blockers: d.blockers.clone(),
            },
            Volatility::Static | Volatility::Reactive | Volatility::Heavy => {
                if d.blockers.is_empty() {
                    EachAsyncKind::Sync
                } else {
                    EachAsyncKind::Deferred {
                        blockers: d.blockers.clone(),
                    }
                }
            }
        },
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
    let is_source_prop = match ctx.reactivity.binding_semantics(sym) {
        BindingSemantics::LegacyBindableProp(_) => true,
        BindingSemantics::Prop(prop) => match &prop.kind {
            PropBindingKind::Source { .. } => true,
            PropBindingKind::Identifier | PropBindingKind::Rest | PropBindingKind::NonSource => {
                false
            }
        },
        BindingSemantics::State(_)
        | BindingSemantics::Derived(_)
        | BindingSemantics::OptimizedDerived(_)
        | BindingSemantics::OptimizedRune(_)
        | BindingSemantics::RuntimeRune { .. }
        | BindingSemantics::Store(_)
        | BindingSemantics::LegacyState(_)
        | BindingSemantics::Const(_)
        | BindingSemantics::OptimizedConst(_)
        | BindingSemantics::DeclarationTag
        | BindingSemantics::OptimizedDeclarationTag
        | BindingSemantics::Contextual(_)
        | BindingSemantics::MaybeReactive
        | BindingSemantics::NonReactive
        | BindingSemantics::LegacyPropsObject
        | BindingSemantics::LegacyApiExport
        | BindingSemantics::Unresolved => false,
    };
    if !is_source_prop {
        return EachCollectionSource::Local;
    }
    EachCollectionSource::Prop { sym }
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
