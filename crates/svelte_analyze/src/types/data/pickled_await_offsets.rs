use rustc_hash::FxHashSet;

#[derive(Default)]
pub struct PickledAwaitOffsets {
    offsets: FxHashSet<u32>,
}

impl PickledAwaitOffsets {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn extend_offsets(&mut self, offsets: impl IntoIterator<Item = u32>) {
        self.offsets.extend(offsets);
    }

    pub fn contains_offset(&self, offset: u32) -> bool {
        self.offsets.contains(&offset)
    }
}
