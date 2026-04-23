pub(crate) struct PreAnchor {
    pub node_name: String,
    pub frag_name: Option<String>,
    pub needs_template_comment: bool,
    pub is_child: bool,
    pub parent_var: Option<String>,
    pub callback_param: Option<String>,
    pub sibling_var: Option<String>,
}
