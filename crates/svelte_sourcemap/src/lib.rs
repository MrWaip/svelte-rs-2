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
        self.map.set_sources(std::iter::once(name));
        self
    }

    pub fn set_sources(&mut self, sources: &[&str]) -> &mut Self {
        self.map.set_sources(sources.iter().copied());
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
    let bytes = filename.as_bytes();
    let mut last_sep = None;
    for (i, &b) in bytes.iter().enumerate() {
        if b == b'/' || b == b'\\' {
            last_sep = Some(i);
        }
    }
    match last_sep {
        Some(i) => &filename[i + 1..],
        None => filename,
    }
}

pub fn get_relative_path(from: &str, to: &str) -> String {
    let split = |s: &str| -> Vec<String> { s.split(['/', '\\']).map(str::to_string).collect() };
    let mut from_parts = split(from);
    let mut to_parts = split(to);
    from_parts.pop();
    while !from_parts.is_empty() && !to_parts.is_empty() && from_parts[0] == to_parts[0] {
        from_parts.remove(0);
        to_parts.remove(0);
    }
    if !from_parts.is_empty() {
        let ups: Vec<&str> = from_parts.iter().map(|_| "..").collect();
        let mut joined = ups;
        let to_refs: Vec<&str> = to_parts.iter().map(String::as_str).collect();
        joined.extend(to_refs);
        joined.join("/")
    } else {
        format!("./{}", to_parts.join("/"))
    }
}

pub fn get_source_name(filename: &str, output_filename: Option<&str>) -> String {
    match output_filename {
        Some(out) => get_relative_path(out, filename),
        None => get_basename(filename).to_string(),
    }
}
