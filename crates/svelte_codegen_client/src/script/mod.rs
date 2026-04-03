mod location;
mod model;
mod pipeline;
mod props;
mod state;
mod traverse;

// ---------------------------------------------------------------------------
// Props flag constants (must match svelte/src/constants.js)
// ---------------------------------------------------------------------------

pub(super) const PROPS_IS_IMMUTABLE: u32 = 1;
pub(super) const PROPS_IS_RUNES: u32 = 1 << 1;
pub(super) const PROPS_IS_UPDATED: u32 = 1 << 2;
pub(super) const PROPS_IS_BINDABLE: u32 = 1 << 3;
pub(super) const PROPS_IS_LAZY_INITIAL: u32 = 1 << 4;

/// Script transformation result carrying statements and comment metadata
/// for preserving JSDoc/leading comments in the final output.
pub(crate) use location::{compute_line_col, sanitize_location};
use model::{
    AsyncDerivedMode, ClassStateField, ClassStateInfo, FunctionInfo, PropKind, PropsGenInfo,
    ScriptTransformer,
};
pub use pipeline::{gen_script, transform_module_script};
