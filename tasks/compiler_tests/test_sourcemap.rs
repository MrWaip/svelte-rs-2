use compiler_tests::{
    cases::{load_sourcemap_case, read_sourcemap_module_source},
    sourcemap_invariants::assert_sourcemap_invariants,
};
use rstest::rstest;
use svelte_compiler::{
    CompileOptions, CompileResult, CssMode, ModuleCompileOptions, SourcemapKind, compile,
    compile_module,
};

fn assert_sourcemap_case(case: &str, mutate: impl FnOnce(&mut CompileOptions)) -> CompileResult {
    let (input, mut opts) = load_sourcemap_case(case);
    mutate(&mut opts);
    let result = compile(&input, &opts);
    let js_output = result
        .js
        .as_ref()
        .unwrap_or_else(|| panic!("[{case}] compile produced no JS"));
    let map = js_output
        .map
        .as_ref()
        .unwrap_or_else(|| panic!("[{case}] JS map is None"));
    assert_sourcemap_invariants(case, &input, map, SourcemapKind::Default);
    result
}

#[rstest]
fn sourcemap_basic() {
    let case = "sourcemap_basic";
    let result = assert_sourcemap_case(case, |_| {});
    let js_output = result.js.expect("compile produced no JS");
    let map = js_output.map.expect("JS map is None");
    let json = map.to_json_string();
    assert!(
        json.contains("\"version\":3"),
        "[{case}] map JSON missing version 3: {json}"
    );
    assert!(map.get_sources().count() > 0, "[{case}] sources empty");
    assert!(
        map.get_tokens().next().is_some(),
        "[{case}] map has no tokens (empty mappings)"
    );
}

#[rstest]
fn sourcemap_sources_content() {
    let case = "sourcemap_sources_content";
    let (input, _) = load_sourcemap_case(case);
    let result = assert_sourcemap_case(case, |opts| {
        opts.filename = "src/Foo.svelte".to_string();
    });
    let js_output = result.js.expect("compile produced no JS");
    let map = js_output.map.expect("JS map is None");
    let sources: Vec<String> = map.get_sources().map(|s| s.to_string()).collect();
    assert_eq!(
        sources.len(),
        1,
        "[{case}] expected 1 source, got {sources:?}"
    );
    assert_eq!(sources[0], "Foo.svelte", "[{case}] sources[0] not basename");
    let content = map
        .get_source_contents()
        .next()
        .flatten()
        .unwrap_or_else(|| panic!("[{case}] sourcesContent[0] missing"));
    assert_eq!(
        content,
        input.as_str(),
        "[{case}] sourcesContent[0] does not equal full Svelte source"
    );
    assert!(
        content.contains("<h1>Hello {name}!</h1>"),
        "[{case}] sourcesContent[0] missing markup"
    );
}

#[rstest]
fn sourcemap_component_block_span() {
    let case = "sourcemap_component_block_span";
    let result = assert_sourcemap_case(case, |_| {});
    let js_output = result.js.expect("compile produced no JS");
    let map = js_output.map.expect("JS map is None");

    let script_open_line: u32 = 2;
    let script_close_line: u32 = 5;
    let mut hits_open = false;
    let mut hits_close = false;
    for tok in map.get_tokens() {
        let line = tok.get_src_line();
        if line == script_open_line {
            hits_open = true;
        }
        if line == script_close_line {
            hits_close = true;
        }
    }
    assert!(
        hits_open || hits_close,
        "[{case}] no token maps to <script> open (line {script_open_line}) or close (line {script_close_line}); body_span not propagated"
    );
}

#[rstest]
fn sourcemap_compile_module_basename() {
    let case = "sourcemap_compile_module";
    let input = read_sourcemap_module_source(case);
    let opts = ModuleCompileOptions {
        filename: "src/widgets/Foo.svelte.js".to_string(),
        ..ModuleCompileOptions::default()
    };
    let result = compile_module(&input, &opts);
    let js_output = result.js.expect("compile_module produced no JS");
    let map = js_output.map.expect("module JS map is None");
    let expected = "Foo.svelte.js";
    assert_eq!(map.get_file(), None, "[{case}] JS map must not carry file");
    let sources: Vec<String> = map.get_sources().map(|s| s.to_string()).collect();
    assert_eq!(
        sources,
        vec![expected.to_string()],
        "[{case}] sources mismatch"
    );
    assert_sourcemap_invariants(case, &input, &map, SourcemapKind::Default);
}

#[rstest]
fn sourcemap_compile_module_fallback() {
    let case = "sourcemap_compile_module";
    let input = read_sourcemap_module_source(case);
    let opts = ModuleCompileOptions::default();
    let result = compile_module(&input, &opts);
    let js_output = result.js.expect("compile_module produced no JS");
    let map = js_output.map.expect("module JS map is None");
    let expected = "input.svelte.js";
    assert_eq!(map.get_file(), None, "[{case}] JS map must not carry file");
    let sources: Vec<String> = map.get_sources().map(|s| s.to_string()).collect();
    assert_eq!(
        sources,
        vec![expected.to_string()],
        "[{case}] sources mismatch"
    );
}

#[rstest]
fn sourcemap_output_filename_relative() {
    let case = "sourcemap_output_filename";
    let result = assert_sourcemap_case(case, |opts| {
        opts.filename = "src/Foo.svelte".to_string();
        opts.output_filename = Some("dist/Foo.js".to_string());
    });
    let js_output = result.js.expect("compile produced no JS");
    let map = js_output.map.expect("JS map is None");
    let expected = "../src/Foo.svelte";
    assert_eq!(map.get_file(), None, "[{case}] JS map must not carry file");
    let sources: Vec<String> = map.get_sources().map(|s| s.to_string()).collect();
    assert_eq!(
        sources,
        vec![expected.to_string()],
        "[{case}] sources mismatch"
    );
}

#[rstest]
fn sourcemap_output_filename_absent() {
    let case = "sourcemap_output_filename";
    let result = assert_sourcemap_case(case, |opts| {
        opts.filename = "src/Foo.svelte".to_string();
    });
    let js_output = result.js.expect("compile produced no JS");
    let map = js_output.map.expect("JS map is None");
    let expected = "Foo.svelte";
    assert_eq!(map.get_file(), None, "[{case}] JS map must not carry file");
    let sources: Vec<String> = map.get_sources().map(|s| s.to_string()).collect();
    assert_eq!(
        sources,
        vec![expected.to_string()],
        "[{case}] sources mismatch"
    );
}

#[rstest]
fn sourcemap_css_granular_mappings() {
    let case = "sourcemap_css_granular";
    let (input, mut opts) = load_sourcemap_case(case);
    opts.css = CssMode::External;
    let result = compile(&input, &opts);
    let css_output = result.css.expect("CSS output missing");
    let map = css_output.map.expect("CSS map None");

    let mut tokens: Vec<(u32, u32)> = map
        .get_tokens()
        .map(|t| (t.get_src_line(), t.get_src_col()))
        .collect();
    tokens.sort_unstable();
    tokens.dedup();
    assert!(
        tokens.len() >= 8,
        "[{case}] expected granular tokens (>=8 distinct src positions), got {}: {tokens:?}",
        tokens.len()
    );
}

#[rstest]
fn sourcemap_css_inline_dev_appends() {
    let case = "sourcemap_css_inline_dev";
    let (input, mut opts) = load_sourcemap_case(case);
    opts.dev = true;
    opts.css = CssMode::Injected;
    let result = compile(&input, &opts);
    let js_output = result.js.expect("compile produced no JS");
    assert!(
        js_output
            .code
            .contains("sourceMappingURL=data:application/json"),
        "[{case}] JS output missing inline sourceMappingURL: {}",
        js_output.code
    );
    assert!(
        js_output.code.contains(";base64,"),
        "[{case}] inline sourceMappingURL not base64-encoded"
    );
}

#[rstest]
fn sourcemap_css_inline_dev_skipped_when_not_dev() {
    let case = "sourcemap_css_inline_dev";
    let (input, mut opts) = load_sourcemap_case(case);
    opts.dev = false;
    opts.css = CssMode::Injected;
    let result = compile(&input, &opts);
    let js_output = result.js.expect("compile produced no JS");
    assert!(
        !js_output.code.contains("sourceMappingURL="),
        "[{case}] non-dev injected output should not carry sourceMappingURL"
    );
}

#[rstest]
fn sourcemap_css_output_filename_set() {
    let case = "sourcemap_css_output_filename";
    let (input, mut opts) = load_sourcemap_case(case);
    opts.css = CssMode::External;
    opts.css_output_filename = Some("dist/Foo.css".to_string());
    let result = compile(&input, &opts);
    let css_output = result.css.as_ref().expect("CSS output missing");
    let map = css_output.map.as_ref().expect("CSS map None");
    assert_eq!(
        map.get_file(),
        Some("Foo.css"),
        "[{case}] map.file != basename of css_output_filename"
    );
}

#[rstest]
fn sourcemap_css_output_filename_fallback() {
    let case = "sourcemap_css_output_filename";
    let (input, mut opts) = load_sourcemap_case(case);
    opts.filename = "src/Foo.svelte".to_string();
    opts.css = CssMode::External;
    let result = compile(&input, &opts);
    let css_output = result.css.as_ref().expect("CSS output missing");
    let map = css_output.map.as_ref().expect("CSS map None");
    assert_eq!(
        map.get_file(),
        Some("Foo.svelte"),
        "[{case}] map.file != basename of filename"
    );
}

#[rstest]
fn sourcemap_css_merges_preprocessor_sources() {
    let input = "<div class=\"a\">x</div>\n<style>.a{color:red}</style>";
    let preprocessor = r#"{"version":3,"sources":["foo.scss"],"sourcesContent":["orig"],"names":[],"mappings":";AAAA"}"#;
    let mut opts = CompileOptions {
        filename: "App.svelte".to_string(),
        css: CssMode::External,
        ..CompileOptions::default()
    };
    opts.preprocessor_map = Some(preprocessor.to_string());
    let result = compile(input, &opts);
    let css_output = result.css.expect("CSS output missing");
    let map = css_output.map.expect("CSS map None");
    let sources: Vec<String> = map.get_sources().map(|s| s.to_string()).collect();
    assert_eq!(
        sources,
        vec!["foo.scss".to_string()],
        "preprocessor source not merged into css sources"
    );
    assert_eq!(
        map.get_file(),
        Some("App.svelte"),
        "css map file must stay basename after merge"
    );
}

#[rstest]
fn sourcemap_js_merges_preprocessor_sources() {
    let input = "<script>let n = 1;</script>\n<h1>{n}</h1>";
    let preprocessor = r#"{"version":3,"sources":["orig.svelte"],"sourcesContent":["orig"],"names":[],"mappings":"AAAA;AAAA;AAAA;AAAA;AAAA"}"#;
    let mut opts = CompileOptions {
        filename: "App.svelte".to_string(),
        ..CompileOptions::default()
    };
    opts.preprocessor_map = Some(preprocessor.to_string());
    let result = compile(input, &opts);
    let js_output = result.js.expect("JS output missing");
    let map = js_output.map.expect("JS map None");
    let sources: Vec<String> = map.get_sources().map(|s| s.to_string()).collect();
    assert_eq!(
        sources,
        vec!["orig.svelte".to_string()],
        "preprocessor source not merged into js sources"
    );
}

#[rstest]
fn sourcemap_css_external() {
    let case = "sourcemap_css_external";
    let (input, mut opts) = load_sourcemap_case(case);
    opts.css = CssMode::External;
    let result = compile(&input, &opts);
    let css_output = result
        .css
        .as_ref()
        .unwrap_or_else(|| panic!("[{case}] CSS output missing"));
    let css_map = css_output
        .map
        .as_ref()
        .unwrap_or_else(|| panic!("[{case}] CSS map is None"));
    let json = css_map.to_json_string();
    assert!(
        json.contains("\"version\":3"),
        "[{case}] CSS map JSON missing version 3: {json}"
    );
    let content = css_map
        .get_source_contents()
        .next()
        .flatten()
        .unwrap_or_else(|| panic!("[{case}] CSS sourcesContent[0] missing"));
    assert!(
        content.contains(".title"),
        "[{case}] CSS sourcesContent[0] missing original selector"
    );
    assert!(
        css_map.get_tokens().next().is_some(),
        "[{case}] CSS map has no tokens"
    );
}
