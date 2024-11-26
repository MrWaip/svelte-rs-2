#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Element(Element),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Element {
    pub name: String,
    pub nodes: Vec<Box<Node>>,
    
}
