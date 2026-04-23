use std::ops::{Deref, DerefMut};

use rustc_hash::{FxHashMap, FxHashSet};

use oxc_ast::ast::{Expression, Statement};
use oxc_semantic::SymbolId;
use svelte_analyze::{
    AnalysisData, BindTargetSemantics, CodegenView, ComponentPropInfo, EventHandlerMode, ExprDeps,
    ExprSite, ExpressionInfo, IdentGen, JsAst, RuntimePlan,
};
use svelte_ast::{
    AwaitBlock, Component, ComponentNode, DebugTag, EachBlock, Element, IfBlock, KeyBlock, NodeId,
    RenderTag, SvelteBoundary, SvelteElement,
};
use svelte_transform::TransformData;

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
    pub fn component_node(&self, id: NodeId) -> &'a ComponentNode {
        self.component.store.component_node(id)
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

    pub fn expression(&self, id: NodeId) -> Option<&ExpressionInfo> {
        self.view.expression(id)
    }
    pub fn expr_deps(&self, site: ExprSite) -> Option<ExprDeps<'_>> {
        self.view.expr_deps(site)
    }

    pub fn runtime_plan(&self) -> RuntimePlan {
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
    pub filename: &'a str,
    pub experimental_async: bool,
    pub dev: bool,

    pub transform_data: TransformData,

    pub parsed: &'a mut JsAst<'a>,

    pub ident_gen: &'a mut IdentGen,

    pub module_hoisted: Vec<Statement<'a>>,

    pub needs_binding_group: bool,

    pub group_index_names: FxHashMap<NodeId, String>,

    pub delegated_events: Vec<String>,
    delegated_events_set: FxHashSet<String>,

    pub css_text: Option<&'a str>,

    pub has_tracing: bool,

    pub(crate) const_tag_blockers: FxHashMap<SymbolId, (String, usize)>,
}

impl<'a> CodegenState<'a> {
    fn new(
        allocator: &'a oxc_allocator::Allocator,
        name: &'a str,
        source: &'a str,
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
            filename,
            experimental_async,
            dev,
            transform_data,
            parsed,
            ident_gen,
            module_hoisted: Vec::new(),
            needs_binding_group: false,
            group_index_names: FxHashMap::default(),
            delegated_events: Vec::new(),
            delegated_events_set: FxHashSet::default(),
            css_text,
            has_tracing: false,
            const_tag_blockers: FxHashMap::default(),
        }
    }

    pub fn gen_ident(&mut self, prefix: &str) -> String {
        self.ident_gen.gen(prefix)
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
    pub fn render_tag(&self, id: NodeId) -> &'a RenderTag {
        self.query.render_tag(id)
    }

    pub fn gen_ident(&mut self, prefix: &str) -> String {
        self.state.gen_ident(prefix)
    }

    pub fn is_dynamic(&self, id: NodeId) -> bool {
        self.query.view.is_dynamic(id)
    }

    pub fn directive_root_ref_id(
        &self,
        attr: &svelte_ast::Attribute,
    ) -> Option<oxc_syntax::reference::ReferenceId> {
        let expr_ref = match attr {
            svelte_ast::Attribute::ExpressionAttribute(a) => Some(&a.expression),
            svelte_ast::Attribute::ClassDirective(a) => Some(&a.expression),
            svelte_ast::Attribute::StyleDirective(a) => match &a.value {
                svelte_ast::StyleDirectiveValue::Expression => Some(&a.expression),
                _ => None,
            },
            svelte_ast::Attribute::BindDirective(a) => Some(&a.expression),
            svelte_ast::Attribute::SpreadAttribute(a) => Some(&a.expression),
            svelte_ast::Attribute::AttachTag(a) => Some(&a.expression),
            svelte_ast::Attribute::UseDirective(a) => a.expression.as_ref(),
            svelte_ast::Attribute::OnDirectiveLegacy(a) => a.expression.as_ref(),
            svelte_ast::Attribute::TransitionDirective(a) => a.expression.as_ref(),
            svelte_ast::Attribute::AnimateDirective(a) => a.expression.as_ref(),
            _ => None,
        }?;
        let expr = self.state.parsed.expr(expr_ref.id())?;
        let mut current = expr;
        loop {
            match current {
                oxc_ast::ast::Expression::StaticMemberExpression(m) => current = &m.object,
                oxc_ast::ast::Expression::ComputedMemberExpression(m) => current = &m.object,
                oxc_ast::ast::Expression::Identifier(id) => return id.reference_id.get(),
                _ => return None,
            }
        }
    }

    pub fn directive_root_reference_semantics(
        &self,
        attr: &svelte_ast::Attribute,
    ) -> svelte_analyze::ReferenceSemantics {
        match self.directive_root_ref_id(attr) {
            Some(ref_id) => self.query.view.reference_semantics(ref_id),
            None => svelte_analyze::ReferenceSemantics::NonReactive,
        }
    }

    pub fn bind_each_context(&self, id: NodeId) -> Option<&[SymbolId]> {
        self.query.view.bind_each_context(id)
    }
    pub fn attr_index(&self, id: NodeId) -> Option<&svelte_analyze::AttrIndex> {
        self.query.view.attr_index(id)
    }
    pub fn each_index_name(&self, id: NodeId) -> Option<String> {
        self.query.view.each_index_name(id).map(str::to_string)
    }
    pub fn is_each_index_sym(&self, sym: SymbolId) -> bool {
        self.query.view.is_each_index_sym(sym)
    }
    pub fn attr_is_import(&self, attr_id: NodeId) -> bool {
        self.query.view.attr_is_import(attr_id)
    }
    pub fn attr_is_function(&self, attr_id: NodeId) -> bool {
        self.query.view.attr_is_function(attr_id)
    }
    pub fn expression(&self, id: NodeId) -> Option<&ExpressionInfo> {
        self.query.expression(id)
    }
    pub fn expr_deps(&self, site: ExprSite) -> Option<ExprDeps<'_>> {
        self.query.expr_deps(site)
    }
    pub fn const_tag_symbol_blocker_expr(&self, sym: SymbolId) -> Option<Expression<'a>> {
        let (name, idx) = self.const_tag_blockers.get(&sym)?;
        Some(
            self.b
                .computed_member_expr(self.b.rid_expr(name), self.b.num_expr(*idx as f64)),
        )
    }
    pub fn expr_has_await(&self, id: NodeId) -> bool {
        self.query.view.expr_is_async(id)
    }

    pub fn runtime_plan(&self) -> RuntimePlan {
        self.query.runtime_plan()
    }

    pub fn expr_has_blockers(&self, id: NodeId) -> bool {
        self.expr_deps(ExprSite::Node(id))
            .is_some_and(|deps| deps.has_blockers())
    }

    pub fn const_tag_blocker_exprs(&mut self, id: NodeId) -> Vec<Expression<'a>> {
        if self.const_tag_blockers.is_empty() {
            return Vec::new();
        }
        let Some(info) = self.expression(id) else {
            return Vec::new();
        };
        let ref_symbols = info.ref_symbols().to_vec();
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
    pub fn has_class_directives(&self, id: NodeId) -> bool {
        self.query.view.has_class_directives(id)
    }
    pub fn has_class_attribute(&self, id: NodeId) -> bool {
        self.query.view.has_class_attribute(id)
    }
    pub fn needs_clsx(&self, id: NodeId) -> bool {
        self.query.view.needs_clsx(id)
    }
    pub fn has_style_directives(&self, id: NodeId) -> bool {
        self.query.view.has_style_directives(id)
    }
    pub fn style_directives(&self, id: NodeId) -> &[svelte_ast::StyleDirective] {
        self.query.view.style_directives(id)
    }
    pub fn needs_input_defaults(&self, id: NodeId) -> bool {
        self.query.view.needs_input_defaults(id)
    }
    pub fn needs_textarea_value_lowering(&self, id: NodeId) -> bool {
        self.query.view.needs_textarea_value_lowering(id)
    }
    pub fn is_customizable_select(&self, id: NodeId) -> bool {
        self.query.view.is_customizable_select(id)
    }
    pub fn is_svelte_fragment_slot(&self, id: NodeId) -> bool {
        self.query.view.is_svelte_fragment_slot(id)
    }
    pub fn needs_var(&self, id: NodeId) -> bool {
        self.query.view.needs_var(id)
    }
    pub fn is_dynamic_attr(&self, id: NodeId) -> bool {
        self.query.view.is_dynamic_attr(id)
    }
    pub fn static_class(&self, id: NodeId) -> Option<&str> {
        self.query.view.static_class(id)
    }
    pub fn static_style(&self, id: NodeId) -> Option<&str> {
        self.query.view.static_style(id)
    }
    pub fn is_bound_contenteditable(&self, id: NodeId) -> bool {
        self.query.view.is_bound_contenteditable(id)
    }
    pub fn has_use_directive(&self, id: NodeId) -> bool {
        self.query.view.has_use_directive(id)
    }
    pub fn class_needs_state(&self, id: NodeId) -> bool {
        self.query.view.class_needs_state(id)
    }
    pub fn class_attr_id(&self, id: NodeId) -> Option<NodeId> {
        self.query.view.class_attr_id(id)
    }
    pub fn is_expression_shorthand(&self, id: NodeId) -> bool {
        self.query.view.is_expression_shorthand(id)
    }
    pub fn component_props(&self, id: NodeId) -> &[ComponentPropInfo] {
        self.query.view.component_props(id)
    }
    pub fn component_binding_sym(&self, id: NodeId) -> Option<SymbolId> {
        self.query.view.component_binding_sym(id)
    }
    pub fn has_component_css_props(&self, id: NodeId) -> bool {
        self.query.view.has_component_css_props(id)
    }
    pub fn component_snippets(&self, id: NodeId) -> &[NodeId] {
        self.query.view.component_snippets(id)
    }
    pub fn is_dynamic_component(&self, id: NodeId) -> bool {
        self.query.view.is_dynamic_component(id)
    }
    pub fn event_handler_mode(&self, attr_id: NodeId) -> Option<EventHandlerMode> {
        self.query.view.event_handler_mode(attr_id)
    }
    pub fn ce_config(&self) -> Option<&svelte_parser::ParsedCeConfig> {
        self.query.view.ce_config()
    }
    pub fn script_rune_calls(&self) -> &svelte_analyze::ScriptRuneCalls {
        self.query.view.script_rune_calls()
    }
    pub fn instance_script_node_id_offset(&self) -> u32 {
        self.query.view.instance_script_node_id_offset()
    }
    pub fn symbol_name(&self, sym: SymbolId) -> &str {
        self.query.view.symbol_name(sym)
    }
    pub fn has_bind_group(&self, id: NodeId) -> bool {
        self.query.view.has_bind_group(id)
    }
    pub fn bind_group_value_attr(&self, id: NodeId) -> Option<NodeId> {
        self.query.view.bind_group_value_attr(id)
    }
    pub fn parent_each_blocks(&self, id: NodeId) -> Vec<NodeId> {
        self.query.view.parent_each_blocks(id).into_iter().collect()
    }
    pub fn bind_blockers(&self, id: NodeId) -> &[u32] {
        self.query.view.bind_blockers(id)
    }
    pub fn bind_target_semantics(&self, id: NodeId) -> Option<BindTargetSemantics> {
        self.query.view.bind_target_semantics(id)
    }
    pub fn attr_expression(&self, id: NodeId) -> Option<&ExpressionInfo> {
        self.query.view.attr_expression(id)
    }
    pub fn needs_expr_memoization(&self, id: NodeId) -> bool {
        self.expr_deps(ExprSite::Node(id))
            .is_some_and(|deps| deps.needs_memo)
    }
    pub fn symbol_blocker(&self, sym: SymbolId) -> Option<u32> {
        self.query.view.symbol_blocker(sym)
    }

    pub fn debug_tag(&self, id: NodeId) -> &'a DebugTag {
        self.query.debug_tag(id)
    }

    pub fn fragment_references_any_symbol(
        &self,
        fragment_id: svelte_ast::FragmentId,
        syms: &rustc_hash::FxHashSet<SymbolId>,
    ) -> bool {
        self.query.view.fragment_references_any_symbol(
            &self.query.component.store,
            fragment_id,
            syms,
        )
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
