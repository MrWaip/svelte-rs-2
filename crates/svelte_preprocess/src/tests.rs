use pretty_assertions::assert_eq;

use crate::{PreprocessOptions, Preprocessed, preprocess_style};

fn preprocess(source: &str) -> Preprocessed {
    preprocess_style(
        source,
        None,
        &PreprocessOptions {
            filename: "App.svelte".to_string(),
            ..PreprocessOptions::default()
        },
    )
}

fn preprocess_with_targets(source: &str, targets: &[&str]) -> Preprocessed {
    preprocess_style(
        source,
        None,
        &PreprocessOptions {
            filename: "App.svelte".to_string(),
            css_targets: targets.iter().map(|query| (*query).to_string()).collect(),
            ..PreprocessOptions::default()
        },
    )
}

fn preprocess_with_prepend(source: &str, prepend: &str) -> Preprocessed {
    preprocess_style(
        source,
        None,
        &PreprocessOptions {
            filename: "App.svelte".to_string(),
            style_prepend: Some(prepend.to_string()),
            ..PreprocessOptions::default()
        },
    )
}

#[track_caller]
fn assert_style_content(result: &Preprocessed, expected_style: &str) {
    let start = result
        .code
        .find('>')
        .map(|index| index + 1)
        .unwrap_or_default();
    let end = result.code.find("</style>").unwrap_or(result.code.len());
    let got = result.code[start..end].trim();
    assert_eq!(
        result.diagnostics.len(),
        0,
        "diagnostics: expected none, got {:?}",
        result.diagnostics
    );
    assert_eq!(
        got, expected_style,
        "style content: expected {expected_style:?}, got {got:?}"
    );
}

#[track_caller]
fn assert_unchanged(result: &Preprocessed, source: &str) {
    assert_eq!(
        result.code, source,
        "code: expected untouched source {source:?}, got {:?}",
        result.code
    );
    assert_eq!(
        result.map.is_none(),
        true,
        "map: expected none for untouched source"
    );
}

#[track_caller]
fn assert_style_failure(result: &Preprocessed, expected_code: &str) {
    let codes: Vec<&str> = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.kind.code())
        .collect();
    assert_eq!(
        codes,
        vec![expected_code],
        "diagnostic codes: expected {expected_code:?}, got {codes:?}"
    );
}

#[test]
fn compiles_scss_variables_into_css() {
    let result = preprocess("<style lang=\"scss\">$c: red; a { color: $c; }</style>");
    assert_style_content(&result, "a {\n  color: red;\n}");
}

#[test]
fn compiles_scss_nesting_into_flat_selectors() {
    let result = preprocess("<style lang=\"scss\">a { b { color: red; } }</style>");
    assert_style_content(&result, "a b {\n  color: red;\n}");
}

#[test]
fn leaves_plain_css_untouched_without_targets() {
    let source = "<style>a { color: red; }</style>";
    let result = preprocess(source);
    assert_unchanged(&result, source);
}

#[test]
fn leaves_component_without_style_block_untouched() {
    let source = "<p>hello</p>";
    let result = preprocess(source);
    assert_unchanged(&result, source);
}

#[test]
fn reports_diagnostic_when_scss_is_invalid() {
    let result = preprocess("<style lang=\"scss\">a { color: $missing; }</style>");
    assert_style_failure(&result, "style_preprocess_failed");
}

#[test]
fn applies_vendor_prefixes_for_css_targets() {
    let result = preprocess_with_targets(
        "<style lang=\"scss\">a { user-select: none; }</style>",
        &["safari >= 12"],
    );
    assert_style_content(
        &result,
        "a {\n  -webkit-user-select: none;\n  user-select: none;\n}",
    );
}

#[test]
fn style_prepend_provides_mixins_to_component_style() {
    let result = preprocess_with_prepend(
        r#"<style lang="scss">.a { @include red; }</style>"#,
        "@mixin red { color: red; }",
    );

    assert_style_content(&result, ".a {\n  color: red;\n}");
}

#[test]
fn style_prepend_is_ignored_without_style_language() {
    let result = preprocess_with_prepend("<style>.a { color: red; }</style>", "@mixin red {}");

    assert_unchanged(&result, "<style>.a { color: red; }</style>");
}

#[test]
fn relative_import_resolves_from_component_directory() {
    let result = preprocess_style(
        r#"<style lang="scss">.a { color: red; }</style>"#,
        None,
        &PreprocessOptions {
            filename: "crates/svelte_preprocess/src/App.svelte".to_string(),
            ..PreprocessOptions::default()
        },
    );

    assert_style_content(&result, ".a {\n  color: red;\n}");
}
