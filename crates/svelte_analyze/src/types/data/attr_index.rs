use compact_str::CompactString;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use std::borrow::Cow;
use svelte_ast::{Attribute, NodeId};

pub struct AttrIndex {
    by_name: FxHashMap<CompactString, SmallVec<[u16; 1]>>,
    by_id: FxHashMap<NodeId, u16>,
}

impl AttrIndex {
    pub fn build(attrs: &[Attribute], source: &str) -> Self {
        let mut by_name: FxHashMap<CompactString, SmallVec<[u16; 1]>> = FxHashMap::default();
        let mut by_id = FxHashMap::default();
        for (i, attr) in attrs.iter().enumerate() {
            by_id.insert(attr.id(), i as u16);
            if let Some(name) = attr_index_name(attr, source) {
                by_name.entry(name).or_default().push(i as u16);
            }
        }
        Self { by_name, by_id }
    }

    #[inline]
    pub fn has(&self, name: &str) -> bool {
        self.by_name.contains_key(lookup_key(name).as_ref())
    }

    #[inline]
    pub fn first<'a>(&self, attrs: &'a [Attribute], name: &str) -> Option<&'a Attribute> {
        let pos = *self.by_name.get(lookup_key(name).as_ref())?.first()?;
        Some(&attrs[pos as usize])
    }

    pub fn all<'idx, 'attrs>(
        &'idx self,
        attrs: &'attrs [Attribute],
        name: &str,
    ) -> impl Iterator<Item = &'attrs Attribute> + 'idx
    where
        'attrs: 'idx,
    {
        self.by_name
            .get(lookup_key(name).as_ref())
            .into_iter()
            .flat_map(move |positions| positions.iter().map(move |&pos| &attrs[pos as usize]))
    }

    #[inline]
    pub fn find_by_id<'a>(&self, attrs: &'a [Attribute], id: NodeId) -> Option<&'a Attribute> {
        let pos = *self.by_id.get(&id)?;
        Some(&attrs[pos as usize])
    }
}

fn attr_index_name(attr: &Attribute, _source: &str) -> Option<CompactString> {
    attr.name().map(|n| {
        if n.bytes().any(|b| b.is_ascii_uppercase()) {
            CompactString::from(n.to_ascii_lowercase())
        } else {
            CompactString::from(n)
        }
    })
}

#[inline]
fn lookup_key(name: &str) -> Cow<'_, str> {
    if name.bytes().any(|b| b.is_ascii_uppercase()) {
        Cow::Owned(name.to_ascii_lowercase())
    } else {
        Cow::Borrowed(name)
    }
}
