use svelte_sourcemap::{JsOutput, SourceMap, Sourcemap};

pub fn finalize_js(
    out: JsOutput,
    source: &str,
    filename: &str,
    source_name: &str,
    preprocessor: Option<&SourceMap>,
) -> JsOutput {
    let JsOutput { code, map } = out;
    let Some(map) = map else {
        return JsOutput { code, map: None };
    };
    let map = match preprocessor {
        Some(preprocessor) => svelte_sourcemap::merge_with_preprocessor(
            map,
            preprocessor,
            filename,
            source_name,
            (0, 0),
        ),
        None => {
            let mut sm = Sourcemap::new(map, source);
            sm.attach_sources_content();
            sm.set_sources_name(source_name);
            sm.into_inner()
        }
    };
    JsOutput {
        code,
        map: Some(map),
    }
}
