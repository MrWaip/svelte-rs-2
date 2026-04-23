use std::cell::Cell;

pub use oxc_syntax::node::NodeId as OxcNodeId;
pub use svelte_span::Span;

#[derive(Clone)]
pub struct ExprRef {
    pub span: Span,
    pub oxc_id: Cell<OxcNodeId>,
}

impl ExprRef {
    pub fn new(span: Span) -> Self {
        Self {
            span,
            oxc_id: Cell::new(OxcNodeId::DUMMY),
        }
    }

    pub fn id(&self) -> OxcNodeId {
        let id = self.oxc_id.get();
        debug_assert!(
            id != OxcNodeId::DUMMY,
            "ExprRef not bound to OxcNodeId; span={:?}",
            self.span
        );
        id
    }

    pub fn bind(&self, id: OxcNodeId) {
        debug_assert!(
            self.oxc_id.get() == OxcNodeId::DUMMY,
            "ExprRef already bound"
        );
        self.oxc_id.set(id);
    }
}

#[derive(Clone)]
pub struct StmtRef {
    pub span: Span,
    pub oxc_id: Cell<OxcNodeId>,
}

impl StmtRef {
    pub fn new(span: Span) -> Self {
        Self {
            span,
            oxc_id: Cell::new(OxcNodeId::DUMMY),
        }
    }

    pub fn id(&self) -> OxcNodeId {
        let id = self.oxc_id.get();
        debug_assert!(id != OxcNodeId::DUMMY, "StmtRef not bound to OxcNodeId");
        id
    }

    pub fn bind(&self, id: OxcNodeId) {
        debug_assert!(
            self.oxc_id.get() == OxcNodeId::DUMMY,
            "StmtRef already bound"
        );
        self.oxc_id.set(id);
    }
}
