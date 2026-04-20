use std::{collections::HashMap, fs::File, io::Write, path::Path, process::Command};

use glob::glob;
use oxc_allocator::Allocator;
use oxc_codegen::Codegen;
use oxc_parser::Parser;
use oxc_span::SourceType;

fn strip_unused_css_comments(css: &str) -> String {
    let mut out = String::with_capacity(css.len());
    let mut idx = 0;
    while let Some(start) = css[idx..].find("/* (unused) ") {
        let start = idx + start;
        out.push_str(&css[idx..start]);
        let Some(end) = css[start..].find("*/") else {
            out.push_str(&css[start..]);
            return out;
        };
        idx = start + end + 2;
    }
    out.push_str(&css[idx..]);
    out
}

fn main() {
    let compiler_svelte_files = glob("./tasks/compiler_tests/cases2/**/*.svelte")
        .expect("Failed to read glob pattern for .svelte");
    let compiler_module_files = glob("./tasks/compiler_tests/cases2/**/*.svelte.js")
        .expect("Failed to read glob pattern for .svelte.js");
    let diagnostic_svelte_files = glob("./tasks/diagnostic_tests/cases/**/*.svelte")
        .expect("Failed to read glob pattern for diagnostic .svelte");
    let files: Vec<String> = compiler_svelte_files
        .chain(compiler_module_files)
        .chain(diagnostic_svelte_files)
        .map(|entry| entry.expect("test invariant").display().to_string())
        .collect();

    let input_json = serde_json::to_string(&files).expect("test invariant");

    // Write input to temp file since /dev/stdin may not be available
    let tmp_input = std::env::temp_dir().join("svelte_gen_input.json");
    std::fs::write(&tmp_input, &input_json).expect("Failed to write temp input file");

    let output = Command::new("node")
        .arg("./tasks/generate_test_cases/generate.mjs")
        .env("INPUT_FILE", &tmp_input)
        .env("NODE_PATH", "./tasks/generate_test_cases/node_modules")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|child| child.wait_with_output())
        .expect("Failed to run node generate.mjs");

    let _ = std::fs::remove_file(&tmp_input);

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("node generate.mjs failed:\n{stderr}");
    }

    let results: HashMap<String, serde_json::Value> =
        serde_json::from_slice(&output.stdout).expect("Failed to parse node output");

    for (path, case) in &results {
        let dir = Path::new(path).parent().expect("test invariant");

        if let Some(diagnostics) = case.get("diagnostics") {
            let diagnostics_path = dir.join("case-svelte.json");
            let diagnostics_json =
                serde_json::to_string_pretty(diagnostics).expect("Failed to serialize diagnostics");
            File::create(&diagnostics_path)
                .expect("test invariant")
                .write_all(diagnostics_json.as_bytes())
                .expect("test invariant");
            continue;
        }

        // Write case-svelte.js (formatted via OXC)
        let js_src = case["js"].as_str().expect("js field missing");
        let js_path = dir.join("case-svelte.js");
        let mut js_file = File::create(&js_path).expect("test invariant");
        let allocator = Allocator::default();
        let parser = Parser::new(&allocator, js_src, SourceType::default());
        let parsed = parser.parse();
        let codegen = Codegen::new();
        let result = codegen.build(&parsed.program);
        js_file
            .write_all(result.code.as_bytes())
            .expect("test invariant");

        // Write case-svelte.css when a reference file exists.
        if let Some(css) = case["css"].as_str() {
            let css_path = dir.join("case-svelte.css");
            let css = strip_unused_css_comments(css);
            File::create(&css_path)
                .expect("test invariant")
                .write_all(css.as_bytes())
                .expect("test invariant");
        }
    }
}
