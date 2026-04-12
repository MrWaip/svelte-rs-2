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
fn inline_runes_option_overrides_compile_option() {
    let js_alloc = oxc_allocator::Allocator::default();
    let (mut component, _, diags) =
        svelte_parser::parse_with_js(&js_alloc, "<svelte:options runes={false} /><p />");
    assert!(
        diags.is_empty(),
        "unexpected parser diagnostics for inline runes test: {diags:?}"
    );

    let options = CompileOptions {
        runes: Some(true),
        ..Default::default()
    };
    apply_compile_options_to_component(&mut component, &options);

    assert!(!resolved_runes_option(&component, &options));
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
fn compile_filename_derived_name_is_sanitized() {
    let opts = CompileOptions {
        filename: "src/routes/+page.svelte".into(),
        ..Default::default()
    };
    let result = compile("", &opts);
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"));
    assert_eq!(
        js,
        r#"import * as $ from "svelte/internal/client";
export default function _page($$anchor) {}
"#
    );
}

#[test]
fn compile_explicit_name_reserved_word_is_deconflicted() {
    let opts = CompileOptions {
        name: Some("class".into()),
        ..Default::default()
    };
    let result = compile("", &opts);
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"));
    assert_eq!(
        js,
        r#"import * as $ from "svelte/internal/client";
export default function class_1($$anchor) {}
"#
    );
}

#[test]
fn compile_explicit_name_conflict_is_deconflicted() {
    let opts = CompileOptions {
        name: Some("App".into()),
        ..Default::default()
    };
    let result = compile("<script>let App = 0;</script>", &opts);
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"));
    assert_eq!(
        js,
        r#"import * as $ from "svelte/internal/client";
export default function App_1($$anchor) {
	let App = 0;
}
"#
    );
}

#[test]
fn compile_filename_derived_name_conflict_is_deconflicted() {
    let opts = CompileOptions {
        filename: "src/routes/counter.svelte".into(),
        ..Default::default()
    };
    let result = compile("<script>let Counter = 0;</script>", &opts);
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"));
    assert_eq!(
        js,
        r#"import * as $ from "svelte/internal/client";
export default function Counter_1($$anchor) {
	let Counter = 0;
}
"#
    );
}

#[test]
fn compile_component_name_ignores_nested_scope_bindings() {
    let opts = CompileOptions {
        name: Some("App".into()),
        ..Default::default()
    };
    let result = compile("<script>function demo() { let App = 0; }</script>", &opts);
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"));
    assert!(
        js.contains("export default function App($$anchor)"),
        "expected nested local binding to not rename component export, got: {js}"
    );
}

#[test]
fn compile_component_name_conflicts_with_module_scope_bindings() {
    let opts = CompileOptions {
        name: Some("App".into()),
        ..Default::default()
    };
    let result = compile("<script module>let App = 0;</script>", &opts);
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"));
    assert!(
        js.contains("export default function App_1($$anchor)"),
        "expected module-scope binding to rename component export, got: {js}"
    );
}

#[test]
fn analyze_runs_despite_parse_errors() {
    // Analyze always runs even when the parser reports errors, so that both
    // parse-layer and analyze-layer diagnostics surface in one pass.
    // Codegen is skipped whenever any error diagnostic is present.
    let result = compile(
        r#"<script>
const id = $props.id();
const id2 = $props.id();
</script><div"#, // unclosed tag → parse error; duplicate $props.id() → analyze error
        &CompileOptions::default(),
    );
    assert!(
        result.js.is_none(),
        "codegen must be skipped when errors present"
    );
    // Both layers contribute diagnostics.
    assert!(
        result
            .diagnostics
            .iter()
            .any(|d| d.kind.code() == "props_id_invalid_placement"
                || d.kind.code() == "props_duplicate"),
        "analyze diagnostics must surface alongside parse errors: {:?}",
        result.diagnostics
    );
}

#[test]
fn compile_const_tag_invalid_reference_experimental_async() {
    let mut opts = CompileOptions::default();
    opts.experimental.async_ = true;
    let result = compile(
        r#"<script>
    import Widget from './Widget.svelte';
</script>

<Widget>
    {@const foo = 1}
    {#snippet children()}
        <p>{foo}</p>
    {/snippet}
</Widget>"#,
        &opts,
    );
    assert!(
        result
            .diagnostics
            .iter()
            .any(|d| d.kind.code() == "const_tag_invalid_reference"),
        "expected const_tag_invalid_reference, got: {:?}",
        result.diagnostics
    );
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
fn module_store_subscription_reports_module_diagnostic_for_js() {
    let opts = ModuleCompileOptions {
        filename: "lib.svelte.js".to_string(),
        ..Default::default()
    };
    let result = compile_module(
        "import { writable } from 'svelte/store'; const count = writable(0); console.log($count);",
        &opts,
    );

    assert!(result.js.is_none(), "unexpected JS output: {:?}", result.js);
    let store_diags = result
        .diagnostics
        .iter()
        .filter(|diag| diag.kind.code() == "store_invalid_subscription_module")
        .count();
    assert_eq!(
        store_diags, 1,
        "unexpected diagnostics: {:?}",
        result.diagnostics
    );
}

#[test]
fn module_store_subscription_reports_module_diagnostic_for_ts() {
    let opts = ModuleCompileOptions {
        filename: "lib.svelte.ts".to_string(),
        ..Default::default()
    };
    let result = compile_module(
        "import { writable } from 'svelte/store'; const count = writable<number>(0); console.log($count);",
        &opts,
    );

    assert!(result.js.is_none(), "unexpected JS output: {:?}", result.js);
    let store_diags = result
        .diagnostics
        .iter()
        .filter(|diag| diag.kind.code() == "store_invalid_subscription_module")
        .count();
    assert_eq!(
        store_diags, 1,
        "unexpected diagnostics: {:?}",
        result.diagnostics
    );
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
fn compile_props_and_props_id_coexist() {
    // $props() and $props.id() are allowed together — reference compiler permits this.
    let result = compile(
        r#"<script>
let { a } = $props();
const id = $props.id();
</script>"#,
        &CompileOptions::default(),
    );
    assert!(
        !result
            .diagnostics
            .iter()
            .any(|d| d.kind.code() == "props_duplicate"),
        "unexpected props_duplicate, got: {:?}",
        result.diagnostics
    );
}

#[test]
fn attribute_invalid_name_digit_start() {
    let result = compile(r#"<div 1foo="x"></div>"#, &CompileOptions::default());
    assert!(
        result
            .diagnostics
            .iter()
            .any(|d| d.kind.code() == "attribute_invalid_name"),
        "expected attribute_invalid_name, got: {:?}",
        result.diagnostics
    );
}

#[test]
fn attribute_invalid_name_dash_start() {
    // Parser allows '-' in attr names including at start; analyze rejects via the illegal-char regex.
    let result = compile(r#"<div -foo="x"></div>"#, &CompileOptions::default());
    assert!(
        result
            .diagnostics
            .iter()
            .any(|d| d.kind.code() == "attribute_invalid_name"),
        "expected attribute_invalid_name, got: {:?}",
        result.diagnostics
    );
}

#[test]
fn css_injected_via_compile_options() {
    let opts = CompileOptions {
        name: Some("App".into()),
        css: CssMode::Injected,
        ..Default::default()
    };
    let result = compile("<style>p { color: red; }</style><p>hello</p>", &opts);
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"));
    // inject path: CSS must be absent from CompileResult.css and present in JS output
    assert!(result.css.is_none(), "css should be None for injected mode");
    assert!(js.contains("$$css"), "expected $$css const in JS output");
    assert!(
        js.contains("$.append_styles"),
        "expected $.append_styles call in JS output"
    );
}

#[test]
fn inline_css_injected_overrides_external_compile_option() {
    let opts = CompileOptions {
        name: Some("App".into()),
        css: CssMode::External,
        ..Default::default()
    };
    let result = compile(
        r#"<svelte:options css="injected" />
<style>p { color: red; }</style>
<p>hello</p>"#,
        &opts,
    );
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"));
    assert!(
        result.css.is_none(),
        "inline injected mode should suppress CompileResult.css"
    );
    assert!(js.contains("$$css"), "expected $$css const in JS output");
    assert!(
        js.contains("$.append_styles"),
        "expected $.append_styles call in JS output"
    );
}

#[test]
fn explicit_external_css_mode_returns_compile_result_css() {
    let opts = CompileOptions {
        name: Some("App".into()),
        css: CssMode::External,
        ..Default::default()
    };
    let result = compile("<style>p { color: red; }</style><p>hello</p>", &opts);
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"));
    let css = result
        .css
        .as_deref()
        .unwrap_or_else(|| panic!("compile produced no CSS"));
    assert!(
        !js.contains("$.append_styles"),
        "external mode must not inject styles into JS"
    );
    assert!(
        !css.is_empty(),
        "external mode must return scoped CSS in CompileResult.css"
    );
}

#[test]
fn attribute_invalid_event_handler_string_value() {
    let result = compile(
        r#"<button onclick="doSomething()"></button>"#,
        &CompileOptions::default(),
    );
    assert!(
        result
            .diagnostics
            .iter()
            .any(|d| d.kind.code() == "attribute_invalid_event_handler"),
        "expected attribute_invalid_event_handler, got: {:?}",
        result.diagnostics
    );
}
