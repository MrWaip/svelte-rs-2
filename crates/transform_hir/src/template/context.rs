use std::borrow::Cow;

use ast_builder::{Builder, BuilderExpression as BExpr, BuilderFunctionArgument as BArg};
use hir::OwnerId;
use oxc_ast::ast::{Expression, Statement};

pub struct FragmentContext<'hir> {
    pub(crate) before_init: Vec<Statement<'hir>>,
    pub(crate) init: Vec<Statement<'hir>>,
    pub(crate) update: Vec<Statement<'hir>>,
    pub(crate) after_update: Vec<Statement<'hir>>,
    pub(crate) template: Vec<Cow<'hir, str>>,
    // anchor: Expression<'a>,
}

impl<'hir> FragmentContext<'hir> {
    pub fn new() -> Self {
        return Self {
            before_init: Vec::new(),
            init: Vec::new(),
            update: Vec::new(),
            after_update: Vec::new(),
            template: Vec::new(),
        };
    }
}

pub struct OwnerContext<'hir, 'short> {
    pub(crate) fragment: &'short mut FragmentContext<'hir>,
    prev: Expression<'hir>,
    // pub(crate) owner_anchor: Option<&'short Expression<'hir>>,
    // pub(crate) self_anchor: Expression<'ast>,
    sibling_offset: usize,
    b: &'hir Builder<'hir>,
    owner_id: OwnerId,
    // pub(crate) skip_reset_element: bool,
}

impl<'hir, 'short> OwnerContext<'hir, 'short> {
    pub fn new(
        fragment_context: &'short mut FragmentContext<'hir>,
        anchor: Expression<'hir>,
        builder: &'hir Builder<'hir>,
        owner_id: OwnerId,
        // parent_node_anchor: Option<&'short Expression<'hir>>,
    ) -> Self {
        return Self {
            fragment: fragment_context,
            prev: anchor, // owner_anchor: parent_node_anchor,
            sibling_offset: 0,
            b: builder,
            owner_id
        };
    }
    
    pub fn owner_id(&self) -> OwnerId {
        return self.owner_id;
    }

    pub fn anchor(&self) -> Expression<'hir> {
        return self.b.clone_expr(&self.prev);
    }

    pub fn trailing_static_nodes(&self) -> bool {
        return self.sibling_offset > 1;
    }

    pub fn sibling_offset(&self) -> usize {
        return self.sibling_offset;
    }

    fn get_node(&mut self, is_text: bool) -> Expression<'hir> {
        let expr = self.b.move_expr(&mut self.prev);

        if self.sibling_offset == 0 {
            return expr;
        }

        let mut args = vec![BArg::Expr(expr)];

        if is_text || self.sibling_offset != 1 {
            args.push(BArg::Num(self.sibling_offset as f64));
        }

        if is_text {
            args.push(BArg::Bool(true));
        }

        return self.b.call_expr("$.sibling", args);
    }

    pub fn flush_node(&mut self, is_text: bool, name: &str) {
        let expression = self.get_node(is_text);

        self.sibling_offset = 1;

        return if expression.is_identifier_reference() {
            self.prev = expression;
        } else {
            let id = self.b.rid_expr(name);

            self.push_init(self.b.var(name, BExpr::Expr(expression)));
            self.prev = id;
        };
    }

    pub fn next_sibling(&mut self) {
        self.sibling_offset += 1;
    }

    pub fn push_init(&mut self, stmt: Statement<'hir>) {
        self.fragment.init.push(stmt);
    }

    pub fn push_template(&mut self, value: Cow<'hir, str>) {
        self.fragment.template.push(value);
    }

    pub fn push_after_update(&mut self, stmt: Statement<'hir>) {
        self.fragment.after_update.push(stmt);
    }

    pub fn push_update(&mut self, stmt: Statement<'hir>) {
        self.fragment.update.push(stmt);
    }

    pub fn push_before_init(&mut self, stmt: Statement<'hir>) {
        self.fragment.before_init.push(stmt);
    }
}
