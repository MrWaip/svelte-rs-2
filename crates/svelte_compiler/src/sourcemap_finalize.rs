use svelte_sourcemap::{JsOutput, SourceMap, Sourcemap};

pub fn finalize_js(out: JsOutput, source: &str, source_name: &str) -> JsOutput {
    let JsOutput { code, map } = out;
    let Some(map) = map else {
        return JsOutput { code, map: None };
    };
    let mut sm = Sourcemap::new(map, source);
    sm.attach_sources_content();
    sm.set_source_name(source_name);
    JsOutput {
        code,
        map: Some(sm.into_inner()),
    }
}

pub fn finalize_css(mut map: SourceMap, target: &str) -> SourceMap {
    map.set_file(target);
    map
}
