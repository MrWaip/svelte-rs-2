use std::iter::empty;

use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{ElementSemantics, SvelteElementTag};
use svelte_ast::SvelteElement;
use svelte_ast_builder::Arg;

use crate::error::{CodegenError, Result};
use crate::fragment::FragmentParent;
use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn svelte_element_dev_init(&mut self, el: &'a SvelteElement) -> Result<()> {
        let mut tag_expr = self.svelte_element_tag(el)?;
        if self.element_async_blockers(el.id).is_some() {
            tag_expr = self.save_block_await(tag_expr);
        }
        let ref_name = if let Expression::Identifier(id) = &tag_expr {
            id.name.to_string()
        } else {
            let name = self.gen_ident("$$tag");
            let decl = self.b.const_stmt(&name, tag_expr);
            self.push_stmt(decl);
            name
        };

        let validate_tag = self.b.thunk(self.b.rid_expr(&ref_name));
        let validate = self
            .b
            .call_stmt("$.validate_dynamic_element_tag", [Arg::Expr(validate_tag)]);
        self.push_stmt(validate);

        if self.analysis.fragment_has_children_by_id(el.fragment) {
            let void_tag = self.b.thunk(self.b.rid_expr(&ref_name));
            let validate_void = self
                .b
                .call_stmt("$.validate_void_dynamic_element", [Arg::Expr(void_tag)]);
            self.push_stmt(validate_void);
        }

        self.svelte_element_tag_refs.insert(el.id, ref_name);
        Ok(())
    }

    pub(crate) fn svelte_element(&mut self, el: &'a SvelteElement) -> Result<()> {
        let attrs_stmts =
            self.child_statements(|cg| cg.emit_element_attributes(el.id, &el.attributes))?;
        let children_stmts = self.child_statements(|cg| {
            cg.fragment(el.fragment, FragmentParent::SvelteElement, false)
        })?;

        let async_blockers = self.element_async_blockers(el.id);

        if !self.dev {
            let mut tag_expr = self.svelte_element_tag(el)?;
            if async_blockers.is_some() {
                tag_expr = self.save_block_await(tag_expr);
            }
            let call = self.build_element_call(tag_expr, attrs_stmts, children_stmts);
            match async_blockers {
                Some(blockers) => {
                    let wrapped = self.wrap_async_block(vec![call], &blockers);
                    self.push_stmt(wrapped);
                }
                None => self.push_stmt(call),
            }
            return Ok(());
        }

        let ref_name = self
            .svelte_element_tag_refs
            .get(&el.id)
            .cloned()
            .ok_or(CodegenError::MissingExpression(el.id))?;

        let (line, col) = self.line_index.line_col(el.span.start);
        let push_element = self.b.call_stmt(
            "$.push_element",
            [
                Arg::Ident("$$renderer"),
                Arg::Expr(self.b.rid_expr(&ref_name)),
                Arg::Num(line as f64),
                Arg::Num(col as f64),
            ],
        );

        let tag_ref = self.b.rid_expr(&ref_name);
        let call = self.build_element_call(tag_ref, attrs_stmts, children_stmts);
        let pop_element = self.b.call_stmt("$.pop_element", empty::<Arg<'_, '_>>());

        match async_blockers {
            Some(blockers) => {
                let wrapped =
                    self.wrap_async_block(vec![push_element, call, pop_element], &blockers);
                self.push_stmt(wrapped);
            }
            None => {
                self.push_stmt(push_element);
                self.push_stmt(call);
                self.push_stmt(pop_element);
            }
        }
        Ok(())
    }

    fn element_async_blockers(&self, id: svelte_ast::NodeId) -> Option<Vec<u32>> {
        match self.analysis.element_semantics.query(id) {
            ElementSemantics::SvelteElement(sem) => Some(sem.async_kind.blockers().to_vec()),
            _ => None,
        }
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
