pub mod ancestor;
pub mod compute_optimization;
pub mod context;
pub mod svelte_table;
pub mod visitor;

use std::mem::take;

use ast_builder::Builder;
use compute_optimization::{compute_optimization, ContentType};
use context::VisitorContext;
use oxc_ast::{
    ast::{BindingPatternKind, Expression, IdentifierReference, VariableDeclarator},
    visit::walk::{walk_assignment_expression, walk_call_expression, walk_update_expression},
    Visit,
};
use oxc_semantic::{ReferenceFlags, SemanticBuilder};
use svelte_table::{RuneKind, SvelteTable};

use ast::{
    metadata::{
        AttributeMetadata, ElementMetadata, FragmentAnchor, FragmentMetadata,
        InterpolationMetadata, InterpolationSetterKind, WithMetadata,
    },
    Ast, ExpressionAttribute,
};
use visitor::{walk::walk_template, TemplateVisitor};

#[derive(Debug, Clone, Copy, Default)]
pub struct ExpressionFlags {
    pub has_reactivity: bool,
    pub has_call_expression: bool,
}

impl ExpressionFlags {
    pub fn empty() -> Self {
        Self {
            has_call_expression: false,
            has_reactivity: false,
        }
    }
}

pub struct Analyzer<'a> {
    b: &'a Builder<'a>,
}

pub struct AnalyzeResult {
    pub svelte_table: SvelteTable,
}

impl<'alloc> Analyzer<'alloc> {
    pub fn new(b: &'alloc Builder<'alloc>) -> Self {
        Self { b }
    }

    pub fn analyze<'link>(&self, ast: &'link mut Ast<'alloc>) -> AnalyzeResult {
        let empty = self.b.program(vec![]);
        let program = ast
            .script
            .as_ref()
            .map(|script| &script.program)
            .unwrap_or_else(|| &empty);

        let ret = SemanticBuilder::new().build(program);

        if !ret.errors.is_empty() {
            todo!();
        }

        let (symbols, scopes) = ret.semantic.into_symbol_table_and_scope_tree();
        let mut svelte_table = SvelteTable::new(symbols, scopes);

        let mut script_visitor = ScriptVisitorImpl {
            svelte_table: &mut svelte_table,
        };

        script_visitor.visit_program(program);

        let mut template_visitor = TemplateVisitorImpl {
            current_reference_flags: ReferenceFlags::empty(),
            current_expression_flags: ExpressionFlags::empty(),
            svelte_table: script_visitor.svelte_table,
            current_concatenation_metadata: AttributeMetadata::default(),
        };

        let mut ctx = VisitorContext::new();

        walk_template(&mut template_visitor, &mut ast.template, &mut ctx);

        AnalyzeResult { svelte_table }
    }
}

pub struct ScriptVisitorImpl<'link> {
    pub svelte_table: &'link mut SvelteTable,
}

pub struct TemplateVisitorImpl<'link> {
    pub svelte_table: &'link mut SvelteTable,
    current_reference_flags: ReferenceFlags,
    current_expression_flags: ExpressionFlags,
    current_concatenation_metadata: AttributeMetadata,
}

impl<'a> Visit<'a> for ScriptVisitorImpl<'_> {
    fn visit_variable_declarator(&mut self, declarator: &VariableDeclarator<'a>) {
        if let Some(Expression::CallExpression(call)) = &declarator.init {
            if call.callee_name() == Some("$state") {
                if let BindingPatternKind::BindingIdentifier(id) = &declarator.id.kind {
                    let symbol_id = id.symbol_id();

                    self.svelte_table.add_rune(symbol_id, RuneKind::State);
                }
            }
        }
    }
}

impl<'a> TemplateVisitor<'a> for TemplateVisitorImpl<'_> {
    fn enter_fragment(&mut self, it: &mut ast::Fragment<'a>, ctx: &mut VisitorContext) {
        self.analyze_fragment(it, ctx.parent().is_template());
    }

    fn enter_element(&mut self, it: &mut ast::Element<'a>, _ctx: &mut VisitorContext) {
        self.svelte_table
            .add_optimization(it.node_id(), compute_optimization(&it.nodes));
    }

    fn exit_element(&mut self, it: &mut ast::Element<'a>, ctx: &mut VisitorContext) {
        let flags = ctx.resolve_element_flags(it.node_id());
        let mut metadata = ElementMetadata::default();
        let optimizations = self.svelte_table.get_optimization(it.node_id()).unwrap();

        metadata.has_dynamic_nodes = flags.dynamic;
        metadata.need_reset = flags.dynamic && optimizations.content_type.is_non_text();
        metadata.need_remove_input_defaults =
            it.kind.is_input() && flags.possible_remove_input_defaults;

        if flags.dynamic {
            ctx.mark_parent_element_as_dynamic();
        }

        it.set_metadata(metadata);
    }

    fn enter_expression(&mut self, it: &Expression<'a>, ctx: &mut VisitorContext) {
        ctx.mark_parent_element_as_dynamic();
        Visit::visit_expression(self, it);
    }

    fn exit_class_directive_attribute(
        &mut self,
        it: &mut ast::ClassDirective<'a>,
        _ctx: &mut VisitorContext,
    ) {
        let flags = self.resolve_expression_flags();

        it.set_metadata(AttributeMetadata {
            has_reactivity: flags.has_reactivity,
        });
    }

    fn enter_bind_directive_attribute(
        &mut self,
        _it: &mut ast::BindDirective<'a>,
        _ctx: &mut VisitorContext<'a>,
    ) {
        self.current_reference_flags = ReferenceFlags::read_write();
    }

    fn exit_bind_directive_attribute(
        &mut self,
        it: &mut ast::BindDirective<'a>,
        ctx: &mut VisitorContext<'a>,
    ) {
        let flags = self.resolve_expression_flags();

        let element_flags = ctx.parent_element_flags().unwrap();

        element_flags.set_possible_remove_input_defaults_by_directive_kind(&it.kind);

        if it.kind.is_group() {
            self.svelte_table.set_need_binding_group();
        }

        it.set_metadata(AttributeMetadata {
            has_reactivity: flags.has_reactivity,
        });
    }

    fn exit_interpolation(&mut self, it: &mut ast::Interpolation<'a>, ctx: &mut VisitorContext) {
        let flags = self.resolve_expression_flags();

        let parent = ctx.parent();
        let node_id = parent.get_node_id();
        let optimizations = self.svelte_table.get_optimization(node_id).unwrap();

        let need_template =
            !optimizations.content_type.is_compressible_sequence() || flags.has_reactivity;

        let setter_kind = if flags.has_reactivity {
            InterpolationSetterKind::SetText
        } else if parent.is_fragment_owner() {
            InterpolationSetterKind::NodeValue
        } else if optimizations.content_type.is_compressible_sequence() {
            InterpolationSetterKind::TextContent
        } else {
            InterpolationSetterKind::NodeValue
        };

        it.set_metadata(InterpolationMetadata {
            setter_kind,
            need_template,
        });
    }

    fn exit_expression_attribute(
        &mut self,
        it: &mut ExpressionAttribute<'a>,
        ctx: &mut VisitorContext,
    ) {
        let flags = self.resolve_expression_flags();

        let element_flags = ctx.parent_element_flags().unwrap();

        element_flags.set_possible_remove_input_defaults_by_attribute_kind(&it.kind);

        it.set_metadata(AttributeMetadata {
            has_reactivity: flags.has_reactivity,
        });
    }

    fn exit_concatenation_attribute(
        &mut self,
        it: &mut ast::ConcatenationAttribute<'a>,
        ctx: &mut VisitorContext,
    ) {
        let metadata = take(&mut self.current_concatenation_metadata);

        let element_flags = ctx.parent_element_flags().unwrap();

        element_flags.set_possible_remove_input_defaults_by_attribute_kind(&it.kind);

        it.set_metadata(metadata);
    }

    fn exit_expression_concatenation_part(
        &mut self,
        _it: &Expression<'a>,
        _ctx: &mut VisitorContext,
    ) {
        let flags = self.resolve_expression_flags();

        self.current_concatenation_metadata.has_reactivity = flags.has_reactivity;
    }
}

impl<'a> Visit<'a> for TemplateVisitorImpl<'_> {
    fn visit_identifier_reference(&mut self, it: &IdentifierReference<'a>) {
        self.reference_identifier(it);
    }

    fn visit_update_expression(&mut self, it: &oxc_ast::ast::UpdateExpression<'a>) {
        self.current_reference_flags = ReferenceFlags::read_write();
        walk_update_expression(self, it);
    }

    fn visit_assignment_expression(&mut self, it: &oxc_ast::ast::AssignmentExpression<'a>) {
        self.current_reference_flags = ReferenceFlags::write();
        walk_assignment_expression(self, it);
    }

    fn visit_call_expression(&mut self, it: &oxc_ast::ast::CallExpression<'a>) {
        self.current_expression_flags.has_call_expression = true;
        walk_call_expression(self, it);
    }
}

impl<'a> TemplateVisitorImpl<'_> {
    fn reference_identifier(&mut self, ident: &IdentifierReference<'a>) {
        let flags = self.resolve_reference_usages();

        let option = self
            .svelte_table
            .add_root_scope_reference(&ident.name, flags);

        if let Some(reference_id) = option {
            ident.reference_id.set(Some(reference_id));

            if self.svelte_table.is_rune_reference(reference_id) {
                self.current_expression_flags.has_reactivity = true;
            }
        }
    }

    fn resolve_expression_flags(&mut self) -> ExpressionFlags {
        take(&mut self.current_expression_flags)
    }

    fn resolve_reference_usages(&mut self) -> ReferenceFlags {
        if self.current_reference_flags.is_empty() {
            ReferenceFlags::Read
        } else {
            take(&mut self.current_reference_flags)
        }
    }

    fn analyze_fragment(&mut self, it: &mut ast::Fragment<'a>, root: bool) {
        let mut metadata = FragmentMetadata::default();
        let optimizations = compute_optimization(&it.nodes);

        metadata.need_start_with_next = optimizations.start_with_compressible && root;
        metadata.is_empty = optimizations.length == 0;

        metadata.anchor = match optimizations.content_type {
            ContentType::Mixed => FragmentAnchor::Fragment,
            ContentType::TextAndInterpolation => FragmentAnchor::Text,
            ContentType::Text => FragmentAnchor::TextInline,
            ContentType::Interpolation => FragmentAnchor::Text,
            ContentType::Element => FragmentAnchor::Element,
            ContentType::Nope => FragmentAnchor::Fragment,
            ContentType::NodeWithFragment => FragmentAnchor::Comment,
        };

        self.svelte_table
            .add_optimization(it.node_id(), optimizations);

        it.set_metadata(metadata);
    }
}

#[cfg(test)]
mod tests {
    use ast::Node;
    use oxc_allocator::Allocator;

    use oxc_ast::AstBuilder;
    use parser::Parser;

    use super::*;

    #[test]
    fn analyze_smoke() {
        let allocator = Allocator::default();
        let ast_builder = AstBuilder::new(&allocator);
        let builder = Builder::new(ast_builder);
        let analyzer = Analyzer::new(&builder);
        let mut parser = Parser::new(
            "<script>let rune_var = $state(10); onMount(() => rune_var = 0);</script>",
            &allocator,
        );
        let mut ast = parser.parse().unwrap();
        let result = analyzer.analyze(&mut ast);

        assert!(!result.svelte_table.runes.is_empty());

        for (id, _rune) in result.svelte_table.runes.iter() {
            assert_eq!(result.svelte_table.symbols.get_name(*id), "rune_var");
        }
    }

    #[test]
    fn metadata_test() {
        let allocator = Allocator::default();
        let ast_builder = AstBuilder::new(&allocator);
        let builder = Builder::new(ast_builder);
        let analyzer = Analyzer::new(&builder);
        let mut parser = Parser::new(
            r#"<div><h1>
                    title
                </h1><div>
                    {name}
                </div>
            </div><span>
                text
            </span>"#,
            &allocator,
        );
        let mut ast = parser.parse().unwrap();

        analyzer.analyze(&mut ast);

        let Node::Element(root_div) = &ast.template.borrow().nodes[0] else {
            unreachable!()
        };

        let Node::Element(root_span) = &ast.template.borrow().nodes[1] else {
            unreachable!()
        };

        let Node::Element(sub_h1) = &root_div.borrow().nodes[0] else {
            unreachable!()
        };

        let Node::Element(sub_div) = &root_div.borrow().nodes[1] else {
            unreachable!()
        };

        assert!(root_div.borrow().get_metadata().has_dynamic_nodes);
        assert!(!root_span.borrow().get_metadata().has_dynamic_nodes);
        assert!(!sub_h1.borrow().get_metadata().has_dynamic_nodes);
        assert!(sub_div.borrow().get_metadata().has_dynamic_nodes);
    }
}
