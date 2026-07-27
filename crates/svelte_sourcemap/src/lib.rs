use std::iter;

pub use oxc_sourcemap::{SourceMap, SourceMapBuilder, Token};
use svelte_span::Span;

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

pub fn splice_region<'a>(
    document: &'a str,
    document_map: Option<&'a SourceMap<'a>>,
    region: Span,
    new_content: &str,
    new_content_map: Option<&SourceMap<'_>>,
    filename: &'a str,
) -> (String, SourceMap<'static>) {
    let file_basename = get_basename(filename);
    let region_start = line_col(document, region.start);
    let region_end = line_col(document, region.end);
    let new_region_end = end_of(region_start, new_content);

    let prefix = &document[..region.start as usize];
    let suffix = &document[region.end as usize..];
    let code = format!("{prefix}{new_content}{suffix}");

    let mut builder = SourceMapBuilder::default();

    let mut before_tokens = Vec::new();
    let mut after_tokens = Vec::new();
    if let Some(map) = document_map {
        for token in map.get_tokens() {
            let dst = (token.get_dst_line(), token.get_dst_col());
            if dst < region_start {
                before_tokens.push(token);
            } else if dst >= region_end {
                after_tokens.push(token);
            }
        }
    }

    match document_map {
        Some(map) => {
            for token in before_tokens {
                let dst = (token.get_dst_line(), token.get_dst_col());
                copy_token(&mut builder, map, token, dst);
            }
        }
        None => {
            add_identity_lines(
                &mut builder,
                document,
                Span::new(0, region.start),
                |pos| pos,
                file_basename,
            );
        }
    }

    if let Some(new_map) = new_content_map {
        let document_lookup = document_map.map(|map| (map, map.generate_lookup_table()));
        for token in new_map.get_tokens() {
            let dst = offset_position(region_start, (token.get_dst_line(), token.get_dst_col()));
            let src_abs =
                offset_position(region_start, (token.get_src_line(), token.get_src_col()));
            let (src_line, src_col, source, content) = resolve_original(
                document_lookup
                    .as_ref()
                    .map(|(map, table)| (*map, &table[..])),
                src_abs,
                document,
                file_basename,
            );
            let source_id = builder.add_source_and_content(source, content);
            let name_id = token
                .get_name_id()
                .and_then(|id| new_map.get_name(id))
                .map(|name| builder.add_name(name));
            builder.add_token(dst.0, dst.1, src_line, src_col, Some(source_id), name_id);
        }
    }

    match document_map {
        Some(map) => {
            for token in after_tokens {
                let dst = (token.get_dst_line(), token.get_dst_col());
                let new_dst = shift_past(dst, region_end, new_region_end);
                copy_token(&mut builder, map, token, new_dst);
            }
        }
        None => {
            add_identity_lines(
                &mut builder,
                document,
                Span::new(region.end, document.len() as u32),
                |pos| shift_past(pos, region_end, new_region_end),
                file_basename,
            );
        }
    }

    let mut result = builder.into_sourcemap().into_owned();
    if result.get_sources().next().is_none() {
        result.set_sources([file_basename]);
    }
    (code, result)
}

fn copy_token<'a>(
    builder: &mut SourceMapBuilder<'a>,
    map: &'a SourceMap<'a>,
    token: Token,
    new_dst: (u32, u32),
) {
    let source_id = token.get_source_id().and_then(|id| {
        let source = map.get_source(id)?;
        let content = map.get_source_content(id).unwrap_or("");
        Some(builder.add_source_and_content(source, content))
    });
    let name_id = token
        .get_name_id()
        .and_then(|id| map.get_name(id))
        .map(|name| builder.add_name(name));
    builder.add_token(
        new_dst.0,
        new_dst.1,
        token.get_src_line(),
        token.get_src_col(),
        source_id,
        name_id,
    );
}

fn resolve_original<'a>(
    document_lookup: Option<(&'a SourceMap<'a>, &[&'a [Token]])>,
    abs: (u32, u32),
    document: &'a str,
    file_basename: &'a str,
) -> (u32, u32, &'a str, &'a str) {
    let Some((map, lookup)) = document_lookup else {
        return (abs.0, abs.1, file_basename, document);
    };
    let Some(resolved) = map.lookup_token(lookup, abs.0, abs.1) else {
        return (abs.0, abs.1, file_basename, document);
    };
    let Some(source_id) = resolved.get_source_id() else {
        return (abs.0, abs.1, file_basename, document);
    };
    let Some(source) = map.get_source(source_id) else {
        return (abs.0, abs.1, file_basename, document);
    };
    let content = map.get_source_content(source_id).unwrap_or(document);
    (
        resolved.get_src_line(),
        resolved.get_src_col(),
        source,
        content,
    )
}

fn add_identity_lines<'a>(
    builder: &mut SourceMapBuilder<'a>,
    document: &'a str,
    span: Span,
    shift: impl Fn((u32, u32)) -> (u32, u32),
    file_basename: &'a str,
) {
    if span.start >= span.end {
        return;
    }
    let source_id = builder.add_source_and_content(file_basename, document);
    let bytes = document.as_bytes();
    let mut pos = line_col(document, span.start);
    let mut offset = span.start;
    let mut at_line_start = true;
    while offset < span.end {
        if at_line_start {
            let dst = shift(pos);
            builder.add_token(dst.0, dst.1, pos.0, pos.1, Some(source_id), None);
            at_line_start = false;
        }
        if bytes[offset as usize] == b'\n' {
            pos = (pos.0 + 1, 0);
            at_line_start = true;
        } else {
            pos.1 += 1;
        }
        offset += 1;
    }
}

fn line_col(source: &str, offset: u32) -> (u32, u32) {
    let bytes = &source.as_bytes()[..offset as usize];
    let line = bytes.iter().filter(|&&b| b == b'\n').count() as u32;
    let col = match bytes.iter().rposition(|&b| b == b'\n') {
        Some(idx) => (bytes.len() - idx - 1) as u32,
        None => bytes.len() as u32,
    };
    (line, col)
}

fn offset_position(base: (u32, u32), rel: (u32, u32)) -> (u32, u32) {
    if rel.0 == 0 {
        (base.0, base.1 + rel.1)
    } else {
        (base.0 + rel.0, rel.1)
    }
}

fn end_of(start: (u32, u32), text: &str) -> (u32, u32) {
    offset_position(start, line_col(text, text.len() as u32))
}

fn shift_past(pos: (u32, u32), old_end: (u32, u32), new_end: (u32, u32)) -> (u32, u32) {
    if pos.0 == old_end.0 {
        (new_end.0, new_end.1 + (pos.1 - old_end.1))
    } else {
        (new_end.0 + (pos.0 - old_end.0), pos.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn splice(document: &str, region: Span, new_content: &str) -> (String, SourceMap<'static>) {
        splice_region(document, None, region, new_content, None, "app.svelte")
    }

    #[track_caller]
    fn assert_code(result: &(String, SourceMap<'static>), expected: &str) {
        assert_eq!(
            result.0, expected,
            "spliced code: expected {expected:?}, got {:?}",
            result.0
        );
    }

    #[track_caller]
    fn assert_position(map: &SourceMap<'_>, dst: (u32, u32), expected: (u32, u32, &str)) {
        let lookup = map.generate_lookup_table();
        let token = map
            .lookup_token(&lookup, dst.0, dst.1)
            .unwrap_or_else(|| panic!("expected a mapping token at {dst:?}"));
        let source_id = token
            .get_source_id()
            .unwrap_or_else(|| panic!("expected a source id at {dst:?}"));
        let source = map
            .get_source(source_id)
            .unwrap_or_else(|| panic!("expected a source name at {dst:?}"));
        let got = (token.get_src_line(), token.get_src_col(), source);
        assert_eq!(
            got, expected,
            "position at {dst:?}: expected {expected:?}, got {got:?}"
        );
    }

    #[test]
    fn replaces_region_content_in_place() {
        let result = splice(
            "<script>let x = 1;</script>",
            Span::new(8, 18),
            "let x = 2;",
        );
        assert_code(&result, "<script>let x = 2;</script>");
    }

    #[test]
    fn identity_maps_untouched_prefix_to_original_position() {
        let (_, map) = splice(
            "<script>let x = 1;</script>",
            Span::new(8, 18),
            "let x = 2;",
        );
        assert_position(&map, (0, 0), (0, 0, "app.svelte"));
    }

    #[test]
    fn identity_maps_shifted_suffix_to_original_position() {
        let (code, map) = splice(
            "<script>let x = 1;</script>",
            Span::new(8, 18),
            "let x = 22;",
        );
        let suffix_col = code.find("</script>").expect("expected closing script tag") as u32;
        assert_position(&map, (0, suffix_col), (0, 18, "app.svelte"));
    }

    #[test]
    fn identity_maps_track_line_numbers_across_multiple_lines() {
        let document = "line0\nline1\nline2\n<style>x</style>\nline4";
        let region = Span::new(
            document.find("<style>").expect("expected style tag") as u32,
            document
                .find("</style>")
                .expect("expected closing style tag") as u32
                + "</style>".len() as u32,
        );
        let (_, map) = splice(document, region, "y\nz");
        assert_position(&map, (2, 0), (2, 0, "app.svelte"));
        assert_position(&map, (5, 0), (4, 0, "app.svelte"));
    }

    #[test]
    fn composes_region_mapping_through_prior_document_map() {
        let document = "let js = 1;";
        let mut document_builder = SourceMapBuilder::default();
        let doc_source = document_builder.add_source_and_content("app.svelte", "let ts = 1;");
        document_builder.add_token(0, 4, 0, 4, Some(doc_source), None);
        let document_map = document_builder.into_sourcemap();

        let mut region_builder = SourceMapBuilder::default();
        let region_source = region_builder.add_source_and_content("region", "js");
        region_builder.add_token(0, 0, 0, 0, Some(region_source), None);
        let new_content_map = region_builder.into_sourcemap();

        let (_, map) = splice_region(
            document,
            Some(&document_map),
            Span::new(4, 6),
            "JS",
            Some(&new_content_map),
            "output.svelte",
        );

        assert_position(&map, (0, 4), (0, 4, "app.svelte"));
    }

    #[test]
    fn preserves_ascending_dst_order_across_prefix_region_and_suffix() {
        let document = "let js = 1; done";
        let mut document_builder = SourceMapBuilder::default();
        let doc_source = document_builder.add_source_and_content("app.svelte", document);
        document_builder.add_token(0, 0, 0, 0, Some(doc_source), None);
        document_builder.add_token(0, 4, 0, 4, Some(doc_source), None);
        document_builder.add_token(0, 13, 0, 13, Some(doc_source), None);
        let document_map = document_builder.into_sourcemap();

        let mut region_builder = SourceMapBuilder::default();
        let region_source = region_builder.add_source_and_content("region", "js");
        region_builder.add_token(0, 0, 0, 0, Some(region_source), None);
        let new_content_map = region_builder.into_sourcemap();

        let (code, map) = splice_region(
            document,
            Some(&document_map),
            Span::new(4, 6),
            "TS",
            Some(&new_content_map),
            "output.svelte",
        );

        assert_eq!(
            code, "let TS = 1; done",
            "spliced code: expected replacement in place"
        );
        assert_position(&map, (0, 0), (0, 0, "app.svelte"));
        assert_position(&map, (0, 4), (0, 4, "app.svelte"));
        assert_position(&map, (0, 13), (0, 13, "app.svelte"));
        map.to_json_string();
    }
}
