use std::fmt::Write;

pub(crate) enum TemplateNode {
    Text(String),
    Comment(Option<String>),
    Element {
        name: String,
        attributes: Vec<(String, Option<String>)>,
        children: Vec<TemplateNode>,
        is_html: bool,
    },
}

pub(crate) struct Template {
    nodes: Vec<TemplateNode>,
    stack: Vec<Vec<TemplateNode>>,
    pub needs_import_node: bool,
    pub contains_script_tag: bool,
}

impl Template {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            stack: Vec::new(),
            needs_import_node: false,
            contains_script_tag: false,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty() && self.stack.is_empty()
    }

    pub fn push_element(&mut self, name: &str, is_html: bool) {
        let el = TemplateNode::Element {
            name: name.to_string(),
            attributes: Vec::new(),
            children: Vec::new(),
            is_html,
        };
        self.current_mut().push(el);
        let children_storage = Vec::new();
        self.stack.push(children_storage);
    }

    pub fn pop_element(&mut self) {
        let Some(children) = self.stack.pop() else {
            return;
        };
        let Some(last) = self.current_mut().last_mut() else {
            return;
        };
        if let TemplateNode::Element { children: slot, .. } = last {
            *slot = children;
        }
    }

    pub fn push_text(&mut self, text: &str) {
        if let Some(TemplateNode::Text(existing)) = self.current_mut().last_mut() {
            existing.push_str(text);
            return;
        }
        self.current_mut()
            .push(TemplateNode::Text(text.to_string()));
    }

    pub fn push_comment(&mut self, data: Option<String>) {
        self.current_mut().push(TemplateNode::Comment(data));
    }

    pub fn set_attribute(&mut self, key: &str, value: Option<String>) {
        let owner_slot = self.current_open_element_slot();
        let Some(last) = owner_slot.and_then(|slot| slot.last_mut()) else {
            return;
        };
        if let TemplateNode::Element { attributes, .. } = last {
            for (k, v) in attributes.iter_mut() {
                if k == key {
                    *v = value;
                    return;
                }
            }
            attributes.push((key.to_string(), value));
        }
    }

    fn current_open_element_slot(&mut self) -> Option<&mut Vec<TemplateNode>> {
        let depth = self.stack.len();
        if depth == 0 {
            return Some(&mut self.nodes);
        }
        if depth == 1 {
            return Some(&mut self.nodes);
        }
        self.stack.get_mut(depth - 2)
    }

    pub fn as_html(&self) -> String {
        let mut out = String::new();
        for node in &self.nodes {
            stringify(node, &mut out);
        }
        out
    }

    fn current_mut(&mut self) -> &mut Vec<TemplateNode> {
        if let Some(top) = self.stack.last_mut() {
            top
        } else {
            &mut self.nodes
        }
    }
}

impl Default for Template {
    fn default() -> Self {
        Self::new()
    }
}

fn stringify(node: &TemplateNode, out: &mut String) {
    match node {
        TemplateNode::Text(s) => out.push_str(s),
        TemplateNode::Comment(Some(data)) => {
            let _ = write!(out, "<!--{data}-->");
        }
        TemplateNode::Comment(None) => out.push_str("<!>"),
        TemplateNode::Element {
            name,
            attributes,
            children,
            is_html,
        } => {
            let _ = write!(out, "<{name}");
            for (key, value) in attributes {
                let effective_key = if *is_html {
                    key.to_lowercase()
                } else {
                    key.clone()
                };
                out.push(' ');
                out.push_str(&effective_key);
                if let Some(val) = value {
                    let _ = write!(out, "=\"{}\"", escape_html_attr(val));
                }
            }
            if is_void(name) {
                out.push_str("/>");
            } else {
                out.push('>');
                for child in children {
                    stringify(child, out);
                }
                let _ = write!(out, "</{name}>");
            }
        }
    }
}

fn escape_html_attr(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(ch),
        }
    }
    out
}

fn is_void(name: &str) -> bool {
    matches!(
        name,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}
