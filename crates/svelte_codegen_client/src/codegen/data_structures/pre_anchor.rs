use compact_str::CompactString;

pub(crate) struct PreAnchor {
    pub node_name: CompactString,
    pub frag_name: Option<CompactString>,
    pub needs_template_comment: bool,
    pub is_child: bool,
    pub parent_var: Option<CompactString>,
    pub parent_is_content: bool,
    pub callback_param: Option<CompactString>,
    pub sibling_var: Option<CompactString>,
}
