use std::ops::{Deref, DerefMut};

use rustc_hash::{FxHashMap, FxHashSet};

use oxc_ast::ast::{Expression, Statement};
use oxc_semantic::SymbolId;
use svelte_analyze::{AnalysisData, CodegenView, IdentGen, JsAst, RuntimeInfo};
use svelte_ast::{
    Attribute, AwaitBlock, Component, DebugTag, EachBlock, Element, IfBlock, KeyBlock, NodeId,
    RenderTag, SvelteBoundary, SvelteElement,
};
use svelte_transform_client::TransformData;

use svelte_ast_builder::Builder;

pub struct CodegenQuery<'a> {
    pub component: &'a Component,
    pub view: CodegenView<'a, 'a>,
    pub analysis: &'a AnalysisData<'a>,
}

impl<'a> CodegenQuery<'a> {
    pub fn new(component: &'a Component, analysis: &'a AnalysisData<'a>) -> Self {
        Self {
            component,
            view: CodegenView::new(analysis),
            analysis,
        }
    }

    pub fn element(&self, id: NodeId) -> &'a Element {
        self.component.store.element(id)
    }
    pub fn node_attributes(&self, id: NodeId) -> &'a [Attribute] {
        self.component.store.get(id).attributes()
    }
    pub fn if_block(&self, id: NodeId) -> &'a IfBlock {
        self.component.store.if_block(id)
    }
    pub fn each_block(&self, id: NodeId) -> &'a EachBlock {
        self.component.store.each_block(id)
    }
    pub fn render_tag(&self, id: NodeId) -> &'a RenderTag {
        self.component.store.render_tag(id)
    }
    pub fn key_block(&self, id: NodeId) -> &'a KeyBlock {
        self.component.store.key_block(id)
    }
    pub fn svelte_element(&self, id: NodeId) -> &'a SvelteElement {
        self.component.store.svelte_element(id)
    }
    pub fn svelte_boundary(&self, id: NodeId) -> &'a SvelteBoundary {
        self.component.store.svelte_boundary(id)
    }
    pub fn await_block(&self, id: NodeId) -> &'a AwaitBlock {
        self.component.store.await_block(id)
    }
    pub fn debug_tag(&self, id: NodeId) -> &'a DebugTag {
        self.component.store.debug_tag(id)
    }

    pub fn runtime_plan(&self) -> RuntimeInfo {
        self.view.runtime_plan()
    }
}

impl<'a> Deref for CodegenQuery<'a> {
    type Target = CodegenView<'a, 'a>;

    fn deref(&self) -> &Self::Target {
        &self.view
    }
}

pub struct CodegenState<'a> {
    pub b: Builder<'a>,
    pub name: &'a str,
    pub source: &'a str,
    pub line_index: &'a svelte_span::LineIndex,
    pub filename: &'a str,
    pub experimental_async: bool,
    pub dev: bool,

    pub transform_data: TransformData,

    pub parsed: &'a mut JsAst<'a>,

    pub ident_gen: &'a mut IdentGen,

    pub module_hoisted: Vec<Statement<'a>>,

    pub delegated_events: Vec<String>,
    delegated_events_set: FxHashSet<String>,

    pub css_text: Option<&'a str>,

    pub has_tracing: bool,

    pub(crate) const_tag_blockers: FxHashMap<SymbolId, (String, usize)>,

    pub(crate) each_item_writeback_places: Option<FxHashMap<SymbolId, Expression<'a>>>,

    pub(crate) hoisted_templates: FxHashMap<String, String>,
}

impl<'a> CodegenState<'a> {
    fn new(
        allocator: &'a oxc_allocator::Allocator,
        name: &'a str,
        source: &'a str,
        line_index: &'a svelte_span::LineIndex,
        filename: &'a str,
        experimental_async: bool,
        dev: bool,
        parsed: &'a mut JsAst<'a>,
        ident_gen: &'a mut IdentGen,
        transform_data: TransformData,
        css_text: Option<&'a str>,
    ) -> Self {
        Self {
            b: Builder::new(allocator),
            name,
            source,
            line_index,
            filename,
            experimental_async,
            dev,
            transform_data,
            parsed,
            ident_gen,
            module_hoisted: Vec::new(),
            delegated_events: Vec::new(),
            delegated_events_set: FxHashSet::default(),
            css_text,
            has_tracing: false,
            const_tag_blockers: FxHashMap::default(),
            each_item_writeback_places: None,
            hoisted_templates: FxHashMap::default(),
        }
    }

    pub fn gen_ident(&mut self, prefix: &str) -> String {
        self.ident_gen.generate(prefix)
    }

    pub fn add_delegated_event(&mut self, event_name: String) {
        if self.delegated_events_set.insert(event_name.clone()) {
            self.delegated_events.push(event_name);
        }
    }
}

pub struct Ctx<'a> {
    pub query: CodegenQuery<'a>,
    pub state: CodegenState<'a>,
}

impl<'a> Deref for Ctx<'a> {
    type Target = CodegenState<'a>;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<'a> DerefMut for Ctx<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

impl<'a> Ctx<'a> {
    pub fn new(
        compile_ctx: svelte_types::CompileContext<'a, 'a>,
        options: &svelte_types::CodegenOptions,
        transform_data: TransformData,
        css_text: Option<&str>,
    ) -> Self {
        let allocator = compile_ctx.alloc;
        let component = compile_ctx.component;
        let analysis = compile_ctx.analysis;
        let parsed = compile_ctx.js_arena;
        let ident_gen = compile_ctx.ident_gen;
        let line_index = compile_ctx.line_index;

        let name = allocator.alloc_str(analysis.component_name());
        let source = component.source.as_str();
        let source = allocator.alloc_str(source);
        let filename = allocator.alloc_str(&options.filename);
        let css_text = css_text.map(|t| allocator.alloc_str(t) as &str);

        Self {
            query: CodegenQuery::new(component, analysis),
            state: CodegenState::new(
                allocator,
                name,
                source,
                line_index,
                filename,
                options.experimental_async,
                options.dev,
                parsed,
                ident_gen,
                transform_data,
                css_text,
            ),
        }
    }

    pub fn element(&self, id: NodeId) -> &'a Element {
        self.query.element(id)
    }
    pub fn node_attributes(&self, id: NodeId) -> &'a [svelte_ast::Attribute] {
        self.query.node_attributes(id)
    }
    pub fn render_tag(&self, id: NodeId) -> &'a RenderTag {
        self.query.render_tag(id)
    }

    pub fn gen_ident(&mut self, prefix: &str) -> String {
        self.state.gen_ident(prefix)
    }

    pub fn attr_index(&self, id: NodeId) -> Option<&svelte_analyze::AttrIndex> {
        self.query.view.attr_index(id)
    }
    pub fn expression_data(&self, id: NodeId) -> Option<&svelte_analyze::ExpressionData> {
        self.query.view.expression_data(id)
    }
    pub fn const_tag_symbol_blocker_expr(&self, sym: SymbolId) -> Option<Expression<'a>> {
        let (name, idx) = self.const_tag_blockers.get(&sym)?;
        Some(
            self.b
                .computed_member_expr(self.b.rid_expr(name), self.b.num_expr(*idx as f64)),
        )
    }
    pub fn runtime_plan(&self) -> RuntimeInfo {
        self.query.runtime_plan()
    }

    pub fn const_tag_blocker_exprs(&mut self, id: NodeId) -> Vec<Expression<'a>> {
        if self.const_tag_blockers.is_empty() {
            return Vec::new();
        }
        let Some(data) = self.expression_data(id) else {
            return Vec::new();
        };
        let ref_symbols: Vec<SymbolId> = data.references.iter().copied().collect();
        let mut result = Vec::new();
        for sym in &ref_symbols {
            if let Some(expr) = self.const_tag_symbol_blocker_expr(*sym) {
                result.push(expr);
            }
        }
        result
    }

    pub fn has_spread(&self, id: NodeId) -> bool {
        self.query.view.has_spread(id)
    }
    pub fn has_attribute(&self, id: NodeId, name: &str) -> bool {
        self.query.analysis.has_attribute(id, name)
    }
    pub fn class_semantics(&self, id: NodeId) -> Option<&svelte_analyze::ClassSemantics> {
        self.node_attributes(id).iter().find_map(|attr| {
            match self.query.analysis.attributes.get(attr.id()) {
                svelte_analyze::AttributeSemantics::Class(class) => Some(class),
                _ => None,
            }
        })
    }
    pub fn style_semantics(&self, id: NodeId) -> Option<&svelte_analyze::StyleSemantics> {
        self.node_attributes(id).iter().find_map(|attr| {
            match self.query.analysis.attributes.get(attr.id()) {
                svelte_analyze::AttributeSemantics::Style(style) => Some(style),
                _ => None,
            }
        })
    }
    pub fn has_class_directives(&self, id: NodeId) -> bool {
        self.class_semantics(id)
            .is_some_and(|c| !c.directives.is_empty())
    }
    pub fn has_class_attribute(&self, id: NodeId) -> bool {
        self.class_semantics(id).is_some_and(|c| c.attr.is_some())
    }
    pub fn class_is_directives_only(&self, id: NodeId) -> bool {
        self.class_semantics(id)
            .is_some_and(|c| c.attr.is_none() && c.static_attr.is_none())
    }
    pub fn style_is_directives_only(&self, id: NodeId) -> bool {
        self.style_semantics(id)
            .is_some_and(|s| s.attr.is_none() && s.static_attr.is_none())
    }
    pub fn needs_clsx(&self, id: NodeId) -> bool {
        self.class_semantics(id).is_some_and(|c| c.needs_clsx)
    }
    pub fn class_directive_info(
        &self,
        id: NodeId,
    ) -> Option<&[svelte_analyze::ClassDirectiveInfo]> {
        self.class_semantics(id).and_then(|c| {
            if c.directives.is_empty() {
                None
            } else {
                Some(c.directives.as_slice())
            }
        })
    }
    pub fn class_directives_volatility(&self, id: NodeId) -> svelte_analyze::Volatility {
        self.class_semantics(id)
            .map(|c| c.directives_volatility)
            .unwrap_or(svelte_analyze::Volatility::Static)
    }
    pub fn style_directives_volatility(&self, id: NodeId) -> svelte_analyze::Volatility {
        self.style_semantics(id)
            .map(|s| s.directives_volatility)
            .unwrap_or(svelte_analyze::Volatility::Static)
    }
    pub fn has_style_directives(&self, id: NodeId) -> bool {
        self.style_semantics(id)
            .is_some_and(|s| !s.directives.is_empty())
    }
    pub fn needs_class_base(&self, id: NodeId) -> bool {
        self.class_semantics(id).is_some_and(|c| c.needs_base)
    }
    pub fn needs_style_base(&self, id: NodeId) -> bool {
        self.style_semantics(id).is_some_and(|s| s.needs_base)
    }
    pub fn style_directives(&self, id: NodeId) -> &[svelte_ast::StyleDirective] {
        self.style_semantics(id)
            .map_or(&[], |s| s.directives.as_slice())
    }
    pub fn needs_input_defaults(&self, id: NodeId) -> bool {
        self.query.view.needs_input_defaults(id)
    }
    pub fn hydration_attribute_changed_ignored(&self, id: NodeId) -> bool {
        self.query.view.hydration_attribute_changed_ignored(id)
    }
    pub fn binding_property_non_reactive_ignored(&self, id: NodeId) -> bool {
        self.query
            .view
            .is_ignored(id, "binding_property_non_reactive")
    }
    pub fn needs_textarea_value_lowering(&self, id: NodeId) -> bool {
        self.query.view.needs_textarea_value_lowering(id)
    }
    pub fn needs_textarea_content_reset(&self, id: NodeId) -> bool {
        self.query.view.needs_textarea_content_reset(id)
    }
    pub fn is_customizable_select(&self, id: NodeId) -> bool {
        self.query.view.is_customizable_select(id)
    }
    pub fn needs_var(&self, id: NodeId) -> bool {
        self.query.view.needs_var(id)
    }
    pub fn static_class(&self, id: NodeId) -> Option<&str> {
        self.class_semantics(id)
            .and_then(|c| c.static_base.as_deref())
    }
    pub fn static_style(&self, id: NodeId) -> Option<&str> {
        self.style_semantics(id)
            .and_then(|s| s.static_base.as_deref())
    }
    pub fn is_bound_contenteditable(&self, id: NodeId) -> bool {
        self.query.view.is_bound_contenteditable(id)
    }
    pub fn has_use_directive(&self, id: NodeId) -> bool {
        self.query.view.has_use_directive(id)
    }
    pub fn class_state_volatility(&self, id: NodeId) -> svelte_analyze::Volatility {
        self.class_semantics(id)
            .map(|c| c.state_volatility)
            .unwrap_or(svelte_analyze::Volatility::Static)
    }
    pub fn class_attr_id(&self, id: NodeId) -> Option<NodeId> {
        self.class_semantics(id).and_then(|c| c.attr)
    }
    pub fn is_expression_shorthand(&self, id: NodeId) -> bool {
        self.query.view.is_expression_shorthand(id)
    }
    pub fn has_component_css_props(&self, id: NodeId) -> bool {
        self.query.view.has_component_css_props(id)
    }
    pub fn component_snippets(&self, id: NodeId) -> &[NodeId] {
        self.query.view.component_snippets(id)
    }
    pub fn ce_config(&self) -> Option<&svelte_parser::ParsedCeConfig> {
        self.query.view.ce_config()
    }
    pub fn symbol_name(&self, sym: SymbolId) -> &str {
        self.query.view.symbol_name(sym)
    }
    pub fn has_bind_group(&self, id: NodeId) -> bool {
        self.query.view.has_bind_group(id)
    }
    pub fn symbol_blocker(&self, sym: SymbolId) -> Option<u32> {
        self.query.view.symbol_blocker(sym)
    }

    pub fn debug_tag(&self, id: NodeId) -> &'a DebugTag {
        self.query.debug_tag(id)
    }

    pub fn add_delegated_event(&mut self, event_name: String) {
        self.state.add_delegated_event(event_name);
    }

    pub fn css_hash(&self) -> &str {
        self.query.view.css_hash()
    }

    pub fn is_css_scoped(&self, id: NodeId) -> bool {
        self.query.view.is_css_scoped(id)
    }
}
