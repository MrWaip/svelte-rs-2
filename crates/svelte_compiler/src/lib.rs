mod arena_reuse;
mod options;
mod sourcemap_finalize;

pub use options::{
    CompileOptions, CssMode, ExperimentalOptions, GenerateMode, ModuleCompileOptions, Namespace,
    RunesOption,
};
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
pub use svelte_sourcemap::{CssOutput, JsOutput, SourceMap, SourcemapKind};

pub struct CompileResult {
    pub js: Option<JsOutput>,
    pub css: Option<CssOutput>,
    pub diagnostics: Vec<Diagnostic>,
}

fn apply_suppress(diagnostics: &mut Vec<Diagnostic>, suppress: &[String]) {
    if suppress.is_empty() {
        return;
    }
    diagnostics.retain(|diagnostic| {
        diagnostic.severity != svelte_diagnostics::Severity::Warning
            || !suppress.iter().any(|code| code == diagnostic.kind.code())
    });
}

fn filename_relative_to_root_dir(filename: &str, root_dir: Option<&str>) -> String {
    let normalized = filename.replace('\\', "/");
    let Some(rd) = root_dir else {
        return normalized;
    };
    let rd_norm = rd.replace('\\', "/");
    if let Some(rest) = normalized.strip_prefix(&rd_norm) {
        rest.strip_prefix('/').unwrap_or(rest).to_string()
    } else {
        normalized
    }
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

fn inline_runes_option(component: &svelte_ast::Component) -> Option<bool> {
    component.options.as_ref().and_then(|opts| opts.runes)
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

fn validate_compile_options(options: &CompileOptions, diagnostics: &mut Vec<Diagnostic>) {
    if options.enable_sourcemap.is_some() {
        diagnostics.push(Diagnostic::warning(
            DiagnosticKind::OptionsRemovedEnableSourcemap,
            svelte_span::Span::default(),
        ));
    }
}

pub fn compile(source: &str, options: &CompileOptions) -> CompileResult {
    let js_alloc = arena_reuse::acquire();
    let result = compile_in(&js_alloc, source, options);
    arena_reuse::release(js_alloc);
    result
}

fn compile_in(
    js_alloc: &oxc_allocator::Allocator,
    source: &str,
    options: &CompileOptions,
) -> CompileResult {
    let candidate_name = options.component_name();
    let preprocessor_map = options
        .preprocessor_map
        .as_deref()
        .and_then(svelte_sourcemap::parse_input_map);

    let (mut component, js_result, mut diagnostics) =
        svelte_parser::parse_with_js(js_alloc, source);
    validate_compile_options(options, &mut diagnostics);
    apply_compile_options_to_component(&mut component, options);
    let css_parsed = svelte_parser::parse_css_block(&component);

    let has_parse_errors = diagnostics
        .iter()
        .any(|d| d.severity == svelte_diagnostics::Severity::Error);

    let analyze_opts = svelte_analyze::AnalyzeOptions {
        custom_element: options.custom_element,
        experimental_async: options.experimental.async_,
        runes: options.runes,
        inline_runes: inline_runes_option(&component),
        accessors: resolved_accessors_option(&component, options),
        immutable: resolved_immutable_option(&component, options),
        preserve_whitespace: resolved_preserve_whitespace_option(&component, options),
        preserve_comments: options.preserve_comments,
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

    let mut css_parse_diags: Vec<Diagnostic> = Vec::new();
    let (js, css, analyze_diags) = {
        let (mut analysis, mut parsed, mut analyze_diags) =
            svelte_analyze::analyze_with_options(&component, js_result, &analyze_opts);

        let mut css_text: Option<String> = None;
        let mut css_map: Option<svelte_sourcemap::SourceMap> = None;
        if let Some((ss, css_diags)) = css_parsed {
            let css_offset = component
                .css
                .as_ref()
                .map_or(0, |block| block.content_span.start);
            css_parse_diags.extend(css_diags.into_iter().map(|mut diag| {
                diag.span = svelte_span::Span::new(
                    diag.span.start + css_offset,
                    diag.span.end + css_offset,
                );
                diag
            }));
            let inject_styles = resolved_css_mode(&component, options) == CssMode::Injected
                || analysis.custom_element.is_target;
            svelte_analyze::analyze_css_pass(
                &component,
                &ss,
                &parsed,
                &options.filename,
                options.root_dir.as_deref(),
                inject_styles,
                &mut analysis,
                &mut analyze_diags,
            );
            let css_block = component
                .css
                .as_ref()
                .unwrap_or_else(|| panic!("css block must exist when css_parsed is Some"));
            let css_source = component.source_text(css_block.content_span);
            let needs_map = options.sourcemap_kind.is_enabled() && (!inject_styles || options.dev);
            if needs_map {
                let (raw_css, mut raw_map) = svelte_transform_css::transform_css_with_sourcemap(
                    &analysis.css.hash,
                    &analysis.css.keyframes,
                    Some(&analysis.css.used_selectors),
                    true,
                    ss,
                    css_source,
                    &options.filename,
                );
                let css_output_filename = options.css_output_filename.as_deref();
                let css_source_name =
                    svelte_sourcemap::get_source_name(&options.filename, css_output_filename);
                let css_file = svelte_sourcemap::get_basename(
                    css_output_filename.unwrap_or(&options.filename),
                );
                raw_map.set_file(css_file);
                raw_map.set_sources([css_source_name.as_str()]);
                if let Some(preprocessor) = preprocessor_map.as_ref() {
                    let (line, col) =
                        svelte_span::LineIndex::new(source).line_col(css_block.content_span.start);
                    let base_offset = (line.saturating_sub(1) as u32, col as u32);
                    raw_map = svelte_sourcemap::merge_with_preprocessor(
                        raw_map,
                        preprocessor,
                        &options.filename,
                        &css_source_name,
                        base_offset,
                    );
                }
                if inject_styles {
                    let mut compacted = svelte_transform_css::compact_css_for_injection(&raw_css);
                    if options.dev && !compacted.is_empty() {
                        let sm = svelte_sourcemap::Sourcemap::new(raw_map, css_source);
                        compacted.push_str(&sm.to_inline_comment());
                    }
                    css_text = Some(compacted);
                } else {
                    css_text = Some(raw_css);
                    css_map = Some(raw_map);
                }
            } else {
                let raw_css = svelte_transform_css::transform_css_with_usage(
                    &analysis.css.hash,
                    &analysis.css.keyframes,
                    Some(&analysis.css.used_selectors),
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
        }

        let (css, injected_css_text) = if analysis.css.inject_styles {
            (None, css_text)
        } else {
            (
                css_text.map(|code| svelte_sourcemap::CssOutput {
                    code,
                    map: css_map,
                    has_global: analysis.css.has_global,
                }),
                None,
            )
        };

        let has_errors = has_parse_errors
            || analyze_diags
                .iter()
                .chain(css_parse_diags.iter())
                .any(|d| d.severity == svelte_diagnostics::Severity::Error);

        if has_errors {
            (None, css, analyze_diags)
        } else {
            let mut ident_gen = svelte_analyze::IdentGen::with_conflicts(
                analysis.scoping.collect_all_symbol_names(),
            );
            let name = analysis.component_name().to_string();
            let _ = ident_gen.generate(&name);
            let line_index = if options.dev {
                svelte_span::LineIndex::new(component.source.as_str())
            } else {
                svelte_span::LineIndex::empty()
            };
            let codegen_options = svelte_types::CodegenOptions {
                dev: options.dev,
                hmr: options.hmr,
                experimental_async: options.experimental.async_,
                disclose_version: options.disclose_version,
                filename: filename_relative_to_root_dir(
                    &options.filename,
                    options.root_dir.as_deref(),
                ),
                sourcemap_kind: options.sourcemap_kind,
            };
            let transform_options = svelte_types::TransformOptions {
                dev: options.dev,
                filename: filename_relative_to_root_dir(
                    &options.filename,
                    options.root_dir.as_deref(),
                ),
            };
            let js = match options.generate {
                GenerateMode::Server => {
                    let mut compile_ctx = svelte_types::CompileContext {
                        alloc: js_alloc,
                        component: &component,
                        analysis: &analysis,
                        js_arena: &mut parsed,
                        ident_gen: &mut ident_gen,
                        line_index: &line_index,
                    };
                    svelte_transform_server::transform_component(
                        &mut compile_ctx,
                        &transform_options,
                    );
                    svelte_codegen_server::generate(
                        compile_ctx,
                        &codegen_options,
                        injected_css_text.as_deref(),
                    )
                    .ok()
                }
                GenerateMode::Client | GenerateMode::False => {
                    let transform_data = {
                        let mut compile_ctx = svelte_types::CompileContext {
                            alloc: js_alloc,
                            component: &component,
                            analysis: &analysis,
                            js_arena: &mut parsed,
                            ident_gen: &mut ident_gen,
                            line_index: &line_index,
                        };
                        svelte_transform_client::transform_component(
                            &mut compile_ctx,
                            &transform_options,
                        )
                    };
                    let compile_ctx = svelte_types::CompileContext {
                        alloc: js_alloc,
                        component: &component,
                        analysis: &analysis,
                        js_arena: &mut parsed,
                        ident_gen: &mut ident_gen,
                        line_index: &line_index,
                    };
                    Some(svelte_codegen_client::generate(
                        compile_ctx,
                        &codegen_options,
                        transform_data,
                        injected_css_text.as_deref(),
                    ))
                }
            };
            (js, css, analyze_diags)
        }
    };

    diagnostics.extend(css_parse_diags);
    if !has_parse_errors {
        diagnostics.extend(analyze_diags);
    }
    apply_suppress(&mut diagnostics, &options.suppress);
    let source_name =
        svelte_sourcemap::get_source_name(&options.filename, options.output_filename.as_deref());
    CompileResult {
        js: js.map(|out| {
            sourcemap_finalize::finalize_js(
                out,
                source,
                &options.filename,
                &source_name,
                preprocessor_map.as_ref(),
            )
        }),
        css,
        diagnostics,
    }
}

pub fn compile_module(source: &str, options: &ModuleCompileOptions) -> CompileResult {
    let js_alloc = arena_reuse::acquire();
    let result = compile_module_in(&js_alloc, source, options);
    arena_reuse::release(js_alloc);
    result
}

fn compile_module_in(
    js_alloc: &oxc_allocator::Allocator,
    source: &str,
    options: &ModuleCompileOptions,
) -> CompileResult {
    let is_ts = options.filename.ends_with(".ts");
    let dev = options.dev;

    let (analysis, mut parsed, mut diagnostics) =
        svelte_analyze::analyze_module(js_alloc, source, is_ts, dev);
    apply_suppress(&mut diagnostics, &options.suppress);

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

    let mut program = parsed
        .program
        .take()
        .expect("analyze_module produced no program");
    let line_index = if dev {
        svelte_span::LineIndex::new(source)
    } else {
        svelte_span::LineIndex::empty()
    };
    let kind = options.sourcemap_kind;
    let filename = filename_relative_to_root_dir(&options.filename, options.root_dir.as_deref());
    let js = match options.generate {
        GenerateMode::Server => {
            let mut ident_gen = svelte_analyze::IdentGen::with_conflicts(
                analysis.scoping.collect_all_symbol_names(),
            );
            let transform_options = svelte_types::TransformOptions {
                dev,
                filename: filename.clone(),
            };
            svelte_transform_server::transform_module(
                js_alloc,
                &mut program,
                &analysis,
                &mut ident_gen,
                &transform_options,
            );
            svelte_codegen_server::generate_module(js_alloc, program)
        }
        GenerateMode::Client | GenerateMode::False => svelte_codegen_client::generate_module(
            js_alloc,
            program,
            &analysis,
            &line_index,
            dev,
            kind,
            &filename,
            source,
        ),
    };

    let source_name = if filename.is_empty() || filename == "(unknown)" {
        "input.svelte.js".to_string()
    } else {
        svelte_sourcemap::get_basename(&filename).to_string()
    };
    let preprocessor_map = options
        .preprocessor_map
        .as_deref()
        .and_then(svelte_sourcemap::parse_input_map);
    CompileResult {
        js: Some(sourcemap_finalize::finalize_js(
            js,
            source,
            &filename,
            &source_name,
            preprocessor_map.as_ref(),
        )),
        css: None,
        diagnostics,
    }
}

#[cfg(test)]
mod tests;
