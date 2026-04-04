use rustc_hash::FxHashSet;
use svelte_ast::{Component, Node, NodeId};

use crate::passes::{
    bind_semantics, collect_symbols, content_types, element_flags, hoistable, js_analyze,
    reactivity, template_side_tables, template_validation,
};
use crate::scope::SymbolId;
use crate::types::data::AnalysisData;
use crate::walker::TemplateVisitor;

pub(crate) struct AwaitBindingBundle {
    binding_preparer: js_analyze::BindingPreparer,
}

impl AwaitBindingBundle {
    pub(crate) fn new() -> Self {
        Self {
            binding_preparer: js_analyze::BindingPreparer,
        }
    }

    pub(crate) fn visitors(&mut self) -> [&mut dyn TemplateVisitor; 1] {
        [&mut self.binding_preparer]
    }
}

pub(crate) struct TemplateSideTablesBundle<'c> {
    side_tables: template_side_tables::TemplateSideTablesVisitor<'c>,
}

impl<'c> TemplateSideTablesBundle<'c> {
    pub(crate) fn new(component: &'c Component) -> Self {
        Self {
            side_tables: template_side_tables::TemplateSideTablesVisitor { component },
        }
    }

    pub(crate) fn visitors(&mut self) -> [&mut dyn TemplateVisitor; 1] {
        [&mut self.side_tables]
    }
}

pub(crate) struct SymbolCollectionBundle {
    collect_symbols: collect_symbols::CollectSymbolsVisitor,
}

impl SymbolCollectionBundle {
    pub(crate) fn new(scoping: crate::types::markers::ScopingBuilt) -> Self {
        Self {
            collect_symbols: collect_symbols::make_visitor(scoping),
        }
    }

    pub(crate) fn visitors(&mut self) -> [&mut dyn TemplateVisitor; 1] {
        [&mut self.collect_symbols]
    }
}

pub(crate) struct ReactivityBundle {
    reactivity: reactivity::ReactivityVisitor,
}

impl ReactivityBundle {
    pub(crate) fn new() -> Self {
        Self {
            reactivity: reactivity::ReactivityVisitor::new(),
        }
    }

    pub(crate) fn visitors(&mut self) -> [&mut dyn TemplateVisitor; 1] {
        [&mut self.reactivity]
    }
}

pub(crate) struct TemplateClassificationBundle<'s> {
    element_flags: element_flags::ElementFlagsVisitor<'s>,
    hoistable: hoistable::HoistableSnippetsVisitor,
    bind_semantics: bind_semantics::BindSemanticsVisitor<'s>,
    content_types: content_types::ContentAndVarVisitor<'s>,
}

impl<'s> TemplateClassificationBundle<'s> {
    pub(crate) fn new(component: &'s Component, data: &AnalysisData, source: &'s str) -> Self {
        let root = data.scoping.root_scope_id();
        let script_syms: FxHashSet<SymbolId> = data
            .script
            .as_ref()
            .map(|s| {
                s.declarations
                    .iter()
                    .filter_map(|d| data.scoping.find_binding(root, &d.name))
                    .collect()
            })
            .unwrap_or_default();
        let top_level_snippet_ids: FxHashSet<NodeId> = component
            .fragment
            .nodes
            .iter()
            .filter_map(|&id| {
                if let Node::SnippetBlock(b) = component.store.get(id) {
                    Some(b.id)
                } else {
                    None
                }
            })
            .collect();

        Self {
            element_flags: element_flags::ElementFlagsVisitor::new(source),
            hoistable: hoistable::HoistableSnippetsVisitor::new(script_syms, top_level_snippet_ids),
            bind_semantics: bind_semantics::BindSemanticsVisitor::new(source),
            content_types: content_types::ContentAndVarVisitor { source },
        }
    }

    pub(crate) fn visitors(&mut self) -> [&mut dyn TemplateVisitor; 4] {
        [
            &mut self.element_flags,
            &mut self.hoistable,
            &mut self.bind_semantics,
            &mut self.content_types,
        ]
    }

    pub(crate) fn finish(self, data: &mut AnalysisData) {
        self.hoistable.finish(data);
    }
}

pub(crate) struct TemplateValidationBundle {
    validation: template_validation::TemplateValidationVisitor,
}

impl TemplateValidationBundle {
    pub(crate) fn new() -> Self {
        Self {
            validation: template_validation::TemplateValidationVisitor::new(),
        }
    }

    pub(crate) fn visitors(&mut self) -> [&mut dyn TemplateVisitor; 1] {
        [&mut self.validation]
    }
}
