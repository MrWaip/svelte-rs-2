use std::borrow::Cow;

pub(crate) enum TemplateNode<'a> {
    Text(String),
    Comment(Option<String>),
    Element {
        name: Cow<'a, str>,
        attributes: Vec<(Cow<'a, str>, Option<Cow<'a, str>>)>,
        children: Vec<TemplateNode<'a>>,
        is_html: bool,
    },
}

pub(crate) struct Template<'a> {
    nodes: Vec<TemplateNode<'a>>,
    stack: Vec<Vec<TemplateNode<'a>>>,
    pub needs_import_node: bool,
}

impl<'a> Template<'a> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            stack: Vec::new(),
            needs_import_node: false,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty() && self.stack.is_empty()
    }

    pub fn push_element(&mut self, name: impl Into<Cow<'a, str>>, is_html: bool) {
        let el = TemplateNode::Element {
            name: name.into(),
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

    pub fn set_attribute(&mut self, key: impl Into<Cow<'a, str>>, value: Option<Cow<'a, str>>) {
        let key = key.into();
        let owner_slot = self.current_open_element_slot();
        let Some(last) = owner_slot.and_then(|slot| slot.last_mut()) else {
            return;
        };
        if let TemplateNode::Element { attributes, .. } = last {
            for (k, v) in attributes.iter_mut() {
                if *k == key {
                    *v = value;
                    return;
                }
            }
            attributes.push((key, value));
        }
    }

    fn current_open_element_slot(&mut self) -> Option<&mut Vec<TemplateNode<'a>>> {
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
        let mut out = String::with_capacity(self.estimate_size());
        for node in &self.nodes {
            stringify(node, &mut out);
        }
        out
    }

    fn estimate_size(&self) -> usize {
        estimate_nodes(&self.nodes)
    }

    fn current_mut(&mut self) -> &mut Vec<TemplateNode<'a>> {
        if let Some(top) = self.stack.last_mut() {
            top
        } else {
            &mut self.nodes
        }
    }
}

impl Default for Template<'_> {
    fn default() -> Self {
        Self::new()
    }
}

fn estimate_nodes(nodes: &[TemplateNode<'_>]) -> usize {
    let mut size = 0;
    for node in nodes {
        match node {
            TemplateNode::Text(s) => size += s.len(),
            TemplateNode::Comment(Some(data)) if !data.is_empty() => size += 7 + data.len(),
            TemplateNode::Comment(_) => size += 3,
            TemplateNode::Element {
                name,
                attributes,
                children,
                ..
            } => {
                size += 2 + name.len() + 3 + name.len() + 1;
                for (key, value) in attributes {
                    size += 1 + key.len();
                    if let Some(val) = value {
                        size += 3 + val.len();
                    }
                }
                size += estimate_nodes(children);
            }
        }
    }
    size
}

#[inline]
fn stringify(node: &TemplateNode<'_>, out: &mut String) {
    match node {
        TemplateNode::Text(s) => out.push_str(s),
        TemplateNode::Comment(Some(data)) if !data.is_empty() => {
            out.push_str("<!--");
            out.push_str(data);
            out.push_str("-->");
        }
        TemplateNode::Comment(_) => out.push_str("<!>"),
        TemplateNode::Element {
            name,
            attributes,
            children,
            is_html,
        } => {
            out.push('<');
            out.push_str(name);
            for (key, value) in attributes {
                out.push(' ');
                if *is_html {
                    push_ascii_lowercase(out, key);
                } else {
                    out.push_str(key);
                }
                if let Some(val) = value {
                    out.push_str("=\"");
                    escape_html_attr_into(val, out);
                    out.push('"');
                }
            }
            if is_void(name) {
                out.push_str("/>");
            } else {
                out.push('>');
                for child in children {
                    stringify(child, out);
                }
                out.push_str("</");
                out.push_str(name);
                out.push('>');
            }
        }
    }
}

#[inline]
fn push_ascii_lowercase(out: &mut String, s: &str) {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let start = i;
        while i < bytes.len() && !bytes[i].is_ascii_uppercase() {
            i += 1;
        }
        if start < i {
            out.push_str(&s[start..i]);
        }
        if i < bytes.len() {
            out.push((bytes[i] | 0x20) as char);
            i += 1;
        }
    }
}

#[inline]
fn escape_html_attr_into(s: &str, out: &mut String) {
    let bytes = s.as_bytes();
    let mut start = 0;
    while let Some(off) = memchr::memchr3(b'&', b'"', b'<', &bytes[start..]) {
        let pos = start + off;
        out.push_str(&s[start..pos]);
        out.push_str(match bytes[pos] {
            b'&' => "&amp;",
            b'"' => "&quot;",
            _ => "&lt;",
        });
        start = pos + 1;
    }
    out.push_str(&s[start..]);
}

pub(crate) fn escape_html_attr(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    escape_html_attr_into(s, &mut out);
    out
}

#[inline]
fn is_void(name: &str) -> bool {
    matches!(
        name,
        "area"
            | "base"
            | "br"
            | "col"
            | "command"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "keygen"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}
