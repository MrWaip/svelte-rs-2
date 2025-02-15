use ancestry::VisitorAncestry;

use crate::ancestor::Ancestor;

pub mod ancestry;

pub struct VisitorContext {
    ancestry: VisitorAncestry,
}

impl VisitorContext {
    pub fn new() -> Self {
        return Self {
            ancestry: VisitorAncestry::new(),
        };
    }

    pub fn parent(&self) -> &Ancestor {
        return self.ancestry.parent();
    }

    pub(crate) fn push_stack(&mut self, ancestor: Ancestor) {
        self.ancestry.push_stack(ancestor);
    }

    pub(crate) fn pop_stack(&mut self) {
        self.ancestry.pop_stack();
    }
}
