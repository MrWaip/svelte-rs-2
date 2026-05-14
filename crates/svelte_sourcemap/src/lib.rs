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
    pub map: Option<SourceMap>,
}

#[derive(Debug, Default, Clone)]
pub struct CssOutput {
    pub code: String,
    pub map: Option<SourceMap>,
    pub has_global: bool,
}

pub struct Sourcemap<'a> {
    map: SourceMap,
    source: &'a str,
}

impl<'a> Sourcemap<'a> {
    pub fn new(map: SourceMap, source: &'a str) -> Self {
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

    pub fn to_inline_comment(&self) -> String {
        format!("\n/*# sourceMappingURL={} */", self.map.to_data_url())
    }

    pub fn into_inner(self) -> SourceMap {
        self.map
    }
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
