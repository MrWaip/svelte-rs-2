mod builder;
mod data;

pub(crate) use builder::build;
pub use data::{
    BoundaryBranch, BoundarySemantics, ElementAsyncKind, ElementReplayEvent, ElementSemantics,
    ElementSemanticsStore, ElementValueRole, LegacyComponentSlotsSemantics, LegacyDefaultSlot,
    LegacySlotSemantics, RegularElementSemantics, SvelteElementSemantics, TextareaBody,
    TextareaSegment,
};
