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

    // Whether the parser already found errors — captured before the closure so it
    // can be used inside without borrowing `diagnostics` mutably at the same time.
    let has_parse_errors =
        diagnostics.iter().any(|d| d.severity == svelte_diagnostics::Severity::Error);

    let analyze_opts = svelte_analyze::AnalyzeOptions {
        custom_element: options.custom_element,
        runes: options.runes.unwrap_or(true),
        dev: options.dev,
        warning_filter: None,
    };

    // Analysis and codegen share the same catch_unwind so that arena-allocated
    // `parsed` (invariant over its lifetime) stays inside the closure.
    // Analysis always runs; codegen is gated on the absence of error diagnostics.
    let codegen_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let (mut analysis, mut parsed, analyze_diags) =
            svelte_analyze::analyze_with_options(&component, js_result, &analyze_opts);

        if let Some(ss) = css_stylesheet {
            svelte_analyze::analyze_css_pass(&js_alloc, &component, ss, &mut analysis);
        }
        let css = analysis.css.css_output.clone();

        let has_errors = has_parse_errors
            || analyze_diags.iter().any(|d| d.severity == svelte_diagnostics::Severity::Error);

        if has_errors {
            return (None, css, analyze_diags);
        }

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
        (Some(js), css, analyze_diags)
    }));

    match codegen_result {
        Ok((js, css, analyze_diags)) => {
            diagnostics.extend(analyze_diags);
            CompileResult { js, css, diagnostics }
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
            CompileResult { js: None, css: None, diagnostics }
        }
    }
}

/// Compile a standalone `.svelte.js`/`.svelte.ts` module to client-side JavaScript.
/// Applies rune transforms ($state, $derived, $effect, etc.) without component wrapping.
pub fn compile_module(source: &str, options: &ModuleCompileOptions) -> CompileResult {
    let is_ts = options.filename.ends_with(".ts");
    let dev = options.dev;

    let js_alloc = oxc_allocator::Allocator::default();

    // Analysis always runs so all diagnostics are surfaced.
    let (analysis, mut diagnostics) =
        svelte_analyze::analyze_module(&js_alloc, source, is_ts, dev);

    // Codegen is skipped when generate=false or any error diagnostic is present.
    if options.generate == GenerateMode::False
        || diagnostics.iter().any(|d| d.severity == svelte_diagnostics::Severity::Error)
    {
        return CompileResult { js: None, css: None, diagnostics };
    }

    let codegen_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        svelte_codegen_client::generate_module(&js_alloc, source, is_ts, &analysis, dev)
    }));

    match codegen_result {
        Ok(js) => CompileResult { js: Some(js), css: None, diagnostics },
        Err(panic_payload) => {
            let message = if let Some(s) = panic_payload.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = panic_payload.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "unknown internal error".to_string()
            };
            diagnostics.push(Diagnostic::internal_error(message));
            CompileResult { js: None, css: None, diagnostics }
        }
    }
}

#[cfg(test)]
mod tests;
