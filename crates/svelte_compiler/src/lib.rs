mod options;

pub use options::{
    CompileOptions, CssMode, ExperimentalOptions, GenerateMode, ModuleCompileOptions, Namespace,
};
use svelte_diagnostics::Diagnostic;

#[derive(serde::Serialize)]
pub struct CompileResult {
    pub js: Option<String>,

    pub css: Option<String>,
    pub diagnostics: Vec<Diagnostic>,
}

fn apply_compile_options_to_component(
    component: &mut svelte_ast::Component,
    options: &CompileOptions,
) {
    if options.namespace == Namespace::Html {
        return;
    }
    let ast_namespace = match options.namespace {
        Namespace::Html => svelte_ast::Namespace::Html,
        Namespace::Svg => svelte_ast::Namespace::Svg,
        Namespace::MathMl => svelte_ast::Namespace::Mathml,
    };
    let opts = component
        .options
        .get_or_insert_with(|| svelte_ast::SvelteOptions {
            span: svelte_ast::Span::default(),
            runes: None,
            namespace: None,
            css: None,
            custom_element: None,
            immutable: None,
            accessors: None,
            preserve_whitespace: None,
            attributes: Vec::new(),
        });
    if opts.namespace.is_none() {
        opts.namespace = Some(ast_namespace);
    }
}

fn resolved_runes_option(component: &svelte_ast::Component, options: &CompileOptions) -> bool {
    component
        .options
        .as_ref()
        .and_then(|opts| opts.runes)
        .or(options.runes)
        .unwrap_or(true)
}

fn resolved_accessors_option(component: &svelte_ast::Component, options: &CompileOptions) -> bool {
    component
        .options
        .as_ref()
        .and_then(|opts| opts.accessors)
        .unwrap_or(options.accessors)
}

fn resolved_immutable_option(component: &svelte_ast::Component, options: &CompileOptions) -> bool {
    component
        .options
        .as_ref()
        .and_then(|opts| opts.immutable)
        .unwrap_or(options.immutable)
}

fn resolved_preserve_whitespace_option(
    component: &svelte_ast::Component,
    options: &CompileOptions,
) -> bool {
    component
        .options
        .as_ref()
        .and_then(|opts| opts.preserve_whitespace)
        .unwrap_or(options.preserve_whitespace)
}

fn resolved_css_mode(component: &svelte_ast::Component, options: &CompileOptions) -> CssMode {
    if component.options.as_ref().and_then(|opts| opts.css) == Some(svelte_ast::CssMode::Injected) {
        CssMode::Injected
    } else {
        options.css
    }
}

pub fn compile(source: &str, options: &CompileOptions) -> CompileResult {
    let candidate_name = options.component_name();

    let js_alloc = oxc_allocator::Allocator::default();
    let (mut component, js_result, mut diagnostics) =
        svelte_parser::parse_with_js(&js_alloc, source);
    apply_compile_options_to_component(&mut component, options);
    let css_parsed = svelte_parser::parse_css_block(&component);

    let has_parse_errors = diagnostics
        .iter()
        .any(|d| d.severity == svelte_diagnostics::Severity::Error);

    let analyze_opts = svelte_analyze::AnalyzeOptions {
        custom_element: options.custom_element,
        experimental_async: options.experimental.async_,
        runes: resolved_runes_option(&component, options),
        accessors: resolved_accessors_option(&component, options),
        immutable: resolved_immutable_option(&component, options),
        preserve_whitespace: resolved_preserve_whitespace_option(&component, options),
        dev: options.dev,
        component_name: candidate_name,
        filename_basename: options
            .filename
            .rsplit_once('/')
            .or_else(|| options.filename.rsplit_once('\\'))
            .map_or(options.filename.as_str(), |(_, basename)| basename)
            .to_string(),
        warning_filter: None,
    };

    let codegen_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let (mut analysis, mut parsed, mut analyze_diags) =
            svelte_analyze::analyze_with_options(&component, js_result, &analyze_opts);

        let mut css_text: Option<String> = None;
        if let Some((ss, css_diags)) = css_parsed {
            analyze_diags.extend(css_diags);
            let inject_styles = resolved_css_mode(&component, options) == CssMode::Injected
                || analysis.output.is_custom_element_target;
            svelte_analyze::analyze_css_pass(
                &component,
                &ss,
                &parsed,
                inject_styles,
                &mut analysis,
                &mut analyze_diags,
            );
            let css_block = component
                .css
                .as_ref()
                .unwrap_or_else(|| panic!("css block must exist when css_parsed is Some"));
            let css_source = component.source_text(css_block.content_span);
            let raw_css = svelte_transform_css::transform_css_with_usage(
                &analysis.output.css.hash,
                &analysis.output.css.keyframes,
                Some(&analysis.output.css.used_selectors),
                true,
                ss,
                css_source,
            );
            css_text = if inject_styles {
                Some(svelte_transform_css::compact_css_for_injection(&raw_css))
            } else {
                Some(raw_css)
            };
        }

        let (css, injected_css_text) = if analysis.output.css.inject_styles {
            (None, css_text)
        } else {
            (css_text, None)
        };

        let has_errors = has_parse_errors
            || analyze_diags
                .iter()
                .any(|d| d.severity == svelte_diagnostics::Severity::Error);

        if has_errors {
            return (None, css, analyze_diags);
        }

        let mut ident_gen =
            svelte_analyze::IdentGen::with_conflicts(analysis.scoping.collect_all_symbol_names());
        let name = analysis.component_name().to_string();
        let _ = ident_gen.generate(&name);
        let line_index = svelte_span::LineIndex::new(component.source.as_str());
        let transform_data = {
            let mut compile_ctx = svelte_types::CompileContext {
                alloc: &js_alloc,
                component: &component,
                analysis: &analysis,
                js_arena: &mut parsed,
                ident_gen: &mut ident_gen,
                line_index: &line_index,
            };
            svelte_transform::transform_component(
                &mut compile_ctx,
                &svelte_types::TransformOptions { dev: options.dev },
            )
        };
        let codegen_options = svelte_types::CodegenOptions {
            dev: options.dev,
            experimental_async: options.experimental.async_,
            filename: options.filename.clone(),
        };
        let compile_ctx = svelte_types::CompileContext {
            alloc: &js_alloc,
            component: &component,
            analysis: &analysis,
            js_arena: &mut parsed,
            ident_gen: &mut ident_gen,
            line_index: &line_index,
        };
        let js = svelte_codegen_client::generate(
            compile_ctx,
            &codegen_options,
            transform_data,
            injected_css_text.as_deref(),
        );
        (Some(js), css, analyze_diags)
    }));

    match codegen_result {
        Ok((js, css, analyze_diags)) => {
            diagnostics.extend(analyze_diags);
            CompileResult {
                js,
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

pub fn compile_module(source: &str, options: &ModuleCompileOptions) -> CompileResult {
    let is_ts = options.filename.ends_with(".ts");
    let dev = options.dev;

    let js_alloc = oxc_allocator::Allocator::default();

    let (analysis, mut parsed, mut diagnostics) =
        svelte_analyze::analyze_module(&js_alloc, source, is_ts, dev);

    if options.generate == GenerateMode::False
        || diagnostics
            .iter()
            .any(|d| d.severity == svelte_diagnostics::Severity::Error)
    {
        return CompileResult {
            js: None,
            css: None,
            diagnostics,
        };
    }

    let program = parsed
        .program
        .take()
        .expect("analyze_module produced no program");
    let line_index = svelte_span::LineIndex::new(source);
    let codegen_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        svelte_codegen_client::generate_module(&js_alloc, program, &analysis, &line_index, dev)
    }));

    match codegen_result {
        Ok(js) => CompileResult {
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
            diagnostics.push(Diagnostic::internal_error(message));
            CompileResult {
                js: None,
                css: None,
                diagnostics,
            }
        }
    }
}

#[cfg(test)]
mod tests;
