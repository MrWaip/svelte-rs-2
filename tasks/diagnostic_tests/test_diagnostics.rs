use std::{fs, fs::read_to_string, path::Path};

use pretty_assertions::assert_eq;
use rstest::rstest;
use serde::Deserialize;
use svelte_compiler::{compile, CompileOptions, Namespace};
use svelte_diagnostics::Severity;

#[derive(Debug, Clone, Deserialize, serde::Serialize, PartialEq, Eq)]
struct ExpectedDiagnostic {
    severity: String,
    code: String,
    start: u32,
    end: u32,
}

fn case_input_and_options(case: &str) -> (String, CompileOptions) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("cases")
        .join(case)
        .join("case.svelte");
    let input = read_to_string(&path).unwrap();

    let dir = path.parent().unwrap();
    let config_path = dir.join("config.json");
    let mut opts = CompileOptions {
        name: Some("App".into()),
        ..Default::default()
    };
    if config_path.exists() {
        let config: serde_json::Value =
            serde_json::from_str(&read_to_string(&config_path).unwrap()).unwrap();
        if let Some(dev) = config.get("dev").and_then(|v| v.as_bool()) {
            opts.dev = dev;
        }
        if let Some(runes) = config.get("runes").and_then(|v| v.as_bool()) {
            opts.runes = Some(runes);
        }
        if let Some(ce) = config.get("customElement").and_then(|v| v.as_bool()) {
            opts.custom_element = ce;
        }
        if let Some(filename) = config.get("filename").and_then(|v| v.as_str()) {
            opts.filename = filename.to_string();
        }
        if let Some(ns) = config.get("namespace").and_then(|v| v.as_str()) {
            opts.namespace = match ns {
                "svg" => Namespace::Svg,
                "mathml" => Namespace::MathMl,
                _ => Namespace::Html,
            };
        }
        if let Some(exp) = config.get("experimental") {
            if let Some(async_val) = exp.get("async").and_then(|v| v.as_bool()) {
                opts.experimental.async_ = async_val;
            }
        }
    }

    (input, opts)
}

fn expected_diagnostics(case: &str) -> Vec<ExpectedDiagnostic> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("cases")
        .join(case)
        .join("case-svelte.json");
    serde_json::from_str(&read_to_string(path).unwrap()).unwrap()
}

fn normalize_actual_diagnostics(case: &str) -> Vec<ExpectedDiagnostic> {
    let (input, opts) = case_input_and_options(case);
    compile(&input, &opts)
        .diagnostics
        .into_iter()
        .map(|diagnostic| ExpectedDiagnostic {
            severity: match diagnostic.severity {
                Severity::Error => "error".into(),
                Severity::Warning => "warning".into(),
            },
            code: diagnostic.kind.code().to_string(),
            start: diagnostic.span.start,
            end: diagnostic.span.end,
        })
        .collect()
}

fn sort_diagnostics(diags: &mut [ExpectedDiagnostic]) {
    diags.sort_by(|left, right| {
        severity_rank(&left.severity)
            .cmp(&severity_rank(&right.severity))
            .then_with(|| left.code.cmp(&right.code))
            .then_with(|| left.start.cmp(&right.start))
            .then_with(|| left.end.cmp(&right.end))
    });
}

fn severity_rank(severity: &str) -> u8 {
    match severity {
        "error" => 0,
        "warning" => 1,
        other => panic!("unexpected severity '{other}'"),
    }
}

fn spans_intersect(left: &ExpectedDiagnostic, right: &ExpectedDiagnostic) -> bool {
    match (
        left.start.cmp(&left.end),
        right.start.cmp(&right.end),
        left.start == left.end,
        right.start == right.end,
    ) {
        (_, _, true, true) => left.start == right.start,
        (_, _, true, false) => point_in_span(left.start, right.start, right.end),
        (_, _, false, true) => point_in_span(right.start, left.start, left.end),
        _ => std::cmp::max(left.start, right.start) < std::cmp::min(left.end, right.end),
    }
}

fn point_in_span(point: u32, start: u32, end: u32) -> bool {
    if start == end {
        point == start
    } else {
        start <= point && point < end
    }
}

fn assert_diagnostics(case: &str) {
    let mut expected = expected_diagnostics(case);
    let mut actual = normalize_actual_diagnostics(case);
    sort_diagnostics(&mut expected);
    sort_diagnostics(&mut actual);
    write_actual_diagnostics(case, &actual);

    assert_eq!(
        actual.len(),
        expected.len(),
        "[{case}] diagnostic count mismatch\nexpected: {expected:#?}\nactual: {actual:#?}"
    );

    for (index, (actual_diag, expected_diag)) in actual.iter().zip(&expected).enumerate() {
        assert_eq!(
            actual_diag.severity, expected_diag.severity,
            "[{case}] severity mismatch at index {index}"
        );
        assert_eq!(
            actual_diag.code, expected_diag.code,
            "[{case}] code mismatch at index {index}"
        );
        assert!(
            spans_intersect(actual_diag, expected_diag),
            "[{case}] span mismatch at index {index}\nexpected: {expected_diag:#?}\nactual: {actual_diag:#?}"
        );
    }
}

fn write_actual_diagnostics(case: &str, actual: &[ExpectedDiagnostic]) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("cases")
        .join(case)
        .join("case-rust.json");
    let json =
        serde_json::to_string_pretty(actual).expect("failed to serialize actual diagnostics");
    fs::write(path, json).expect("failed to write actual diagnostics snapshot");
}

#[rstest]
fn a11y_accesskey() {
    assert_diagnostics("a11y_accesskey");
}

#[rstest]
fn props_identifier_no_store_rune_conflict() {
    assert_diagnostics("props_identifier_no_store_rune_conflict");
}
