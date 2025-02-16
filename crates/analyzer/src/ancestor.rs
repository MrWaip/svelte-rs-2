#[derive(Debug, Clone, Copy)]
pub enum Ancestor {
    IfBlock,
    Template,
    Element,
}

impl Ancestor {
    pub fn is_template(&self) -> bool {
        return matches!(self, Ancestor::Template);
    }
}
