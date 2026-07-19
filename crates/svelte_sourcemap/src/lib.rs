use std::iter;

pub use oxc_sourcemap::{SourceMap, SourceMapBuilder, Token};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SourcemapKind {
    None,
    #[default]
    Default,
    Inline,
}

impl SourcemapKind {
    pub fn is_enabled(self) -> bool {
        !matches!(self, Self::None)
    }
}

#[derive(Debug, Default, Clone)]
pub struct JsOutput {
    pub code: String,
    pub map: Option<SourceMap<'static>>,
}

#[derive(Debug, Default, Clone)]
pub struct CssOutput {
    pub code: String,
    pub map: Option<SourceMap<'static>>,
    pub has_global: bool,
}

pub struct Sourcemap<'a> {
    map: SourceMap<'static>,
    source: &'a str,
}

impl<'a> Sourcemap<'a> {
    pub fn new(map: SourceMap<'static>, source: &'a str) -> Self {
        Self { map, source }
    }

    pub fn attach_sources_content(&mut self) -> &mut Self {
        self.map.set_source_contents(vec![Some(self.source)]);
        self
    }

    pub fn set_source_name(&mut self, name: &str) -> &mut Self {
        self.map.set_file(name);
        self.map.set_sources(iter::once(name));
        self
    }

    pub fn set_sources_name(&mut self, name: &str) -> &mut Self {
        self.map.set_sources(iter::once(name));
        self
    }

    pub fn to_inline_comment(&self) -> String {
        format!("\n/*# sourceMappingURL={} */", self.map.to_data_url())
    }

    pub fn into_inner(self) -> SourceMap<'static> {
        self.map
    }
}

pub fn parse_input_map(json: &str) -> Option<SourceMap<'static>> {
    if let Ok(map) = SourceMap::from_json_string(json) {
        return Some(map.into_owned());
    }
    let mut value: serde_json::Value = serde_json::from_str(json).ok()?;
    if let Some(sources) = value.get_mut("sources").and_then(|v| v.as_array_mut()) {
        for source in sources.iter_mut() {
            if source.is_null() {
                *source = serde_json::Value::String(String::new());
            }
        }
    }
    let json_str = value.to_string();
    SourceMap::from_json_string(&json_str)
        .ok()
        .map(|m| m.into_owned())
}

pub fn merge_with_preprocessor(
    base: SourceMap<'static>,
    preprocessor: &SourceMap<'_>,
    filename: &str,
    source_name: &str,
    base_offset: (u32, u32),
) -> SourceMap<'static> {
    let file_basename = get_basename(filename);
    let mut combined = compose(&base, preprocessor, base_offset);
    if let Some(file) = base.get_file() {
        combined.set_file(file);
    }
    if combined.get_sources().next().is_none() {
        combined.set_sources([file_basename]);
    }
    if file_basename != source_name {
        let relative: Vec<String> = combined
            .get_sources()
            .map(|source| get_relative_path(source_name, source))
            .collect();
        combined.set_sources(relative.iter().map(String::as_str));
    }
    combined
}

fn compose(
    base: &SourceMap<'_>,
    over: &SourceMap<'_>,
    base_offset: (u32, u32),
) -> SourceMap<'static> {
    let (offset_line, offset_col) = base_offset;
    let over_lookup = over.generate_lookup_table();
    let mut builder = SourceMapBuilder::default();
    for token in base.get_tokens() {
        let look_line = offset_line + token.get_src_line();
        let look_col = if token.get_src_line() == 0 {
            offset_col + token.get_src_col()
        } else {
            token.get_src_col()
        };
        let Some(resolved) = over.lookup_token(&over_lookup, look_line, look_col) else {
            continue;
        };
        let Some(source_id) = resolved.get_source_id() else {
            continue;
        };
        let Some(source) = over.get_source(source_id) else {
            continue;
        };
        if source.is_empty() {
            continue;
        }
        let content = over
            .get_source_content(source_id)
            .map_or("", |content| content);
        let new_source_id = builder.add_source_and_content(source, content);
        let name_id = resolved
            .get_name_id()
            .and_then(|name_id| over.get_name(name_id))
            .map(|name| builder.add_name(name));
        builder.add_token(
            token.get_dst_line(),
            token.get_dst_col(),
            resolved.get_src_line(),
            resolved.get_src_col(),
            Some(new_source_id),
            name_id,
        );
    }
    builder.into_sourcemap().into_owned()
}

pub fn get_basename(filename: &str) -> &str {
    filename
        .rsplit_once(['/', '\\'])
        .map_or(filename, |(_, base)| base)
}

pub fn get_relative_path(from: &str, to: &str) -> String {
    let from_parts: Vec<&str> = from.split(['/', '\\']).collect();
    let to_parts: Vec<&str> = to.split(['/', '\\']).collect();
    let from_dirs = &from_parts[..from_parts.len().saturating_sub(1)];

    let common = from_dirs
        .iter()
        .zip(to_parts.iter())
        .take_while(|(a, b)| a == b)
        .count();

    let ups = from_dirs.len() - common;
    let tail = &to_parts[common..];

    if ups == 0 {
        let mut out = String::with_capacity(2 + tail.iter().map(|s| s.len() + 1).sum::<usize>());
        out.push_str("./");
        for (i, part) in tail.iter().enumerate() {
            if i > 0 {
                out.push('/');
            }
            out.push_str(part);
        }
        out
    } else {
        let mut parts: Vec<&str> = Vec::with_capacity(ups + tail.len());
        parts.extend(iter::repeat_n("..", ups));
        parts.extend_from_slice(tail);
        parts.join("/")
    }
}

pub fn get_source_name(filename: &str, output_filename: Option<&str>) -> String {
    match output_filename {
        Some(out) => get_relative_path(out, filename),
        None => get_basename(filename).to_string(),
    }
}
