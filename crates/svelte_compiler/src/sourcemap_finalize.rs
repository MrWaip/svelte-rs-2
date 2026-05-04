use svelte_sourcemap::{JsOutput, SourceMap, Sourcemap, get_basename, get_source_name};

use crate::CompileOptions;

pub fn finalize_js(out: JsOutput, options: &CompileOptions, source: &str) -> JsOutput {
    let JsOutput { code, map } = out;
    let Some(map) = map else {
        return JsOutput { code, map: None };
    };

    let source_name = get_source_name(&options.filename, options.output_filename.as_deref());
    let mut sm = Sourcemap::new(map, source);
    sm.attach_sources_content();
    sm.set_source_name(&source_name);

    JsOutput {
        code,
        map: Some(sm.into_inner()),
    }
}

pub fn finalize_css(mut map: SourceMap, css_output_filename: Option<&str>, filename: &str) -> SourceMap {
    let target = css_output_filename.unwrap_or(filename);
    map.set_file(target);
    map
}

pub fn finalize_module_js(out: JsOutput, filename: &str, source: &str) -> JsOutput {
    let JsOutput { code, map } = out;
    let Some(map) = map else {
        return JsOutput { code, map: None };
    };

    let source_name = if filename.is_empty() || filename == "(unknown)" {
        "input.svelte.js".to_string()
    } else {
        get_basename(filename).to_string()
    };
    let mut sm = Sourcemap::new(map, source);
    sm.attach_sources_content();
    sm.set_source_name(&source_name);

    JsOutput {
        code,
        map: Some(sm.into_inner()),
    }
}
