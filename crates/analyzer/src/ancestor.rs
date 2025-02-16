use ast::Node;
use rccell::RcCell;

#[derive(Debug, Clone)]
pub enum Ancestor<'a> {
    Template,
    IfBlock(RcCell<Node<'a>>),
    Element(RcCell<Node<'a>>),
}

impl<'a> Ancestor<'a> {
    pub fn is_template(&self) -> bool {
        return matches!(self, Ancestor::Template);
    }
}
