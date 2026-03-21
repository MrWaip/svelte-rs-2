mod options;

pub use options::{CompileOptions, CssMode, GenerateMode, ModuleCompileOptions, Namespace};
use svelte_diagnostics::Diagnostic;

#[derive(serde::Serialize)]
pub struct CompileResult {
    pub js: Option<String>,
    pub diagnostics: Vec<Diagnostic>,
}

/// Compile a Svelte source file to client-side JavaScript.
/// Always returns a result — never panics. If codegen fails, `js` is `None`.
pub fn compile(source: &str, options: &CompileOptions) -> CompileResult {
    let name = options.component_name();

    let js_alloc = oxc_allocator::Allocator::default();
    let (component, js_result, mut diagnostics) = svelte_parser::parse_with_js(&js_alloc, source);

    let codegen_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut ident_gen = svelte_analyze::IdentGen::new();
        let (analysis, mut parsed, analyze_diags) = svelte_analyze::analyze_with_options(&component, js_result, options.custom_element);
        let transform_data = svelte_transform::transform_component(&js_alloc, &component, &analysis, &mut parsed, &mut ident_gen);
        let js = svelte_codegen_client::generate(&js_alloc, &component, &analysis, &mut parsed, &mut ident_gen, transform_data, &name, options.dev, source, &options.filename);
        (js, analyze_diags)
    }));

    match codegen_result {
        Ok((js, analyze_diags)) => {
            diagnostics.extend(analyze_diags);
            CompileResult {
                js: Some(js),
                diagnostics,
            }
        }
        Err(panic_payload) => {
            let message = if let Some(s) = panic_payload.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = panic_payload.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "unknown internal error".to_string()
            };
            diagnostics.push(Diagnostic::internal_error(message));
            CompileResult {
                js: None,
                diagnostics,
            }
        }
    }
}

/// Compile a standalone `.svelte.js`/`.svelte.ts` module to client-side JavaScript.
/// Applies rune transforms ($state, $derived, $effect, etc.) without component wrapping.
pub fn compile_module(source: &str, options: &ModuleCompileOptions) -> CompileResult {
    let is_ts = options.filename.ends_with(".ts");
    let dev = options.dev;

    // Analysis-only mode: skip codegen entirely
    if options.generate == GenerateMode::False {
        let (_, diagnostics) = svelte_analyze::analyze_module(source, is_ts, dev);
        return CompileResult {
            js: None,
            diagnostics,
        };
    }

    let js_alloc = oxc_allocator::Allocator::default();

    let codegen_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let (analysis, analyze_diags) = svelte_analyze::analyze_module(source, is_ts, dev);
        let js = svelte_codegen_client::generate_module(&js_alloc, source, is_ts, &analysis, dev);
        (js, analyze_diags)
    }));

    match codegen_result {
        Ok((js, diagnostics)) => CompileResult {
            js: Some(js),
            diagnostics,
        },
        Err(panic_payload) => {
            let message = if let Some(s) = panic_payload.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = panic_payload.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "unknown internal error".to_string()
            };
            CompileResult {
                js: None,
                diagnostics: vec![Diagnostic::internal_error(message)],
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn check(source: &str, expected: &str) {
        let opts = CompileOptions { name: Some("App".into()), ..Default::default() };
        let result = compile(source, &opts);
        let js = result.js.unwrap_or_else(|| panic!("compile produced no JS"));
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
        // Unmutated $state — treated as static (no template_effect)
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
        // Even with parse errors, best-effort codegen may produce output
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
        // dev doesn't change output yet, but must not panic
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
        assert!(result.diagnostics.is_empty(), "TS source should parse without errors");
    }

    #[test]
    fn module_default_options_still_work() {
        let result = compile_module("let x = $state(0);", &ModuleCompileOptions::default());
        assert!(result.js.is_some());
    }
}
