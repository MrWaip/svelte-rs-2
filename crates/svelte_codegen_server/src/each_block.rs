use oxc_ast::NONE;
use oxc_ast::ast::{BindingPattern, Expression, Statement, VariableDeclarationKind};
use oxc_span::SPAN;
use oxc_syntax::operator::{BinaryOperator, UpdateOperator};
use svelte_analyze::{
    BlockSemantics, EachAsyncKind, EachBlockSemantics, EachFlavor, EachIndexKind,
};
use svelte_ast::EachBlock;
use svelte_ast_builder::Arg;

use crate::error::{CodegenError, Result};
use crate::fragment::FragmentParent;
use crate::model::ServerCodegen;

const LENGTH_NAME: &str = "$$length";
const INTERNAL_INDEX_NAME: &str = "$$index";

impl<'a> ServerCodegen<'a> {
    pub(crate) fn reserve_each_index_names(&mut self) {
        let root = self.component.root;
        self.reserve_each_index_in_fragment(root);
    }

    fn reserve_each_index_in_fragment(&mut self, fragment: svelte_ast::FragmentId) {
        let node_ids: Vec<svelte_ast::NodeId> =
            self.component.store.fragment(fragment).nodes.to_vec();
        for id in node_ids {
            self.reserve_each_index_in_node(id);
        }
    }

    fn reserve_bind_pairs_in_attributes(&mut self, attributes: &'a [svelte_ast::Attribute]) {
        use svelte_analyze::{AttributeSemantics, ComponentBindKind};
        use svelte_ast::Attribute;
        for attr in attributes {
            let Attribute::BindDirective(directive) = attr else {
                continue;
            };
            let AttributeSemantics::ComponentBind(sem) = self.analysis.attributes.get(directive.id)
            else {
                continue;
            };
            if !matches!(sem.kind, ComponentBindKind::FunctionPair) {
                continue;
            }
            let get_name = self.ident_gen.generate("bind_get");
            let set_name = self.ident_gen.generate("bind_set");
            self.bind_pair_names
                .insert(directive.id, (get_name, set_name));
        }
    }

    fn reserve_each_index_in_node(&mut self, id: svelte_ast::NodeId) {
        use svelte_ast::Node;
        match self.component.store.get(id) {
            Node::Element(el) => self.reserve_each_index_in_fragment(el.fragment),
            Node::SlotElementLegacy(el) => self.reserve_each_index_in_fragment(el.fragment),
            Node::SvelteElement(el) => self.reserve_each_index_in_fragment(el.fragment),
            Node::SvelteFragmentLegacy(el) => self.reserve_each_index_in_fragment(el.fragment),
            Node::SvelteHead(head) => self.reserve_each_index_in_fragment(head.fragment),
            Node::SvelteBoundary(b) => self.reserve_each_index_in_fragment(b.fragment),
            Node::KeyBlock(block) => self.reserve_each_index_in_fragment(block.fragment),
            Node::SnippetBlock(block) => self.reserve_each_index_in_fragment(block.body),
            Node::IfBlock(block) => {
                self.reserve_each_index_in_fragment(block.consequent);
                if let Some(alt) = block.alternate {
                    self.reserve_each_index_in_fragment(alt);
                }
            }
            Node::AwaitBlock(block) => {
                if let Some(p) = block.pending {
                    self.reserve_each_index_in_fragment(p);
                }
                if let Some(t) = block.then {
                    self.reserve_each_index_in_fragment(t);
                }
                if let Some(c) = block.catch {
                    self.reserve_each_index_in_fragment(c);
                }
            }
            Node::ComponentNode(cn) => {
                self.reserve_bind_pairs_in_attributes(&cn.attributes);
                if let Some(view) = self.component.store.get(id).as_component_like() {
                    self.reserve_each_index_in_fragment(view.fragment);
                    for slot in view.legacy_slots {
                        self.reserve_each_index_in_fragment(slot.fragment);
                    }
                }
            }
            Node::SvelteComponentLegacy(_) | Node::SvelteSelf(_) => {
                if let Some(view) = self.component.store.get(id).as_component_like() {
                    self.reserve_each_index_in_fragment(view.fragment);
                    for slot in view.legacy_slots {
                        self.reserve_each_index_in_fragment(slot.fragment);
                    }
                }
            }
            Node::EachBlock(block) => {
                let body = block.body;
                let fallback = block.fallback;
                let block_id = block.id;
                let array_name = self.ident_gen.generate("each_array");
                self.each_array_names.insert(block_id, array_name);
                self.reserve_each_index_in_fragment(body);
                if let Some(fb) = fallback {
                    self.reserve_each_index_in_fragment(fb);
                }
                let name = self.ident_gen.generate("$$index");
                self.each_index_names.insert(block_id, name);
            }
            Node::Text(_)
            | Node::Comment(_)
            | Node::ExpressionTag(_)
            | Node::RenderTag(_)
            | Node::HtmlTag(_)
            | Node::ConstTag(_)
            | Node::DeclarationTag(_)
            | Node::DebugTag(_)
            | Node::SvelteWindow(_)
            | Node::SvelteDocument(_)
            | Node::SvelteBody(_)
            | Node::Error(_) => {}
        }
    }

    pub(crate) fn each_block(&mut self, block: &'a EachBlock) -> Result<()> {
        let sem = match self.analysis.block_semantics(block.id) {
            BlockSemantics::Each(sem) => sem.clone(),
            _ => return Err(CodegenError::Unsupported(block.id, "each block")),
        };
        let async_blockers = match &sem.async_kind {
            EachAsyncKind::Sync => None,
            EachAsyncKind::Awaited { blockers } => Some(blockers.clone()),
            EachAsyncKind::Deferred { .. } => {
                return Err(CodegenError::Unsupported(block.id, "deferred each block"));
            }
        };
        if async_blockers.is_some() && block.fallback.is_some() {
            return Err(CodegenError::Unsupported(
                block.id,
                "async each with fallback",
            ));
        }

        let array_name = self
            .each_array_names
            .get(&block.id)
            .cloned()
            .unwrap_or_else(|| self.gen_ident("each_array"));
        let mut collection = self.take_expression(block.id, &block.expression)?;
        if async_blockers.is_some() {
            collection = self.save_block_await(collection);
        }
        let ensure = self
            .b
            .call_expr("$.ensure_array_like", [Arg::Expr(collection)]);
        let array_decl = self.b.const_stmt(&array_name, ensure);

        let for_loop = self.build_each_for_loop(block, &sem, &array_name)?;

        if let Some(blockers) = async_blockers {
            self.push_text("<!--[-->");
            let wrapped = self.wrap_async_block(vec![array_decl, for_loop], &blockers);
            self.push_stmt(wrapped);
        } else if block.fallback.is_some() {
            let guard = self.build_each_fallback(block, &array_name, for_loop)?;
            self.push_stmt(array_decl);
            self.push_stmt(guard);
        } else {
            self.push_text("<!--[-->");
            self.push_stmt(array_decl);
            self.push_stmt(for_loop);
        }
        self.push_text("<!--]-->");
        Ok(())
    }

    fn build_each_for_loop(
        &mut self,
        block: &'a EachBlock,
        sem: &EachBlockSemantics,
        array_name: &str,
    ) -> Result<Statement<'a>> {
        let internal_index = self
            .each_index_names
            .get(&block.id)
            .cloned()
            .unwrap_or_else(|| INTERNAL_INDEX_NAME.to_string());
        let (loop_index_name, group_rebind) = match (sem.flavor, &sem.index) {
            (EachFlavor::BindGroup, EachIndexKind::Declared { sym, .. }) => (
                internal_index,
                Some(self.analysis.scoping.symbol_name(*sym).to_string()),
            ),
            (EachFlavor::Regular, EachIndexKind::Declared { sym, .. }) => {
                (self.analysis.scoping.symbol_name(*sym).to_string(), None)
            }
            (EachFlavor::BindGroup | EachFlavor::Regular, EachIndexKind::Absent) => {
                (internal_index, None)
            }
        };

        let context_pattern = self.take_each_context_pattern(block)?;

        let content =
            self.child_statements(|cg| cg.fragment(block.body, FragmentParent::EachBlock))?;

        let mut body: Vec<Statement<'a>> = Vec::new();
        if let Some(pattern) = context_pattern {
            let member = self.b.computed_member_expr(
                self.b.rid_expr(array_name),
                self.b.rid_expr(&loop_index_name),
            );
            body.push(self.let_pattern_stmt(pattern, member));
        }
        if let Some(user_index) = group_rebind {
            let init = self.b.rid_expr(&loop_index_name);
            body.push(self.b.let_init_stmt(&user_index, init));
        }
        body.extend(content);

        Ok(self.count_for_statement(&loop_index_name, array_name, body))
    }

    fn build_each_fallback(
        &mut self,
        block: &'a EachBlock,
        array_name: &str,
        for_loop: Statement<'a>,
    ) -> Result<Statement<'a>> {
        let length = self
            .b
            .static_member_expr(self.b.rid_expr(array_name), "length");
        let test = self.b.ast.expression_binary(
            SPAN,
            length,
            BinaryOperator::StrictInequality,
            self.b.num_expr(0.0),
        );

        let open_marker = self.renderer_push_string_stmt("<!--[-->");
        let consequent = self.b.block_stmt(vec![open_marker, for_loop]);

        let Some(fallback) = block.fallback else {
            return Err(CodegenError::Unsupported(block.id, "each fallback"));
        };
        let mut fallback_body =
            self.child_statements(|cg| cg.fragment(fallback, FragmentParent::EachBlock))?;
        let else_marker = self.renderer_push_string_stmt("<!--[!-->");
        fallback_body.insert(0, else_marker);
        let alternate = self.b.block_stmt(fallback_body);

        Ok(self.b.if_stmt(test, consequent, Some(alternate)))
    }

    fn take_each_context_pattern(
        &mut self,
        block: &EachBlock,
    ) -> Result<Option<BindingPattern<'a>>> {
        let Some(context_ref) = block.context.as_ref() else {
            return Ok(None);
        };
        let Some(stmt) = self.js_arena.take_stmt(context_ref.id()) else {
            return Err(CodegenError::MissingExpression(block.id));
        };
        let Statement::VariableDeclaration(mut decl) = stmt else {
            return Err(CodegenError::Unsupported(
                block.id,
                "each context declaration",
            ));
        };
        if decl.declarations.is_empty() {
            return Err(CodegenError::Unsupported(
                block.id,
                "each context declarator",
            ));
        }
        Ok(Some(decl.declarations.remove(0).id))
    }

    fn let_pattern_stmt(&self, pattern: BindingPattern<'a>, init: Expression<'a>) -> Statement<'a> {
        let declarator = self.b.ast.variable_declarator(
            SPAN,
            VariableDeclarationKind::Let,
            pattern,
            NONE,
            Some(init),
            false,
        );
        let declaration = self.b.ast.variable_declaration(
            SPAN,
            VariableDeclarationKind::Let,
            self.b.ast.vec_from_array([declarator]),
            false,
        );
        Statement::VariableDeclaration(self.b.alloc(declaration))
    }

    fn count_for_statement(
        &self,
        index_name: &str,
        array_name: &str,
        body: Vec<Statement<'a>>,
    ) -> Statement<'a> {
        let b = &self.b;

        let index_pattern = b
            .ast
            .binding_pattern_binding_identifier(SPAN, b.ast.atom(index_name));
        let index_declarator = b.ast.variable_declarator(
            SPAN,
            VariableDeclarationKind::Let,
            index_pattern,
            NONE,
            Some(b.num_expr(0.0)),
            false,
        );

        let length_pattern = b
            .ast
            .binding_pattern_binding_identifier(SPAN, b.ast.atom(LENGTH_NAME));
        let length_init = b.static_member_expr(b.rid_expr(array_name), "length");
        let length_declarator = b.ast.variable_declarator(
            SPAN,
            VariableDeclarationKind::Let,
            length_pattern,
            NONE,
            Some(length_init),
            false,
        );

        let init = b.ast.for_statement_init_variable_declaration(
            SPAN,
            VariableDeclarationKind::Let,
            b.ast.vec_from_array([index_declarator, length_declarator]),
            false,
        );

        let test = b.ast.expression_binary(
            SPAN,
            b.rid_expr(index_name),
            BinaryOperator::LessThan,
            b.rid_expr(LENGTH_NAME),
        );

        let update_target = b
            .ast
            .simple_assignment_target_assignment_target_identifier(SPAN, b.ast.atom(index_name));
        let update = b
            .ast
            .expression_update(SPAN, UpdateOperator::Increment, false, update_target);

        let body_block = b.block_stmt(body);
        b.ast
            .statement_for(SPAN, Some(init), Some(test), Some(update), body_block)
    }
}
