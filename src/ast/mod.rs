use rccell::RcCell;



pub struct Ast {
    pub template: Vec<RcCell<Node>>,
}

pub trait FormatNode {
    fn format_node(&self) -> String;
}

pub trait AstNode {
    fn push(&mut self, node: RcCell<Node>);
}

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Element(Element),
}

impl FormatNode for Node {
    fn format_node(&self) -> String {
        return match self {
            Node::Element(element) => element.format_node(),
        };
    }
}

impl AstNode for Node {
    fn push(&mut self, node: RcCell<Node>) {
        match self {
            Node::Element(element) => element.push(node),
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Element {
    pub name: String,
    pub nodes: Vec<RcCell<Node>>,
}

impl FormatNode for Element {
    fn format_node(&self) -> String {
        let mut result = String::new();

        result.push_str("<");
        result.push_str(&self.name);
        result.push_str(">");

        for node in self.nodes.iter() {
            let formatted = node.borrow().format_node();
            result.push_str(&formatted);
        }

        result.push_str("</");
        result.push_str(&self.name);
        result.push_str(">");

        return result;
    }
}

impl AstNode for Element {
    fn push(&mut self, node: RcCell<Node>) {
        self.nodes.push(node);
    }
}
