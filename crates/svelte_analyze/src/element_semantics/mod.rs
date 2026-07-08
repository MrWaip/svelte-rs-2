mod builder;
mod data;

pub(crate) use builder::build;
pub use data::{
    BoundaryBranch, BoundarySemantics, ElementAsyncKind, ElementSemantics, ElementSemanticsStore,
    ElementValueRole, LegacyComponentSlotsSemantics, LegacyDefaultSlot, LegacySlotSemantics,
    RegularElementSemantics, SvelteElementSemantics,
};
