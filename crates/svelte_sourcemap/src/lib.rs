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
    let mut document_ids = match document_map {
        Some(map) => BuilderIds::new(map),
        None => BuilderIds::empty(),
    };

    match document_map {
        Some(map) => {
            for token in map.get_tokens() {
                let dst = (token.get_dst_line(), token.get_dst_col());
                if dst >= region_start {
                    continue;
                }
                copy_token(&mut builder, &mut document_ids, map, token, dst);
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
        let region_source = region_source_id(new_map, file_basename);
        let mut region_ids = BuilderIds::new(new_map);
        let mut fallback_source = None;

        for token in new_map.get_tokens() {
            let dst = offset_position(region_start, (token.get_dst_line(), token.get_dst_col()));
            let name_id = token
                .get_name_id()
                .and_then(|id| region_ids.name(&mut builder, new_map, id));

            if !maps_into_region(token, region_source) {
                let source_id = token
                    .get_source_id()
                    .and_then(|id| region_ids.source(&mut builder, new_map, id));
                builder.add_token(
                    dst.0,
                    dst.1,
                    token.get_src_line(),
                    token.get_src_col(),
                    source_id,
                    name_id,
                );
                continue;
            }

            let src_abs =
                offset_position(region_start, (token.get_src_line(), token.get_src_col()));
            let mut resolved = None;
            if let Some((map, lookup)) = &document_lookup {
                resolved = resolve_original(&mut builder, &mut document_ids, map, lookup, src_abs);
            }
            let (src_line, src_col, source_id) = match resolved {
                Some(origin) => origin,
                None => {
                    let source_id = *fallback_source.get_or_insert_with(|| {
                        builder.add_source_and_content(file_basename, document)
                    });
                    (src_abs.0, src_abs.1, source_id)
                }
            };
            builder.add_token(dst.0, dst.1, src_line, src_col, Some(source_id), name_id);
        }
    }

    match document_map {
        Some(map) => {
            for token in map.get_tokens() {
                let dst = (token.get_dst_line(), token.get_dst_col());
                if dst < region_end {
                    continue;
                }
                let new_dst = shift_past(dst, region_end, new_region_end);
                copy_token(&mut builder, &mut document_ids, map, token, new_dst);
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

#[derive(Default)]
struct BuilderIds {
    sources: Vec<Option<u32>>,
    names: Vec<Option<u32>>,
}

impl BuilderIds {
    fn new(map: &SourceMap<'_>) -> Self {
        Self {
            sources: vec![None; map.get_sources().len()],
            names: vec![None; map.get_names().len()],
        }
    }

    fn empty() -> Self {
        Self::default()
    }

    fn source<'a>(
        &mut self,
        builder: &mut SourceMapBuilder<'a>,
        map: &'a SourceMap<'a>,
        source_id: u32,
    ) -> Option<u32> {
        let index = source_id as usize;
        if let Some(&Some(id)) = self.sources.get(index) {
            return Some(id);
        }
        let source = map.get_source(source_id)?;
        let content = map.get_source_content(source_id).unwrap_or("");
        let id = builder.add_source_and_content(source, content);
        self.sources[index] = Some(id);
        Some(id)
    }

    fn name<'a>(
        &mut self,
        builder: &mut SourceMapBuilder<'a>,
        map: &'a SourceMap<'a>,
        name_id: u32,
    ) -> Option<u32> {
        let index = name_id as usize;
        if let Some(&Some(id)) = self.names.get(index) {
            return Some(id);
        }
        let name = map.get_name(name_id)?;
        let id = builder.add_name(name);
        self.names[index] = Some(id);
        Some(id)
    }
}

fn region_source_id(map: &SourceMap<'_>, file_basename: &str) -> Option<u32> {
    for (index, source) in map.get_sources().enumerate() {
        if source == file_basename {
            return Some(index as u32);
        }
    }
    None
}

fn maps_into_region(token: Token, region_source: Option<u32>) -> bool {
    let Some(source_id) = token.get_source_id() else {
        return true;
    };
    let Some(region) = region_source else {
        return false;
    };
    source_id == region
}

fn copy_token<'a>(
    builder: &mut SourceMapBuilder<'a>,
    ids: &mut BuilderIds,
    map: &'a SourceMap<'a>,
    token: Token,
    new_dst: (u32, u32),
) {
    let source_id = token
        .get_source_id()
        .and_then(|id| ids.source(builder, map, id));
    let name_id = token
        .get_name_id()
        .and_then(|id| ids.name(builder, map, id));
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
    builder: &mut SourceMapBuilder<'a>,
    ids: &mut BuilderIds,
    map: &'a SourceMap<'a>,
    lookup: &[&[Token]],
    abs: (u32, u32),
) -> Option<(u32, u32, u32)> {
    let resolved = map.lookup_token(lookup, abs.0, abs.1)?;
    let source_id = resolved.get_source_id()?;
    let mapped = ids.source(builder, map, source_id)?;
    Some((resolved.get_src_line(), resolved.get_src_col(), mapped))
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

    #[track_caller]
    fn assert_sources(map: &SourceMap<'_>, expected: &[&str]) {
        let got: Vec<&str> = map.get_sources().collect();
        assert_eq!(got, expected, "sources: expected {expected:?}, got {got:?}");
    }

    fn splice_style_preprocessed_from_another_file() -> (String, SourceMap<'static>) {
        let document = "<div>x</div>\n<style>.a{color:red}</style>";
        let region = Span::new(20, 33);

        let mut region_builder = SourceMapBuilder::default();
        let imported = region_builder.add_source_and_content("foo.scss", ".b{color:blue}");
        let component = region_builder.add_source_and_content("app.svelte", ".a{color:red}");
        region_builder.add_token(0, 0, 0, 0, Some(imported), None);
        region_builder.add_token(1, 0, 0, 0, Some(component), None);
        let region_map = region_builder.into_sourcemap();

        splice_region(
            document,
            None,
            region,
            ".b{color:blue}\n.a{color:red}",
            Some(&region_map),
            "app.svelte",
        )
    }

    #[test]
    fn keeps_preprocessor_source_that_is_not_the_component() {
        let (_, map) = splice_style_preprocessed_from_another_file();
        assert_position(&map, (1, 7), (0, 0, "foo.scss"));
    }

    #[test]
    fn lists_component_and_preprocessor_sources_side_by_side() {
        let (_, map) = splice_style_preprocessed_from_another_file();
        assert_sources(&map, &["app.svelte", "foo.scss"]);
    }

    #[test]
    fn still_offsets_region_tokens_that_point_at_the_component() {
        let (_, map) = splice_style_preprocessed_from_another_file();
        assert_position(&map, (2, 0), (1, 7, "app.svelte"));
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
        let region_source = region_builder.add_source_and_content("output.svelte", "js");
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
        let region_source = region_builder.add_source_and_content("output.svelte", "js");
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
