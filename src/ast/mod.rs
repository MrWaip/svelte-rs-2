use rccell::RcCell;

pub struct Ast {
    pub template: Vec<RcCell<Node>>,
}

pub trait FormatNode {
    fn format_node(&self) -> String;
}

pub trait AsNode {
    fn as_node(self) -> Node;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Element(Element),
    Text(Text),
}

impl FormatNode for Node {
    fn format_node(&self) -> String {
        return match self {
            Node::Element(element) => element.format_node(),
            Node::Text(text) => text.format_node(),
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

impl AsNode for Element {
    fn as_node(self) -> Node {
        return Node::Element(self);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Text {
    pub value: String,
}

impl FormatNode for Text {
    fn format_node(&self) -> String {
        return self.value.clone();
    }
}

impl AsNode for Text {
    fn as_node(self) -> Node {
        return Node::Text(self);
    }
}
