use crate::ancestor::Ancestor;

pub struct VisitorAncestry<'a> {
    stack: Vec<Ancestor<'a>>,
}

impl Default for VisitorAncestry<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> VisitorAncestry<'a> {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn parent(&self) -> &Ancestor<'a> {
        self.stack.last().unwrap()
    }

    pub fn pop_stack(&mut self) {
        self.stack.pop();
    }

    pub fn push_stack(&mut self, ancestor: Ancestor<'a>) {
        self.stack.push(ancestor);
    }
}
