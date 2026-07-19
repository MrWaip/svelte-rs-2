use compact_str::CompactString;
use smallvec::SmallVec;
use svelte_ast::{Attribute, NodeId};

const INLINE_ATTRS: usize = 8;

pub struct AttrIndex {
    entries: SmallVec<[(CompactString, u16); INLINE_ATTRS]>,
}

impl AttrIndex {
    pub fn build(attrs: &[Attribute], source: &str) -> Self {
        let mut entries: SmallVec<[(CompactString, u16); INLINE_ATTRS]> = SmallVec::new();
        for (i, attr) in attrs.iter().enumerate() {
            if let Some(name) = attr_index_name(attr, source) {
                entries.push((name, i as u16));
            }
        }
        Self { entries }
    }

    #[inline]
    pub fn has(&self, name: &str) -> bool {
        self.entries
            .iter()
            .any(|(n, _)| n.eq_ignore_ascii_case(name))
    }

    #[inline]
    pub fn first<'a>(&self, attrs: &'a [Attribute], name: &str) -> Option<&'a Attribute> {
        self.entries
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(name))
            .map(|(_, pos)| &attrs[*pos as usize])
    }

    pub fn all<'attrs>(
        &self,
        attrs: &'attrs [Attribute],
        name: &str,
    ) -> impl Iterator<Item = &'attrs Attribute> {
        let positions: SmallVec<[u16; 4]> = self
            .entries
            .iter()
            .filter(|(n, _)| n.eq_ignore_ascii_case(name))
            .map(|(_, pos)| *pos)
            .collect();
        positions.into_iter().map(move |pos| &attrs[pos as usize])
    }

    #[inline]
    pub fn find_by_id<'a>(&self, attrs: &'a [Attribute], id: NodeId) -> Option<&'a Attribute> {
        attrs.iter().find(|attr| attr.id() == id)
    }
}

fn attr_index_name(attr: &Attribute, _source: &str) -> Option<CompactString> {
    let n = match attr {
        Attribute::StringAttribute(a) => a.name.as_str(),
        Attribute::ExpressionAttribute(a) => a.name.as_str(),
        Attribute::BooleanAttribute(a) => a.name.as_str(),
        Attribute::ConcatenationAttribute(a) => a.name.as_str(),
        Attribute::SpreadAttribute(_)
        | Attribute::ClassDirective(_)
        | Attribute::StyleDirective(_)
        | Attribute::BindDirective(_)
        | Attribute::LetDirectiveLegacy(_)
        | Attribute::UseDirective(_)
        | Attribute::OnDirectiveLegacy(_)
        | Attribute::TransitionDirective(_)
        | Attribute::AnimateDirective(_)
        | Attribute::AttachTag(_) => return None,
    };
    Some(CompactString::from(n))
}
