#[derive(Clone)]
pub(crate) enum FragmentAnchor {
    Root,
    Child { parent_var: String },
    CallbackParam { name: String, append_inside: bool },
    SiblingVar { var: String },
}
