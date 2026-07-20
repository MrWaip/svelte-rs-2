use std::{
    cmp::Reverse,
    collections::{BTreeSet, HashMap},
    env,
    fs::{self, File},
    io::{self, BufWriter, IsTerminal, Write},
    panic,
    path::{Path, PathBuf},
    process::{self, Command, ExitCode, Stdio},
    sync::{
        Mutex,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    thread,
    time::{Duration, Instant},
};

use rayon::prelude::*;
use serde::Deserialize;
use similar::{ChangeTag, TextDiff};
use svelte_compiler::{
    CompileOptions, GenerateMode, ModuleCompileOptions, RunesOption, compile, compile_module,
};
use svelte_diagnostics::Severity;

const USAGE: &str = "usage: sweep [--mode=auto|runes|legacy] [--chunk=N] [--print-diffs] [--out=<file>] <directory>";

const MIN_CHUNK: usize = 300;
const MAX_CHUNK: usize = 4000;

fn auto_chunk(files: usize) -> usize {
    let cores = thread::available_parallelism()
        .map(|value| value.get())
        .unwrap_or(8);
    (files / (cores * 8)).clamp(MIN_CHUNK, MAX_CHUNK)
}

const PROGRESS_TICK: Duration = Duration::from_millis(120);

fn with_progress<T>(enabled: bool, tick: impl Fn() + Sync, work: impl FnOnce() -> T) -> T {
    if !enabled {
        return work();
    }
    let stop = AtomicBool::new(false);
    thread::scope(|scope| {
        scope.spawn(|| {
            while !stop.load(Ordering::Relaxed) {
                tick();
                thread::sleep(PROGRESS_TICK);
            }
        });
        let result = work();
        stop.store(true, Ordering::Relaxed);
        result
    })
}

fn format_secs(elapsed: Duration) -> String {
    format!("{:.2}s", elapsed.as_secs_f64())
}

fn draw_progress(text: &str) {
    let width = terminal_size::terminal_size()
        .map(|(terminal_size::Width(w), _)| w as usize)
        .unwrap_or(120)
        .saturating_sub(1)
        .max(1);
    let line: String = text.chars().take(width).collect();
    let mut err = io::stderr().lock();
    let _ = write!(err, "\r\x1b[2K{line}");
    let _ = err.flush();
}

fn clear_progress() {
    let mut err = io::stderr().lock();
    let _ = write!(err, "\r\x1b[2K");
    let _ = err.flush();
}

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
    chunk: Option<usize>,
    print_diffs: bool,
    out: Option<String>,
}

fn parse_cli(args: &[String]) -> Result<Cli, String> {
    let mut directory: Option<String> = None;
    let mut mode = Mode::Auto;
    let mut chunk: Option<usize> = None;
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
            let parsed: usize = value
                .parse()
                .map_err(|_| format!("invalid --chunk value: {value}"))?;
            if parsed == 0 {
                return Err("--chunk must be greater than zero".to_string());
            }
            chunk = Some(parsed);
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

    let mode_label = match cli.mode {
        Mode::Auto => "auto",
        Mode::Runes => "runes",
        Mode::Legacy => "legacy",
    };
    println!("scan: {}", directory.display());
    println!("mode: {mode_label}");

    panic::set_hook(Box::new(|_| {}));
    let progress = io::stderr().is_terminal();

    let enumerate_started = Instant::now();
    let files = enumerate_files(&directory, progress);
    if progress {
        clear_progress();
    }
    if files.is_empty() {
        eprintln!("sweep: no svelte files found in {}", directory.display());
        return ExitCode::from(2);
    }
    let enumerate_elapsed = enumerate_started.elapsed();
    println!(
        "files: {} ({})",
        files.len(),
        format_secs(enumerate_elapsed)
    );

    let chunk = cli.chunk.unwrap_or_else(|| auto_chunk(files.len()));

    let reference_started = Instant::now();
    let reference = match fetch_reference(&root, &files, cli.mode, chunk, progress) {
        Ok(reference) => reference,
        Err(err) => {
            eprintln!("sweep: reference compiler failed: {err}");
            return ExitCode::from(4);
        }
    };
    let reference_elapsed = reference_started.elapsed();

    let root_dir = root.display().to_string();
    let total = files.len();
    let processed = AtomicUsize::new(0);
    let mismatched = AtomicUsize::new(0);
    let current = Mutex::new(String::new());

    let compare_started = Instant::now();
    let findings: Vec<Finding> = with_progress(
        progress,
        || {
            let done = processed.load(Ordering::Relaxed);
            let bad = mismatched.load(Ordering::Relaxed);
            let latest = current.lock().map(|file| file.clone()).unwrap_or_default();
            draw_progress(&format!(
                "[{done}/{total}] ok={} bad={bad} {latest}",
                done - bad
            ));
        },
        || {
            files
                .par_iter()
                .flat_map(|file| {
                    let file_findings = sweep_file(file, &root_dir, cli.mode, reference.get(file));
                    if let Ok(mut latest) = current.lock() {
                        file.clone_into(&mut latest);
                    }
                    processed.fetch_add(1, Ordering::Relaxed);
                    if !file_findings.is_empty() {
                        mismatched.fetch_add(1, Ordering::Relaxed);
                    }
                    file_findings
                })
                .collect()
        },
    );
    if progress {
        clear_progress();
    }
    let compare_elapsed = compare_started.elapsed();

    if let Err(err) = report(&files, &findings, cli.print_diffs, cli.out.as_deref()) {
        eprintln!("sweep: write report: {err}");
    }

    println!(
        "time: enumerate {}  reference {}  compare {}",
        format_secs(enumerate_elapsed),
        format_secs(reference_elapsed),
        format_secs(compare_elapsed)
    );

    if findings.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn enumerate_files(directory: &Path, progress: bool) -> Vec<String> {
    let counter = AtomicUsize::new(0);
    let mut files = walk_dir(directory, progress, &counter);
    files.sort();
    files.dedup();
    files
}

fn is_target(name: &str) -> bool {
    name.ends_with(".svelte") || name.ends_with(".svelte.js") || name.ends_with(".svelte.ts")
}

fn walk_dir(dir: &Path, progress: bool, counter: &AtomicUsize) -> Vec<String> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };

    let mut subdirs: Vec<PathBuf> = Vec::new();
    let mut files: Vec<String> = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        let is_dir = match entry.file_type() {
            Ok(file_type) if file_type.is_symlink() => path.is_dir(),
            Ok(file_type) => file_type.is_dir(),
            Err(_) => false,
        };
        if is_dir {
            subdirs.push(path);
        } else if entry.file_name().to_str().is_some_and(is_target) {
            files.push(path.display().to_string());
            if progress {
                let seen = counter.fetch_add(1, Ordering::Relaxed) + 1;
                if seen.is_multiple_of(256) {
                    draw_progress(&format!("finding: {seen} files"));
                }
            }
        }
    }

    let nested: Vec<Vec<String>> = subdirs
        .par_iter()
        .map(|sub| walk_dir(sub, progress, counter))
        .collect();
    for chunk in nested {
        files.extend(chunk);
    }
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
    progress: bool,
) -> Result<HashMap<String, ReferenceFile>, String> {
    let script = root.join("tasks/sweep/reference.mjs");
    let config = mode_config_json(mode);
    let chunks: Vec<(usize, &[String])> = files.chunks(chunk).enumerate().collect();
    let total_chunks = chunks.len();
    let total_files = files.len();
    let done = AtomicUsize::new(0);

    let partials: Vec<Result<HashMap<String, ReferenceFile>, String>> = with_progress(
        progress,
        || {
            draw_progress(&format!(
                "reference: [{}/{total_chunks} chunks] {total_files} files",
                done.load(Ordering::Relaxed)
            ));
        },
        || {
            chunks
                .par_iter()
                .map(|(index, batch)| {
                    let result = run_reference_chunk(root, &script, &config, *index, batch);
                    done.fetch_add(1, Ordering::Relaxed);
                    result
                })
                .collect()
        },
    );
    if progress {
        clear_progress();
    }

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

    let source = match fs::read_to_string(file) {
        Ok(source) => source,
        Err(err) => return vec![finding("read-error", file, "", 0, Some(err.to_string()))],
    };
    let is_module = file.ends_with(".svelte.js") || file.ends_with(".svelte.ts");

    if let Some(message) = &reference.format_error {
        let ours = run_our_cell(&source, file, root_dir, is_module, mode, &CELLS[0]);
        if matches!(ours, OurCell::Error { .. }) {
            return vec![];
        }
        return vec![finding("format-error", file, "", 0, Some(message.clone()))];
    }

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
    test_support::canonicalize_injected_css_in_js(&test_support::canonicalize_js(source))
}

fn normalize_css(css: &str) -> String {
    test_support::canonicalize_injected_css(css)
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

fn colorize_diff(diff: &str) -> String {
    const RED: &str = "\x1b[31m";
    const GREEN: &str = "\x1b[32m";
    const CYAN: &str = "\x1b[36m";
    const RESET: &str = "\x1b[0m";
    diff.split_inclusive('\n')
        .map(|raw| {
            let line = raw.strip_suffix('\n').unwrap_or(raw);
            let newline = if raw.ends_with('\n') { "\n" } else { "" };
            if line.starts_with("+++") || line.starts_with("---") || line.starts_with("@@") {
                format!("{CYAN}{line}{RESET}{newline}")
            } else if line.starts_with('+') {
                format!("{GREEN}{line}{RESET}{newline}")
            } else if line.starts_with('-') {
                format!("{RED}{line}{RESET}{newline}")
            } else {
                raw.to_string()
            }
        })
        .collect()
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

    let color = out.is_none() && io::stdout().is_terminal();
    let stdout = io::stdout();
    {
        let mut sink: Box<dyn Write> = match out {
            Some(path) => Box::new(BufWriter::new(File::create(path)?)),
            None => Box::new(stdout.lock()),
        };
        write_detail(&mut sink, findings, &grouped, print_diffs, color)?;
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
    color: bool,
) -> io::Result<()> {
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
                if color {
                    write!(sink, "{}", colorize_diff(diff))?;
                } else {
                    write!(sink, "{diff}")?;
                }
            }
        }
    }

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
    sorted.sort_by_key(|b| Reverse(b.size));
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
