use oxc_semantic::ScopeId;
use svelte_ast::{
    Attribute, BindDirective, ClassDirective, ComponentNode, EachBlock, Element,
    StyleDirective, StyleDirectiveValue, SvelteBody, SvelteDocument, SvelteElement, SvelteWindow,
};

use crate::data::AnalysisData;
use crate::walker::TemplateVisitor;

/// Pre-computes bind/directive semantics so codegen doesn't re-derive
/// symbol classifications from source text via string-based lookups.
pub(crate) struct BindSemanticsVisitor<'s> {
    source: &'s str,
    /// Stack of ancestor each blocks (inner = top). Used to find which
    /// each blocks' context vars appear in bind:group expressions.
    each_block_stack: Vec<(svelte_ast::NodeId, ScopeId)>,
}

impl<'s> BindSemanticsVisitor<'s> {
    pub(crate) fn new(source: &'s str) -> Self {
        Self { source, each_block_stack: Vec::new() }
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

    /// Extract identifier-like tokens from an expression string.
    fn extract_identifiers(expr: &str) -> Vec<String> {
        expr.split(|c: char| !c.is_alphanumeric() && c != '_' && c != '$')
            .filter(|s| {
                !s.is_empty()
                    && s.chars()
                        .next()
                        .is_some_and(|c| c.is_alphabetic() || c == '_' || c == '$')
            })
            .map(|s| s.to_string())
            .collect()
    }

    /// Pre-compute each-block variable names referenced in a bind:this expression.
    fn classify_bind_this(&self, dir: &BindDirective, scope: ScopeId, data: &mut AnalysisData) {
        if dir.name != "this" {
            return;
        }
        let expr_text = if dir.shorthand {
            return; // shorthand bind:this is always a simple identifier
        } else if let Some(span) = dir.expression_span {
            self.source[span.start as usize..span.end as usize].trim()
        } else {
            return;
        };

        if svelte_types::is_simple_identifier(expr_text) {
            return; // simple identifiers don't need each-block context
        }

        let each_vars: Vec<String> = Self::extract_identifiers(expr_text)
            .into_iter()
            .filter(|name| {
                data.scoping.find_binding(scope, name)
                    .is_some_and(|sym| data.scoping.is_each_block_var(sym))
            })
            .collect();

        if !each_vars.is_empty() {
            data.bind_semantics.bind_each_context.insert(dir.id, each_vars);
        }
    }

    fn classify_bind(&self, dir: &BindDirective, data: &mut AnalysisData) {
        if dir.shorthand {
            // Shorthand: dir.name IS the binding name — string lookup acceptable
            if Self::is_mutable_rune(&dir.name, data) {
                data.bind_semantics.mutable_rune_targets.insert(dir.id);
            }
            return;
        }

        // Non-shorthand: use pre-resolved SymbolId from attr_expressions
        let Some(info) = data.attr_expressions.get(&dir.id) else { return };
        if !matches!(info.kind, svelte_types::ExpressionKind::Identifier(_)) { return }

        if let Some(sym_id) = info.references.first().and_then(|r| r.symbol_id) {
            if data.scoping.is_rune(sym_id) && data.scoping.is_mutated(sym_id) {
                data.bind_semantics.mutable_rune_targets.insert(dir.id);
            }
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
        scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        match attr {
            Attribute::BindDirective(dir) => {
                self.classify_bind(dir, data);
                self.classify_bind_this(dir, scope, data);
            }
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
        body_scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        let text = self.source[block.expression_span.start as usize..block.expression_span.end as usize].trim();
        if Self::is_prop_source(text, data) {
            data.bind_semantics.prop_source_nodes.insert(block.id);
        }
        self.each_block_stack.push((block.id, body_scope));
    }

    fn leave_each_block(
        &mut self,
        _block: &EachBlock,
        _parent_scope: ScopeId,
        _body_scope: ScopeId,
        _data: &mut AnalysisData,
    ) {
        self.each_block_stack.pop();
    }

    fn leave_element(
        &mut self,
        el: &Element,
        _scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        // Detect bind:group → mark element and find value attribute
        let bind_group = el.attributes.iter().find_map(|a| {
            if let Attribute::BindDirective(bd) = a {
                if bd.name == "group" { return Some(bd); }
            }
            None
        });
        if let Some(bg) = bind_group {
            data.bind_semantics.has_bind_group.insert(el.id);
            // Find the value attribute on the same element (for getter thunk wrapping)
            if let Some(val_attr) = el.attributes.iter().find(|a| {
                matches!(a, Attribute::ExpressionAttribute(ea) if ea.name == "value")
            }) {
                data.bind_semantics.bind_group_value_attr.insert(bg.id, val_attr.id());
            }

            // Walk ancestor each blocks to find which ones declare vars
            // referenced in the bind:group expression
            if let Some(expr_span) = bg.expression_span {
                let expr_text = self.source[expr_span.start as usize..expr_span.end as usize].trim();
                let idents = Self::extract_identifiers(expr_text);
                let mut parent_eaches = Vec::new();
                for &(each_id, body_scope) in self.each_block_stack.iter().rev() {
                    let has_match = idents.iter().any(|name| {
                        data.scoping.find_binding(body_scope, name)
                            .is_some_and(|sym| {
                                data.scoping.is_each_block_var(sym)
                                    && data.scoping.symbol_scope_id(sym) == body_scope
                            })
                    });
                    if has_match {
                        parent_eaches.push(each_id);
                        data.bind_semantics.contains_group_binding.insert(each_id);
                    }
                }
                if !parent_eaches.is_empty() {
                    data.bind_semantics.parent_each_blocks.insert(bg.id, parent_eaches);
                }
            }
        }

        // Detect contenteditable + bind:innerHTML|innerText|textContent combination.
        // Text children of such elements use nodeValue= init instead of $.set_text() update.
        let has_contenteditable = el.attributes.iter().any(|a| {
            match a {
                Attribute::BooleanAttribute(ba) => ba.name == "contenteditable",
                Attribute::StringAttribute(sa) if sa.name == "contenteditable" => {
                    let val = self.source[sa.value_span.start as usize..sa.value_span.end as usize].trim();
                    val == "true"
                }
                _ => false,
            }
        });
        if !has_contenteditable { return; }

        let has_content_bind = el.attributes.iter().any(|a| {
            matches!(a, Attribute::BindDirective(bd) if matches!(bd.name.as_str(), "innerHTML" | "innerText" | "textContent"))
        });
        if has_content_bind {
            data.element_flags.bound_contenteditable.insert(el.id);
        }
    }

}
