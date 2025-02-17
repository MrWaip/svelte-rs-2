use ast::{Element, IfBlock, Template};
use rccell::RcCell;

#[derive(Debug, Clone)]
pub enum Ancestor<'a> {
    Template(RcCell<Template<'a>>),
    IfBlock(RcCell<IfBlock<'a>>),
    Element(RcCell<Element<'a>>),
}

impl<'a> Ancestor<'a> {
    pub fn is_template(&self) -> bool {
        return matches!(self, Ancestor::Template(_));
    }
}
