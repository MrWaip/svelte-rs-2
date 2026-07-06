use std::iter::empty;

use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::SvelteElementTag;
use svelte_ast::SvelteElement;
use svelte_ast_builder::Arg;

use crate::error::{CodegenError, Result};
use crate::fragment::FragmentParent;
use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn svelte_element_dev_init(&mut self, el: &'a SvelteElement) -> Result<()> {
        let tag_expr = self.svelte_element_tag(el)?;
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

        if !self.dev {
            let tag_expr = self.svelte_element_tag(el)?;
            self.push_element_call(tag_expr, attrs_stmts, children_stmts);
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
        self.push_stmt(push_element);

        let tag_ref = self.b.rid_expr(&ref_name);
        self.push_element_call(tag_ref, attrs_stmts, children_stmts);

        let pop_element = self.b.call_stmt("$.pop_element", empty::<Arg<'_, '_>>());
        self.push_stmt(pop_element);
        Ok(())
    }

    fn push_element_call(
        &mut self,
        tag: Expression<'a>,
        attrs_stmts: Vec<Statement<'a>>,
        children_stmts: Vec<Statement<'a>>,
    ) {
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

        let call = self.b.call_stmt("$.element", args);
        self.push_stmt(call);
    }

    fn svelte_element_tag(&mut self, el: &'a SvelteElement) -> Result<Expression<'a>> {
        match self.analysis.svelte_element_tag(el.id).cloned() {
            Some(SvelteElementTag::Known(name)) => Ok(self.b.str_expr(&name)),
            Some(SvelteElementTag::Dynamic(oxc_id)) => self.take_expr_by_oxc_id(el.id, oxc_id),
            None => Err(CodegenError::MissingExpression(el.id)),
        }
    }
}
