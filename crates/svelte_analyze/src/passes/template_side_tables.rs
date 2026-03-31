//! Template side tables and symbol marks.
//!
//! Scopes are created by `template_scoping`. Bindings are created by
//! `TemplateSemanticVisitor` via OXC Visit. This pass populates side tables
//! (each_blocks, const_tags) and applies symbol marks
//! (each_block_var, each_non_reactive, snippet_name).
//!
//! Only `$$item` (synthetic destructured-context binding) uses `add_binding`
//! directly — all other bindings go through OXC.
//!
//! Marks that depend on bindings (find_binding) go in leave_* hooks —
//! by that time dispatch_stmt has already created the bindings via OXC Visit.

use oxc_ast::ast::{
    ArrowFunctionExpression, BindingIdentifier, BindingPattern, Expression,
    Statement, VariableDeclarator,
};
use oxc_ast_visit::Visit;
use svelte_ast::{Attribute, ConstTag, EachBlock, Node, SnippetBlock};

use crate::scope::ComponentScoping;
use crate::types::data::{FragmentKey, StmtHandle};
use crate::utils::binding_pattern::collect_binding_names;
use crate::walker::{TemplateVisitor, VisitContext};

pub(crate) struct TemplateSideTablesVisitor<'c> {
    pub component: &'c svelte_ast::Component,
}

/// Extract the first VariableDeclarator from a parsed statement handle.
fn get_declarator<'a>(ctx: &VisitContext<'a>, handle: StmtHandle) -> Option<&'a VariableDeclarator<'a>> {
    ctx.parsed()?
        .stmt(handle)
        .and_then(|stmt| match stmt {
            Statement::VariableDeclaration(decl) => decl.declarations.first(),
            _ => None,
        })
}

impl TemplateVisitor for TemplateSideTablesVisitor<'_> {
    fn visit_each_block(&mut self, block: &EachBlock, ctx: &mut VisitContext<'_>) {
        if let Some(handle) = block.context_span.and_then(|cs| ctx.parsed().and_then(|p| p.stmt_handle(cs.start))) {
            ctx.data.each_context_stmt_handles.insert(block.id, handle);
        }
        if let Some(handle) = block.index_span.and_then(|span| ctx.parsed().and_then(|p| p.stmt_handle(span.start))) {
            ctx.data.each_index_stmt_handles.insert(block.id, handle);
        }
        let is_destructured = block.context_span
            .and_then(|cs| ctx.parsed().and_then(|p| p.stmt_handle(cs.start)))
            .and_then(|handle| get_declarator(ctx, handle))
            .is_some_and(|d| !matches!(&d.id, BindingPattern::BindingIdentifier(_)));

        if is_destructured {
            let child_scope = ctx.data.scoping
                .fragment_scope(&FragmentKey::EachBody(block.id))
                .expect("EachBody scope must exist");
            // $$item is synthetic — no OXC AST node for it
            let ctx_sym = ctx.data.scoping.add_binding(child_scope, "$$item");
            ctx.data.scoping.mark_each_block_var(ctx_sym);
            ctx.data.each_blocks.is_destructured.insert(block.id);
        }

        // Index SymbolId is populated in leave_each_block (after dispatch_stmt creates bindings)

        if block.body.nodes.iter().any(|&nid| {
            if let Node::Element(el) = ctx.store.get(nid) {
                el.attributes.iter().any(|a| matches!(a, Attribute::AnimateDirective(_)))
            } else {
                false
            }
        }) {
            ctx.data.each_blocks.has_animate.insert(block.id);
        }
    }

    fn leave_each_block(&mut self, block: &EachBlock, ctx: &mut VisitContext<'_>) {
        let child_scope = ctx.data.scoping
            .fragment_scope(&FragmentKey::EachBody(block.id))
            .expect("EachBody scope must exist");

        let is_destructured = ctx.data.each_blocks.is_destructured.contains(&block.id);

        if is_destructured {
            // Destructured context: name is always "$$item"
            ctx.data.each_blocks.context_names.insert(block.id, "$$item".to_string());

            // Default bindings use $.derived_safe_equal (signals), non-default use getters.
            if let Some(parsed) = ctx.parsed() {
                if let Some(stmt) = block.context_span
                    .and_then(|cs| parsed.stmt_handle(cs.start))
                    .and_then(|handle| parsed.stmt(handle))
                {
                    let mut marker = DestructuredGetterMarker {
                        scoping: &mut ctx.data.scoping,
                        in_default: false,
                    };
                    marker.visit_statement(stmt);
                }
            }
        } else {
            let ctx_name = block.context_span
                .and_then(|cs| ctx.parsed().and_then(|p| p.stmt_handle(cs.start)))
                .and_then(|handle| get_declarator(ctx, handle))
                .and_then(|d| d.id.get_binding_identifier())
                .map(|ident| ident.name.as_str());

            if let Some(ctx_name) = ctx_name {
                ctx.data.each_blocks.context_names.insert(block.id, ctx_name.to_string());

                if let Some(ctx_sym) = ctx.data.scoping.find_binding(child_scope, ctx_name) {
                    ctx.data.scoping.mark_each_block_var(ctx_sym);

                    // key_is_item: key expression resolves to the same symbol as context
                    if let Some(key_span) = block.key_span {
                        let is_key_item = ctx.parsed()
                            .and_then(|p| p.expr_handle(key_span.start))
                            .and_then(|handle| ctx.parsed().and_then(|p| p.expr(handle)))
                            .and_then(|expr| match expr {
                                Expression::Identifier(ident) => ident.reference_id.get(),
                                _ => None,
                            })
                            .and_then(|ref_id| ctx.data.scoping.get_reference(ref_id).symbol_id())
                            .is_some_and(|sym| sym == ctx_sym);
                        if is_key_item {
                            ctx.data.scoping.mark_each_non_reactive(ctx_sym);
                            ctx.data.each_blocks.key_is_item.insert(block.id);
                        }
                    }
                }
            }
        }

        if let Some(idx_span) = block.index_span {
            let idx_name = ctx.parsed()
                .and_then(|p| p.stmt_handle(idx_span.start))
                .and_then(|handle| get_declarator(ctx, handle))
                .and_then(|d| d.id.get_binding_identifier())
                .map(|ident| ident.name.as_str());
            if let Some(idx_name) = idx_name {
                if let Some(idx_sym) = ctx.data.scoping.find_binding(child_scope, idx_name) {
                    ctx.data.scoping.mark_each_block_var(idx_sym);
                    ctx.data.each_blocks.index_syms.insert(block.id, idx_sym);
                }
            }
        }
    }

    fn leave_snippet_block(&mut self, block: &SnippetBlock, ctx: &mut VisitContext<'_>) {
        if let Some(handle) = ctx.parsed().and_then(|p| p.stmt_handle(block.expression_span.start)) {
            ctx.data.snippet_stmt_handles.insert(block.id, handle);
        }
        let name = block.name(&self.component.source);
        if let Some(name_sym) = ctx.data.scoping.find_binding(ctx.scope, name) {
            ctx.data.scoping.mark_snippet_name(name_sym);
        }
        // Mark snippet params and collect param names
        if let Some(parsed) = ctx.parsed() {
            if let Some(stmt) = parsed
                .stmt_handle(block.expression_span.start)
                .and_then(|handle| parsed.stmt(handle))
            {
                let mut marker = SnippetParamMarker { scoping: &mut ctx.data.scoping };
                marker.visit_statement(stmt);

                // SnippetParamMarker mutates scoping, so names must be collected in a separate pass
                let mut collector = SnippetParamNameCollector { names: Vec::new() };
                collector.visit_statement(stmt);
                if !collector.names.is_empty() {
                    ctx.data.snippets.params.insert(block.id, collector.names);
                }
            }
        }
    }

    fn visit_const_tag(&mut self, tag: &ConstTag, ctx: &mut VisitContext<'_>) {
        if let Some(parsed) = ctx.parsed() {
            if let Some(oxc_ast::ast::Statement::VariableDeclaration(decl)) = parsed
                .stmt_handle(tag.expression_span.start)
                .and_then(|handle| parsed.stmt(handle))
            {
                if let Some(declarator) = decl.declarations.first() {
                    let mut name_strings = Vec::new();
                    collect_binding_names(&declarator.id, &mut name_strings);
                    ctx.data.const_tags.names.insert(tag.id, name_strings);
                }
            }
        }
    }
}

/// OXC Visit that collects param names from `const name = (a, b) => {}`.
/// Mirrors `SnippetParamMarker` descent but collects names instead of marking symbols.
struct SnippetParamNameCollector {
    names: Vec<String>,
}

impl<'a> Visit<'a> for SnippetParamNameCollector {
    fn visit_binding_identifier(&mut self, ident: &BindingIdentifier<'a>) {
        self.names.push(ident.name.to_string());
    }

    fn visit_variable_declarator(&mut self, decl: &VariableDeclarator<'a>) {
        // Skip decl.id — snippet name is not a param
        if let Some(init) = &decl.init {
            self.visit_expression(init);
        }
    }

    fn visit_arrow_function_expression(&mut self, arrow: &ArrowFunctionExpression<'a>) {
        // Only visit params — skip body to avoid collecting inner bindings
        self.visit_formal_parameters(&arrow.params);
    }
}

/// OXC Visit that marks arrow function param bindings as snippet params.
/// Descends through VariableDeclaration → ArrowFunctionExpression → params only.
struct SnippetParamMarker<'s> {
    scoping: &'s mut ComponentScoping,
}

impl<'a> Visit<'a> for SnippetParamMarker<'_> {
    fn visit_binding_identifier(&mut self, ident: &BindingIdentifier<'a>) {
        if let Some(sym_id) = ident.symbol_id.get() {
            self.scoping.mark_getter(sym_id);
        }
    }

    fn visit_variable_declarator(&mut self, decl: &VariableDeclarator<'a>) {
        // Skip decl.id — snippet name is not a param
        if let Some(init) = &decl.init {
            self.visit_expression(init);
        }
    }

    fn visit_arrow_function_expression(&mut self, arrow: &ArrowFunctionExpression<'a>) {
        // Only visit params — skip body to avoid marking inner bindings
        self.visit_formal_parameters(&arrow.params);
    }
}

/// OXC Visit that marks non-default destructured each bindings as getters.
/// Default bindings (`{ value = "N/A" }`) become $.derived_safe_equal (signals, not getters).
struct DestructuredGetterMarker<'s> {
    scoping: &'s mut ComponentScoping,
    in_default: bool,
}

impl<'a> Visit<'a> for DestructuredGetterMarker<'_> {
    fn visit_binding_identifier(&mut self, ident: &BindingIdentifier<'a>) {
        if !self.in_default {
            if let Some(sym_id) = ident.symbol_id.get() {
                self.scoping.mark_getter(sym_id);
            }
        }
    }

    fn visit_assignment_pattern(&mut self, pat: &oxc_ast::ast::AssignmentPattern<'a>) {
        // Default value pattern — bindings inside are signals, not getters
        self.in_default = true;
        self.visit_binding_pattern(&pat.left);
        self.in_default = false;
    }

    fn visit_variable_declarator(&mut self, decl: &VariableDeclarator<'a>) {
        // Skip declarator init (the `= x` part) — only visit the pattern
        self.visit_binding_pattern(&decl.id);
    }
}
