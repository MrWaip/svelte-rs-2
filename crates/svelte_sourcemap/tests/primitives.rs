use svelte_sourcemap::{
    SourceMap, SourceMapBuilder, Sourcemap, get_basename, get_relative_path, get_source_name,
    merge_with_preprocessor, parse_input_map,
};

fn empty_map() -> SourceMap<'static> {
    SourceMapBuilder::default().into_sourcemap()
}

fn base_map(tokens: &[(u32, u32, u32, u32)]) -> SourceMap<'static> {
    let mut builder = SourceMapBuilder::default();
    let source_id = builder.add_source_and_content("App.svelte", "");
    builder.set_file("App.svelte");
    for &(dst_line, dst_col, src_line, src_col) in tokens {
        builder.add_token(dst_line, dst_col, src_line, src_col, Some(source_id), None);
    }
    builder.into_sourcemap()
}

fn preprocessor_map(source: &str, tokens: &[(u32, u32, u32, u32)]) -> SourceMap<'static> {
    let mut builder = SourceMapBuilder::default();
    let source_id = builder.add_source_and_content(source, "original contents");
    for &(dst_line, dst_col, src_line, src_col) in tokens {
        builder.add_token(dst_line, dst_col, src_line, src_col, Some(source_id), None);
    }
    builder.into_sourcemap().into_owned()
}

#[track_caller]
fn assert_sources(map: &SourceMap<'_>, expected: &[&str]) {
    let sources: Vec<&str> = map.get_sources().collect();
    assert_eq!(
        sources, expected,
        "merged sources: expected {expected:?}, got {sources:?}"
    );
}

#[track_caller]
fn assert_first_token_src(map: &SourceMap<'_>, expected: (u32, u32)) {
    let token = map.get_tokens().next().expect("merged map has a token");
    let got = (token.get_src_line(), token.get_src_col());
    assert_eq!(
        got, expected,
        "first token source position: expected {expected:?}, got {got:?}"
    );
}

#[test]
fn attach_sources_content_writes_first_entry() {
    let mut sm = Sourcemap::new(empty_map(), "<svelte>contents</svelte>");
    sm.attach_sources_content();
    let map = sm.into_inner();
    let first = map
        .get_source_contents()
        .next()
        .flatten()
        .expect("source content present");
    assert_eq!(first, "<svelte>contents</svelte>");
}

#[test]
fn set_source_name_writes_file_and_sources() {
    let mut sm = Sourcemap::new(empty_map(), "src");
    sm.set_source_name("../src/Foo.svelte");
    let map = sm.into_inner();
    assert_eq!(map.get_file(), Some("../src/Foo.svelte"),);
    let sources: Vec<&str> = map.get_sources().collect();
    assert_eq!(sources, vec!["../src/Foo.svelte"]);
}

#[test]
fn merge_resolves_source_through_preprocessor() {
    let base = base_map(&[(0, 0, 0, 0)]);
    let pre = preprocessor_map("foo.scss", &[(0, 0, 5, 2)]);
    let merged = merge_with_preprocessor(base, &pre, "src/App.svelte", "App.svelte", (0, 0));
    assert_sources(&merged, &["foo.scss"]);
}

#[test]
fn merge_traces_position_into_original_source() {
    let base = base_map(&[(0, 0, 0, 0)]);
    let pre = preprocessor_map("foo.scss", &[(0, 0, 5, 2)]);
    let merged = merge_with_preprocessor(base, &pre, "src/App.svelte", "App.svelte", (0, 0));
    assert_first_token_src(&merged, (5, 2));
}

#[test]
fn merge_applies_base_offset_for_css_block() {
    let base = base_map(&[(0, 0, 0, 0)]);
    let pre = preprocessor_map("foo.scss", &[(3, 0, 9, 4)]);
    let merged = merge_with_preprocessor(base, &pre, "src/App.svelte", "App.svelte", (3, 0));
    assert_first_token_src(&merged, (9, 4));
}

#[test]
fn parse_input_map_tolerates_null_sources() {
    let json = r#"{"version":3,"file":"App.svelte","sources":[null,"foo.scss"],"sourcesContent":[null,"orig"],"names":[],"mappings":"AAAA"}"#;
    let map = parse_input_map(json).expect("null source must not fail parsing");
    let sources: Vec<&str> = map.get_sources().collect();
    assert_eq!(
        sources,
        vec!["", "foo.scss"],
        "null source should be coerced to empty string, got {sources:?}"
    );
}

#[test]
fn merge_skips_empty_source_from_null_entry() {
    let base = base_map(&[(0, 0, 0, 0), (0, 5, 1, 0)]);
    let pre = parse_input_map(
        r#"{"version":3,"sources":[null,"foo.scss"],"names":[],"mappings":"AAAA;ACAA"}"#,
    )
    .expect("parse");
    let merged = merge_with_preprocessor(base, &pre, "src/App.svelte", "App.svelte", (0, 0));
    assert_sources(&merged, &["foo.scss"]);
}

#[test]
fn merge_falls_back_to_basename_when_base_has_no_tokens() {
    let base = base_map(&[]);
    let pre = preprocessor_map("foo.scss", &[(0, 0, 5, 2)]);
    let merged = merge_with_preprocessor(base, &pre, "src/App.svelte", "App.svelte", (0, 0));
    assert_sources(&merged, &["App.svelte"]);
}

#[test]
fn to_inline_comment_starts_with_known_prefix() {
    let sm = Sourcemap::new(empty_map(), "src");
    let comment = sm.to_inline_comment();
    assert!(comment.starts_with("\n/*# sourceMappingURL=data:application/json"));
    assert!(comment.ends_with(" */"));
}

#[test]
fn get_basename_strips_path() {
    assert_eq!(get_basename("src/Foo.svelte"), "Foo.svelte");
    assert_eq!(get_basename("a/b/c/d.js"), "d.js");
}

#[test]
fn get_basename_handles_backslash() {
    assert_eq!(get_basename("src\\Foo.svelte"), "Foo.svelte");
    assert_eq!(get_basename("a\\b/c\\d.js"), "d.js");
}

#[test]
fn get_basename_no_separator() {
    assert_eq!(get_basename("Foo.svelte"), "Foo.svelte");
    assert_eq!(get_basename(""), "");
}

#[test]
fn get_relative_path_no_common_prefix() {
    assert_eq!(
        get_relative_path("dist/Foo.js", "src/Foo.svelte"),
        "../src/Foo.svelte",
    );
}

#[test]
fn get_relative_path_common_prefix() {
    assert_eq!(
        get_relative_path("src/Foo.js", "src/Foo.svelte"),
        "./Foo.svelte",
    );
}

#[test]
fn get_relative_path_equal_paths() {
    assert_eq!(
        get_relative_path("Foo.svelte", "Foo.svelte"),
        "./Foo.svelte"
    );
}

#[test]
fn get_relative_path_deep_nesting() {
    assert_eq!(
        get_relative_path("a/b/c/Foo.js", "x/y/Foo.svelte"),
        "../../../x/y/Foo.svelte",
    );
}

#[test]
fn get_relative_path_windows_backslash() {
    assert_eq!(
        get_relative_path("dist\\Foo.js", "src\\Foo.svelte"),
        "../src/Foo.svelte",
    );
}

#[test]
fn get_source_name_with_output_filename() {
    assert_eq!(
        get_source_name("src/Foo.svelte", Some("dist/Foo.js")),
        "../src/Foo.svelte",
    );
}

#[test]
fn get_source_name_without_output_filename() {
    assert_eq!(get_source_name("src/Foo.svelte", None), "Foo.svelte");
}
