use std::{
    fs::{File, read_to_string},
    io::Write,
};

use pretty_assertions::assert_eq;
use svelte_compiler::{compile, compile_module};
use test_support::{strip_js_comments, strip_reference_only_css_markers};

use crate::cases::{cluster_case_dir, load_cluster_case, load_cluster_module_case};
use crate::sourcemap_invariants::assert_sourcemap_invariants;

fn normalize_css(s: &str) -> String {
    let stripped = strip_reference_only_css_markers(s);
    stripped.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn assert_compiler_module(case: &str) {
    let dir = cluster_case_dir(case);
    let (input, opts) = load_cluster_module_case(case);

    let result = compile_module(&input, &opts);
    let js_output = result
        .js
        .unwrap_or_else(|| panic!("[{case}] compile_module produced no JS"));
    let js = js_output.code;

    let expected = read_to_string(dir.join("case-svelte.js")).expect("test invariant");

    File::create(dir.join("case-rust.js"))
        .expect("test invariant")
        .write_all(js.as_bytes())
        .expect("test invariant");

    assert_eq!(
        strip_js_comments(&js),
        strip_js_comments(&expected),
        "[{case}] JS mismatch"
    );

    if let Some(map) = js_output.map.as_ref() {
        assert_sourcemap_invariants(case, &input, map, svelte_compiler::SourcemapKind::Default);
    }
}

pub fn assert_compiler(case: &str) {
    let dir = cluster_case_dir(case);
    let (input, opts) = load_cluster_case(case);
    let result = compile(&input, &opts);
    let js_output = result
        .js
        .unwrap_or_else(|| panic!("[{case}] compile produced no JS"));
    let js = js_output.code;

    let expected_js = read_to_string(dir.join("case-svelte.js")).expect("test invariant");

    File::create(dir.join("case-rust.js"))
        .expect("test invariant")
        .write_all(js.as_bytes())
        .expect("test invariant");

    assert_eq!(
        strip_js_comments(&js),
        strip_js_comments(&expected_js),
        "[{case}] JS mismatch"
    );

    if let Some(map) = js_output.map.as_ref() {
        assert_sourcemap_invariants(case, &input, map, svelte_compiler::SourcemapKind::Default);
    }

    let expected_css_path = dir.join("case-svelte.css");
    if expected_css_path.exists() {
        let expected_css = read_to_string(&expected_css_path).expect("test invariant");
        let actual_css = result.css.map(|out| out.code).unwrap_or_default();
        File::create(dir.join("case-rust.css"))
            .expect("test invariant")
            .write_all(actual_css.as_bytes())
            .expect("test invariant");
        assert_eq!(
            normalize_css(&actual_css),
            normalize_css(&expected_css),
            "[{case}] CSS mismatch"
        );
    }
}
