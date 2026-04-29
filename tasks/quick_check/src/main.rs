use std::{
    collections::HashMap,
    env, fs, panic,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

use oxc_allocator::Allocator;
use oxc_codegen::Codegen;
use oxc_parser::Parser;
use oxc_span::SourceType;
use pretty_assertions::StrComparison;
use svelte_compiler::{CompileOptions, compile};

const USAGE: &str = "usage: quick_check <path-to-.svelte-file>";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let Some(raw_path) = args.get(1) else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };

    let input_path = match fs::canonicalize(raw_path) {
        Ok(p) => p,
        Err(err) => {
            eprintln!("quick_check: cannot open {raw_path}: {err}");
            return ExitCode::from(2);
        }
    };

    let source = match fs::read_to_string(&input_path) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("quick_check: read {}: {err}", input_path.display());
            return ExitCode::from(2);
        }
    };

    let workspace_root = resolve_workspace_root();

    let our_js = match run_our_compiler(&source) {
        OurOutcome::Js(js) => format_js(&js),
        OurOutcome::NoJs(diagnostics) => {
            eprintln!("quick_check: rust compiler returned no JS");
            for d in diagnostics {
                eprintln!("  {d}");
            }
            return ExitCode::from(3);
        }
        OurOutcome::Panic(msg) => {
            eprintln!("quick_check: rust compiler panicked");
            eprintln!("  {msg}");
            return ExitCode::from(3);
        }
    };

    let ref_js = match run_reference_compiler(&workspace_root, &input_path) {
        Ok(js) => format_js(&js),
        Err(err) => {
            eprintln!("quick_check: reference compiler failed: {err}");
            return ExitCode::from(4);
        }
    };

    if our_js == ref_js {
        println!(
            "OK: rust output matches reference ({} lines)",
            our_js.lines().count()
        );
        return ExitCode::SUCCESS;
    }

    println!("MISMATCH: rust output diverges from reference");
    println!();
    println!("{}", StrComparison::new(&our_js, &ref_js));
    ExitCode::from(1)
}

enum OurOutcome {
    Js(String),
    NoJs(Vec<String>),
    Panic(String),
}

fn run_our_compiler(source: &str) -> OurOutcome {
    let opts = CompileOptions {
        name: Some("App".into()),
        ..Default::default()
    };
    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| compile(source, &opts)));
    match result {
        Ok(res) => match res.js {
            Some(js) => OurOutcome::Js(js),
            None => {
                let diagnostics = res
                    .diagnostics
                    .into_iter()
                    .map(|d| format!("{d:?}"))
                    .collect();
                OurOutcome::NoJs(diagnostics)
            }
        },
        Err(payload) => {
            let msg = panic_payload_message(&payload);
            OurOutcome::Panic(msg)
        }
    }
}

fn panic_payload_message(payload: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&'static str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "<non-string panic payload>".to_string()
    }
}

fn run_reference_compiler(workspace_root: &Path, input_path: &Path) -> Result<String, String> {
    let generate_mjs = workspace_root.join("tasks/generate_test_cases/generate.mjs");
    let node_modules = workspace_root.join("tasks/generate_test_cases/node_modules");

    if !node_modules.exists() {
        return Err(format!(
            "reference deps not installed. Run: cd {} && npm install",
            workspace_root.join("tasks/generate_test_cases").display()
        ));
    }

    let file_key = input_path.display().to_string();
    let input_json = serde_json::to_string(&vec![file_key.clone()])
        .map_err(|e| format!("serialize input list: {e}"))?;

    let tmp_input = env::temp_dir().join(format!("svelte_quick_check_{}.json", std::process::id()));
    fs::write(&tmp_input, input_json).map_err(|e| format!("write temp input: {e}"))?;

    let output = Command::new("node")
        .arg(&generate_mjs)
        .env("INPUT_FILE", &tmp_input)
        .env("NODE_PATH", &node_modules)
        .output();
    let _ = fs::remove_file(&tmp_input);

    let output = output.map_err(|e| format!("spawn node: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "node generate.mjs exit {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let parsed: HashMap<String, serde_json::Value> =
        serde_json::from_slice(&output.stdout).map_err(|e| format!("parse node output: {e}"))?;
    let entry = parsed
        .get(&file_key)
        .ok_or_else(|| format!("no entry for {file_key} in node output"))?;

    entry
        .get("js")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| "reference compiler did not return a `js` field".to_string())
}

fn format_js(src: &str) -> String {
    let allocator = Allocator::default();
    let parser = Parser::new(&allocator, src, SourceType::default());
    let parsed = parser.parse();
    Codegen::new().build(&parsed.program).code
}

fn resolve_workspace_root() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .and_then(|p| p.parent())
        .map(Path::to_path_buf)
        .unwrap_or(manifest)
}
