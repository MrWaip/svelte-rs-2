use svelte_ast::Namespace;

pub(crate) fn from_namespace(namespace: Namespace) -> &'static str {
    match namespace {
        Namespace::Html => "$.from_html",
        Namespace::Svg => "$.from_svg",
        Namespace::Mathml => "$.from_mathml",
    }
}
