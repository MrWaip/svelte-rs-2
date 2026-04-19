use crate::scope::SymbolId;
use crate::types::data::{AnalysisData, BindTargetSemantics};
use crate::walker::{TemplateVisitor, VisitContext};
use smallvec::SmallVec;
use svelte_ast::{Attribute, BindDirective, ClassDirective, Element, StyleDirective};

/// Pre-computes bind/directive semantics so codegen doesn't re-derive
/// symbol classifications from source text via string-based lookups.
pub(crate) struct BindSemanticsVisitor<'s> {
    source: &'s str,
}

impl<'s> BindSemanticsVisitor<'s> {
    pub(crate) fn new(source: &'s str) -> Self {
        Self { source }
    }

    fn shorthand_symbol(node_id: svelte_ast::NodeId, data: &AnalysisData) -> Option<SymbolId> {
        data.shorthand_symbol(node_id)
    }

    /// Pre-compute each-block variable names referenced in a bind:this expression.
    fn classify_bind_this(dir: &BindDirective, data: &mut AnalysisData) {
        if dir.shorthand
            || !data
                .bind_target_semantics(dir.id)
                .is_some_and(|semantics| semantics.is_this())
        {
            return;
        }

        let Some(info) = data.attr_expressions.get(dir.id) else {
            return;
        };

        let each_vars: SmallVec<[SymbolId; 4]> = info
            .ref_symbols()
            .iter()
            .copied()
            .filter(|&sym| {
                matches!(
                    data.declaration_semantics(data.scoping.symbol_declaration(sym)),
                    crate::DeclarationSemantics::Contextual(
                        crate::ContextualDeclarationSemantics::EachItem(_)
                            | crate::ContextualDeclarationSemantics::EachIndex(_),
                    )
                )
            })
            .collect();

        if !each_vars.is_empty() {
            data.template
                .bind_semantics
                .bind_this_each_context
                .insert(dir.id, each_vars);
        }
    }

    fn classify_bind(dir: &BindDirective, data: &mut AnalysisData) {
        if let Some(semantics) = data.parent(dir.id).and_then(|parent| {
            BindTargetSemantics::from_parent_kind_and_name(parent.kind, dir.name.as_str())
        }) {
            data.template
                .bind_semantics
                .target_semantics
                .insert(dir.id, semantics);
        }

        if let Some(sym_id) = data.bind_target_symbol(dir.id) {
            if data.script.blocker_data.has_async {
                if let Some(idx) = data.script.blocker_data.symbol_blocker(sym_id) {
                    data.template
                        .bind_semantics
                        .bind_blockers
                        .insert(dir.id, smallvec::smallvec![idx]);
                }
            }
        }
    }

    fn classify_class(dir: &ClassDirective, data: &mut AnalysisData) {
        let _ = Self::shorthand_symbol(dir.id, data);
    }

    fn classify_style(dir: &StyleDirective, data: &mut AnalysisData) {
        if !dir.shorthand {
            return;
        }
        let _ = Self::shorthand_symbol(dir.id, data);
    }
}

impl<'s> TemplateVisitor for BindSemanticsVisitor<'s> {
    fn visit_attribute(&mut self, attr: &Attribute, ctx: &mut VisitContext<'_, '_>) {
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
    fn leave_element(&mut self, el: &Element, ctx: &mut VisitContext<'_, '_>) {
        let bind_group_id = el.attributes.iter().find_map(|attr| {
            let Attribute::BindDirective(dir) = attr else {
                return None;
            };
            ctx.data
                .bind_target_semantics(dir.id)
                .is_some_and(|semantics| semantics.is_group())
                .then_some(dir.id)
        });
        let bind_group_value_attr_id = ctx
            .data
            .expression_attribute(el.id, &el.attributes, "value")
            .map(|attr| attr.id);
        let has_contenteditable = ctx.data.has_true_boolean_attribute(
            el.id,
            &el.attributes,
            "contenteditable",
            self.source,
        );
        let has_content_bind = el.attributes.iter().any(|attr| {
            let Attribute::BindDirective(dir) = attr else {
                return false;
            };
            ctx.data
                .bind_target_semantics(dir.id)
                .is_some_and(|semantics| semantics.is_contenteditable())
        });

        // Detect bind:group → mark element and find value attribute
        if let Some(bind_group_id) = bind_group_id {
            ctx.data
                .template
                .bind_semantics
                .has_bind_group
                .insert(el.id);
            if let Some(value_attr_id) = bind_group_value_attr_id {
                ctx.data
                    .template
                    .bind_semantics
                    .bind_group_value_attr
                    .insert(bind_group_id, value_attr_id);
            }

            let parent_eaches = ctx.data.parent_each_blocks(bind_group_id);
            for each_id in parent_eaches {
                ctx.data
                    .blocks
                    .each_context
                    .mark_contains_group_binding(each_id);
            }
        }

        if has_contenteditable && has_content_bind {
            ctx.data.elements.flags.bound_contenteditable.insert(el.id);
        }
    }
}
