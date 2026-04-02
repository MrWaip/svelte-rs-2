use svelte_ast::{
    Attribute, BindDirective, ClassDirective, Element, StyleDirective, StyleDirectiveValue,
};

use crate::scope::SymbolId;
use crate::types::data::AnalysisData;
use crate::walker::{ParentKind, TemplateVisitor, VisitContext};

/// Pre-computes bind/directive semantics so codegen doesn't re-derive
/// symbol classifications from source text via string-based lookups.
pub(crate) struct BindSemanticsVisitor<'s> {
    source: &'s str,
}

impl<'s> BindSemanticsVisitor<'s> {
    pub(crate) fn new(source: &'s str) -> Self {
        Self { source }
    }

    fn is_mutable_rune(sym_id: SymbolId, data: &AnalysisData) -> bool {
        data.scoping.is_rune(sym_id) && data.scoping.is_mutated(sym_id)
    }

    fn shorthand_symbol(node_id: svelte_ast::NodeId, data: &AnalysisData) -> Option<SymbolId> {
        data.shorthand_symbol(node_id)
    }

    /// Pre-compute each-block variable names referenced in a bind:this expression.
    fn classify_bind_this(dir: &BindDirective, data: &mut AnalysisData) {
        if dir.name != "this" || dir.shorthand {
            return;
        }

        let Some(info) = data.attr_expressions.get(dir.id) else {
            return;
        };

        let each_vars: Vec<String> = info
            .ref_symbols
            .iter()
            .copied()
            .filter(|&sym| data.scoping.is_each_block_var(sym))
            .map(|sym| data.scoping.symbol_name(sym).to_string())
            .collect();

        if !each_vars.is_empty() {
            data.bind_semantics
                .bind_each_context
                .insert(dir.id, each_vars);
        }
    }

    fn classify_bind(dir: &BindDirective, data: &mut AnalysisData) {
        if let Some(sym_id) = data.bind_target_symbol(dir.id) {
            if Self::is_mutable_rune(sym_id, data) {
                data.bind_semantics.mutable_rune_targets.insert(dir.id);
            }
            if data.blocker_data.has_async {
                if let Some(idx) = data.blocker_data.symbol_blocker(sym_id) {
                    data.bind_semantics
                        .bind_blockers
                        .insert(dir.id, smallvec::smallvec![idx]);
                }
            }
        }
    }

    fn classify_class(dir: &ClassDirective, data: &mut AnalysisData) {
        if Self::shorthand_symbol(dir.id, data).is_some_and(|sym| Self::is_mutable_rune(sym, data))
        {
            data.bind_semantics.mutable_rune_targets.insert(dir.id);
        }
    }

    fn classify_style(dir: &StyleDirective, data: &mut AnalysisData) {
        if !matches!(dir.value, StyleDirectiveValue::Shorthand) {
            return;
        }
        if Self::shorthand_symbol(dir.id, data).is_some_and(|sym| Self::is_mutable_rune(sym, data))
        {
            data.bind_semantics.mutable_rune_targets.insert(dir.id);
        }
    }
}

impl<'s> TemplateVisitor for BindSemanticsVisitor<'s> {
    fn visit_attribute(&mut self, attr: &Attribute, ctx: &mut VisitContext<'_>) {
        match attr {
            Attribute::BindDirective(dir) => {
                Self::classify_bind(dir, ctx.data);
                Self::classify_bind_this(dir, ctx.data);
            }
            Attribute::ClassDirective(dir) => Self::classify_class(dir, ctx.data),
            Attribute::StyleDirective(dir) => Self::classify_style(dir, ctx.data),
            _ => {}
        }
    }

    fn visit_each_block(&mut self, block: &svelte_ast::EachBlock, ctx: &mut VisitContext<'_>) {
        if let Some(info) = ctx.data.expressions.get(block.id) {
            if info
                .ref_symbols
                .iter()
                .any(|&s| ctx.data.scoping.is_prop_source(s))
            {
                ctx.data.bind_semantics.prop_source_nodes.insert(block.id);
            }
        }
    }

    fn leave_element(&mut self, el: &Element, ctx: &mut VisitContext<'_>) {
        // Detect bind:group → mark element and find value attribute
        let bind_group = el.attributes.iter().find_map(|a| {
            if let Attribute::BindDirective(bd) = a {
                if bd.name == "group" {
                    return Some(bd);
                }
            }
            None
        });
        if let Some(bg) = bind_group {
            ctx.data.bind_semantics.has_bind_group.insert(el.id);
            if let Some(val_attr) = el
                .attributes
                .iter()
                .find(|a| matches!(a, Attribute::ExpressionAttribute(ea) if ea.name == "value"))
            {
                ctx.data
                    .bind_semantics
                    .bind_group_value_attr
                    .insert(bg.id, val_attr.id());
            }

            // Find ancestor each-blocks whose context vars are referenced in bind:group
            let referenced_syms: Vec<_> = ctx
                .data
                .attr_expressions
                .get(bg.id)
                .map(|info| {
                    info.ref_symbols
                        .iter()
                        .copied()
                        .filter(|&sym| ctx.data.scoping.is_each_block_var(sym))
                        .collect()
                })
                .unwrap_or_default();

            if !referenced_syms.is_empty() {
                let ancestor_eaches: Vec<_> = ctx
                    .ancestors()
                    .filter(|p| p.kind == ParentKind::EachBlock)
                    .map(|p| (p.id, ctx.data.each_body_scope(p.id, ctx.scope)))
                    .collect();

                let mut parent_eaches = Vec::new();
                for (each_id, body_scope) in ancestor_eaches {
                    let has_match = referenced_syms
                        .iter()
                        .any(|&sym| ctx.data.scoping.symbol_scope_id(sym) == body_scope);
                    if has_match {
                        parent_eaches.push(each_id);
                        ctx.data
                            .bind_semantics
                            .contains_group_binding
                            .insert(each_id);
                    }
                }
                if !parent_eaches.is_empty() {
                    ctx.data
                        .bind_semantics
                        .parent_each_blocks
                        .insert(bg.id, parent_eaches);
                }
            }
        }

        // Detect contenteditable + bind:innerHTML|innerText|textContent
        let has_contenteditable = el.attributes.iter().any(|a| match a {
            Attribute::BooleanAttribute(ba) => ba.name == "contenteditable",
            Attribute::StringAttribute(sa) if sa.name == "contenteditable" => {
                let val =
                    self.source[sa.value_span.start as usize..sa.value_span.end as usize].trim();
                val == "true"
            }
            _ => false,
        });
        if !has_contenteditable {
            return;
        }

        let has_content_bind = el.attributes.iter().any(|a| {
            matches!(a, Attribute::BindDirective(bd) if matches!(bd.name.as_str(), "innerHTML" | "innerText" | "textContent"))
        });
        if has_content_bind {
            ctx.data.element_flags.bound_contenteditable.insert(el.id);
        }
    }
}
