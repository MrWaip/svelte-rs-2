pub struct Ast {
    pub template: Vec<Node>,
}

pub trait FormatNode {
    fn format_node(&self) -> String;
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

#[derive(Debug, PartialEq, Eq)]
pub struct Element {
    pub name: String,
    pub nodes: Vec<Box<Node>>,
}

impl FormatNode for Element {
    fn format_node(&self) -> String {
        let mut result = String::new();

        result.push_str("<");
        result.push_str(&self.name);
        result.push_str(">");

        for node in self.nodes.iter() {
            let formatted = node.format_node();
            result.push_str(&formatted);
        }

        result.push_str("</");
        result.push_str(&self.name);
        result.push_str(">");

        return result;
    }
}
