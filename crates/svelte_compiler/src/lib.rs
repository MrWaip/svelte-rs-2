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
    let css_parsed = svelte_parser::parse_css_block(&component);

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
        let (mut analysis, mut parsed, mut analyze_diags) =
            svelte_analyze::analyze_with_options(&component, js_result, &analyze_opts);

        let mut css_text: Option<String> = None;
        if let Some((ss, css_diags)) = css_parsed {
            analyze_diags.extend(css_diags);
            // css:"injected" can come from compile options OR from <svelte:options css="injected">
            let inject_styles = options.css == CssMode::Injected
                || component.options.as_ref().and_then(|o| o.css) == Some(svelte_ast::CssMode::Injected);
            svelte_analyze::analyze_css_pass(&component, &ss, inject_styles, &mut analysis);
            let css_block = component.css.as_ref()
                .unwrap_or_else(|| panic!("css block must exist when css_parsed is Some"));
            let css_source = component.source_text(css_block.content_span);
            let raw_css = svelte_transform_css::transform_css(&analysis.css.hash, &analysis.css.keyframes, ss, css_source);
            css_text = if inject_styles {
                Some(svelte_transform_css::compact_css_for_injection(&raw_css))
            } else {
                Some(raw_css)
            };
        }
        // External CSS is returned in CompileResult.css; injected CSS goes into the JS output.
        // css_text is passed to codegen only for the injected path — external mode doesn't need it there.
        let (css, injected_css_text) = if analysis.css.inject_styles {
            (None, css_text)
        } else {
            (css_text, None)
        };

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
            injected_css_text.as_deref(),
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
