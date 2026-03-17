use std::{collections::HashMap, fs::File, io::Write, path::Path, process::Command};

use glob::glob;
use oxc_allocator::Allocator;
use oxc_codegen::Codegen;
use oxc_parser::Parser;
use oxc_span::SourceType;

fn main() {
    let svelte_files = glob("./tasks/compiler_tests/cases2/**/*.svelte")
        .expect("Failed to read glob pattern for .svelte");
    let module_files = glob("./tasks/compiler_tests/cases2/**/*.svelte.js")
        .expect("Failed to read glob pattern for .svelte.js");
    let files: Vec<String> = svelte_files
        .chain(module_files)
        .map(|entry| entry.unwrap().display().to_string())
        .collect();

    let input_json = serde_json::to_string(&files).unwrap();

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

    let results: HashMap<String, String> =
        serde_json::from_slice(&output.stdout).expect("Failed to parse node output");

    for (path, source) in &results {
        let p = Path::new(path).parent().unwrap().join("case-svelte.js");
        let mut file = File::create(&p).unwrap();

        let allocator = Allocator::default();
        let parser = Parser::new(&allocator, source, SourceType::default());
        let result = parser.parse();
        let codegen = Codegen::new();
        let result = codegen.build(&result.program);

        file.write_all(result.code.as_bytes()).unwrap();
    }
}
