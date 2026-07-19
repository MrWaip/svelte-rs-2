use svelte_sourcemap::{SourceMap, SourcemapKind};

const SKELETON_TOLERANCE: f32 = 0.5;

pub fn assert_sourcemap_invariants(case: &str, input: &str, map: &SourceMap, kind: SourcemapKind) {
    if !kind.is_enabled() {
        assert!(
            map.get_tokens().next().is_none(),
            "[{case}] map present but SourcemapKind::None"
        );
        return;
    }

    assert_sources_content_matches_input(case, input, map);
    assert_below_skeleton_threshold(case, map);
    assert_generated_positions_monotone(case, map);
}

fn assert_sources_content_matches_input(case: &str, input: &str, map: &SourceMap<'_>) {
    let first = map.get_source_contents().next().flatten();
    let actual = first.unwrap_or_else(|| panic!("[{case}] sourcesContent[0] missing"));
    assert_eq!(actual, input, "[{case}] sourcesContent[0] != input");
}

fn assert_below_skeleton_threshold(case: &str, map: &SourceMap<'_>) {
    let mut total = 0u32;
    let mut zero_zero = 0u32;
    for tok in map.get_tokens() {
        total += 1;
        if tok.get_src_line() == 0 && tok.get_src_col() == 0 {
            zero_zero += 1;
        }
    }
    if total == 0 {
        return;
    }
    let share = zero_zero as f32 / total as f32;
    assert!(
        share < SKELETON_TOLERANCE,
        "[{case}] {zero_zero}/{total} segments at (0,0) — span propagation degenerate"
    );
}

fn assert_generated_positions_monotone(case: &str, map: &SourceMap<'_>) {
    let mut current_line = 0u32;
    let mut current_col = 0u32;
    for (idx, tok) in map.get_tokens().enumerate() {
        let line = tok.get_dst_line();
        let col = tok.get_dst_col();
        if line < current_line {
            panic!("[{case}] token #{idx}: generated line went backwards {current_line} → {line}");
        }
        if line == current_line && col < current_col {
            panic!(
                "[{case}] token #{idx}: generated column went backwards {current_col} → {col} on line {line}"
            );
        }
        current_line = line;
        current_col = col;
    }
}
