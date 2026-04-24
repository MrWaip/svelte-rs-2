pub use svelte_ast::FragmentKey;

pub trait FragmentKeyExt {
    fn needs_text_first_next(&self) -> bool;
}

impl FragmentKeyExt for FragmentKey {
    fn needs_text_first_next(&self) -> bool {
        matches!(
            self,
            Self::Root
                | Self::EachBody(_)
                | Self::EachFallback(_)
                | Self::SnippetBody(_)
                | Self::ComponentNode(_)
                | Self::NamedSlot(_, _)
                | Self::SvelteBoundaryBody(_)
        )
    }
}
