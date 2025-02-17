use ast::{Element, IfBlock};
use rccell::RcCell;

#[derive(Debug, Clone)]
pub enum Ancestor<'a> {
    Template,
    IfBlock(RcCell<IfBlock<'a>>),
    Element(RcCell<Element<'a>>),
}

impl<'a> Ancestor<'a> {
    pub fn is_template(&self) -> bool {
        return matches!(self, Ancestor::Template);
    }
}
