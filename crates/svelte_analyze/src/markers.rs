/// Proof that `build_scoping()` has completed for this component.
/// Required by `resolve_references()` — cannot be constructed externally.
pub(crate) struct ScopingBuilt(());

/// Proof that `resolve_references()` has completed for this component.
/// Reserved for future passes that depend on resolved `symbol_id` fields.
pub(crate) struct ReferencesResolved(());

impl ScopingBuilt {
    pub(crate) fn new() -> Self { Self(()) }
}

impl ReferencesResolved {
    pub(crate) fn new() -> Self { Self(()) }
}
