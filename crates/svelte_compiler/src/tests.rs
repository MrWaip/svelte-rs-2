use super::*;

fn check(source: &str, expected: &str) {
    let opts = CompileOptions {
        name: Some("App".into()),
        runes: RunesOption::Runes,
        ..Default::default()
    };
    let result = compile(source, &opts);
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"))
        .code;
    assert_eq!(js, expected);
}

#[test]
fn inline_runes_option_overrides_compile_option() {
    let options = CompileOptions {
        name: Some("App".into()),
        runes: RunesOption::Runes,
        ..Default::default()
    };
    let result = compile(
        "<svelte:options runes={false} /><script>let count = 0;</script><p>{count}</p>",
        &options,
    );
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"))
        .code;
    assert!(
        js.contains("svelte/internal/flags/legacy"),
        "expected legacy flag import (inline runes={{false}} must beat compile runes), got:\n{js}"
    );
}

#[test]
fn auto_mode_no_signals_resolves_to_legacy() {
    let options = CompileOptions {
        name: Some("App".into()),
        runes: RunesOption::Auto,
        ..Default::default()
    };
    let result = compile(
        "<script>let count = 0;</script><p>{count}</p>",
        &options,
    );
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"))
        .code;
    assert!(
        js.contains("svelte/internal/flags/legacy"),
        "auto mode without rune signals must resolve to legacy, got:\n{js}"
    );
}

#[test]
fn auto_mode_state_rune_resolves_to_runes() {
    let options = CompileOptions {
        name: Some("App".into()),
        runes: RunesOption::Auto,
        ..Default::default()
    };
    let result = compile(
        "<script>let count = $state(0);</script><p>{count}</p>",
        &options,
    );
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"))
        .code;
    assert!(
        !js.contains("svelte/internal/flags/legacy"),
        "auto mode with $state must resolve to runes, got:\n{js}"
    );
}

#[test]
fn auto_mode_effect_rune_resolves_to_runes() {
    let options = CompileOptions {
        name: Some("App".into()),
        runes: RunesOption::Auto,
        ..Default::default()
    };
    let result = compile(
        "<script>$effect(() => { console.log('tick'); });</script>",
        &options,
    );
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"))
        .code;
    assert!(
        !js.contains("svelte/internal/flags/legacy"),
        "auto mode with $effect must resolve to runes, got:\n{js}"
    );
}

#[test]
fn auto_mode_shadowed_rune_name_stays_legacy() {
    let options = CompileOptions {
        name: Some("App".into()),
        runes: RunesOption::Auto,
        ..Default::default()
    };
    let result = compile(
        "<script>function $state(x) { return x; } let v = $state(0);</script><p>{v}</p>",
        &options,
    );
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"))
        .code;
    assert!(
        js.contains("svelte/internal/flags/legacy"),
        "auto mode must treat shadowed rune name as a regular function, got:\n{js}"
    );
}

#[test]
fn auto_mode_top_level_await_in_module_resolves_to_runes() {
    let options = CompileOptions {
        name: Some("App".into()),
        runes: RunesOption::Auto,
        experimental: ExperimentalOptions { async_: true },
        ..Default::default()
    };
    let result = compile(
        "<script module>const data = await fetch('/api');</script><p>ok</p>",
        &options,
    );
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"))
        .code;
    assert!(
        !js.contains("svelte/internal/flags/legacy"),
        "auto mode with top-level await in module must resolve to runes, got:\n{js}"
    );
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
        runes: RunesOption::Runes,
        ..Default::default()
    };
    let result = compile("", &opts);
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"))
        .code;
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
        runes: RunesOption::Runes,
        ..Default::default()
    };
    let result = compile("", &opts);
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"))
        .code;
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
        runes: RunesOption::Runes,
        ..Default::default()
    };
    let result = compile("<script>let App = 0;</script>", &opts);
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"))
        .code;
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
        runes: RunesOption::Runes,
        ..Default::default()
    };
    let result = compile("<script>let Counter = 0;</script>", &opts);
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"))
        .code;
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
        .unwrap_or_else(|| panic!("compile produced no JS"))
        .code;
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
        .unwrap_or_else(|| panic!("compile produced no JS"))
        .code;
    assert!(
        js.contains("export default function App_1($$anchor)"),
        "expected module-scope binding to rename component export, got: {js}"
    );
}

#[test]
fn analyze_runs_despite_parse_errors() {
    let result = compile(
        r#"<script>
const id = $props.id();
const id2 = $props.id();
</script><div"#,
        &CompileOptions::default(),
    );
    assert!(
        result.js.is_none(),
        "codegen must be skipped when errors present"
    );

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
fn compile_dev_props_member_mutation_uses_ownership_validator() {
    let opts = CompileOptions {
        dev: true,
        ..Default::default()
    };
    let result = compile(
        r#"<script>
let { user } = $props();
function rename() {
    user.name = 'next';
}
</script>"#,
        &opts,
    );
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"))
        .code;
    assert!(
        js.contains("$.create_ownership_validator($$props)"),
        "expected ownership validator setup, got:\n{js}"
    );
    assert!(
        js.contains("$$ownership_validator.mutation(\"user\""),
        "expected ownership mutation wrapper for prop member write, got:\n{js}"
    );
}

#[test]
fn compile_dev_bindable_prop_member_mutation_uses_prop_alias() {
    let opts = CompileOptions {
        dev: true,
        ..Default::default()
    };
    let result = compile(
        r#"<script>
let { value: local = $bindable() } = $props();
function bump() {
    local.count = 1;
}
</script>"#,
        &opts,
    );
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"))
        .code;
    assert!(
        js.contains("$$ownership_validator.mutation(\"value\""),
        "expected ownership mutation wrapper to use prop alias, got:\n{js}"
    );
    assert!(
        js.contains("\"count\""),
        "expected ownership mutation path to include mutated member, got:\n{js}"
    );
}

#[test]
fn compile_dev_bindable_prop_member_update_uses_ownership_validator() {
    let opts = CompileOptions {
        dev: true,
        ..Default::default()
    };
    let result = compile(
        r#"<script>
let { value: local = $bindable() } = $props();
function bump() {
    local.count++;
}
</script>"#,
        &opts,
    );
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"))
        .code;
    assert!(
        js.contains("$.create_ownership_validator($$props)"),
        "expected ownership validator setup, got:\n{js}"
    );
    assert!(
        js.contains("$$ownership_validator.mutation(\"value\""),
        "expected ownership mutation wrapper for update expression, got:\n{js}"
    );
    assert!(
        js.contains("\"count\""),
        "expected ownership mutation path to include updated member, got:\n{js}"
    );
}

#[test]
fn compile_dev_props_member_mutation_in_return_uses_ownership_validator() {
    let opts = CompileOptions {
        dev: true,
        ..Default::default()
    };
    let result = compile(
        r#"<script>
let { user } = $props();
function rename() {
    return user.name = 'next';
}
</script>"#,
        &opts,
    );
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"))
        .code;
    assert!(
        js.contains("$.create_ownership_validator($$props)"),
        "expected ownership validator setup for non-statement mutation, got:\n{js}"
    );
    assert!(
        js.contains("$$ownership_validator.mutation(\"user\""),
        "expected ownership mutation wrapper for non-statement prop member write, got:\n{js}"
    );
}

#[test]
fn compile_dev_shadowed_bindable_member_update_does_not_use_ownership_validator() {
    let opts = CompileOptions {
        dev: true,
        ..Default::default()
    };
    let result = compile(
        r#"<script>
let { value: local = $bindable() } = $props();
function bump(local) {
    local.count++;
}
</script>"#,
        &opts,
    );
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"))
        .code;
    assert!(
        !js.contains("$.create_ownership_validator($$props)"),
        "unexpected ownership validator setup for shadowed local, got:\n{js}"
    );
    assert!(
        !js.contains("$$ownership_validator.mutation(\"value\""),
        "unexpected ownership mutation wrapper for shadowed local, got:\n{js}"
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
        .unwrap_or_else(|| panic!("compile produced no JS"))
        .code;

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
        .unwrap_or_else(|| panic!("compile produced no JS"))
        .code;
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

fn read_js_string_after(js: &str, key: &str) -> Option<String> {
    let mut idx = 0;
    while let Some(found) = js[idx..].find(key) {
        let after = &js[idx + found + key.len()..];
        let after = after.trim_start();
        if let Some(rest) = after.strip_prefix('"') {
            let mut out = String::new();
            let mut chars = rest.chars();
            while let Some(c) = chars.next() {
                if c == '"' {
                    return Some(out);
                }
                if c == '\\' {
                    match chars.next() {
                        Some('n') => out.push('\n'),
                        Some('t') => out.push('\t'),
                        Some('r') => out.push('\r'),
                        Some('"') => out.push('"'),
                        Some('\\') => out.push('\\'),
                        Some(other) => {
                            out.push('\\');
                            out.push(other);
                        }
                        None => return None,
                    }
                } else {
                    out.push(c);
                }
            }
            return None;
        }
        idx += found + key.len();
    }
    None
}

fn normalize_css_for_compare(css: &str) -> String {
    let collapsed: String = css.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut out = String::with_capacity(collapsed.len());
    let bytes = collapsed.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b' ' {
            let prev = out.as_bytes().last().copied();
            let next = bytes.get(i + 1).copied();
            let structural = |b: Option<u8>| matches!(b, Some(b'{' | b'}' | b':' | b';' | b','));
            if structural(prev) || structural(next) {
                i += 1;
                continue;
            }
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

#[test]
fn css_injected_keyframes_preserve_semantics() {
    let opts = CompileOptions {
        name: Some("App".into()),
        css: CssMode::Injected,
        ..Default::default()
    };
    let source = r#"<svelte:options css="injected" />

<style>
    @keyframes pulse {
        0% { opacity: 0.4; }
        100% { opacity: 1; }
    }
    .x { animation: pulse 1s; }
</style>

<div class="x">x</div>"#;

    let result = compile(source, &opts);
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"))
        .code;

    let hash = read_js_string_after(&js, "hash:").expect("expected hash literal in $$css const");
    let actual_code =
        read_js_string_after(&js, "code:").expect("expected code literal in $$css const");

    let expected = format!(
        "@keyframes {hash}-pulse {{ 0% {{ opacity: 0.4; }} 100% {{ opacity: 1; }} }} .x.{hash} {{ animation: {hash}-pulse 1s; }}"
    );

    assert_eq!(
        normalize_css_for_compare(&actual_code),
        normalize_css_for_compare(&expected),
        "injected css `code:` mismatch"
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
        .unwrap_or_else(|| panic!("compile produced no JS"))
        .code;
    let css_output = result
        .css
        .unwrap_or_else(|| panic!("compile produced no CSS"));
    let css = css_output.code.as_str();
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
fn filename_relative_to_root_dir_strips_prefix() {
    assert_eq!(
        filename_relative_to_root_dir("/repo/src/x.svelte", Some("/repo")),
        "src/x.svelte"
    );
}

#[test]
fn filename_relative_to_root_dir_strips_trailing_slash_root() {
    assert_eq!(
        filename_relative_to_root_dir("/repo/src/x.svelte", Some("/repo/")),
        "src/x.svelte"
    );
}

#[test]
fn filename_relative_to_root_dir_keeps_when_no_match() {
    assert_eq!(
        filename_relative_to_root_dir("/elsewhere/x.svelte", Some("/repo")),
        "/elsewhere/x.svelte"
    );
}

#[test]
fn filename_relative_to_root_dir_no_root_dir_returns_normalized() {
    assert_eq!(
        filename_relative_to_root_dir("/abs/x.svelte", None),
        "/abs/x.svelte"
    );
}

#[test]
fn filename_relative_to_root_dir_normalizes_backslashes() {
    assert_eq!(
        filename_relative_to_root_dir("C:\\repo\\src\\x.svelte", Some("C:\\repo")),
        "src/x.svelte"
    );
}

#[test]
fn component_as_named_slot_fill_does_not_consume_root_ident() {
    let options = CompileOptions {
        name: Some("App".into()),
        runes: RunesOption::Legacy,
        ..Default::default()
    };
    let source = "<script>\n    let x = 0;\n</script>\n\n<Wrap>\n    <Inner slot=\"image\" />\n    <span slot=\"action\">{x}</span>\n</Wrap>\n";
    let result = compile(source, &options);
    let js = result
        .js
        .unwrap_or_else(|| panic!("compile produced no JS"))
        .code;
    assert!(
        js.contains("var root_2 = $.from_html(`<span slot=\"action\">"),
        "expected `var root_2` for the span template (reference reserves root_1 for the component-as-slot fill), got:\n{js}"
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

