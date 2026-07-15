pub const SPAN: Span = Span::new(0, 0);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn size(&self) -> usize {
        debug_assert!(self.start <= self.end);
        (self.end - self.start) as usize
    }

    pub fn merge(&self, other: &Self) -> Self {
        Self::new(self.start.min(other.start), self.end.max(other.end))
    }

    pub fn source_text<'a>(&self, source_text: &'a str) -> &'a str {
        &source_text[self.start as usize..self.end as usize]
    }
}

pub trait GetSpan {
    fn span(&self) -> Span;
}

#[derive(Debug, Clone)]
pub struct LineIndex {
    line_starts: Vec<u32>,
    source: Box<str>,
}

impl LineIndex {
    pub fn new(source: &str) -> Self {
        let bytes = source.as_bytes();
        let mut line_starts = Vec::with_capacity(bytes.len() / 40 + 1);
        line_starts.push(0);
        for (i, &b) in bytes.iter().enumerate() {
            if b == b'\n' {
                line_starts.push((i + 1) as u32);
            }
        }
        Self {
            line_starts,
            source: source.into(),
        }
    }

    pub fn empty() -> Self {
        Self {
            line_starts: vec![0],
            source: "".into(),
        }
    }

    pub fn line_col(&self, byte_offset: u32) -> (usize, usize) {
        let line_idx = self
            .line_starts
            .partition_point(|&start| start <= byte_offset)
            .saturating_sub(1);
        let line_start = self.line_starts[line_idx] as usize;
        let col = self.utf16_column(line_start, byte_offset as usize);
        (line_idx + 1, col)
    }

    fn utf16_column(&self, line_start: usize, byte_offset: usize) -> usize {
        let mut col = 0;
        let mut pos = line_start;
        for ch in self.source[line_start..].chars() {
            if pos >= byte_offset {
                break;
            }
            col += ch.len_utf16();
            pos += ch.len_utf8();
        }
        col
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive(source: &str, offset: u32) -> (usize, usize) {
        let offset = offset as usize;
        let bytes = source.as_bytes();
        let mut line = 1usize;
        let mut col = 0usize;
        for &b in &bytes[..offset.min(bytes.len())] {
            if b == b'\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
        }
        (line, col)
    }

    #[test]
    fn matches_naive_for_ascii() {
        let src = "abc\ndef\n\nghij\nk";
        let idx = LineIndex::new(src);
        for offset in 0..=src.len() as u32 {
            assert_eq!(idx.line_col(offset), naive(src, offset), "offset={offset}");
        }
    }

    #[test]
    fn empty_source() {
        let idx = LineIndex::new("");
        assert_eq!(idx.line_col(0), (1, 0));
    }

    #[test]
    fn offset_past_end_clamps() {
        let src = "ab\ncd";
        let idx = LineIndex::new(src);
        let (line, _) = idx.line_col(999);
        assert_eq!(line, 2);
    }

    #[test]
    fn column_counts_utf16_units_not_bytes() {
        let prefix = "<div>Текст ";
        let src = format!("{prefix}<b></b></div>");
        let idx = LineIndex::new(&src);
        let (line, col) = idx.line_col(prefix.len() as u32);
        assert_eq!((line, col), (1, 11));
    }

    #[test]
    fn column_counts_astral_char_as_surrogate_pair() {
        let prefix = "<div>😀 ";
        let src = format!("{prefix}<b></b></div>");
        let idx = LineIndex::new(&src);
        let (line, col) = idx.line_col(prefix.len() as u32);
        assert_eq!((line, col), (1, 8));
    }
}
