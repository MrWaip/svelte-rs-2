use crate::ancestor::Ancestor;

pub struct VisitorAncestry<'a> {
    stack: Vec<Ancestor<'a>>,
}

impl<'a> VisitorAncestry<'a> {
    pub fn new() -> Self {
        return Self { stack: vec![] };
    }

    pub fn parent(&self) -> &Ancestor<'a> {
        return self.stack.last().unwrap();
    }

    pub fn pop_stack(&mut self) {
        self.stack.pop();
    }

    pub fn push_stack(&mut self, ancestor: Ancestor<'a>) {
        self.stack.push(ancestor);
    }
}
