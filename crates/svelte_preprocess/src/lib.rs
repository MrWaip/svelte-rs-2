use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, RwLock};

use rustc_hash::{FxHashMap, FxHasher};

use grass::{InputSyntax, Options as GrassOptions, OutputStyle};
use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};
use lightningcss::targets::{Browsers, Targets};
use svelte_ast::Attribute;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_sourcemap::SourceMap;
use svelte_span::Span;

#[derive(Debug, Clone, Default)]
pub struct PreprocessOptions {
    pub filename: String,
    pub load_paths: Vec<PathBuf>,
    pub style_prepend: Option<String>,
    pub css_targets: Vec<String>,
    pub cache_styles: bool,
}

static STYLE_CACHE: LazyLock<RwLock<FxHashMap<u64, String>>> =
    LazyLock::new(|| RwLock::new(FxHashMap::default()));

fn style_cache_key(input: &str, load_paths: &[PathBuf], syntax: StyleLanguage) -> u64 {
    let mut hasher = FxHasher::default();
    input.hash(&mut hasher);
    for path in load_paths {
        path.hash(&mut hasher);
    }
    match syntax {
        StyleLanguage::Scss => 0u8.hash(&mut hasher),
        StyleLanguage::Sass => 1u8.hash(&mut hasher),
    }
    hasher.finish()
}

fn cached_style(key: u64) -> Option<String> {
    let cache = STYLE_CACHE.read().ok()?;
    cache.get(&key).cloned()
}

fn store_style(key: u64, compiled: &str) {
    let Ok(mut cache) = STYLE_CACHE.write() else {
        return;
    };
    cache.insert(key, compiled.to_string());
}

pub struct Preprocessed {
    pub code: String,
    pub map: Option<SourceMap<'static>>,
    pub diagnostics: Vec<Diagnostic>,
}

impl Preprocessed {
    fn unchanged(source: &str) -> Self {
        Self {
            code: source.to_string(),
            map: None,
            diagnostics: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyleLanguage {
    Scss,
    Sass,
}

impl StyleLanguage {
    fn from_lang(lang: &str) -> Option<Self> {
        match lang {
            "scss" => Some(Self::Scss),
            "sass" => Some(Self::Sass),
            _ => None,
        }
    }

    fn input_syntax(self) -> InputSyntax {
        match self {
            Self::Scss => InputSyntax::Scss,
            Self::Sass => InputSyntax::Sass,
        }
    }
}

pub fn preprocess_style(
    source: &str,
    document_map: Option<&SourceMap<'_>>,
    options: &PreprocessOptions,
) -> Preprocessed {
    let (component, _diagnostics) = svelte_parser::Parser::new(source).parse();
    let Some(css) = component.css.as_ref() else {
        return Preprocessed::unchanged(source);
    };

    let language = style_language(source, &css.attributes);
    let applies_targets = !options.css_targets.is_empty();
    if language.is_none() && !applies_targets {
        return Preprocessed::unchanged(source);
    }

    let content = css.content_span.source_text(source);
    let mut compiled = content.to_string();

    if let Some(language) = language {
        let load_paths = effective_load_paths(options);
        let prepared = build_style_input(content, options.style_prepend.as_deref());
        let resolved = resolve_load_path_specifiers(&prepared, &load_paths);
        let cache_key = style_cache_key(&resolved, &load_paths, language);
        let hit = match options.cache_styles {
            true => cached_style(cache_key),
            false => None,
        };

        match hit {
            Some(cached) => compiled = cached,
            None => {
                let grass_options = GrassOptions::default()
                    .style(OutputStyle::Expanded)
                    .input_syntax(language.input_syntax())
                    .quiet(true)
                    .load_paths(&load_paths);

                compiled = match grass::from_string(resolved, &grass_options) {
                    Ok(compiled) => compiled,
                    Err(error) => {
                        return Preprocessed {
                            code: source.to_string(),
                            map: None,
                            diagnostics: vec![style_error(&error.to_string(), css.content_span)],
                        };
                    }
                };

                if options.cache_styles {
                    store_style(cache_key, &compiled);
                }
            }
        }
    }

    if applies_targets {
        compiled = match apply_css_targets(&compiled, &options.css_targets, &options.filename) {
            Ok(lowered) => lowered,
            Err(message) => {
                return Preprocessed {
                    code: source.to_string(),
                    map: None,
                    diagnostics: vec![style_error(&message, css.content_span)],
                };
            }
        };
    }

    let region = Span::new(css.content_span.start, css.content_span.end);
    let (code, map) = svelte_sourcemap::splice_region(
        source,
        document_map,
        region,
        &compiled,
        None,
        &options.filename,
    );

    Preprocessed {
        code,
        map: Some(map),
        diagnostics: Vec::new(),
    }
}

fn effective_load_paths(options: &PreprocessOptions) -> Vec<PathBuf> {
    let mut paths = Vec::with_capacity(options.load_paths.len() + 1);
    let source_dir = Path::new(&options.filename).parent();
    if let Some(dir) = source_dir
        && dir.is_dir()
    {
        paths.push(dir.to_path_buf());
    }
    paths.extend(options.load_paths.iter().cloned());
    paths
}

fn build_style_input(content: &str, prepend: Option<&str>) -> String {
    let Some(prepend) = prepend else {
        return content.to_string();
    };
    if prepend.trim().is_empty() {
        return content.to_string();
    }

    let mut input = String::with_capacity(prepend.len() + content.len() + 1);
    input.push_str(prepend);
    if !prepend.ends_with('\n') {
        input.push('\n');
    }
    input.push_str(content);
    input
}

const LOAD_RULES: [&str; 3] = ["@use", "@forward", "@import"];

fn resolve_load_path_specifiers(source: &str, load_paths: &[PathBuf]) -> String {
    if load_paths.is_empty() {
        return source.to_string();
    }
    if !LOAD_RULES.iter().any(|rule| source.contains(rule)) {
        return source.to_string();
    }

    let bytes = source.as_bytes();
    let mut out = String::with_capacity(source.len());
    let mut cursor = 0;

    while cursor < bytes.len() {
        let Some(rule_start) = find_load_rule(source, cursor) else {
            out.push_str(&source[cursor..]);
            return out;
        };
        let Some((specifier, quote, start, end)) = read_specifier(source, rule_start) else {
            let next = rule_start + 1;
            out.push_str(&source[cursor..next]);
            cursor = next;
            continue;
        };

        out.push_str(&source[cursor..start]);
        match resolve_specifier(specifier, load_paths) {
            Some(resolved) => {
                out.push(quote);
                out.push_str(&resolved);
                out.push(quote);
            }
            None => out.push_str(&source[start..end]),
        }
        cursor = end;
    }

    out
}

fn find_load_rule(source: &str, from: usize) -> Option<usize> {
    let mut best: Option<usize> = None;
    for rule in LOAD_RULES {
        let Some(found) = source[from..].find(rule) else {
            continue;
        };
        let position = from + found;
        if best.is_none_or(|current| position < current) {
            best = Some(position);
        }
    }
    best
}

fn read_specifier(source: &str, rule_start: usize) -> Option<(&str, char, usize, usize)> {
    let after_rule = source[rule_start..].find(['"', '\''])?;
    let quote_start = rule_start + after_rule;
    let quote = source[quote_start..].chars().next()?;
    let between = &source[rule_start..quote_start];
    if between.contains(';') || between.contains('\n') && between.trim_end().ends_with(';') {
        return None;
    }

    let rest = &source[quote_start + 1..];
    let close = rest.find(quote)?;
    let specifier = &rest[..close];
    Some((specifier, quote, quote_start, quote_start + 1 + close + 1))
}

fn resolve_specifier(specifier: &str, load_paths: &[PathBuf]) -> Option<String> {
    if specifier.starts_with('.') || specifier.starts_with('/') {
        return None;
    }

    for load_path in load_paths {
        if let Some(found) = resolve_in_load_path(specifier, load_path) {
            return Some(found.to_string_lossy().into_owned());
        }
    }
    None
}

fn resolve_in_load_path(specifier: &str, load_path: &Path) -> Option<PathBuf> {
    let candidate = load_path.join(specifier);
    if has_style_extension(specifier) {
        if candidate.is_file() {
            return Some(candidate);
        }
        if let Some(partial) = partial_variant(&candidate)
            && partial.is_file()
        {
            return Some(partial);
        }
    }
    resolve_package_export(specifier, load_path)
}

const EXPORT_CONDITIONS: [&str; 6] = ["sass", "style", "svelte", "import", "require", "default"];

fn resolve_package_export(specifier: &str, load_path: &Path) -> Option<PathBuf> {
    let (package, subpath) = split_package_specifier(specifier)?;
    let package_dir = load_path.join(&package);
    let manifest = fs::read_to_string(package_dir.join("package.json")).ok()?;
    let parsed: serde_json::Value = serde_json::from_str(&manifest).ok()?;
    let exports = parsed.get("exports")?;
    let entry = exports.get(&subpath)?;
    let target = pick_export_target(entry)?;
    let resolved = package_dir.join(target.trim_start_matches("./"));
    if !resolved.is_file() {
        return None;
    }
    Some(resolved)
}

fn split_package_specifier(specifier: &str) -> Option<(String, String)> {
    let mut segments = specifier.split('/');
    let first = segments.next()?;
    if first.is_empty() {
        return None;
    }

    let package = if first.starts_with('@') {
        let scope_name = segments.next()?;
        format!("{first}/{scope_name}")
    } else {
        first.to_string()
    };

    let rest: Vec<&str> = segments.collect();
    if rest.is_empty() {
        return Some((package, ".".to_string()));
    }
    Some((package, format!("./{}", rest.join("/"))))
}

fn pick_export_target(entry: &serde_json::Value) -> Option<String> {
    if let Some(target) = entry.as_str() {
        return Some(target.to_string());
    }

    let map = entry.as_object()?;
    for condition in EXPORT_CONDITIONS {
        let Some(value) = map.get(condition) else {
            continue;
        };
        if let Some(target) = pick_export_target(value) {
            return Some(target);
        }
    }
    None
}

fn partial_variant(path: &Path) -> Option<PathBuf> {
    let parent = path.parent()?;
    let name = path.file_name()?.to_str()?;
    Some(parent.join(format!("_{name}")))
}

fn has_style_extension(specifier: &str) -> bool {
    specifier.ends_with(".scss") || specifier.ends_with(".sass") || specifier.ends_with(".css")
}

fn apply_css_targets(css: &str, queries: &[String], filename: &str) -> Result<String, String> {
    let browsers = Browsers::from_browserslist(queries).map_err(|error| error.to_string())?;
    let targets = Targets {
        browsers,
        ..Targets::default()
    };

    let parser_options = ParserOptions {
        filename: filename.to_string(),
        ..ParserOptions::default()
    };
    let mut stylesheet =
        StyleSheet::parse(css, parser_options).map_err(|error| error.to_string())?;
    stylesheet
        .minify(MinifyOptions {
            targets,
            ..MinifyOptions::default()
        })
        .map_err(|error| error.to_string())?;

    let printed = stylesheet
        .to_css(PrinterOptions {
            targets,
            ..PrinterOptions::default()
        })
        .map_err(|error| error.to_string())?;

    Ok(printed.code)
}

pub fn style_language(source: &str, attributes: &[Attribute]) -> Option<StyleLanguage> {
    for attribute in attributes {
        let Attribute::StringAttribute(attr) = attribute else {
            continue;
        };
        if attr.name != "lang" {
            continue;
        }
        return StyleLanguage::from_lang(attr.value(source));
    }
    None
}

fn style_error(message: &str, span: Span) -> Diagnostic {
    Diagnostic::error(
        DiagnosticKind::StylePreprocessFailed {
            message: message.to_string(),
        },
        span,
    )
}

#[cfg(test)]
mod tests;
