use std::{
    any::Any,
    collections::HashMap,
    env, fs,
    io::{self, Read},
    panic,
    path::{Path, PathBuf},
    process::{self, Command, ExitCode},
};

use oxc_allocator::Allocator;
use oxc_codegen::Codegen;
use oxc_parser::Parser;
use oxc_span::SourceType;
use pretty_assertions::StrComparison;
use svelte_compiler::{
    CompileOptions, GenerateMode, ModuleCompileOptions, RunesOption, compile, compile_module,
};

const USAGE: &str = "usage: quick_check <path-to-.svelte-file|-> [--mode=auto|runes|legacy] [--generate=client|server] [--dev] [--filename=<name>] [--print=diff|ours|ref|both]\n  pass `-` to read source from stdin (extension inferred from --filename, default .svelte)";

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum PrintMode {
    #[default]
    Diff,
    Ours,
    Ref,
    Both,
}

#[derive(Default)]
struct CliOptions {
    mode: Option<RunesOption>,
    generate: Option<GenerateMode>,
    dev: bool,
    filename: Option<String>,
    print: PrintMode,
}

fn parse_cli(args: &[String]) -> Result<(String, CliOptions), String> {
    let mut path: Option<String> = None;
    let mut opts = CliOptions::default();
    for arg in args.iter().skip(1) {
        if let Some(v) = arg.strip_prefix("--mode=") {
            opts.mode = Some(match v {
                "auto" => RunesOption::Auto,
                "runes" => RunesOption::Runes,
                "legacy" => RunesOption::Legacy,
                other => return Err(format!("unknown --mode value: {other}")),
            });
        } else if let Some(v) = arg.strip_prefix("--generate=") {
            opts.generate = Some(match v {
                "client" => GenerateMode::Client,
                "server" => GenerateMode::Server,
                "false" => GenerateMode::False,
                other => return Err(format!("unknown --generate value: {other}")),
            });
        } else if arg == "--dev" {
            opts.dev = true;
        } else if let Some(v) = arg.strip_prefix("--filename=") {
            opts.filename = Some(v.to_string());
        } else if let Some(v) = arg.strip_prefix("--print=") {
            opts.print = match v {
                "diff" => PrintMode::Diff,
                "ours" => PrintMode::Ours,
                "ref" => PrintMode::Ref,
                "both" => PrintMode::Both,
                other => return Err(format!("unknown --print value: {other}")),
            };
        } else if arg.starts_with("--") {
            return Err(format!("unknown flag: {arg}"));
        } else if path.is_none() {
            path = Some(arg.clone());
        } else {
            return Err(format!("unexpected positional argument: {arg}"));
        }
    }
    let path = path.ok_or_else(|| USAGE.to_string())?;
    Ok((path, opts))
}

fn reference_config_json(opts: &CliOptions) -> Option<String> {
    let mut entries: Vec<String> = Vec::new();
    match opts.mode {
        Some(RunesOption::Auto) | None => entries.push("\"runes\":null".to_string()),
        Some(RunesOption::Runes) => entries.push("\"runes\":true".to_string()),
        Some(RunesOption::Legacy) => entries.push("\"runes\":false".to_string()),
    }
    match opts.generate {
        Some(GenerateMode::Client) => entries.push("\"generate\":\"client\"".to_string()),
        Some(GenerateMode::Server) => entries.push("\"generate\":\"server\"".to_string()),
        Some(GenerateMode::False) => entries.push("\"generate\":false".to_string()),
        None => {}
    }
    if opts.dev {
        entries.push("\"dev\":true".to_string());
    }
    if let Some(ref name) = opts.filename {
        entries.push(format!(
            "\"filename\":{}",
            serde_json::to_string(name).expect("filename serializable")
        ));
    }
    if entries.is_empty() {
        None
    } else {
        Some(format!("{{{}}}", entries.join(",")))
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let (raw_path, cli_opts) = match parse_cli(&args) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(2);
        }
    };

    let _stdin_temp;
    let (source, input_path) = if raw_path == "-" {
        let mut source = String::new();
        if let Err(err) = io::stdin().read_to_string(&mut source) {
            eprintln!("quick_check: read stdin: {err}");
            return ExitCode::from(2);
        }
        let ext = stdin_extension(&cli_opts);
        let temp = env::temp_dir().join(format!("svelte_quick_check_stdin_{}{ext}", process::id()));
        if let Err(err) = fs::write(&temp, &source) {
            eprintln!("quick_check: write stdin temp {}: {err}", temp.display());
            return ExitCode::from(2);
        }
        _stdin_temp = TempFileGuard(temp.clone());
        (source, temp)
    } else {
        let input_path = match fs::canonicalize(&raw_path) {
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
        (source, input_path)
    };

    let workspace_root = resolve_workspace_root();

    let is_module = is_module_path(&input_path);
    let (our_js, our_css) = match run_our_compiler(&source, &cli_opts, is_module) {
        OurOutcome::Js { js, css } => (format_js(&js), css),
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

    let (ref_js, ref_css) = match run_reference_compiler(&workspace_root, &input_path, &cli_opts) {
        Ok(out) => (format_js(&out.js), out.css),
        Err(err) => {
            eprintln!("quick_check: reference compiler failed: {err}");
            return ExitCode::from(4);
        }
    };

    let our_js =
        test_support::canonicalize_injected_css_in_js(&test_support::canonicalize_js(&our_js));
    let ref_js =
        test_support::canonicalize_injected_css_in_js(&test_support::canonicalize_js(&ref_js));

    let applied = describe_applied_options(&cli_opts);
    let our_css_norm = our_css
        .as_deref()
        .map(test_support::canonicalize_injected_css);
    let ref_css_norm = ref_css
        .as_deref()
        .map(test_support::canonicalize_injected_css);
    let js_match = our_js == ref_js;
    let css_match = our_css_norm == ref_css_norm;

    if cli_opts.print != PrintMode::Diff {
        if matches!(cli_opts.print, PrintMode::Ours | PrintMode::Both) {
            println!("==== RUST JS ====");
            println!("{our_js}");
            if let Some(css) = our_css_norm.as_deref().filter(|s| !s.is_empty()) {
                println!("==== RUST CSS ====");
                println!("{css}");
            }
        }
        if matches!(cli_opts.print, PrintMode::Ref | PrintMode::Both) {
            println!("==== REFERENCE JS ====");
            println!("{ref_js}");
            if let Some(css) = ref_css_norm.as_deref().filter(|s| !s.is_empty()) {
                println!("==== REFERENCE CSS ====");
                println!("{css}");
            }
        }
        println!(
            "---- {} ----",
            if js_match && css_match {
                "OK"
            } else {
                "MISMATCH"
            }
        );
        return if js_match && css_match {
            ExitCode::SUCCESS
        } else {
            ExitCode::from(1)
        };
    }

    if js_match && css_match {
        let css_bytes: usize = our_css_norm.as_deref().map(str::len).unwrap_or(0);
        println!(
            "OK: rust output matches reference (js: {} lines, css: {} bytes normalized){applied}",
            our_js.lines().count(),
            css_bytes,
        );
        return ExitCode::SUCCESS;
    }

    println!("MISMATCH: rust output diverges from reference{applied}");
    if !js_match {
        println!();
        println!("-- JS --");
        println!("{}", StrComparison::new(&our_js, &ref_js));
    }
    if !css_match {
        println!();
        println!("-- CSS (whitespace-normalized) --");
        let our = our_css_norm.as_deref().unwrap_or("<none>");
        let reference = ref_css_norm.as_deref().unwrap_or("<none>");
        println!("{}", StrComparison::new(&our, &reference));
    }
    ExitCode::from(1)
}

enum OurOutcome {
    Js { js: String, css: Option<String> },
    NoJs(Vec<String>),
    Panic(String),
}

struct ReferenceOutput {
    js: String,
    css: Option<String>,
}

fn is_module_path(path: &Path) -> bool {
    let s = path.to_string_lossy();
    s.ends_with(".svelte.js") || s.ends_with(".svelte.ts")
}

struct TempFileGuard(PathBuf);

impl Drop for TempFileGuard {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

fn stdin_extension(cli: &CliOptions) -> &'static str {
    match cli.filename.as_deref() {
        Some(name) if name.ends_with(".svelte.ts") => ".svelte.ts",
        Some(name) if name.ends_with(".svelte.js") => ".svelte.js",
        _ => ".svelte",
    }
}

fn run_our_compiler(source: &str, cli: &CliOptions, is_module: bool) -> OurOutcome {
    if is_module {
        return run_our_module_compiler(source, cli);
    }
    let mut opts = CompileOptions {
        name: Some("App".into()),
        ..Default::default()
    };
    if let Some(mode) = cli.mode {
        opts.runes = mode;
    }
    if let Some(generate) = cli.generate {
        opts.generate = generate;
    }
    if cli.dev {
        opts.dev = true;
    }
    if let Some(ref name) = cli.filename {
        opts.filename = name.clone();
    }
    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| compile(source, &opts)));
    match result {
        Ok(res) => match res.js {
            Some(js) => OurOutcome::Js {
                js: js.code,
                css: res.css.map(|c| c.code),
            },
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

fn run_our_module_compiler(source: &str, cli: &CliOptions) -> OurOutcome {
    let filename = cli
        .filename
        .clone()
        .unwrap_or_else(|| "case.svelte.js".to_string());
    let mut opts = ModuleCompileOptions {
        filename,
        ..Default::default()
    };
    if let Some(generate) = cli.generate {
        opts.generate = generate;
    }
    if cli.dev {
        opts.dev = true;
    }
    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| compile_module(source, &opts)));
    match result {
        Ok(res) => match res.js {
            Some(js) => OurOutcome::Js {
                js: js.code,
                css: None,
            },
            None => {
                let diagnostics = res
                    .diagnostics
                    .into_iter()
                    .map(|d| format!("{d:?}"))
                    .collect();
                OurOutcome::NoJs(diagnostics)
            }
        },
        Err(payload) => OurOutcome::Panic(panic_payload_message(&payload)),
    }
}

fn panic_payload_message(payload: &Box<dyn Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&'static str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "<non-string panic payload>".to_string()
    }
}

fn run_reference_compiler(
    workspace_root: &Path,
    input_path: &Path,
    cli: &CliOptions,
) -> Result<ReferenceOutput, String> {
    let generate_mjs = workspace_root.join("tasks/generate_test_cases/generate.mjs");
    let node_modules = workspace_root.join("node_modules");

    if !node_modules.exists() {
        return Err(format!(
            "reference deps not installed. Run: npm install (from {})",
            workspace_root.display()
        ));
    }

    let file_key = input_path.display().to_string();
    let input_json = serde_json::to_string(&vec![file_key.clone()])
        .map_err(|e| format!("serialize input list: {e}"))?;

    let tmp_input = env::temp_dir().join(format!("svelte_quick_check_{}.json", process::id()));
    fs::write(&tmp_input, input_json).map_err(|e| format!("write temp input: {e}"))?;

    let mut cmd = Command::new("node");
    cmd.arg(&generate_mjs).env("INPUT_FILE", &tmp_input);
    if let Some(cfg) = reference_config_json(cli) {
        cmd.env("QUICK_CHECK_CONFIG", cfg);
    }
    let output = cmd.output();
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

    let js = entry
        .get("js")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| "reference compiler did not return a `js` field".to_string())?;
    let css = entry
        .get("css")
        .and_then(|v| if v.is_null() { None } else { v.as_str() })
        .map(|s| s.to_string());
    Ok(ReferenceOutput { js, css })
}

fn format_js(src: &str) -> String {
    let allocator = Allocator::default();
    let parser = Parser::new(&allocator, src, SourceType::default());
    let parsed = parser.parse();
    Codegen::new().build(&parsed.program).code
}

fn describe_applied_options(cli: &CliOptions) -> String {
    let mut parts: Vec<String> = Vec::new();
    match cli.mode {
        Some(RunesOption::Auto) => parts.push("mode=auto".to_string()),
        Some(RunesOption::Runes) => parts.push("mode=runes".to_string()),
        Some(RunesOption::Legacy) => parts.push("mode=legacy".to_string()),
        None => {}
    }
    match cli.generate {
        Some(GenerateMode::Client) => parts.push("generate=client".to_string()),
        Some(GenerateMode::Server) => parts.push("generate=server".to_string()),
        Some(GenerateMode::False) => parts.push("generate=false".to_string()),
        None => {}
    }
    if cli.dev {
        parts.push("dev".to_string());
    }
    if let Some(ref name) = cli.filename {
        parts.push(format!("filename={name}"));
    }
    if parts.is_empty() {
        String::new()
    } else {
        format!(" [{}]", parts.join(", "))
    }
}

fn resolve_workspace_root() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .and_then(|p| p.parent())
        .map(Path::to_path_buf)
        .unwrap_or(manifest)
}
