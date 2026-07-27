mod builder;
mod data;

pub(crate) use builder::build;
pub use data::{
    BoundaryBranch, BoundarySemantics, ComponentElementSemantics, ElementAsyncKind,
    ElementPropertyReset, ElementReplayEvent, ElementSemantics, ElementSemanticsStore,
    ElementValueRole, LegacyComponentSlotsSemantics, LegacyDefaultSlot, LegacySlotSemantics,
    RegularElementSemantics, SvelteElementSemantics, TextareaBody, TextareaSegment,
};
