use std::{
    collections::{BTreeSet, HashMap},
    env,
    fs::{self, File},
    io::{self, BufWriter, Write},
    panic,
    path::{Path, PathBuf},
    process::{self, Command, ExitCode, Stdio},
};

use glob::glob;
use oxc_allocator::Allocator;
use oxc_codegen::Codegen;
use oxc_parser::Parser;
use oxc_span::SourceType;
use rayon::prelude::*;
use serde::Deserialize;
use similar::{ChangeTag, TextDiff};
use svelte_compiler::{
    CompileOptions, GenerateMode, ModuleCompileOptions, RunesOption, compile, compile_module,
};
use svelte_diagnostics::Severity;

const USAGE: &str = "usage: sweep [--mode=auto|runes|legacy] [--chunk=N] [--print-diffs] [--out=<file>] <directory>";

const DEFAULT_CHUNK: usize = 300;

struct CellSpec {
    label: &'static str,
    reference_key: &'static str,
    dev: bool,
    generate: GenerateMode,
}

const CELLS: [CellSpec; 4] = [
    CellSpec {
        label: "client",
        reference_key: "clientProd",
        dev: false,
        generate: GenerateMode::Client,
    },
    CellSpec {
        label: "client-dev",
        reference_key: "clientDev",
        dev: true,
        generate: GenerateMode::Client,
    },
    CellSpec {
        label: "server",
        reference_key: "serverProd",
        dev: false,
        generate: GenerateMode::Server,
    },
    CellSpec {
        label: "server-dev",
        reference_key: "serverDev",
        dev: true,
        generate: GenerateMode::Server,
    },
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Auto,
    Runes,
    Legacy,
}

struct Cli {
    directory: String,
    mode: Mode,
    chunk: usize,
    print_diffs: bool,
    out: Option<String>,
}

fn parse_cli(args: &[String]) -> Result<Cli, String> {
    let mut directory: Option<String> = None;
    let mut mode = Mode::Auto;
    let mut chunk = DEFAULT_CHUNK;
    let mut print_diffs = false;
    let mut out: Option<String> = None;
    for arg in args.iter().skip(1) {
        if let Some(value) = arg.strip_prefix("--out=") {
            if value.is_empty() {
                return Err("--out requires a file path".to_string());
            }
            out = Some(value.to_string());
        } else if let Some(value) = arg.strip_prefix("--mode=") {
            mode = match value {
                "auto" => Mode::Auto,
                "runes" => Mode::Runes,
                "legacy" => Mode::Legacy,
                other => return Err(format!("unknown --mode value: {other}")),
            };
        } else if let Some(value) = arg.strip_prefix("--chunk=") {
            chunk = value
                .parse()
                .map_err(|_| format!("invalid --chunk value: {value}"))?;
            if chunk == 0 {
                return Err("--chunk must be greater than zero".to_string());
            }
        } else if arg == "--print-diffs" {
            print_diffs = true;
        } else if arg.starts_with("--") {
            return Err(format!("unknown flag: {arg}"));
        } else if directory.is_none() {
            directory = Some(arg.clone());
        } else {
            return Err(format!("unexpected positional argument: {arg}"));
        }
    }
    Ok(Cli {
        directory: directory.ok_or_else(|| USAGE.to_string())?,
        mode,
        chunk,
        print_diffs,
        out,
    })
}

#[derive(Deserialize)]
struct ReferenceCell {
    js: Option<String>,
    css: Option<String>,
    #[serde(default)]
    warnings: Vec<String>,
    error: Option<String>,
    #[serde(default)]
    codes: Vec<String>,
}

#[derive(Deserialize)]
struct ReferenceFile {
    #[serde(rename = "readError")]
    read_error: Option<String>,
    #[serde(rename = "formatError")]
    format_error: Option<String>,
    #[serde(rename = "clientProd")]
    client_prod: Option<ReferenceCell>,
    #[serde(rename = "clientDev")]
    client_dev: Option<ReferenceCell>,
    #[serde(rename = "serverProd")]
    server_prod: Option<ReferenceCell>,
    #[serde(rename = "serverDev")]
    server_dev: Option<ReferenceCell>,
}

impl ReferenceFile {
    fn cell(&self, key: &str) -> Option<&ReferenceCell> {
        match key {
            "clientProd" => self.client_prod.as_ref(),
            "clientDev" => self.client_dev.as_ref(),
            "serverProd" => self.server_prod.as_ref(),
            "serverDev" => self.server_dev.as_ref(),
            _ => None,
        }
    }
}

enum OurCell {
    Ok {
        js: String,
        css: Option<String>,
        error_codes: BTreeSet<String>,
    },
    Error {
        codes: BTreeSet<String>,
    },
    Panic,
}

struct Finding {
    reason: &'static str,
    file: String,
    mode: &'static str,
    size: usize,
    diff: Option<String>,
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let cli = match parse_cli(&args) {
        Ok(cli) => cli,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(2);
        }
    };

    let root = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let directory = Path::new(&cli.directory);
    let directory = if directory.is_absolute() {
        directory.to_path_buf()
    } else {
        root.join(directory)
    };
    if !directory.is_dir() {
        eprintln!("sweep: not a directory: {}", directory.display());
        return ExitCode::from(2);
    }
    if !root.join("node_modules").exists() {
        eprintln!("sweep: reference deps not installed. Run: npm install");
        return ExitCode::from(2);
    }

    let files = enumerate_files(&directory);
    if files.is_empty() {
        eprintln!("sweep: no svelte files found in {}", directory.display());
        return ExitCode::from(2);
    }

    let mode_label = match cli.mode {
        Mode::Auto => "auto",
        Mode::Runes => "runes",
        Mode::Legacy => "legacy",
    };
    println!("scan: {}", directory.display());
    println!("mode: {mode_label}");
    println!("files: {}", files.len());

    panic::set_hook(Box::new(|_| {}));

    let reference = match fetch_reference(&root, &files, cli.mode, cli.chunk) {
        Ok(reference) => reference,
        Err(err) => {
            eprintln!("sweep: reference compiler failed: {err}");
            return ExitCode::from(4);
        }
    };

    let root_dir = root.display().to_string();
    let findings: Vec<Finding> = files
        .par_iter()
        .flat_map(|file| sweep_file(file, &root_dir, cli.mode, reference.get(file)))
        .collect();

    if let Err(err) = report(&files, &findings, cli.print_diffs, cli.out.as_deref()) {
        eprintln!("sweep: write report: {err}");
    }

    if findings.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn enumerate_files(directory: &Path) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    for pattern in ["**/*.svelte", "**/*.svelte.js", "**/*.svelte.ts"] {
        let full = directory.join(pattern);
        let Some(full) = full.to_str() else { continue };
        let Ok(entries) = glob(full) else { continue };
        for entry in entries.flatten() {
            files.push(entry.display().to_string());
        }
    }
    files.sort();
    files.dedup();
    files
}

fn mode_config_json(mode: Mode) -> String {
    match mode {
        Mode::Auto => "{}".to_string(),
        Mode::Runes => "{\"runes\":true}".to_string(),
        Mode::Legacy => "{\"runes\":false}".to_string(),
    }
}

fn fetch_reference(
    root: &Path,
    files: &[String],
    mode: Mode,
    chunk: usize,
) -> Result<HashMap<String, ReferenceFile>, String> {
    let script = root.join("tasks/sweep/reference.mjs");
    let config = mode_config_json(mode);
    let chunks: Vec<(usize, &[String])> = files.chunks(chunk).enumerate().collect();

    let partials: Vec<Result<HashMap<String, ReferenceFile>, String>> = chunks
        .par_iter()
        .map(|(index, batch)| run_reference_chunk(root, &script, &config, *index, batch))
        .collect();

    let mut reference = HashMap::with_capacity(files.len());
    for partial in partials {
        reference.extend(partial?);
    }
    Ok(reference)
}

fn run_reference_chunk(
    root: &Path,
    script: &Path,
    config: &str,
    index: usize,
    batch: &[String],
) -> Result<HashMap<String, ReferenceFile>, String> {
    let input_json =
        serde_json::to_string(batch).map_err(|err| format!("serialize input list: {err}"))?;
    let input_path = env::temp_dir().join(format!("svelte_sweep_{}_{index}.json", process::id()));
    fs::write(&input_path, input_json).map_err(|err| format!("write temp input: {err}"))?;

    let output = Command::new("node")
        .arg(script)
        .current_dir(root)
        .env("INPUT_FILE", &input_path)
        .env("SWEEP_CONFIG", config)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();
    let _ = fs::remove_file(&input_path);

    let output = output.map_err(|err| format!("spawn node: {err}"))?;
    if !output.status.success() {
        return Err(format!(
            "node reference.mjs exit {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    serde_json::from_slice(&output.stdout).map_err(|err| format!("parse node output: {err}"))
}

fn sweep_file(
    file: &str,
    root_dir: &str,
    mode: Mode,
    reference: Option<&ReferenceFile>,
) -> Vec<Finding> {
    let Some(reference) = reference else {
        return vec![finding("reference-missing", file, "", 0, None)];
    };
    if let Some(message) = &reference.read_error {
        return vec![finding("read-error", file, "", 0, Some(message.clone()))];
    }
    if let Some(message) = &reference.format_error {
        return vec![finding("format-error", file, "", 0, Some(message.clone()))];
    }

    let source = match fs::read_to_string(file) {
        Ok(source) => source,
        Err(err) => return vec![finding("read-error", file, "", 0, Some(err.to_string()))],
    };
    let is_module = file.ends_with(".svelte.js") || file.ends_with(".svelte.ts");

    let our_cells: Vec<OurCell> = CELLS
        .iter()
        .map(|cell| run_our_cell(&source, file, root_dir, is_module, mode, cell))
        .collect();

    let mut findings = Vec::new();
    for (index, cell) in CELLS.iter().enumerate() {
        let Some(their) = reference.cell(cell.reference_key) else {
            findings.push(finding("reference-missing", file, cell.label, 0, None));
            continue;
        };
        if let Some(item) = compare_cell(file, cell.label, &our_cells[index], their) {
            findings.push(item);
        }
    }

    if let (OurCell::Ok { css: our_css, .. }, Some(their)) =
        (&our_cells[0], reference.cell(CELLS[0].reference_key))
        && their.error.is_none()
        && let Some(item) = compare_css(file, our_css.as_deref(), their.css.as_deref())
    {
        findings.push(item);
    }

    findings
}

fn run_our_cell(
    source: &str,
    file: &str,
    root_dir: &str,
    is_module: bool,
    mode: Mode,
    cell: &CellSpec,
) -> OurCell {
    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        if is_module {
            let opts = ModuleCompileOptions {
                filename: file.to_string(),
                dev: cell.dev,
                generate: cell.generate,
                root_dir: Some(root_dir.to_string()),
                ..Default::default()
            };
            compile_module(source, &opts)
        } else {
            let opts = CompileOptions {
                name: Some("App".to_string()),
                filename: file.to_string(),
                dev: cell.dev,
                generate: cell.generate,
                disclose_version: false,
                runes: runes_option(mode),
                root_dir: Some(root_dir.to_string()),
                ..Default::default()
            };
            compile(source, &opts)
        }
    }));

    match result {
        Ok(result) => {
            let error_codes = error_severity_codes(&result.diagnostics);
            match result.js {
                Some(js) => OurCell::Ok {
                    js: js.code,
                    css: result.css.map(|css| css.code),
                    error_codes,
                },
                None => OurCell::Error { codes: error_codes },
            }
        }
        Err(_) => OurCell::Panic,
    }
}

fn runes_option(mode: Mode) -> RunesOption {
    match mode {
        Mode::Auto => RunesOption::Auto,
        Mode::Runes => RunesOption::Runes,
        Mode::Legacy => RunesOption::Legacy,
    }
}

fn error_severity_codes(diagnostics: &[svelte_diagnostics::Diagnostic]) -> BTreeSet<String> {
    diagnostics
        .iter()
        .filter(|diagnostic| matches!(diagnostic.severity, Severity::Error))
        .map(|diagnostic| diagnostic.kind.code().to_string())
        .collect()
}

fn compare_cell(
    file: &str,
    mode: &'static str,
    our: &OurCell,
    their: &ReferenceCell,
) -> Option<Finding> {
    let their_error = their.error.is_some();
    let our_error = matches!(our, OurCell::Error { .. } | OurCell::Panic);

    if our_error || their_error {
        if our_error && their_error {
            let our_codes = our_error_codes(our);
            let their_codes: BTreeSet<String> = their.codes.iter().cloned().collect();
            if !their_codes.is_empty() && their_codes.iter().all(|code| our_codes.contains(code)) {
                return None;
            }
            return Some(finding("errors-differ", file, mode, 0, None));
        }
        if our_error {
            let detail = match our {
                OurCell::Panic => Some("rust panic".to_string()),
                OurCell::Error { codes } => {
                    Some(codes.iter().cloned().collect::<Vec<_>>().join(", "))
                }
                OurCell::Ok { .. } => None,
            };
            return Some(finding("rust-error", file, mode, 0, detail));
        }
        return Some(finding(
            "reference-error",
            file,
            mode,
            0,
            their.error.clone(),
        ));
    }

    let (our_js, our_diag) = match our {
        OurCell::Ok {
            js, error_codes, ..
        } => (js, error_codes),
        _ => unreachable!(),
    };
    let their_diag: BTreeSet<String> = their.warnings.iter().cloned().collect();

    if !their_diag.is_empty() {
        if *our_diag == their_diag {
            return None;
        }
        return Some(finding("diagnostics-mismatch", file, mode, 0, None));
    }
    if !our_diag.is_empty() {
        return Some(finding("diagnostics-mismatch", file, mode, 0, None));
    }

    let their_js = their.js.as_deref().unwrap_or_default();
    let our_canonical = canonical_js(our_js);
    let their_canonical = canonical_js(their_js);
    if our_canonical != their_canonical {
        let diff = render_diff(&their_canonical, &our_canonical);
        let size = diff_size(&their_canonical, &our_canonical);
        return Some(finding("js", file, mode, size, Some(diff)));
    }
    None
}

fn compare_css(file: &str, our: Option<&str>, their: Option<&str>) -> Option<Finding> {
    let our_canonical = normalize_css(our.unwrap_or_default());
    let their_canonical = normalize_css(their.unwrap_or_default());
    if our_canonical == their_canonical {
        return None;
    }
    let diff = render_diff(&their_canonical, &our_canonical);
    let size = diff_size(&their_canonical, &our_canonical);
    Some(finding("css", file, "", size, Some(diff)))
}

fn our_error_codes(our: &OurCell) -> BTreeSet<String> {
    match our {
        OurCell::Error { codes } => codes.clone(),
        _ => BTreeSet::new(),
    }
}

fn canonical_js(source: &str) -> String {
    let formatted = format_js(source);
    test_support::strip_js_comments(&test_support::canonicalize_injected_css_in_js(&formatted))
}

fn format_js(source: &str) -> String {
    let allocator = Allocator::default();
    let parsed = Parser::new(&allocator, source, SourceType::default()).parse();
    Codegen::new().build(&parsed.program).code
}

fn normalize_css(css: &str) -> String {
    let stripped = test_support::strip_reference_only_css_markers(css);
    stripped.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn diff_size(before: &str, after: &str) -> usize {
    TextDiff::from_lines(before, after)
        .iter_all_changes()
        .filter(|change| change.tag() != ChangeTag::Equal)
        .count()
}

fn render_diff(before: &str, after: &str) -> String {
    TextDiff::from_lines(before, after)
        .unified_diff()
        .header("reference", "rust")
        .to_string()
}

fn finding(
    reason: &'static str,
    file: &str,
    mode: &'static str,
    size: usize,
    diff: Option<String>,
) -> Finding {
    Finding {
        reason,
        file: file.to_string(),
        mode,
        size,
        diff,
    }
}

const REASON_ORDER: [&str; 9] = [
    "reference-missing",
    "read-error",
    "format-error",
    "rust-error",
    "reference-error",
    "errors-differ",
    "diagnostics-mismatch",
    "js",
    "css",
];

fn report(
    files: &[String],
    findings: &[Finding],
    print_diffs: bool,
    out: Option<&str>,
) -> io::Result<()> {
    let affected: BTreeSet<&str> = findings
        .iter()
        .map(|finding| finding.file.as_str())
        .collect();
    let ok = files.len() - affected.len();

    let mut grouped: HashMap<&str, Vec<&Finding>> = HashMap::new();
    for finding in findings {
        grouped.entry(finding.reason).or_default().push(finding);
    }

    let stdout = io::stdout();
    {
        let mut sink: Box<dyn Write> = match out {
            Some(path) => Box::new(BufWriter::new(File::create(path)?)),
            None => Box::new(stdout.lock()),
        };
        write_detail(&mut sink, findings, &grouped, print_diffs)?;
        sink.flush()?;
    }

    print_summary_table(files.len(), ok, &grouped);
    if let Some(path) = out {
        println!("report: {path}");
    }
    Ok(())
}

fn write_detail(
    sink: &mut dyn Write,
    findings: &[Finding],
    grouped: &HashMap<&str, Vec<&Finding>>,
    print_diffs: bool,
) -> io::Result<()> {
    for reason in REASON_ORDER {
        let Some(items) = grouped.get(reason) else {
            continue;
        };
        writeln!(sink)?;
        if reason == "js" || reason == "css" {
            write_sized_bucket(sink, reason, items)?;
        } else {
            writeln!(sink, "[{reason}] {}", items.len())?;
            for item in items {
                if item.mode.is_empty() {
                    writeln!(sink, "  {}", item.file)?;
                } else {
                    writeln!(sink, "  {} [{}]", item.file, item.mode)?;
                }
            }
        }
    }

    if print_diffs {
        for finding in findings {
            if let Some(diff) = &finding.diff
                && (finding.reason == "js" || finding.reason == "css")
            {
                writeln!(sink)?;
                let tag = if finding.mode.is_empty() {
                    finding.reason.to_string()
                } else {
                    format!("{}/{}", finding.reason, finding.mode)
                };
                writeln!(sink, "--- {} [{tag}] ---", finding.file)?;
                write!(sink, "{diff}")?;
            }
        }
    }
    Ok(())
}

fn print_summary_table(total: usize, ok: usize, grouped: &HashMap<&str, Vec<&Finding>>) {
    let rows: Vec<(&str, usize, usize)> = REASON_ORDER
        .iter()
        .filter_map(|reason| {
            grouped.get(reason).map(|items| {
                let files: BTreeSet<&str> = items.iter().map(|item| item.file.as_str()).collect();
                (*reason, files.len(), items.len())
            })
        })
        .collect();

    if rows.is_empty() {
        println!();
        println!("OK: {ok}/{total} files matched");
        return;
    }

    let reason_width = rows
        .iter()
        .map(|(reason, ..)| reason.len())
        .max()
        .unwrap_or(0)
        .max("reason".len());

    let reason_bar = "═".repeat(reason_width + 2);
    let reason_dash = "─".repeat(reason_width + 2);

    println!();
    println!("{reason_bar}╤═══════╤═══════");
    println!(
        " {:<reason_width$} │ {:>5} │ {:>5} ",
        "reason", "files", "hits"
    );
    println!("{reason_dash}┼───────┼───────");
    for (reason, files, hits) in &rows {
        println!(" {reason:<reason_width$} │ {files:>5} │ {hits:>5} ");
    }
    println!("{reason_bar}╧═══════╧═══════");
    println!();
    println!("OK:       {ok}/{total} files matched");
    println!("MISMATCH: {}/{total} files", total - ok);
}

fn write_sized_bucket(sink: &mut dyn Write, reason: &str, items: &[&Finding]) -> io::Result<()> {
    let mut sorted: Vec<&&Finding> = items.iter().collect();
    sorted.sort_by(|a, b| b.size.cmp(&a.size));
    writeln!(sink, "[{reason}] {}", items.len())?;
    let mut subgroups: [(&str, Vec<&&Finding>); 5] = [
        ("xs", Vec::new()),
        ("s", Vec::new()),
        ("m", Vec::new()),
        ("l", Vec::new()),
        ("xl", Vec::new()),
    ];
    for item in sorted {
        subgroups[size_bucket(item.size)].1.push(item);
    }
    for (tag, entries) in &subgroups {
        if entries.is_empty() {
            continue;
        }
        writeln!(sink, "  [{tag}] {}", entries.len())?;
        for entry in entries {
            if entry.mode.is_empty() {
                writeln!(sink, "    {} ({})", entry.file, entry.size)?;
            } else {
                writeln!(sink, "    {} [{}] ({})", entry.file, entry.mode, entry.size)?;
            }
        }
    }
    Ok(())
}

fn size_bucket(size: usize) -> usize {
    match size {
        0..=2 => 0,
        3..=10 => 1,
        11..=50 => 2,
        51..=200 => 3,
        _ => 4,
    }
}
