/// Proof that `build_scoping()` has completed for this component.
/// Required by `resolve_references()` — cannot be constructed externally.
pub(crate) struct ScopingBuilt(());

impl ScopingBuilt {
    pub(crate) fn new() -> Self { Self(()) }
}
