use crate::ancestor::Ancestor;

pub struct VisitorAncestry {
    stack: Vec<Ancestor>,
}

impl VisitorAncestry {
    pub fn new() -> Self {
        return Self { stack: vec![] };
    }

    pub fn parent(&self) -> &Ancestor {
        return self.stack.last().unwrap();
    }

    pub fn pop_stack(&mut self) {
        self.stack.pop();
    }

    pub fn push_stack(&mut self, ancestor: Ancestor) {
        self.stack.push(ancestor);
    }
}
