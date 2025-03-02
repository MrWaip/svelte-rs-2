use bitflags::bitflags;

bitflags! {
     #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct OwnerContentTypeFlags: u32 {
        const Text = 1 << 0;
        const Interpolation = 1 << 1;
        const Concatenation = 1 << 2;
        const Element = 1 << 2;
        const IfBlock = 1 << 2;
    }
}

impl OwnerContentTypeFlags {
    pub fn set_from(&mut self, node: &hir::Node) {
        match node {
            hir::Node::Text(_) => self.insert(OwnerContentTypeFlags::Text),
            hir::Node::Interpolation(_) => self.insert(OwnerContentTypeFlags::Interpolation),
            hir::Node::Element(_) => self.insert(OwnerContentTypeFlags::Element),
            hir::Node::Concatenation(_) => self.insert(OwnerContentTypeFlags::Concatenation),
            hir::Node::IfBlock(_) => self.insert(OwnerContentTypeFlags::IfBlock),
            hir::Node::EachBlock => todo!(),
            hir::Node::Script => todo!(),
            hir::Node::Comment => todo!(),
            hir::Node::Phantom => todo!(),
        }
    }

    // metadata.anchor = match optimizations.content_type {
    //     ContentType::Mixed => FragmentAnchor::Fragment,
    //     ContentType::TextAndInterpolation => FragmentAnchor::Text,
    //     ContentType::Text => FragmentAnchor::TextInline,
    //     ContentType::Interpolation => FragmentAnchor::Text,
    //     ContentType::Element => FragmentAnchor::Element,
    //     ContentType::Nope => FragmentAnchor::Fragment,
    //     ContentType::NodeWithFragment => FragmentAnchor::Comment,
    // };
    //
    pub fn only_element(&self) -> bool {
        return self.contains(OwnerContentTypeFlags::Element);
    }

    pub fn only_text(&self) -> bool {
        return self.contains(OwnerContentTypeFlags::Text);
    }

    pub fn any_text_like(&self) -> bool {
        let allowed = OwnerContentTypeFlags::Text
            | OwnerContentTypeFlags::Interpolation
            | OwnerContentTypeFlags::Concatenation;

        return self.intersects(allowed) && (self.bits() & !allowed.bits()) == 0;
    }

    pub fn only_fragment_owner(&self) -> bool {
        return self.contains(OwnerContentTypeFlags::IfBlock);
    }
}

pub enum OwnerContentType {
    Common(OwnerContentTypeFlags),
    IfBlock(OwnerContentTypeFlags, OwnerContentTypeFlags),
}

impl OwnerContentType {
    pub fn as_common_or_empty(&self) -> OwnerContentTypeFlags {
        return match self {
            OwnerContentType::Common(flags) => flags.clone(),
            OwnerContentType::IfBlock(_, _) => OwnerContentTypeFlags::empty(),
        };
    }
}
