use ancestry::VisitorAncestry;

use crate::ancestor::Ancestor;

pub mod ancestry;

pub struct VisitorContext<'a> {
    ancestry: VisitorAncestry<'a>,
}

impl<'a> VisitorContext<'a> {
    pub fn new() -> Self {
        return Self {
            ancestry: VisitorAncestry::new(),
        };
    }

    pub fn parent(&self) -> &Ancestor<'a> {
        return self.ancestry.parent();
    }

    pub(crate) fn push_stack(&mut self, ancestor: Ancestor<'a>) {
        self.ancestry.push_stack(ancestor);
    }

    pub(crate) fn pop_stack(&mut self) {
        self.ancestry.pop_stack();
    }
}
