use bitflags::bitflags;

bitflags! {
     #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct OwnerContentTypeFlags: u32 {
        const Text = 1 << 1;
        const Interpolation = 1 << 2;
        const Concatenation = 1 << 3;
        const Element = 1 << 4;
        const IfBlock = 1 << 5;
        const Comment = 1 << 6;
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
            hir::Node::Comment(_) => (),
            hir::Node::Phantom => todo!(),
        }
    }

    pub fn only_element(&self) -> bool {
        *self == OwnerContentTypeFlags::Element
    }

    pub fn only_synthetic_node(&self) -> bool {
        match *self {
            OwnerContentTypeFlags::IfBlock => true,
            _ => false,
        }
    }

    pub fn only_text(&self) -> bool {
        *self == OwnerContentTypeFlags::Text
    }

    pub fn none_text(&self) -> bool {
        if self.is_empty() {
            return false;
        }

        !self.only_text()
    }

    pub fn any_text_like(&self) -> bool {
        let allowed = OwnerContentTypeFlags::Text
            | OwnerContentTypeFlags::Interpolation
            | OwnerContentTypeFlags::Concatenation;

        self.intersects(allowed) && (self.bits() & !allowed.bits()) == 0
    }

    pub fn any_interpolation_like(&self) -> bool {
        let allowed = OwnerContentTypeFlags::Interpolation | OwnerContentTypeFlags::Concatenation;

        self.intersects(allowed) && (self.bits() & !allowed.bits()) == 0
    }

    pub fn only_fragment_owner(&self) -> bool {
        *self == OwnerContentTypeFlags::IfBlock
    }
}

pub enum OwnerContentType {
    Common(OwnerContentTypeFlags),
    IfBlock(OwnerContentTypeFlags, OwnerContentTypeFlags),
}

impl OwnerContentType {
    pub fn as_common_or_empty(&self) -> OwnerContentTypeFlags {
        match self {
            OwnerContentType::Common(flags) => *flags,
            OwnerContentType::IfBlock(_, _) => OwnerContentTypeFlags::empty(),
        }
    }

    pub fn as_if(&self) -> (OwnerContentTypeFlags, OwnerContentTypeFlags) {
        let OwnerContentType::IfBlock(consequent, alternate) = self else {
            unreachable!()
        };

        (*consequent, *alternate)
    }
}
