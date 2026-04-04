use compact_str::CompactString;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use svelte_ast::Attribute;

/// Per-element attribute index enabling O(1) lookups by name.
///
/// Built once during analysis from a `Vec<Attribute>` slice; positions stored
/// as `u16` indices into that same slice so callers pass the original `attrs`
/// back for any operation that needs the actual value.
///
/// Variants with `Span`-based names (`UseDirective`, `TransitionDirective`,
/// `AnimateDirective`) and nameless variants are not indexed.
pub struct AttrIndex {
    by_name: FxHashMap<CompactString, SmallVec<[u16; 1]>>,
}

impl AttrIndex {
    pub fn build(attrs: &[Attribute]) -> Self {
        let mut by_name: FxHashMap<CompactString, SmallVec<[u16; 1]>> = FxHashMap::default();
        for (i, attr) in attrs.iter().enumerate() {
            if let Some(name) = attr.name() {
                by_name
                    .entry(CompactString::from(name))
                    .or_default()
                    .push(i as u16);
            }
        }
        Self { by_name }
    }

    /// O(1). Returns `true` if at least one attribute with this name is present.
    #[inline]
    pub fn has(&self, name: &str) -> bool {
        self.by_name.contains_key(name)
    }

    /// O(1). Returns the first attribute with this name.
    /// Covers the overwhelmingly common case of a single occurrence.
    #[inline]
    pub fn first<'a>(&self, attrs: &'a [Attribute], name: &str) -> Option<&'a Attribute> {
        let pos = *self.by_name.get(name)?.first()?;
        Some(&attrs[pos as usize])
    }

    /// O(1) start, O(k) iteration. Returns all attributes with this name.
    /// Useful for repeated directives such as `class:foo` or legacy `on:click`.
    pub fn all<'idx, 'attrs>(
        &'idx self,
        attrs: &'attrs [Attribute],
        name: &str,
    ) -> impl Iterator<Item = &'attrs Attribute> + 'idx
    where
        'attrs: 'idx,
    {
        self.by_name
            .get(name)
            .into_iter()
            .flat_map(move |positions| positions.iter().map(move |&pos| &attrs[pos as usize]))
    }
}
