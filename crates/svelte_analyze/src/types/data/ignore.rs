use super::*;

#[derive(Debug, Default)]
pub struct IgnoreData {
    node_snapshot: FxHashMap<NodeId, u32>,
    /// Span-based ignore lookup: statement span.start → snapshot index.
    /// Used for JS script comments where NodeId is not available.
    span_snapshot: FxHashMap<u32, u32>,
    snapshots: Vec<FxHashSet<String>>,
    intern: FxHashMap<Vec<String>, u32>,
}

impl IgnoreData {
    pub fn new() -> Self {
        let empty_set = FxHashSet::default();
        let mut intern = FxHashMap::default();
        intern.insert(Vec::new(), 0);
        Self {
            node_snapshot: FxHashMap::default(),
            span_snapshot: FxHashMap::default(),
            snapshots: vec![empty_set],
            intern,
        }
    }

    pub fn is_ignored(&self, node_id: NodeId, code: &str) -> bool {
        self.node_snapshot
            .get(&node_id)
            .and_then(|&idx| self.snapshots.get(idx as usize))
            .is_some_and(|set| set.contains(code))
    }

    /// Check if a specific ignore code is active for a statement at the given span start.
    pub fn is_ignored_at_span(&self, span_start: u32, code: &str) -> bool {
        self.span_snapshot
            .get(&span_start)
            .and_then(|&idx| self.snapshots.get(idx as usize))
            .is_some_and(|set| set.contains(code))
    }

    /// Scan OXC program comments for `svelte-ignore` directives and populate span_snapshot.
    /// Each comment's `attached_to` field points to the statement it precedes.
    /// Multiple comments targeting the same statement have their codes merged.
    pub fn scan_program_comments(&mut self, program: &oxc_ast::ast::Program<'_>, runes: bool) {
        let src = program.source_text;
        // Multiple consecutive svelte-ignore comments can precede the same statement;
        // merge their codes before interning
        let mut by_attached: FxHashMap<u32, FxHashSet<String>> = FxHashMap::default();

        for comment in program.comments.iter() {
            let s = comment.span.start as usize;
            let e = comment.span.end as usize;
            if e > src.len() {
                continue;
            }
            let raw = &src[s..e];
            if !raw.contains("svelte-ignore") {
                continue;
            }
            // OXC comment spans include the delimiter characters in the raw text,
            // so strip them before extracting codes
            let (inner, inner_offset) = if comment.is_line() {
                (&raw[2..], s as u32 + 2)
            } else if raw.len() >= 4 {
                (&raw[2..raw.len() - 2], s as u32 + 2)
            } else {
                continue;
            };
            let result = svelte_diagnostics::extract_svelte_ignore::extract_svelte_ignore(
                inner_offset,
                inner,
                runes,
            );
            if !result.codes.is_empty() {
                let entry = by_attached.entry(comment.attached_to).or_default();
                for code in result.codes {
                    entry.insert(code);
                }
            }
        }

        for (attached_to, codes) in by_attached {
            let idx = self.intern_snapshot(&codes);
            if idx != 0 {
                self.span_snapshot.insert(attached_to, idx);
            }
        }
    }

    pub(crate) fn intern_snapshot(&mut self, codes: &FxHashSet<String>) -> u32 {
        let mut sorted: Vec<String> = codes.iter().cloned().collect();
        sorted.sort();
        if let Some(&idx) = self.intern.get(&sorted) {
            return idx;
        }
        let idx = self.snapshots.len() as u32;
        self.snapshots.push(codes.clone());
        self.intern.insert(sorted, idx);
        idx
    }

    pub(crate) fn set_snapshot(&mut self, node_id: NodeId, idx: u32) {
        if idx != 0 {
            self.node_snapshot.insert(node_id, idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxc_allocator::Allocator;
    use oxc_parser::Parser;
    use oxc_span::SourceType;

    fn parse_and_scan(src: &str) -> IgnoreData {
        let alloc = Allocator::default();
        let result = Parser::new(&alloc, src, SourceType::mjs()).parse();
        let mut ignore = IgnoreData::new();
        ignore.scan_program_comments(&result.program, true);
        ignore
    }

    #[test]
    fn scan_line_comment() {
        let ignore = parse_and_scan("// svelte-ignore state_snapshot_uncloneable\nlet x = 1;");
        assert!(ignore.is_ignored_at_span(44, "state_snapshot_uncloneable"));
        assert!(!ignore.is_ignored_at_span(44, "await_waterfall"));
    }

    #[test]
    fn scan_block_comment() {
        // "/* svelte-ignore await_waterfall */\n" = 36 bytes, so `let` starts at 36
        let ignore = parse_and_scan("/* svelte-ignore await_waterfall */\nlet x = 1;");
        assert!(ignore.is_ignored_at_span(36, "await_waterfall"));
    }

    #[test]
    fn scan_multiple_codes() {
        // "// svelte-ignore await_waterfall, state_snapshot_uncloneable\n" = 61 bytes
        let ignore = parse_and_scan(
            "// svelte-ignore await_waterfall, state_snapshot_uncloneable\nlet x = 1;",
        );
        assert!(ignore.is_ignored_at_span(61, "await_waterfall"));
        assert!(ignore.is_ignored_at_span(61, "state_snapshot_uncloneable"));
    }

    #[test]
    fn no_match_for_unrelated_span() {
        let ignore = parse_and_scan("let y = 2;\n// svelte-ignore await_waterfall\nlet x = 1;");
        // First statement has no ignore
        assert!(!ignore.is_ignored_at_span(0, "await_waterfall"));
    }
}
