use std::{
    fs::{File, read_to_string},
    io::Write,
};

use pretty_assertions::assert_eq;
use svelte_compiler::{GenerateMode, compile, compile_module};
use test_support::{strip_js_comments, strip_reference_only_css_markers};

use crate::cases::{cluster_case_dir, load_cluster_case, load_cluster_module_case};
use crate::sourcemap_invariants::assert_sourcemap_invariants;

fn normalize_css(s: &str) -> String {
    let stripped = strip_reference_only_css_markers(s);
    stripped.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn assert_compiler_module_prod(case: &str) {
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

pub fn assert_compiler_module_dev(case: &str) {
    let dir = cluster_case_dir(case);
    let (input, opts) = load_cluster_module_case(case);

    let mut dev_opts = opts.clone();
    dev_opts.dev = true;
    let dev_js = compile_module(&input, &dev_opts)
        .js
        .unwrap_or_else(|| panic!("[{case}] dev compile_module produced no JS"))
        .code;
    let expected_dev = read_to_string(dir.join("case-svelte.dev.js")).expect("test invariant");
    File::create(dir.join("case-rust.dev.js"))
        .expect("test invariant")
        .write_all(dev_js.as_bytes())
        .expect("test invariant");
    assert_eq!(
        strip_js_comments(&dev_js),
        strip_js_comments(&expected_dev),
        "[{case}] dev JS mismatch"
    );
}

pub fn assert_compiler_module_ssr(case: &str) {
    let dir = cluster_case_dir(case);
    let (input, opts) = load_cluster_module_case(case);

    let mut server_opts = opts.clone();
    server_opts.generate = GenerateMode::Server;
    let server_js = compile_module(&input, &server_opts)
        .js
        .unwrap_or_else(|| panic!("[{case}] server compile_module produced no JS"))
        .code;
    let expected_server =
        read_to_string(dir.join("case-svelte.server.js")).expect("test invariant");
    File::create(dir.join("case-rust.server.js"))
        .expect("test invariant")
        .write_all(server_js.as_bytes())
        .expect("test invariant");
    assert_eq!(
        strip_js_comments(&server_js),
        strip_js_comments(&expected_server),
        "[{case}] server JS mismatch"
    );
}

pub fn assert_compiler_module_ssr_dev(case: &str) {
    let dir = cluster_case_dir(case);
    let (input, opts) = load_cluster_module_case(case);

    let mut server_opts = opts.clone();
    server_opts.generate = GenerateMode::Server;
    server_opts.dev = true;
    let server_js = compile_module(&input, &server_opts)
        .js
        .unwrap_or_else(|| panic!("[{case}] server dev compile_module produced no JS"))
        .code;
    let expected_server =
        read_to_string(dir.join("case-svelte.server.dev.js")).expect("test invariant");
    File::create(dir.join("case-rust.server.dev.js"))
        .expect("test invariant")
        .write_all(server_js.as_bytes())
        .expect("test invariant");
    assert_eq!(
        strip_js_comments(&server_js),
        strip_js_comments(&expected_server),
        "[{case}] server dev JS mismatch"
    );
}

pub fn assert_compiler_prod(case: &str) {
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

pub fn assert_compiler_dev(case: &str) {
    let dir = cluster_case_dir(case);
    let (input, opts) = load_cluster_case(case);
    let mut dev_opts = opts.clone();
    dev_opts.dev = true;
    let dev_js = compile(&input, &dev_opts)
        .js
        .unwrap_or_else(|| panic!("[{case}] dev compile produced no JS"))
        .code;
    let expected_dev_js = read_to_string(dir.join("case-svelte.dev.js")).expect("test invariant");
    File::create(dir.join("case-rust.dev.js"))
        .expect("test invariant")
        .write_all(dev_js.as_bytes())
        .expect("test invariant");
    assert_eq!(
        strip_js_comments(&dev_js),
        strip_js_comments(&expected_dev_js),
        "[{case}] dev JS mismatch"
    );
}

pub fn assert_compiler_ssr(case: &str) {
    let dir = cluster_case_dir(case);
    let (input, opts) = load_cluster_case(case);
    let mut server_opts = opts.clone();
    server_opts.generate = GenerateMode::Server;
    let server_js = compile(&input, &server_opts)
        .js
        .unwrap_or_else(|| panic!("[{case}] server compile produced no JS"))
        .code;
    let expected_server_js =
        read_to_string(dir.join("case-svelte.server.js")).expect("test invariant");
    File::create(dir.join("case-rust.server.js"))
        .expect("test invariant")
        .write_all(server_js.as_bytes())
        .expect("test invariant");
    assert_eq!(
        strip_js_comments(&server_js),
        strip_js_comments(&expected_server_js),
        "[{case}] server JS mismatch"
    );
}

pub fn assert_compiler_ssr_dev(case: &str) {
    let dir = cluster_case_dir(case);
    let (input, opts) = load_cluster_case(case);
    let mut server_opts = opts.clone();
    server_opts.generate = GenerateMode::Server;
    server_opts.dev = true;
    let server_js = compile(&input, &server_opts)
        .js
        .unwrap_or_else(|| panic!("[{case}] server dev compile produced no JS"))
        .code;
    let expected_server_js =
        read_to_string(dir.join("case-svelte.server.dev.js")).expect("test invariant");
    File::create(dir.join("case-rust.server.dev.js"))
        .expect("test invariant")
        .write_all(server_js.as_bytes())
        .expect("test invariant");
    assert_eq!(
        strip_js_comments(&server_js),
        strip_js_comments(&expected_server_js),
        "[{case}] server dev JS mismatch"
    );
}
