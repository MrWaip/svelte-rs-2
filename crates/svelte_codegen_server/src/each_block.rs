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
    pub(crate) fn each_block(
        &mut self,
        block: &'a EachBlock,
        preserve_whitespace: bool,
    ) -> Result<()> {
        let sem = match self.analysis.block_semantics(block.id) {
            BlockSemantics::Each(sem) => sem.clone(),
            _ => return Err(CodegenError::Unsupported(block.id, "each block")),
        };
        if !matches!(sem.async_kind, EachAsyncKind::Sync) {
            return Err(CodegenError::Unsupported(block.id, "async each block"));
        }

        let array_name = self.gen_ident("each_array");
        let collection = self.take_expression(block.id, &block.expression)?;
        let ensure = self
            .b
            .call_expr("$.ensure_array_like", [Arg::Expr(collection)]);
        let array_decl = self.b.const_stmt(&array_name, ensure);

        let for_loop = self.build_each_for_loop(block, &sem, &array_name, preserve_whitespace)?;

        if block.fallback.is_some() {
            let guard =
                self.build_each_fallback(block, &array_name, for_loop, preserve_whitespace)?;
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
        preserve_whitespace: bool,
    ) -> Result<Statement<'a>> {
        let (loop_index_name, group_rebind) = match (sem.flavor, &sem.index) {
            (EachFlavor::BindGroup, EachIndexKind::Declared { sym, .. }) => (
                INTERNAL_INDEX_NAME.to_string(),
                Some(self.analysis.scoping.symbol_name(*sym).to_string()),
            ),
            (EachFlavor::Regular, EachIndexKind::Declared { sym, .. }) => {
                (self.analysis.scoping.symbol_name(*sym).to_string(), None)
            }
            (EachFlavor::BindGroup | EachFlavor::Regular, EachIndexKind::Absent) => {
                (INTERNAL_INDEX_NAME.to_string(), None)
            }
        };

        let context_pattern = self.take_each_context_pattern(block)?;

        let content = self.child_statements(|cg| {
            cg.fragment(block.body, FragmentParent::EachBlock, preserve_whitespace)
        })?;

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
        preserve_whitespace: bool,
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
        let mut fallback_body = self.child_statements(|cg| {
            cg.fragment(fallback, FragmentParent::EachBlock, preserve_whitespace)
        })?;
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
