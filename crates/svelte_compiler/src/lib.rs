mod options;

pub use options::{
    CompileOptions, CssMode, ExperimentalOptions, GenerateMode, ModuleCompileOptions, Namespace,
};
use svelte_diagnostics::Diagnostic;

#[derive(serde::Serialize)]
pub struct CompileResult {
    pub js: Option<String>,
    /// Transformed (scoped) CSS text, or `None` when the component has no `<style>` block.
    pub css: Option<String>,
    pub diagnostics: Vec<Diagnostic>,
}

/// Compile a Svelte source file to client-side JavaScript.
/// Always returns a result — never panics. If codegen fails, `js` is `None`.
pub fn compile(source: &str, options: &CompileOptions) -> CompileResult {
    let name = options.component_name();

    let js_alloc = oxc_allocator::Allocator::default();
    let (component, js_result, mut diagnostics) = svelte_parser::parse_with_js(&js_alloc, source);
    let css_stylesheet = svelte_parser::parse_css_block(&js_alloc, &component);

    // Skip analysis and codegen if the parser produced any errors — the AST
    // may be incomplete and downstream passes would produce misleading output.
    if diagnostics.iter().any(|d| d.severity == svelte_diagnostics::Severity::Error) {
        return CompileResult {
            js: None,
            css: None,
            diagnostics,
        };
    }

    let codegen_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let analyze_opts = svelte_analyze::AnalyzeOptions {
            custom_element: options.custom_element,
            runes: options.runes.unwrap_or(true),
            dev: options.dev,
            warning_filter: None,
        };
        let (mut analysis, mut parsed, analyze_diags) =
            svelte_analyze::analyze_with_options(&component, js_result, &analyze_opts);
        if let Some(ss) = css_stylesheet {
            svelte_analyze::analyze_css_pass(&component, ss, &mut analysis);
        }
        let css = analysis.css.css_output.clone();
        let mut ident_gen =
            svelte_analyze::IdentGen::with_conflicts(analysis.scoping.collect_all_symbol_names());
        let transform_data = svelte_transform::transform_component(
            &js_alloc,
            &component,
            &analysis,
            &mut parsed,
            &mut ident_gen,
        );
        let js = svelte_codegen_client::generate(
            &js_alloc,
            &component,
            &analysis,
            &mut parsed,
            &mut ident_gen,
            transform_data,
            &name,
            options.dev,
            source,
            &options.filename,
            options.experimental.async_,
        );
        (js, css, analyze_diags)
    }));

    match codegen_result {
        Ok((js, css, analyze_diags)) => {
            diagnostics.extend(analyze_diags);
            CompileResult {
                js: Some(js),
                css,
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
                css: None,
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

    let js_alloc = oxc_allocator::Allocator::default();

    // Analysis-only mode: skip codegen entirely
    if options.generate == GenerateMode::False {
        let (_, diagnostics) = svelte_analyze::analyze_module(&js_alloc, source, is_ts, dev);
        return CompileResult {
            js: None,
            css: None,
            diagnostics,
        };
    }

    let codegen_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let (analysis, analyze_diags) =
            svelte_analyze::analyze_module(&js_alloc, source, is_ts, dev);
        let js = svelte_codegen_client::generate_module(&js_alloc, source, is_ts, &analysis, dev);
        (js, analyze_diags)
    }));

    match codegen_result {
        Ok((js, diagnostics)) => CompileResult {
            js: Some(js),
            css: None,
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
                css: None,
                diagnostics: vec![Diagnostic::internal_error(message)],
            }
        }
    }
}

#[cfg(test)]
mod tests;
