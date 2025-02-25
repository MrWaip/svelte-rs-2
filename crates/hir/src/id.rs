use oxc_index::Idx;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OwnerId(usize);

impl OwnerId {
    pub const fn new(idx: usize) -> Self {
        return Self(idx);
    }
}

impl Idx for OwnerId {
    fn from_usize(idx: usize) -> Self {
        Self::new(idx)
    }

    fn index(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AttributeId(usize);

impl AttributeId {
    pub const fn new(idx: usize) -> Self {
        return Self(idx);
    }
}

impl Idx for AttributeId {
    fn from_usize(idx: usize) -> Self {
        Self::new(idx)
    }

    fn index(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExpressionId(usize);

impl ExpressionId {
    pub const fn new(idx: usize) -> Self {
        return Self(idx);
    }
}

impl Idx for ExpressionId {
    fn from_usize(idx: usize) -> Self {
        Self::new(idx)
    }

    fn index(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NodeId(usize);

impl NodeId {
    pub const fn new(idx: usize) -> Self {
        return Self(idx);
    }
}

impl Idx for NodeId {
    fn from_usize(idx: usize) -> Self {
        Self::new(idx)
    }

    fn index(self) -> usize {
        self.0
    }
}
