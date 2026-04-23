use oxc_ast::ast::{ArrowFunctionExpression, Function};
use oxc_ast_visit::walk::{walk_arrow_function_expression, walk_function};
use oxc_ast_visit::Visit;
use oxc_semantic::{ScopeFlags, SymbolId};
use rustc_hash::FxHashSet;
use svelte_ast::{Attribute, Component, ConcatPart, Fragment, Node, StyleDirectiveValue};
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::{AnalysisData, JsAst};

pub(super) fn validate(
    component: &Component,
    data: &AnalysisData<'_>,
    parsed: &JsAst<'_>,
    runes: bool,
    diags: &mut Vec<Diagnostic>,
) {
    if !runes {
        return;
    }

    let Some(_program) = &parsed.program else {
        return;
    };

    let mut validator = TemplateValidator {
        component,
        data,
        parsed,
        diags,
        warned: FxHashSet::default(),
    };
    validator.visit_fragment(&component.fragment, false);
}

struct TemplateValidator<'a, 'b> {
    component: &'a Component,
    data: &'a AnalysisData<'a>,
    parsed: &'a JsAst<'a>,
    diags: &'b mut Vec<Diagnostic>,
    warned: FxHashSet<SymbolId>,
}

impl<'a> TemplateValidator<'a, '_> {
    fn visit_fragment(&mut self, fragment: &Fragment, in_dynamic_block: bool) {
        for &id in &fragment.nodes {
            match self.component.store.get(id) {
                Node::Text(_) | Node::Comment(_) | Node::DebugTag(_) | Node::Error(_) => {}
                Node::ExpressionTag(tag) => {
                    self.visit_expr_ref(&tag.expression, false, in_dynamic_block);
                }
                Node::HtmlTag(tag) => {
                    self.visit_expr_ref(&tag.expression, false, in_dynamic_block);
                }
                Node::RenderTag(tag) => {
                    self.visit_expr_ref(&tag.expression, false, in_dynamic_block);
                }
                Node::ConstTag(tag) => {
                    self.visit_stmt_ref(&tag.decl, false, in_dynamic_block);
                }
                Node::Element(el) => {
                    self.visit_attributes(&el.attributes, in_dynamic_block);
                    self.visit_fragment(&el.fragment, in_dynamic_block);
                }
                Node::SlotElementLegacy(el) => {
                    self.visit_attributes(&el.attributes, in_dynamic_block);
                    self.visit_fragment(&el.fragment, in_dynamic_block);
                }
                Node::ComponentNode(node) => {
                    self.visit_attributes(&node.attributes, in_dynamic_block);
                    self.visit_fragment(&node.fragment, in_dynamic_block);
                }
                Node::IfBlock(block) => {
                    self.visit_expr_ref(&block.test, false, in_dynamic_block);
                    self.visit_fragment(&block.consequent, true);
                    if let Some(alt) = &block.alternate {
                        self.visit_fragment(alt, true);
                    }
                }
                Node::EachBlock(block) => {
                    self.visit_expr_ref(&block.expression, false, in_dynamic_block);
                    if let Some(r) = block.context.as_ref() {
                        self.visit_stmt_ref(r, false, true);
                    }
                    if let Some(r) = block.index.as_ref() {
                        self.visit_stmt_ref(r, false, true);
                    }
                    if let Some(r) = block.key.as_ref() {
                        self.visit_expr_ref(r, false, true);
                    }
                    self.visit_fragment(&block.body, true);
                    if let Some(fragment) = &block.fallback {
                        self.visit_fragment(fragment, true);
                    }
                }
                Node::SnippetBlock(block) => {
                    self.visit_stmt_ref(&block.decl, false, in_dynamic_block);
                    self.visit_fragment(&block.body, in_dynamic_block);
                }
                Node::KeyBlock(block) => {
                    self.visit_expr_ref(&block.expression, false, in_dynamic_block);
                    self.visit_fragment(&block.fragment, true);
                }
                Node::SvelteHead(head) => {
                    self.visit_fragment(&head.fragment, in_dynamic_block);
                }
                Node::SvelteFragmentLegacy(node) => {
                    self.visit_attributes(&node.attributes, in_dynamic_block);
                    self.visit_fragment(&node.fragment, in_dynamic_block);
                }
                Node::SvelteElement(el) => {
                    if let Some(tag_ref) = el.tag.as_ref() {
                        self.visit_expr_ref(tag_ref, false, in_dynamic_block);
                    }
                    self.visit_attributes(&el.attributes, in_dynamic_block);
                    self.visit_fragment(&el.fragment, in_dynamic_block);
                }
                Node::SvelteWindow(node) => {
                    self.visit_attributes(&node.attributes, in_dynamic_block);
                }
                Node::SvelteDocument(node) => {
                    self.visit_attributes(&node.attributes, in_dynamic_block);
                }
                Node::SvelteBody(node) => {
                    self.visit_attributes(&node.attributes, in_dynamic_block);
                }
                Node::SvelteBoundary(node) => {
                    self.visit_attributes(&node.attributes, in_dynamic_block);
                    self.visit_fragment(&node.fragment, in_dynamic_block);
                }
                Node::AwaitBlock(node) => {
                    self.visit_expr_ref(&node.expression, false, in_dynamic_block);
                    if let Some(fragment) = &node.pending {
                        self.visit_fragment(fragment, true);
                    }
                    if let Some(fragment) = &node.then {
                        self.visit_fragment(fragment, true);
                    }
                    if let Some(fragment) = &node.catch {
                        self.visit_fragment(fragment, true);
                    }
                }
            }
        }
    }

    fn visit_attributes(&mut self, attributes: &[Attribute], in_dynamic_block: bool) {
        for attr in attributes {
            match attr {
                Attribute::ExpressionAttribute(attr) => {
                    self.visit_expr_ref(&attr.expression, false, in_dynamic_block);
                }
                Attribute::ConcatenationAttribute(attr) => {
                    for part in &attr.parts {
                        if let ConcatPart::Dynamic { expr, .. } = part {
                            self.visit_expr_ref(expr, false, in_dynamic_block);
                        }
                    }
                }
                Attribute::SpreadAttribute(attr) => {
                    self.visit_expr_ref(&attr.expression, false, in_dynamic_block);
                }
                Attribute::ClassDirective(attr) => {
                    self.visit_expr_ref(&attr.expression, false, in_dynamic_block);
                }
                Attribute::StyleDirective(attr) => match &attr.value {
                    StyleDirectiveValue::Expression => {
                        self.visit_expr_ref(&attr.expression, false, in_dynamic_block);
                    }
                    StyleDirectiveValue::Concatenation(parts) => {
                        for part in parts {
                            if let ConcatPart::Dynamic { expr, .. } = part {
                                self.visit_expr_ref(expr, false, in_dynamic_block);
                            }
                        }
                    }
                    StyleDirectiveValue::String(_) => {}
                },
                Attribute::BindDirective(attr) => {
                    let bind_this = self
                        .data
                        .bind_target_semantics(attr.id)
                        .is_some_and(|semantics| semantics.is_this());
                    self.visit_expr_ref(&attr.expression, bind_this, in_dynamic_block);
                }
                Attribute::LetDirectiveLegacy(attr) => {
                    if let Some(r) = attr.binding.as_ref() {
                        self.visit_stmt_ref(r, false, in_dynamic_block);
                    }
                }
                Attribute::UseDirective(attr) => {
                    if let Some(r) = attr.expression.as_ref() {
                        self.visit_expr_ref(r, false, in_dynamic_block);
                    }
                }
                Attribute::OnDirectiveLegacy(attr) => {
                    if let Some(r) = attr.expression.as_ref() {
                        self.visit_expr_ref(r, false, in_dynamic_block);
                    }
                }
                Attribute::TransitionDirective(attr) => {
                    if let Some(r) = attr.expression.as_ref() {
                        self.visit_expr_ref(r, false, in_dynamic_block);
                    }
                }
                Attribute::AnimateDirective(attr) => {
                    if let Some(r) = attr.expression.as_ref() {
                        self.visit_expr_ref(r, false, in_dynamic_block);
                    }
                }
                Attribute::AttachTag(attr) => {
                    self.visit_expr_ref(&attr.expression, false, in_dynamic_block);
                }
                Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_) => {}
            }
        }
    }

    fn visit_expr_ref(
        &mut self,
        expr_ref: &svelte_ast::ExprRef,
        bind_this: bool,
        in_dynamic_block: bool,
    ) {
        let Some(expr) = self.parsed.expr(expr_ref.id()) else {
            return;
        };

        let mut visitor = ReferenceVisitor {
            data: self.data,
            diags: self.diags,
            warned: &mut self.warned,
            bind_this,
            in_dynamic_block,
            function_depth: 0,
        };
        visitor.visit_expression(expr);
    }

    fn visit_stmt_ref(
        &mut self,
        stmt_ref: &svelte_ast::StmtRef,
        bind_this: bool,
        in_dynamic_block: bool,
    ) {
        let Some(stmt) = self.parsed.stmt(stmt_ref.id()) else {
            return;
        };

        let mut visitor = ReferenceVisitor {
            data: self.data,
            diags: self.diags,
            warned: &mut self.warned,
            bind_this,
            in_dynamic_block,
            function_depth: 0,
        };
        visitor.visit_statement(stmt);
    }
}

struct ReferenceVisitor<'a, 'b> {
    data: &'a AnalysisData<'a>,
    diags: &'b mut Vec<Diagnostic>,
    warned: &'b mut FxHashSet<SymbolId>,
    bind_this: bool,
    in_dynamic_block: bool,
    function_depth: u32,
}

impl<'a> Visit<'a> for ReferenceVisitor<'_, '_> {
    fn visit_identifier_reference(&mut self, ident: &oxc_ast::ast::IdentifierReference<'a>) {
        let Some(ref_id) = ident.reference_id.get() else {
            return;
        };
        let Some(reference) = self.data.scoping.try_get_reference(ref_id) else {
            return;
        };
        let Some(sym_id) = reference.symbol_id() else {
            return;
        };
        if self.warned.contains(&sym_id) {
            return;
        }
        // `is_mutated_any` catches both JS-side writes (`x = ...`) and
        // template-side member mutations (`bind:value={x.y}` → `x` is
        // member_mutated). Both are legitimate triggers for the
        // non-reactive-update warning when the declaration is plain.
        if !self.data.scoping.is_component_top_level_symbol(sym_id)
            || is_reactive_binding(self.data, sym_id)
            || !self.data.scoping.is_mutated_any(sym_id)
        {
            return;
        }
        if self.function_depth > 0 {
            return;
        }
        if self.bind_this && !self.in_dynamic_block {
            return;
        }

        let decl_span = self.data.scoping.symbol_span(sym_id);
        self.diags.push(Diagnostic::warning(
            DiagnosticKind::NonReactiveUpdate {
                name: self.data.scoping.symbol_name(sym_id).to_string(),
            },
            Span::new(decl_span.start, decl_span.end),
        ));
        self.warned.insert(sym_id);
    }

    fn visit_arrow_function_expression(&mut self, arrow: &ArrowFunctionExpression<'a>) {
        self.function_depth += 1;
        walk_arrow_function_expression(self, arrow);
        self.function_depth -= 1;
    }

    fn visit_function(&mut self, func: &Function<'a>, flags: ScopeFlags) {
        self.function_depth += 1;
        walk_function(self, func, flags);
        self.function_depth -= 1;
    }
}

/// True when `sym` refers to any reactive source (including runes that
/// have been optimized to a plain `let` — they're still reassignable from
/// the outside, so mutating them isn't a non-reactive-update mistake).
fn is_reactive_binding(data: &AnalysisData<'_>, sym: crate::scope::SymbolId) -> bool {
    use crate::types::data::DeclarationSemantics;
    !matches!(
        data.declaration_semantics(data.scoping.symbol_declaration(sym)),
        DeclarationSemantics::NonReactive | DeclarationSemantics::Unresolved,
    )
}
