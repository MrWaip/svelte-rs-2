use svelte_ast::Component;

use crate::passes::{
    bind_semantics, collect_symbols, content_types, dynamism, element_flags, template_side_tables,
    template_validation,
};
use crate::types::data::AnalysisData;
use crate::walker::TemplateVisitor;

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
    dynamism: dynamism::DynamismVisitor,
}

impl ReactivityBundle {
    pub(crate) fn new() -> Self {
        Self {
            dynamism: dynamism::DynamismVisitor::new(),
        }
    }

    pub(crate) fn visitors(&mut self) -> [&mut dyn TemplateVisitor; 1] {
        [&mut self.dynamism]
    }
}

pub(crate) struct TemplateClassificationBundle<'s> {
    element_flags: element_flags::ElementFlagsVisitor<'s>,
    bind_semantics: bind_semantics::BindSemanticsVisitor<'s>,
    content_types: content_types::ContentAndVarVisitor<'s>,
}

impl<'s> TemplateClassificationBundle<'s> {
    pub(crate) fn new(_component: &'s Component, _data: &AnalysisData, source: &'s str) -> Self {
        Self {
            element_flags: element_flags::ElementFlagsVisitor::new(source),
            bind_semantics: bind_semantics::BindSemanticsVisitor::new(source),
            content_types: content_types::ContentAndVarVisitor { source },
        }
    }

    pub(crate) fn visitors(&mut self) -> [&mut dyn TemplateVisitor; 3] {
        [
            &mut self.bind_semantics,
            &mut self.element_flags,
            &mut self.content_types,
        ]
    }

    pub(crate) fn finish(self, _data: &mut AnalysisData) {}
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
