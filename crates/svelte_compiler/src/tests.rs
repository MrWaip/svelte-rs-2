use super::*;

fn check(source: &str, expected: &str) {
    let opts = CompileOptions {
        name: Some("App".into()),
        ..Default::default()
    };
    let result = compile(source, &opts);
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"));
    assert_eq!(js, expected);
}

#[test]
fn empty_component() {
    check(
        "",
        r#"import * as $ from "svelte/internal/client";
export default function App($$anchor) {}
"#,
    );
}

#[test]
fn only_script() {
    check(
        r#"<script>
    let i = 10;
    i++;
</script>"#,
        r#"import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let i = 10;
	i++;
}
"#,
    );
}

#[test]
fn single_interpolation_rune() {
    check(
        r#"<script>
    let name = $state();
</script>{name}"#,
        r#"import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let name = void 0;
	$.next();
	var text = $.text();
	text.nodeValue = name;
	$.append($$anchor, text);
}
"#,
    );
}

#[test]
fn error_recovery_returns_diagnostics() {
    let result = compile("<div>", &CompileOptions::default());
    assert!(!result.diagnostics.is_empty());
}

#[test]
fn module_generate_false_returns_no_js() {
    let opts = ModuleCompileOptions {
        generate: GenerateMode::False,
        ..Default::default()
    };
    let result = compile_module("let x = $state(0);", &opts);
    assert!(result.js.is_none());
}

#[test]
fn module_dev_flag_passed_through() {
    let opts = ModuleCompileOptions {
        dev: true,
        ..Default::default()
    };
    let result = compile_module("let x = $state(0);", &opts);
    assert!(result.js.is_some());
}

#[test]
fn module_typescript_from_filename() {
    let opts = ModuleCompileOptions {
        filename: "lib.svelte.ts".to_string(),
        ..Default::default()
    };
    let source = "let x: number = $state(0);";
    let result = compile_module(source, &opts);
    assert!(result.js.is_some());
    assert!(
        result.diagnostics.is_empty(),
        "TS source should parse without errors"
    );
}

#[test]
fn module_default_options_still_work() {
    let result = compile_module("let x = $state(0);", &ModuleCompileOptions::default());
    assert!(result.js.is_some());
}

#[test]
#[ignore = "missing: const_tag_invalid_expression validation"]
fn compile_const_tag_invalid_expression() {
    let result = compile(
        "{#if visible}{@const a = 1, b = 2}<p>{a}</p>{/if}",
        &CompileOptions::default(),
    );
    assert!(
        result
            .diagnostics
            .iter()
            .any(|d| d.kind.code() == "const_tag_invalid_expression"),
        "expected const_tag_invalid_expression, got: {:?}",
        result.diagnostics
    );
}

#[test]
fn compile_props_id_invalid_placement() {
    let result = compile(
        r#"<script>
function setup() {
    const id = $props.id();
}
</script>"#,
        &CompileOptions::default(),
    );
    assert!(
        result
            .diagnostics
            .iter()
            .any(|d| d.kind.code() == "props_id_invalid_placement"),
        "expected props_id_invalid_placement, got: {:?}",
        result.diagnostics
    );
}

#[test]
fn compile_props_id_duplicate_with_props() {
    let result = compile(
        r#"<script>
let { a } = $props();
const id = $props.id();
</script>"#,
        &CompileOptions::default(),
    );
    assert!(
        result
            .diagnostics
            .iter()
            .any(|d| d.kind.code() == "props_duplicate"),
        "expected props_duplicate, got: {:?}",
        result.diagnostics
    );
}

#[test]
fn attribute_invalid_name_digit_start() {
    let result = compile(r#"<div 1foo="x"></div>"#, &CompileOptions::default());
    assert!(
        result.diagnostics.iter().any(|d| d.kind.code() == "attribute_invalid_name"),
        "expected attribute_invalid_name, got: {:?}",
        result.diagnostics
    );
}

#[test]
fn attribute_invalid_name_dash_start() {
    // Parser allows '-' in attr names including at start; analyze rejects via the illegal-char regex.
    let result = compile(r#"<div -foo="x"></div>"#, &CompileOptions::default());
    assert!(
        result.diagnostics.iter().any(|d| d.kind.code() == "attribute_invalid_name"),
        "expected attribute_invalid_name, got: {:?}",
        result.diagnostics
    );
}

#[test]
fn attribute_invalid_event_handler_string_value() {
    let result = compile(r#"<button onclick="doSomething()"></button>"#, &CompileOptions::default());
    assert!(
        result.diagnostics.iter().any(|d| d.kind.code() == "attribute_invalid_event_handler"),
        "expected attribute_invalid_event_handler, got: {:?}",
        result.diagnostics
    );
}
