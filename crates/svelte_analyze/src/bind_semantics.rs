use oxc_semantic::ScopeId;
use svelte_ast::{
    Attribute, BindDirective, ClassDirective, ComponentNode, EachBlock, Element, RenderTag,
    StyleDirective, StyleDirectiveValue, SvelteBody, SvelteDocument, SvelteElement, SvelteWindow,
};

use crate::data::AnalysisData;
use crate::walker::TemplateVisitor;

/// Pre-computes bind/directive semantics so codegen doesn't re-derive
/// symbol classifications from source text via string-based lookups.
pub(crate) struct BindSemanticsVisitor<'s> {
    source: &'s str,
}

impl<'s> BindSemanticsVisitor<'s> {
    pub(crate) fn new(source: &'s str) -> Self {
        Self { source }
    }

    /// Check whether `name` resolves to a mutable rune at root scope.
    fn is_mutable_rune(name: &str, data: &AnalysisData) -> bool {
        let root = data.scoping.root_scope_id();
        data.scoping
            .find_binding(root, name)
            .is_some_and(|sym| data.scoping.is_rune(sym) && data.scoping.is_mutated(sym))
    }

    /// Check whether `name` resolves to a prop source at root scope.
    fn is_prop_source(name: &str, data: &AnalysisData) -> bool {
        let root = data.scoping.root_scope_id();
        data.scoping
            .find_binding(root, name)
            .is_some_and(|s| data.scoping.is_prop_source(s))
    }

    fn classify_bind(&self, dir: &BindDirective, data: &mut AnalysisData) {
        let name = if dir.shorthand {
            &dir.name
        } else if let Some(span) = dir.expression_span {
            let text = self.source[span.start as usize..span.end as usize].trim();
            text
        } else {
            return;
        };

        if Self::is_mutable_rune(name, data) {
            data.bind_semantics.mutable_rune_targets.insert(dir.id);
        }
    }

    fn classify_class(&self, dir: &ClassDirective, data: &mut AnalysisData) {
        // Check directive name against scoping. For shorthand `class:active`,
        // name == variable. For `class:active={active}`, name also matches.
        // For `class:foo={bar}`, name "foo" won't be a rune — correct.
        if Self::is_mutable_rune(&dir.name, data) {
            data.bind_semantics.mutable_rune_targets.insert(dir.id);
        }
    }

    fn classify_style(&self, dir: &StyleDirective, data: &mut AnalysisData) {
        if !matches!(dir.value, StyleDirectiveValue::Shorthand) {
            return;
        }
        if Self::is_mutable_rune(&dir.name, data) {
            data.bind_semantics.mutable_rune_targets.insert(dir.id);
        }
    }

    fn classify_attrs(&self, attrs: &[Attribute], data: &mut AnalysisData) {
        for attr in attrs {
            match attr {
                Attribute::BindDirective(dir) => self.classify_bind(dir, data),
                Attribute::ClassDirective(dir) => self.classify_class(dir, data),
                Attribute::StyleDirective(dir) => self.classify_style(dir, data),
                _ => {}
            }
        }
    }
}

impl<'s> TemplateVisitor for BindSemanticsVisitor<'s> {
    fn visit_bind_directive(
        &mut self,
        dir: &BindDirective,
        _el: &Element,
        _scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        self.classify_bind(dir, data);
    }

    fn visit_attribute(
        &mut self,
        attr: &Attribute,
        _el: &Element,
        _scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        match attr {
            Attribute::ClassDirective(dir) => self.classify_class(dir, data),
            Attribute::StyleDirective(dir) => self.classify_style(dir, data),
            _ => {}
        }
    }

    fn visit_component_attribute(
        &mut self,
        attr: &Attribute,
        _cn: &ComponentNode,
        _scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        match attr {
            Attribute::BindDirective(dir) => self.classify_bind(dir, data),
            Attribute::ClassDirective(dir) => self.classify_class(dir, data),
            Attribute::StyleDirective(dir) => self.classify_style(dir, data),
            _ => {}
        }
    }

    fn visit_svelte_window(
        &mut self,
        w: &SvelteWindow,
        _scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        self.classify_attrs(&w.attributes, data);
    }

    fn visit_svelte_document(
        &mut self,
        doc: &SvelteDocument,
        _scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        self.classify_attrs(&doc.attributes, data);
    }

    fn visit_svelte_body(
        &mut self,
        body: &SvelteBody,
        _scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        self.classify_attrs(&body.attributes, data);
    }

    fn visit_svelte_element(
        &mut self,
        el: &SvelteElement,
        _scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        self.classify_attrs(&el.attributes, data);
    }

    fn visit_each_block(
        &mut self,
        block: &EachBlock,
        _parent_scope: ScopeId,
        _body_scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        let text = self.source[block.expression_span.start as usize..block.expression_span.end as usize].trim();
        if Self::is_prop_source(text, data) {
            data.bind_semantics.prop_source_nodes.insert(block.id);
        }
    }

    fn visit_render_tag(
        &mut self,
        tag: &RenderTag,
        _scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        // Render tag prop_source detection requires inspecting per-argument OXC AST
        // structure (checking if arg is a zero-arg CallExpression on a prop_source).
        // This is deferred — codegen will continue using scoping queries for now,
        // but keyed by SymbolId rather than by name string.
        let _ = tag;
    }
}
