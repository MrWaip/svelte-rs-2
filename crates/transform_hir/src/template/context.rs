use oxc_ast::ast::{Expression, Statement};

pub struct FragmentContext<'hir> {
    pub(crate) before_init: Vec<Statement<'hir>>,
    pub(crate) init: Vec<Statement<'hir>>,
    pub(crate) update: Vec<Statement<'hir>>,
    pub(crate) after_update: Vec<Statement<'hir>>,
    pub(crate) template: Vec<String>,
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
    pub(crate) anchor: Expression<'hir>,
    // pub(crate) owner_anchor: Option<&'short Expression<'hir>>,
    // pub(crate) self_anchor: Expression<'ast>,
    // pub(crate) sibling_offset: usize,
    // pub(crate) skip_reset_element: bool,
}

impl<'hir, 'short> OwnerContext<'hir, 'short> {
    pub fn new(
        fragment_context: &'short mut FragmentContext<'hir>,
        anchor: Expression<'hir>,
        // parent_node_anchor: Option<&'short Expression<'hir>>,
    ) -> Self {
        return Self {
            fragment: fragment_context,
            anchor, // owner_anchor: parent_node_anchor,
        };
    }

    pub fn push_init(&mut self, stmt: Statement<'hir>) {
        self.fragment.init.push(stmt);
    }

    pub fn push_template(&mut self, value: String) {
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
