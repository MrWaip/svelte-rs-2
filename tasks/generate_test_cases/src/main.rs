use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use glob::glob;
use oxc_allocator::Allocator;
use oxc_codegen::Codegen;
use oxc_parser::Parser;
use oxc_span::SourceType;
use test_support::strip_reference_only_css_markers;

fn main() {
    let compiler_svelte_files = glob("./tasks/compiler_tests/cases2/**/*.svelte")
        .expect("Failed to read glob pattern for .svelte");
    let compiler_module_files = glob("./tasks/compiler_tests/cases2/**/*.svelte.js")
        .expect("Failed to read glob pattern for .svelte.js");
    let compiler_module_ts_files = glob("./tasks/compiler_tests/cases2/**/*.svelte.ts")
        .expect("Failed to read glob pattern for .svelte.ts");
    let cluster_svelte_files = glob("./tasks/compiler_tests/cluster_cases/**/*.svelte")
        .expect("Failed to read glob pattern for cluster .svelte");
    let cluster_module_files = glob("./tasks/compiler_tests/cluster_cases/**/*.svelte.js")
        .expect("Failed to read glob pattern for cluster .svelte.js");
    let cluster_module_ts_files = glob("./tasks/compiler_tests/cluster_cases/**/*.svelte.ts")
        .expect("Failed to read glob pattern for cluster .svelte.ts");
    let diagnostic_svelte_files = glob("./tasks/diagnostic_tests/cases/**/*.svelte")
        .expect("Failed to read glob pattern for diagnostic .svelte");
    let diagnostic_module_files = glob("./tasks/diagnostic_tests/cases/**/*.svelte.js")
        .expect("Failed to read glob pattern for diagnostic .svelte.js");
    let files: Vec<String> = compiler_svelte_files
        .chain(compiler_module_files)
        .chain(compiler_module_ts_files)
        .chain(cluster_svelte_files)
        .chain(cluster_module_files)
        .chain(cluster_module_ts_files)
        .chain(diagnostic_svelte_files)
        .chain(diagnostic_module_files)
        .map(|entry| entry.expect("test invariant").display().to_string())
        .filter(|path| !path.contains("/cluster_cases/preprocess/"))
        .collect();

    let input_json = serde_json::to_string(&files).expect("test invariant");

    // Write input to temp file since /dev/stdin may not be available
    let tmp_input = env::temp_dir().join("svelte_gen_input.json");
    fs::write(&tmp_input, &input_json).expect("Failed to write temp input file");

    let output = Command::new("node")
        .arg("./tasks/generate_test_cases/generate.mjs")
        .env("INPUT_FILE", &tmp_input)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .and_then(|child| child.wait_with_output())
        .expect("Failed to run node generate.mjs");

    let _ = fs::remove_file(&tmp_input);

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

        write_formatted_js(
            &dir.join("case-svelte.js"),
            case["js"].as_str().expect("js field missing"),
        );

        if let Some(js_dev_src) = case.get("jsDev").and_then(|v| v.as_str()) {
            write_formatted_js(&dir.join("case-svelte.dev.js"), js_dev_src);
        }

        if let Some(js_server_src) = case.get("jsServer").and_then(|v| v.as_str()) {
            write_formatted_js(&dir.join("case-svelte.server.js"), js_server_src);
        }

        if let Some(js_server_dev_src) = case.get("jsServerDev").and_then(|v| v.as_str()) {
            write_formatted_js(&dir.join("case-svelte.server.dev.js"), js_server_dev_src);
        }

        if let Some(css) = case["css"].as_str() {
            let css_path = dir.join("case-svelte.css");
            let css = strip_reference_only_css_markers(css);
            File::create(&css_path)
                .expect("test invariant")
                .write_all(css.as_bytes())
                .expect("test invariant");
        }
    }

    generate_preprocess_expectations();
}

fn generate_preprocess_expectations() {
    let case_dirs: Vec<String> = glob("./tasks/compiler_tests/cluster_cases/preprocess/*")
        .expect("Failed to read glob pattern for preprocess cases")
        .filter_map(|entry| entry.ok())
        .filter(|path| path.is_dir())
        .map(|path| path.display().to_string())
        .collect();

    if case_dirs.is_empty() {
        return;
    }

    let input_json = serde_json::to_string(&case_dirs).expect("test invariant");
    let tmp_input = env::temp_dir().join("svelte_gen_preprocess_input.json");
    fs::write(&tmp_input, &input_json).expect("Failed to write temp input file");

    let output = Command::new("node")
        .arg("./tasks/compiler_tests/generate_preprocess.mjs")
        .env("INPUT_FILE", &tmp_input)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .and_then(|child| child.wait_with_output())
        .expect("Failed to run node generate_preprocess.mjs");

    let _ = fs::remove_file(&tmp_input);

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("node generate_preprocess.mjs failed:\n{stderr}");
    }

    let results: HashMap<String, serde_json::Value> =
        serde_json::from_slice(&output.stdout).expect("Failed to parse node output");

    for (dir, expected) in &results {
        let path = Path::new(dir).join("expected.json");
        let json = serde_json::to_string_pretty(expected).expect("test invariant");
        File::create(&path)
            .expect("test invariant")
            .write_all(json.as_bytes())
            .expect("test invariant");
    }
}

fn write_formatted_js(path: &Path, src: &str) {
    let allocator = Allocator::default();
    let parsed = Parser::new(&allocator, src, SourceType::default()).parse();
    let result = Codegen::new().build(&parsed.program);
    File::create(path)
        .expect("test invariant")
        .write_all(result.code.as_bytes())
        .expect("test invariant");
}
