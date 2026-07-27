use std::iter::empty;

use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{ElementAsyncKind, ElementSemantics, SvelteElementTag};
use svelte_ast::SvelteElement;
use svelte_ast_builder::Arg;

use crate::error::{CodegenError, Result};
use crate::fragment::FragmentParent;
use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn svelte_element_dev_init(&mut self, el: &'a SvelteElement) -> Result<()> {
        let mut tag_expr = self.svelte_element_tag(el)?;
        if !self.svelte_element_async_kinds(el.id).0.is_sync() {
            tag_expr = self.save_block_await(tag_expr);
        }
        let ref_name = if let Expression::Identifier(id) = &tag_expr {
            id.name.to_string()
        } else {
            let name = self.gen_ident("$$tag");
            let decl = self.b.const_stmt(&name, tag_expr);
            self.hoist_stmt(decl);
            name
        };

        let validate_tag = self.b.thunk(self.b.rid_expr(&ref_name));
        let validate = self
            .b
            .call_stmt("$.validate_dynamic_element_tag", [Arg::Expr(validate_tag)]);
        self.hoist_stmt(validate);

        if self.analysis.fragment_has_children_by_id(el.fragment) {
            let void_tag = self.b.thunk(self.b.rid_expr(&ref_name));
            let validate_void = self
                .b
                .call_stmt("$.validate_void_dynamic_element", [Arg::Expr(void_tag)]);
            self.hoist_stmt(validate_void);
        }

        self.svelte_element_tag_refs.insert(el.id, ref_name);
        Ok(())
    }

    pub(crate) fn svelte_element(&mut self, el: &'a SvelteElement) -> Result<()> {
        if self.dev {
            self.svelte_element_dev_init(el)?;
        }
        let (tag_async, attributes_async) = self.svelte_element_async_kinds(el.id);

        let (attrs_stmts, attr_hoists) = self.with_promise_hoisting(|cg| {
            cg.child_statements(|c| c.emit_element_attributes(el.id, &el.attributes))
        });
        let attrs_stmts = attrs_stmts?;
        let children_stmts =
            self.child_statements(|cg| cg.fragment(el.fragment, FragmentParent::SvelteElement))?;

        let tag_expr = self.svelte_element_call_tag(el, &tag_async)?;
        let call = self.build_element_call(tag_expr, attrs_stmts, children_stmts);

        let mut statements: Vec<Statement<'a>> = Vec::new();
        if self.dev {
            statements.push(self.svelte_element_push_element(el)?);
        }
        if attributes_async.is_sync() {
            statements.extend(attr_hoists);
            statements.push(call);
        } else {
            let mut body = attr_hoists;
            body.push(call);
            let arrow = self.b.arrow_block_expr_async(
                self.b.params(["$$renderer"]),
                body,
                attributes_async.awaited(),
            );
            statements.push(self.wrap_arrow(
                arrow,
                attributes_async.blockers(),
                "$$renderer.child",
                "$$renderer.async",
            ));
        }
        if self.dev {
            statements.push(self.b.call_stmt("$.pop_element", empty::<Arg<'_, '_>>()));
        }

        if tag_async.is_sync() {
            for stmt in statements {
                self.push_stmt(stmt);
            }
            return Ok(());
        }
        let wrapped =
            self.wrap_async_block_flagged(statements, tag_async.blockers(), tag_async.awaited());
        self.push_stmt(wrapped);
        Ok(())
    }

    fn svelte_element_async_kinds(
        &self,
        id: svelte_ast::NodeId,
    ) -> (ElementAsyncKind, ElementAsyncKind) {
        match self.analysis.element_semantics.query(id) {
            ElementSemantics::SvelteElement(sem) => (
                sem.tag_async_kind.clone(),
                sem.attributes_async_kind.clone(),
            ),
            _ => (ElementAsyncKind::Sync, ElementAsyncKind::Sync),
        }
    }

    fn svelte_element_call_tag(
        &mut self,
        el: &'a SvelteElement,
        tag_async: &ElementAsyncKind,
    ) -> Result<Expression<'a>> {
        if !self.dev {
            let tag_expr = self.svelte_element_tag(el)?;
            if tag_async.is_sync() {
                return Ok(tag_expr);
            }
            return Ok(self.save_block_await(tag_expr));
        }
        let ref_name = self
            .svelte_element_tag_refs
            .get(&el.id)
            .cloned()
            .ok_or(CodegenError::MissingExpression(el.id))?;
        Ok(self.b.rid_expr(&ref_name))
    }

    fn svelte_element_push_element(&mut self, el: &'a SvelteElement) -> Result<Statement<'a>> {
        let ref_name = self
            .svelte_element_tag_refs
            .get(&el.id)
            .cloned()
            .ok_or(CodegenError::MissingExpression(el.id))?;
        let (line, col) = self.line_index.line_col(el.span.start);
        Ok(self.b.call_stmt(
            "$.push_element",
            [
                Arg::Ident("$$renderer"),
                Arg::Expr(self.b.rid_expr(&ref_name)),
                Arg::Num(line as f64),
                Arg::Num(col as f64),
            ],
        ))
    }

    fn build_element_call(
        &mut self,
        tag: Expression<'a>,
        attrs_stmts: Vec<Statement<'a>>,
        children_stmts: Vec<Statement<'a>>,
    ) -> Statement<'a> {
        let attrs_arg = if attrs_stmts.is_empty() {
            None
        } else {
            Some(self.b.arrow_block_expr(self.b.no_params(), attrs_stmts))
        };
        let children_arg = if children_stmts.is_empty() {
            None
        } else {
            Some(self.b.arrow_block_expr(self.b.no_params(), children_stmts))
        };

        let mut args = vec![Arg::Ident("$$renderer"), Arg::Expr(tag)];
        if let Some(children) = children_arg {
            let attrs = attrs_arg.unwrap_or_else(|| self.b.void_zero_expr());
            args.push(Arg::Expr(attrs));
            args.push(Arg::Expr(children));
        } else if let Some(attrs) = attrs_arg {
            args.push(Arg::Expr(attrs));
        }

        self.b.call_stmt("$.element", args)
    }

    fn svelte_element_tag(&mut self, el: &'a SvelteElement) -> Result<Expression<'a>> {
        match self.analysis.svelte_element_tag(el.id).cloned() {
            Some(SvelteElementTag::Known(name)) => Ok(self.b.str_expr(&name)),
            Some(SvelteElementTag::Dynamic(oxc_id)) => self.take_expr_by_oxc_id(el.id, oxc_id),
            None => Err(CodegenError::MissingExpression(el.id)),
        }
    }
}
