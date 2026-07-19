use compact_str::CompactString;

#[derive(Clone)]
pub(crate) enum FragmentAnchor {
    Root,
    Child {
        parent_var: CompactString,
    },
    ElementContentChild {
        parent_var: CompactString,
    },
    CallbackParam {
        name: CompactString,
        append_inside: bool,
    },
    SiblingVar {
        var: CompactString,
    },
}

impl FragmentAnchor {
    #[inline]
    pub fn child(parent_var: impl Into<CompactString>) -> Self {
        Self::Child {
            parent_var: parent_var.into(),
        }
    }

    #[inline]
    pub fn element_content_child(parent_var: impl Into<CompactString>) -> Self {
        Self::ElementContentChild {
            parent_var: parent_var.into(),
        }
    }

    #[inline]
    pub fn callback_param(name: impl Into<CompactString>, append_inside: bool) -> Self {
        Self::CallbackParam {
            name: name.into(),
            append_inside,
        }
    }

    #[inline]
    pub fn sibling_var(var: impl Into<CompactString>) -> Self {
        Self::SiblingVar { var: var.into() }
    }
}
