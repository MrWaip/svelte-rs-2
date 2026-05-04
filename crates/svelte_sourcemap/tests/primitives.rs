use svelte_sourcemap::{
    SourceMap, SourceMapBuilder, Sourcemap, get_basename, get_relative_path, get_source_name,
};

fn empty_map() -> SourceMap {
    SourceMapBuilder::default().into_sourcemap()
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
    assert_eq!(first.as_ref(), "<svelte>contents</svelte>");
}

#[test]
fn set_source_name_writes_file_and_sources() {
    let mut sm = Sourcemap::new(empty_map(), "src");
    sm.set_source_name("../src/Foo.svelte");
    let map = sm.into_inner();
    assert_eq!(
        map.get_file().map(|s| s.as_ref()),
        Some("../src/Foo.svelte"),
    );
    let sources: Vec<&str> = map.get_sources().map(|s| s.as_ref()).collect();
    assert_eq!(sources, vec!["../src/Foo.svelte"]);
}

#[test]
fn set_sources_overrides_explicit_list() {
    let mut sm = Sourcemap::new(empty_map(), "src");
    sm.set_sources(&["input.svelte.js"]);
    let map = sm.into_inner();
    let sources: Vec<&str> = map.get_sources().map(|s| s.as_ref()).collect();
    assert_eq!(sources, vec!["input.svelte.js"]);
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
    assert_eq!(get_relative_path("Foo.svelte", "Foo.svelte"), "./Foo.svelte");
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
