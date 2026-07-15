use std::mem;
use svelte_ast::Component;

use crate::passes::{
    collect_symbols, content_types, element_flags, template_side_tables, template_validation,
};
use crate::types::data::AnalysisData;
use crate::types::markers::ScopingBuilt;
use crate::walker::TemplateVisitor;

pub(crate) struct TemplateSideTablesBundle<'c> {
    side_tables: template_side_tables::TemplateSideTablesVisitor<'c>,
    collect_symbols: collect_symbols::CollectSymbolsVisitor,
}

impl<'c> TemplateSideTablesBundle<'c> {
    pub(crate) fn new(component: &'c Component, scoping: ScopingBuilt) -> Self {
        Self {
            side_tables: template_side_tables::TemplateSideTablesVisitor {
                component,
                expression_tag_buckets: Vec::new(),
            },
            collect_symbols: collect_symbols::make_visitor(scoping),
        }
    }

    pub(crate) fn visitors(&mut self) -> [&mut dyn TemplateVisitor; 2] {
        [&mut self.side_tables, &mut self.collect_symbols]
    }

    pub(crate) fn take_expression_tag_buckets(&mut self) -> template_side_tables::FragmentBuckets {
        mem::take(&mut self.side_tables.expression_tag_buckets)
    }
}

pub(crate) struct TemplateClassificationBundle<'s> {
    element_flags: element_flags::ElementFlagsVisitor<'s>,
    content_types: content_types::ContentAndVarVisitor,
    validation: template_validation::TemplateValidationVisitor,
}

impl<'s> TemplateClassificationBundle<'s> {
    pub(crate) fn new(_component: &'s Component, _data: &AnalysisData, source: &'s str) -> Self {
        Self {
            element_flags: element_flags::ElementFlagsVisitor::new(source),
            content_types: content_types::ContentAndVarVisitor,
            validation: template_validation::TemplateValidationVisitor::new(),
        }
    }

    pub(crate) fn visitors(&mut self) -> [&mut dyn TemplateVisitor; 3] {
        [
            &mut self.element_flags,
            &mut self.content_types,
            &mut self.validation,
        ]
    }

    pub(crate) fn finish(self, _data: &mut AnalysisData) {}
}
