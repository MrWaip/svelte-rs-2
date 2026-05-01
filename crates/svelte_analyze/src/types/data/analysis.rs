use super::*;
use crate::types::script::PropsDeclaration;
use svelte_ast::{Attribute, BindDirective, ExpressionAttribute, Namespace, StringAttribute};
use svelte_component_semantics::SymbolFlags;

pub struct ScriptAnalysis {
    pub info: Option<ScriptInfo>,
    pub props_id: Option<String>,
    pub exports: Vec<ExportInfo>,
    pub instance_node_id_offset: u32,
    pub module_node_id_offset: u32,
    pub has_class_state_fields: bool,
    pub has_store_member_mutations: bool,
    pub runes: bool,
    pub accessors: bool,
    pub immutable: bool,
    pub preserve_whitespace: bool,
    pub experimental_async: bool,
    pub dev: bool,
    pub ce_config: Option<svelte_parser::ParsedCeConfig>,
    pub proxy_state_inits: ProxyStateInits,
    pub pickled_await_offsets: PickledAwaitOffsets,
    pub blocker_data: BlockerData,
    pub script_rune_calls: ScriptRuneCalls,
}

impl ScriptAnalysis {
    fn new() -> Self {
        Self {
            info: None,
            props_id: None,
            exports: Vec::new(),
            instance_node_id_offset: 0,
            module_node_id_offset: 0,
            has_class_state_fields: false,
            has_store_member_mutations: false,
            runes: true,
            accessors: false,
            immutable: false,
            preserve_whitespace: false,
            experimental_async: false,
            dev: false,
            ce_config: None,
            proxy_state_inits: ProxyStateInits::new(),
            pickled_await_offsets: PickledAwaitOffsets::new(),
            blocker_data: BlockerData::default(),
            script_rune_calls: ScriptRuneCalls::new(),
        }
    }

    pub fn props_declaration(&self) -> Option<&PropsDeclaration> {
        self.info.as_ref()?.props_declaration.as_ref()
    }
}

pub struct ElementAnalysis {
    pub(crate) facts: ElementFacts,
    pub flags: ElementFlags,
    pub directive_modifiers: DirectiveModifierFlags,
    pub(crate) html_tag_in_svg: NodeBitSet,
    pub(crate) html_tag_in_mathml: NodeBitSet,
}

impl ElementAnalysis {
    fn new(node_count: u32) -> Self {
        Self {
            facts: ElementFacts::new(node_count),
            flags: ElementFlags::new(node_count),
            directive_modifiers: DirectiveModifierFlags::new(node_count),
            html_tag_in_svg: NodeBitSet::new(node_count),
            html_tag_in_mathml: NodeBitSet::new(node_count),
        }
    }
}

pub struct TemplateAnalysis {
    pub fragment_facts: FragmentFacts,
    pub fragment_namespaces: FragmentNamespaces,
    pub rich_content_facts: RichContentFacts,
    pub(crate) fragment_blockers: Vec<Option<SmallVec<[u32; 2]>>>,
    pub snippets: SnippetData,
    pub const_tags: ConstTagData,
    pub debug_tags: DebugTagData,
    pub title_elements: TitleElementData,
    pub template_topology: TemplateTopology,
    pub template_elements: TemplateElementIndex,
    pub template_semantics: TemplateSemanticsData,
    pub bind_semantics: BindSemanticsData,
}

impl TemplateAnalysis {
    fn new(node_count: u32) -> Self {
        Self {
            fragment_facts: FragmentFacts::new(),
            fragment_namespaces: FragmentNamespaces::new(),
            rich_content_facts: RichContentFacts::new(),
            fragment_blockers: Vec::new(),
            snippets: SnippetData::new(node_count),
            const_tags: ConstTagData::new(node_count),
            debug_tags: DebugTagData::new(),
            title_elements: TitleElementData::new(),
            template_topology: TemplateTopology::new(node_count),
            template_elements: TemplateElementIndex::new(node_count),
            template_semantics: TemplateSemanticsData::new(node_count),
            bind_semantics: BindSemanticsData::new(node_count),
        }
    }

    pub fn fragment_blockers_by_id(&self, id: svelte_ast::FragmentId) -> &[u32] {
        self.fragment_blockers
            .get(id.0 as usize)
            .and_then(|o| o.as_ref())
            .map_or(&[], |v| v.as_slice())
    }

    pub(crate) fn insert_fragment_blockers_by_id(
        &mut self,
        id: svelte_ast::FragmentId,
        blockers: SmallVec<[u32; 2]>,
    ) {
        let idx = id.0 as usize;
        if self.fragment_blockers.len() <= idx {
            self.fragment_blockers.resize(idx + 1, None);
        }
        self.fragment_blockers[idx] = Some(blockers);
    }
}

pub struct BlockAnalysis {}

impl BlockAnalysis {
    fn new(_node_count: u32) -> Self {
        Self {}
    }
}

pub struct OutputPlanData {
    pub needs_context: bool,
    pub needs_sanitized_legacy_slots: bool,
    pub custom_element_slot_names: Vec<String>,
    pub component_name: String,
    pub is_custom_element_target: bool,
    pub custom_element_compile_flag: bool,
    pub runtime_plan: RuntimePlan,
    pub ignore_data: IgnoreData,
    pub needs_component_bind_ownership: bool,

    pub css: CssAnalysis,
}

impl OutputPlanData {
    fn new(node_count: u32) -> Self {
        Self {
            needs_context: false,
            needs_sanitized_legacy_slots: false,
            custom_element_slot_names: Vec::new(),
            component_name: String::new(),
            is_custom_element_target: false,
            custom_element_compile_flag: false,
            runtime_plan: RuntimePlan::default(),
            ignore_data: IgnoreData::new(),
            needs_component_bind_ownership: false,
            css: CssAnalysis::empty(node_count),
        }
    }
}

pub struct AnalysisData<'a> {
    pub expressions: NodeTable<ExpressionInfo>,
    pub attr_expressions: NodeTable<ExpressionInfo>,
    pub scoping: ComponentScoping<'a>,
    pub script: ScriptAnalysis,
    pub elements: ElementAnalysis,
    pub template: TemplateAnalysis,
    pub blocks: BlockAnalysis,
    pub output: OutputPlanData,
    pub reactivity: ReactivitySemantics,
    pub(crate) block_semantics_store: crate::block_semantics::BlockSemanticsStore,
    pub dynamism: crate::passes::dynamism::DynamismData,
}

impl<'a> AnalysisData<'a> {
    pub(crate) fn new_empty(node_count: u32) -> Self {
        Self {
            expressions: NodeTable::new(node_count),
            attr_expressions: NodeTable::new(node_count),
            scoping: ComponentScoping::new_empty(),
            script: ScriptAnalysis::new(),
            elements: ElementAnalysis::new(node_count),
            template: TemplateAnalysis::new(node_count),
            blocks: BlockAnalysis::new(node_count),
            output: OutputPlanData::new(node_count),
            reactivity: ReactivitySemantics::new(node_count),
            block_semantics_store: crate::block_semantics::BlockSemanticsStore::new(node_count),
            dynamism: crate::passes::dynamism::DynamismData::new(node_count),
        }
    }
}

impl<'a> AnalysisData<'a> {
    pub(crate) fn record_element_facts(&mut self, id: NodeId, entry: ElementFactsEntry) {
        self.elements.facts.record_entry(id, entry);
    }
    pub fn html_tag_in_svg(&self, id: NodeId) -> bool {
        self.elements.html_tag_in_svg.contains(&id)
    }
    pub fn html_tag_in_mathml(&self, id: NodeId) -> bool {
        self.elements.html_tag_in_mathml.contains(&id)
    }
    pub fn blocker_data(&self) -> &BlockerData {
        &self.script.blocker_data
    }
    pub fn script_rune_calls(&self) -> &ScriptRuneCalls {
        &self.script.script_rune_calls
    }
    pub fn is_pickled_await(&self, offset: u32) -> bool {
        self.script.pickled_await_offsets.contains_offset(offset)
    }
    pub fn is_dynamic(&self, id: NodeId) -> bool {
        self.dynamism.is_dynamic_node(id)
    }
    pub fn class_needs_state(&self, element_id: NodeId) -> bool {
        let class_attr_dynamic = self
            .elements
            .flags
            .class_attr_id(element_id)
            .is_some_and(|attr_id| self.dynamism.is_dynamic_attr(attr_id));
        class_attr_dynamic || self.elements.flags.has_dynamic_class_directives(element_id)
    }
    pub fn component_name(&self) -> &str {
        &self.output.component_name
    }
    pub fn expression(&self, id: NodeId) -> Option<&ExpressionInfo> {
        self.expressions.get(id)
    }
    pub fn expr_role(&self, id: NodeId) -> Option<ExprRole> {
        self.expressions
            .get(id)
            .and_then(|info| info.expr_role())
            .or_else(|| {
                self.attr_expressions
                    .get(id)
                    .and_then(|info| info.expr_role())
            })
    }
    pub fn expr_is_async(&self, id: NodeId) -> bool {
        self.expr_role(id) == Some(ExprRole::Async)
    }
    pub fn binding_origin_key(&self, sym: SymbolId) -> Option<&str> {
        self.scoping.binding_origin_key(sym)
    }
    pub fn each_item_indirect_sources(&self, item_sym: SymbolId) -> Option<&[SymbolId]> {
        self.reactivity.each_item_indirect_sources(item_sym)
    }
    pub fn symbol_for_reference(
        &self,
        ref_id: svelte_component_semantics::ReferenceId,
    ) -> Option<SymbolId> {
        self.scoping.symbol_for_reference(ref_id)
    }
    pub fn symbol_for_identifier_reference(
        &self,
        id: &oxc_ast::ast::IdentifierReference<'a>,
    ) -> Option<SymbolId> {
        self.scoping.symbol_for_identifier_reference(id)
    }
    pub fn binding_origin_key_for_reference(
        &self,
        ref_id: svelte_component_semantics::ReferenceId,
    ) -> Option<&str> {
        self.scoping.binding_origin_key_for_reference(ref_id)
    }
    pub fn binding_origin_key_for_identifier_reference(
        &self,
        id: &oxc_ast::ast::IdentifierReference<'a>,
    ) -> Option<&str> {
        self.scoping.binding_origin_key_for_identifier_reference(id)
    }
    pub fn iter_store_bindings(
        &self,
    ) -> impl Iterator<Item = (SymbolId, StoreBindingSemantics)> + '_ {
        self.reactivity.iter_store_bindings()
    }
    pub fn binding_semantics(&self, sym: SymbolId) -> BindingSemantics {
        self.reactivity.binding_semantics(sym)
    }
    pub fn declarator_semantics(
        &self,
        decl_node: svelte_component_semantics::OxcNodeId,
    ) -> crate::types::data::DeclaratorSemantics {
        self.reactivity.declarator_semantics(decl_node)
    }
    pub fn reference_semantics(
        &self,
        ref_id: svelte_component_semantics::ReferenceId,
    ) -> ReferenceSemantics {
        self.reactivity.reference_semantics(ref_id)
    }
    pub fn uses_runes(&self) -> bool {
        self.reactivity.uses_runes()
    }

    pub fn block_semantics(&self, id: NodeId) -> &crate::block_semantics::BlockSemantics {
        self.block_semantics_store.get(id)
    }
    pub fn fragment_facts_by_id(&self, id: svelte_ast::FragmentId) -> Option<&FragmentFactsEntry> {
        self.template.fragment_facts.lookup_by_id(id)
    }
    pub fn fragment_has_children_by_id(&self, id: svelte_ast::FragmentId) -> bool {
        self.template
            .fragment_facts
            .lookup_by_id(id)
            .is_some_and(FragmentFactsEntry::has_children)
    }
    pub fn fragment_child_count_by_id(&self, id: svelte_ast::FragmentId) -> u32 {
        self.template
            .fragment_facts
            .lookup_by_id(id)
            .map_or(0, FragmentFactsEntry::child_count)
    }
    pub fn fragment_single_child_by_id(&self, id: svelte_ast::FragmentId) -> Option<NodeId> {
        self.template
            .fragment_facts
            .lookup_by_id(id)
            .and_then(FragmentFactsEntry::single_child)
    }
    pub fn custom_element_slot_names(&self) -> &[String] {
        &self.output.custom_element_slot_names
    }
    pub fn fragment_non_trivial_child_count_by_id(&self, id: svelte_ast::FragmentId) -> u32 {
        self.template
            .fragment_facts
            .lookup_by_id(id)
            .map_or(0, FragmentFactsEntry::non_trivial_child_count)
    }
    pub fn fragment_has_non_trivial_children_by_id(&self, id: svelte_ast::FragmentId) -> bool {
        self.template
            .fragment_facts
            .lookup_by_id(id)
            .is_some_and(FragmentFactsEntry::has_non_trivial_children)
    }
    pub fn fragment_has_expression_child_by_id(&self, id: svelte_ast::FragmentId) -> bool {
        self.template
            .fragment_facts
            .lookup_by_id(id)
            .is_some_and(FragmentFactsEntry::has_expression_child)
    }
    pub fn fragment_single_expression_child_by_id(
        &self,
        id: svelte_ast::FragmentId,
    ) -> Option<NodeId> {
        self.template
            .fragment_facts
            .lookup_by_id(id)
            .and_then(FragmentFactsEntry::single_expression_child)
    }
    pub fn fragment_single_non_trivial_child_by_id(
        &self,
        id: svelte_ast::FragmentId,
    ) -> Option<NodeId> {
        self.template
            .fragment_facts
            .lookup_by_id(id)
            .and_then(FragmentFactsEntry::single_non_trivial_child)
    }
    pub fn fragment_has_direct_animate_child_by_id(&self, id: svelte_ast::FragmentId) -> bool {
        self.template
            .fragment_facts
            .lookup_by_id(id)
            .is_some_and(FragmentFactsEntry::has_direct_animate_child)
    }
    pub fn fragment_has_rich_content_by_id(
        &self,
        id: svelte_ast::FragmentId,
        parent: RichContentParentKind,
    ) -> bool {
        self.template
            .rich_content_facts
            .has_rich_content_by_id(id, parent)
    }
    pub fn element_facts(&self, id: NodeId) -> Option<&ElementFactsEntry> {
        self.elements.facts.entry(id)
    }
    pub fn attr_index(&self, id: NodeId) -> Option<&AttrIndex> {
        self.elements.facts.attr_index(id)
    }
    pub fn has_attribute(&self, id: NodeId, name: &str) -> bool {
        self.attr_index(id).is_some_and(|index| index.has(name))
    }
    pub fn attribute<'n>(
        &self,
        id: NodeId,
        attrs: &'n [Attribute],
        name: &str,
    ) -> Option<&'n Attribute> {
        self.attr_index(id)?.first(attrs, name)
    }
    pub fn string_attribute<'n>(
        &self,
        id: NodeId,
        attrs: &'n [Attribute],
        name: &str,
    ) -> Option<&'n svelte_ast::StringAttribute> {
        self.attribute(id, attrs, name).and_then(|attr| match attr {
            Attribute::StringAttribute(attr) => Some(attr),
            _ => None,
        })
    }
    pub fn expression_attribute<'n>(
        &self,
        id: NodeId,
        attrs: &'n [Attribute],
        name: &str,
    ) -> Option<&'n ExpressionAttribute> {
        self.attribute(id, attrs, name).and_then(|attr| match attr {
            Attribute::ExpressionAttribute(attr) => Some(attr),
            _ => None,
        })
    }
    pub fn bind_directive<'n>(
        &self,
        id: NodeId,
        attrs: &'n [Attribute],
        name: &str,
    ) -> Option<&'n BindDirective> {
        self.attribute(id, attrs, name).and_then(|attr| match attr {
            Attribute::BindDirective(attr) => Some(attr),
            _ => None,
        })
    }
    pub fn static_text_attribute_value<'n>(
        &self,
        id: NodeId,
        attrs: &'n [Attribute],
        name: &str,
        source: &'n str,
    ) -> Option<&'n str> {
        self.string_attribute(id, attrs, name)
            .map(|attr: &'n StringAttribute| attr.value_span.source_text(source))
    }
    pub fn has_true_boolean_attribute(
        &self,
        id: NodeId,
        attrs: &[Attribute],
        name: &str,
        source: &str,
    ) -> bool {
        self.attribute(id, attrs, name)
            .is_some_and(|attr| match attr {
                Attribute::BooleanAttribute(_) => true,
                Attribute::StringAttribute(attr) => {
                    attr.value_span.source_text(source).trim() == "true"
                }
                _ => false,
            })
    }
    pub fn has_spread(&self, id: NodeId) -> bool {
        self.elements.facts.has_spread(id)
    }
    pub fn has_runtime_attrs(&self, id: NodeId) -> bool {
        self.elements.facts.has_runtime_attrs(id)
    }
    pub fn namespace(&self, id: NodeId) -> Option<NamespaceKind> {
        self.elements.facts.namespace(id)
    }
    pub fn creation_namespace(&self, id: NodeId) -> Option<Namespace> {
        self.elements.facts.creation_namespace(id)
    }
    pub fn is_void(&self, id: NodeId) -> bool {
        self.elements.facts.is_void(id)
    }
    pub fn is_custom_element(&self, id: NodeId) -> bool {
        self.elements.facts.is_custom_element(id)
    }
    pub fn event_modifiers(&self, id: NodeId) -> EventModifier {
        self.elements.directive_modifiers.get(id)
    }
    pub fn parent(&self, id: NodeId) -> Option<ParentRef> {
        self.template.template_topology.parent(id)
    }
    pub fn expr_parent(&self, id: NodeId) -> Option<ParentRef> {
        self.template.template_topology.expr_parent(id)
    }
    pub fn ancestors(&self, id: NodeId) -> crate::types::data::template_topology::Ancestors<'_> {
        self.template.template_topology.ancestors(id)
    }
    pub fn expr_ancestors(
        &self,
        id: NodeId,
    ) -> crate::types::data::template_topology::Ancestors<'_> {
        self.template.template_topology.expr_ancestors(id)
    }
    pub fn nearest_element(&self, id: NodeId) -> Option<NodeId> {
        self.template.template_topology.nearest_element(id)
    }
    pub fn nearest_element_for_expr(&self, id: NodeId) -> Option<NodeId> {
        self.template.template_topology.nearest_element_for_expr(id)
    }
    pub fn template_element(&self, id: NodeId) -> Option<&TemplateElementEntry> {
        self.template.template_elements.entry(id)
    }
    pub fn template_elements(&self) -> &[NodeId] {
        self.template.template_elements.all_elements()
    }
    pub fn template_elements_with_tag(&self, tag_name: &str) -> &[NodeId] {
        self.template.template_elements.elements_with_tag(tag_name)
    }
    pub fn template_element_parent(&self, id: NodeId) -> Option<NodeId> {
        self.template.template_elements.parent_element(id)
    }
    pub fn template_element_previous_sibling(&self, id: NodeId) -> Option<NodeId> {
        self.template.template_elements.previous_sibling(id)
    }
    pub fn template_element_next_sibling(&self, id: NodeId) -> Option<NodeId> {
        self.template.template_elements.next_sibling(id)
    }
    pub fn template_element_tag_name(&self, id: NodeId) -> Option<&str> {
        self.template.template_elements.tag_name(id)
    }
    pub fn template_element_static_id(&self, id: NodeId) -> Option<&str> {
        self.elements.facts.static_id(id)
    }
    pub fn template_element_has_static_class(&self, id: NodeId, class_name: &str) -> bool {
        self.elements.facts.has_static_class(id, class_name)
    }
    pub fn template_element_may_match_class(&self, id: NodeId) -> bool {
        self.elements.facts.may_match_class(id)
    }
    pub fn template_element_may_match_id(&self, id: NodeId) -> bool {
        self.elements.facts.may_match_id(id)
    }
    pub fn template_elements_for_class(
        &self,
        class_name: &str,
    ) -> impl Iterator<Item = NodeId> + '_ {
        self.template.template_elements.class_candidates(class_name)
    }
    pub fn template_elements_for_id(&self, id_name: &str) -> impl Iterator<Item = NodeId> + '_ {
        self.template.template_elements.id_candidates(id_name)
    }
    pub fn template_element_previous_siblings(
        &self,
        id: NodeId,
    ) -> impl Iterator<Item = NodeId> + '_ {
        self.template.template_elements.previous_siblings(id)
    }

    pub fn each_index_sym(&self, id: NodeId) -> Option<SymbolId> {
        match self.block_semantics_store.get(id) {
            crate::block_semantics::BlockSemantics::Each(sem) => match sem.index {
                crate::block_semantics::EachIndexKind::Declared { sym, .. } => Some(sym),
                crate::block_semantics::EachIndexKind::Absent => None,
            },
            _ => None,
        }
    }

    pub fn each_block_for_index_sym(&self, sym: SymbolId) -> Option<NodeId> {
        self.block_semantics_store.block_for_each_index_sym(sym)
    }

    pub fn each_is_destructured(&self, id: NodeId) -> bool {
        matches!(
            self.block_semantics_store.get(id),
            crate::block_semantics::BlockSemantics::Each(sem)
                if matches!(sem.item, crate::block_semantics::EachItemKind::Pattern(_)),
        )
    }
    pub fn bind_each_context(&self, id: NodeId) -> Option<&[SymbolId]> {
        self.template.bind_semantics.bind_this_each_context(id)
    }
    pub fn bind_target_semantics(&self, id: NodeId) -> Option<&BindTargetSemantics> {
        self.template.bind_semantics.bind_target_semantics(id)
    }

    pub fn parent_each_blocks(&self, id: NodeId) -> SmallVec<[NodeId; 4]> {
        let Some(info) = self.attr_expressions.get(id) else {
            return SmallVec::new();
        };
        let ref_symbols = info.ref_symbols();
        self.ancestors(id)
            .filter(|parent| parent.kind == ParentKind::EachBlock)
            .filter_map(|parent| {
                ref_symbols
                    .iter()
                    .any(|&sym| self.reactivity.contextual_owner(sym) == Some(parent.id))
                    .then_some(parent.id)
            })
            .collect()
    }
    pub fn css_hash(&self) -> &str {
        &self.output.css.hash
    }
    pub fn is_css_scoped(&self, id: NodeId) -> bool {
        self.output.css.scoped_elements.contains(&id)
    }
    pub fn inject_styles(&self) -> bool {
        self.output.css.inject_styles
    }
    pub fn attr_expression(&self, id: NodeId) -> Option<&ExpressionInfo> {
        self.attr_expressions.get(id)
    }
    pub fn node_ref_symbols(&self, id: NodeId) -> &[SymbolId] {
        self.template.template_semantics.node_ref_symbols(id)
    }
    pub fn shorthand_symbol(&self, id: NodeId) -> Option<SymbolId> {
        self.template
            .template_semantics
            .node_ref_symbols(id)
            .first()
            .copied()
    }
    pub fn bind_target_symbol(&self, id: NodeId) -> Option<SymbolId> {
        self.attr_expressions
            .get(id)
            .and_then(|info| {
                info.is_identifier()
                    .then(|| info.ref_symbols().first().copied())
            })
            .flatten()
            .or_else(|| self.shorthand_symbol(id))
    }
    pub fn attr_is_import(&self, attr_id: NodeId) -> bool {
        self.attr_expressions.get(attr_id).is_some_and(|info| {
            info.ref_symbols()
                .first()
                .copied()
                .or_else(|| {
                    info.identifier_name().and_then(|name| {
                        self.scoping
                            .find_binding(self.scoping.root_scope_id(), name)
                    })
                })
                .is_some_and(|sym| self.scoping.is_import(sym))
        })
    }
    pub fn attr_is_function(&self, attr_id: NodeId) -> bool {
        self.attr_expressions.get(attr_id).is_some_and(|info| {
            info.ref_symbols()
                .first()
                .copied()
                .or_else(|| {
                    info.identifier_name().and_then(|name| {
                        self.scoping
                            .find_binding(self.scoping.root_scope_id(), name)
                    })
                })
                .is_some_and(|sym| {
                    self.scoping
                        .symbol_flags(sym)
                        .contains(SymbolFlags::Function)
                })
        })
    }
    pub fn expr_deps(&self, site: ExprSite) -> Option<ExprDeps<'_>> {
        match site {
            ExprSite::Node(id) => {
                let info = self.expressions.get(id)?;
                let blockers = self.collect_blockers(info.ref_symbols());
                Some(ExprDeps {
                    info,
                    blockers,
                    needs_memo: info.needs_memoized_value()
                        && (info.has_await() || !info.ref_symbols().is_empty()),
                })
            }
            ExprSite::Attr(id) => {
                let info = self.attr_expressions.get(id)?;
                let blockers = self.collect_blockers(info.ref_symbols());
                Some(ExprDeps {
                    info,
                    blockers,
                    needs_memo: self.dynamism.is_dynamic_attr(id) && info.needs_memoized_value(),
                })
            }
        }
    }
    pub fn component_attr_needs_memo(&self, attr_id: NodeId) -> bool {
        self.attr_expressions.get(attr_id).is_some_and(|e| {
            e.has_call() || (!e.is_simple_shape() && self.dynamism.is_dynamic_attr(attr_id))
        })
    }
    pub fn needs_expr_memoization(&self, id: NodeId) -> bool {
        self.expr_deps(ExprSite::Node(id))
            .is_some_and(|deps| deps.needs_memo)
    }
    pub fn expr_has_blockers(&self, id: NodeId) -> bool {
        self.expr_deps(ExprSite::Node(id))
            .is_some_and(|deps| deps.has_blockers())
    }
    pub fn expression_blockers(&self, id: NodeId) -> SmallVec<[u32; 2]> {
        self.expr_deps(ExprSite::Node(id))
            .map(|deps| deps.blockers)
            .unwrap_or_default()
    }
    pub fn attr_expression_blockers(&self, id: NodeId) -> SmallVec<[u32; 2]> {
        self.expr_deps(ExprSite::Attr(id))
            .map(|deps| deps.blockers)
            .unwrap_or_default()
    }
    fn collect_blockers(&self, ref_symbols: &[SymbolId]) -> SmallVec<[u32; 2]> {
        let mut result = SmallVec::new();
        if !self.script.blocker_data.has_async {
            return result;
        }
        for sym in ref_symbols {
            if let Some(&idx) = self.script.blocker_data.symbol_blockers.get(sym)
                && !result.contains(&idx)
            {
                result.push(idx);
            }
        }
        result.sort_unstable();
        result
    }
    pub fn known_value(&self, name: &str) -> Option<&str> {
        let root = self.scoping.root_scope_id();
        let sym_id = self.scoping.find_binding(root, name)?;
        self.scoping.known_value_by_sym(sym_id)
    }
    pub fn effective_fragment_scope(
        &self,
        fragment_id: svelte_ast::FragmentId,
        parent_scope: oxc_semantic::ScopeId,
    ) -> oxc_semantic::ScopeId {
        self.scoping
            .fragment_scope_by_id(fragment_id)
            .unwrap_or(parent_scope)
    }
    pub fn fragment_references_any_symbol(
        &self,
        store: &svelte_ast::AstStore,
        fragment_id: svelte_ast::FragmentId,
        syms: &FxHashSet<SymbolId>,
    ) -> bool {
        if syms.is_empty() {
            return false;
        }
        let lowered = crate::passes::fragment_topology::fragment_items(store, fragment_id);
        for id in lowered {
            match store.get(id) {
                svelte_ast::Node::ExpressionTag(_) => {
                    if self.node_expr_references_syms(id, syms) {
                        return true;
                    }
                }
                svelte_ast::Node::Element(el) => {
                    if self.fragment_references_any_symbol(store, el.fragment, syms) {
                        return true;
                    }
                }
                svelte_ast::Node::SlotElementLegacy(el) => {
                    if self.fragment_references_any_symbol(store, el.fragment, syms) {
                        return true;
                    }
                }
                svelte_ast::Node::SvelteFragmentLegacy(el) => {
                    if self.fragment_references_any_symbol(store, el.fragment, syms) {
                        return true;
                    }
                }
                svelte_ast::Node::IfBlock(block) => {
                    if self.node_expr_references_syms(id, syms)
                        || self.fragment_references_any_symbol(store, block.consequent, syms)
                        || block.alternate.is_some_and(|alt| {
                            self.fragment_references_any_symbol(store, alt, syms)
                        })
                    {
                        return true;
                    }
                }
                svelte_ast::Node::EachBlock(block) => {
                    if self.node_expr_references_syms(id, syms)
                        || self.fragment_references_any_symbol(store, block.body, syms)
                        || block
                            .fallback
                            .is_some_and(|fb| self.fragment_references_any_symbol(store, fb, syms))
                    {
                        return true;
                    }
                }
                svelte_ast::Node::RenderTag(_) | svelte_ast::Node::HtmlTag(_) => {
                    if self.node_expr_references_syms(id, syms) {
                        return true;
                    }
                }
                svelte_ast::Node::KeyBlock(block) => {
                    if self.node_expr_references_syms(id, syms)
                        || self.fragment_references_any_symbol(store, block.fragment, syms)
                    {
                        return true;
                    }
                }
                svelte_ast::Node::SvelteElement(el) => {
                    if self.node_expr_references_syms(id, syms)
                        || self.fragment_references_any_symbol(store, el.fragment, syms)
                    {
                        return true;
                    }
                }
                svelte_ast::Node::SvelteBoundary(b) => {
                    if self.fragment_references_any_symbol(store, b.fragment, syms) {
                        return true;
                    }
                }
                svelte_ast::Node::ComponentNode(cn) => {
                    if self.fragment_references_any_symbol(store, cn.fragment, syms) {
                        return true;
                    }
                }
                svelte_ast::Node::AwaitBlock(_) => {
                    if self.node_expr_references_syms(id, syms) {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }
    fn node_expr_references_syms(&self, id: NodeId, syms: &FxHashSet<SymbolId>) -> bool {
        self.expressions
            .get(id)
            .is_some_and(|info| info.ref_symbols().iter().any(|s| syms.contains(s)))
    }
}
