use super::*;

fn parse(source: &str) -> Component {
    let (component, diagnostics) = Parser::new(source).parse();
    assert!(
        diagnostics.is_empty(),
        "unexpected diagnostics: {diagnostics:?}"
    );
    component
}

/// Get a node reference from the component's root fragment by index.
fn node_at<'a>(c: &'a Component, index: usize) -> &'a Node {
    c.store.get(c.fragment.nodes[index])
}

/// Get a node reference from any fragment by index.
fn frag_node_at<'a>(c: &'a Component, fragment: &Fragment, index: usize) -> &'a Node {
    c.store.get(fragment.nodes[index])
}

fn assert_node(c: &Component, index: usize, expected: &str) {
    assert_eq!(c.source_text(node_at(c, index).span()), expected);
}

fn assert_script(c: &Component, expected: &str) {
    let script = c
        .instance_script
        .as_ref()
        .expect("expected instance script");
    assert_eq!(c.source_text(script.content_span), expected);
}

fn assert_if_block(c: &Component, index: usize, expected_test: &str) {
    if let Node::IfBlock(ref ib) = node_at(c, index) {
        assert_eq!(c.source_text(ib.test_span), expected_test);
    } else {
        panic!("expected IfBlock at index {index}");
    }
}

#[test]
fn smoke_text_and_element() {
    let c = parse("prefix <div>text</div>");
    assert_node(&c, 0, "prefix ");
    assert_node(&c, 1, "<div>text</div>");
}

#[test]
fn self_closed_element() {
    let c = parse("<img /><body><input/></body>");
    assert_node(&c, 0, "<img />");
    assert_node(&c, 1, "<body><input/></body>");
}

#[test]
fn interpolation() {
    let c = parse("{ id - 22 + 1 }");
    assert_node(&c, 0, "{ id - 22 + 1 }");
}

#[test]
fn if_block() {
    let c = parse("{#if true}<div>title</div>{/if}");
    assert_node(&c, 0, "{#if true}<div>title</div>{/if}");
    assert_if_block(&c, 0, "true");
}

#[test]
fn if_else_block() {
    let c = parse("{#if true}<div>title</div>{:else}<h1>big</h1>{/if}");
    assert_node(&c, 0, "{#if true}<div>title</div>{:else}<h1>big</h1>{/if}");
    assert_if_block(&c, 0, "true");
}

#[test]
fn if_elseif_else_block() {
    let c = parse("{#if false}one{:else if true}two{:else}three{/if}");
    assert_node(&c, 0, "{#if false}one{:else if true}two{:else}three{/if}");
    assert_if_block(&c, 0, "false");
}

#[test]
fn each_block() {
    let c = parse("{#each values as value}item: {value}{/each}");
    assert_node(&c, 0, "{#each values as value}item: {value}{/each}");
}

#[test]
#[ignore = "missing: parser support for item-less each blocks with index"]
fn each_block_no_item_with_index() {
    let c = parse("{#each { length: 8 }, rank}<div>{rank}</div>{/each}");
    assert_node(&c, 0, "{#each { length: 8 }, rank}<div>{rank}</div>{/each}");
}

#[test]
fn script_tag() {
    let c = parse("<script>const i = 10;</script>");
    assert_script(&c, "const i = 10;");
}

#[test]
fn script_tag_lang_ts() {
    let c = parse(r#"<script lang="ts">const i: number = 10;</script>"#);
    let script = c
        .instance_script
        .as_ref()
        .expect("expected instance script");
    assert_eq!(script.language, ScriptLanguage::TypeScript);
}

#[test]
fn comment() {
    let c = parse("<!-- some comment -->");
    assert_node(&c, 0, "<!-- some comment -->");
}

#[test]
fn element_with_attributes() {
    let c = parse(r#"<div lang="ts" disabled value={expr}>text</div>"#);
    assert_node(&c, 0, r#"<div lang="ts" disabled value={expr}>text</div>"#);
}

#[test]
fn nested_if_in_element() {
    let c = parse("<div>{#if true}inside{/if}</div>");
    assert_node(&c, 0, "<div>{#if true}inside{/if}</div>");
}

#[test]
fn unclosed_element_returns_diagnostic() {
    let (component, diagnostics) = Parser::new("<div>").parse();
    assert!(
        !diagnostics.is_empty(),
        "expected diagnostics for unclosed element"
    );
    // AST should still contain the auto-closed element
    assert_eq!(component.fragment.nodes.len(), 1);
    assert!(component
        .store
        .get(component.fragment.nodes[0])
        .is_element());
}

#[test]
fn multiple_roots() {
    let c = parse("<div>a</div><span>b</span>text");
    assert_node(&c, 0, "<div>a</div>");
    assert_node(&c, 1, "<span>b</span>");
    assert_node(&c, 2, "text");
}

#[test]
fn script_context_default() {
    let c = parse("<script>let x = 1;</script>");
    let script = c
        .instance_script
        .as_ref()
        .expect("expected instance script");
    assert_eq!(script.context, ScriptContext::Default);
}

#[test]
fn script_context_module_attribute() {
    let c = parse("<script module>let x = 1;</script>");
    let script = c.module_script.as_ref().expect("expected module script");
    assert_eq!(script.context, ScriptContext::Module);
}

#[test]
fn script_context_module_context_attribute() {
    let c = parse(r#"<script context="module">let x = 1;</script>"#);
    let script = c.module_script.as_ref().expect("expected module script");
    assert_eq!(script.context, ScriptContext::Module);
}

fn assert_snippet_block(
    c: &Component,
    index: usize,
    expected_name: &str,
    expected_expression: &str,
) {
    if let Node::SnippetBlock(ref sb) = node_at(c, index) {
        assert_eq!(sb.name(&c.source), expected_name);
        assert_eq!(c.source_text(sb.expression_span), expected_expression);
    } else {
        panic!("expected SnippetBlock at index {index}");
    }
}

fn assert_render_tag(c: &Component, index: usize, expected_expr: &str) {
    if let Node::RenderTag(ref rt) = node_at(c, index) {
        assert_eq!(c.source_text(rt.expression_span), expected_expr);
    } else {
        panic!("expected RenderTag at index {index}");
    }
}

#[test]
fn snippet_block_basic() {
    let c = parse("{#snippet greeting(name)}<p>Hello {name}</p>{/snippet}");
    assert_node(
        &c,
        0,
        "{#snippet greeting(name)}<p>Hello {name}</p>{/snippet}",
    );
    assert_snippet_block(&c, 0, "greeting", "greeting(name)");
}

#[test]
fn snippet_block_no_params() {
    let c = parse("{#snippet footer()}<p>footer</p>{/snippet}");
    assert_snippet_block(&c, 0, "footer", "footer()");
}

#[test]
fn snippet_block_multiple_params() {
    let c = parse("{#snippet card(title, body)}<div>{title} {body}</div>{/snippet}");
    assert_snippet_block(&c, 0, "card", "card(title, body)");
}

#[test]
fn render_tag_basic() {
    let c = parse("{@render greeting(message)}");
    assert_render_tag(&c, 0, "greeting(message)");
}

#[test]
fn render_tag_no_args() {
    let c = parse("{@render footer()}");
    assert_render_tag(&c, 0, "footer()");
}

#[test]
fn render_tag_multiple_args() {
    let c = parse("{@render card(title, body)}");
    assert_render_tag(&c, 0, "card(title, body)");
}

#[test]
fn snippet_and_render_together() {
    let c = parse("{#snippet greet(name)}<p>{name}</p>{/snippet}{@render greet(x)}");
    assert_snippet_block(&c, 0, "greet", "greet(name)");
    assert_render_tag(&c, 1, "greet(x)");
}

// --- HtmlTag tests ---

fn assert_html_tag(c: &Component, index: usize, expected_expr: &str) {
    if let Node::HtmlTag(ref ht) = node_at(c, index) {
        assert_eq!(c.source_text(ht.expression_span), expected_expr);
    } else {
        panic!("expected HtmlTag at index {index}");
    }
}

#[test]
fn html_tag_basic() {
    let c = parse("{@html content}");
    assert_html_tag(&c, 0, "content");
}

#[test]
fn html_tag_complex_expression() {
    let c = parse("{@html '<p>' + name + '</p>'}");
    assert_html_tag(&c, 0, "'<p>' + name + '</p>'");
}

// --- ConstTag tests ---

fn assert_const_tag(c: &Component, fragment: &Fragment, index: usize, expected_expr: &str) {
    if let Node::ConstTag(ref ct) = frag_node_at(c, fragment, index) {
        assert_eq!(c.source_text(ct.expression_span), expected_expr);
    } else {
        panic!("expected ConstTag at index {index}");
    }
}

#[test]
fn const_tag_basic() {
    let c = parse("{#each items as item}{@const doubled = item * 2}<p>{doubled}</p>{/each}");
    if let Node::EachBlock(ref eb) = node_at(&c, 0) {
        assert_const_tag(&c, &eb.body, 0, "doubled = item * 2");
    } else {
        panic!("expected EachBlock at index 0");
    }
}

#[test]
fn const_tag_in_if_block() {
    let c = parse("{#if show}{@const x = count + 1}<p>{x}</p>{/if}");
    if let Node::IfBlock(ref ib) = node_at(&c, 0) {
        assert_const_tag(&c, &ib.consequent, 0, "x = count + 1");
    } else {
        panic!("expected IfBlock at index 0");
    }
}

// --- KeyBlock tests ---

fn assert_key_block(c: &Component, index: usize, expected_expr: &str) {
    if let Node::KeyBlock(ref kb) = node_at(c, index) {
        assert_eq!(c.source_text(kb.expression_span), expected_expr);
    } else {
        panic!("expected KeyBlock at index {index}");
    }
}

#[test]
fn key_block_basic() {
    let c = parse("{#key count}<div>{count}</div>{/key}");
    assert_node(&c, 0, "{#key count}<div>{count}</div>{/key}");
    assert_key_block(&c, 0, "count");
}

#[test]
fn key_block_complex_expr() {
    let c = parse("{#key item.id}content{/key}");
    assert_key_block(&c, 0, "item.id");
}

// --- Escape sequence tests (Bug #1) ---

#[test]
fn interpolation_with_escaped_quotes() {
    let c = parse(r#"{ name.replace("\"", "'") }"#);
    assert_node(&c, 0, r#"{ name.replace("\"", "'") }"#);
}

// --- Style tag tests (Bug #2) ---

fn assert_css(c: &Component, expected_content: &str) {
    let css = c.css.as_ref().expect("expected css");
    assert_eq!(c.source_text(css.content_span), expected_content);
}

#[test]
fn style_tag() {
    let c = parse("<style>.foo { color: red; }</style>");
    assert_css(&c, ".foo { color: red; }");
}

#[test]
fn style_tag_with_selectors() {
    let c = parse("<style>a > b { color: red; }</style>");
    assert_css(&c, "a > b { color: red; }");
}

#[test]
fn style_tag_with_script() {
    let c = parse("<script>let x = 1;</script><style>.foo { color: red; }</style>");
    assert_script(&c, "let x = 1;");
    assert_css(&c, ".foo { color: red; }");
}

#[test]
fn duplicate_style_tag_returns_diagnostic() {
    let (_, diagnostics) = Parser::new("<style>a{}</style><style>b{}</style>").parse();
    assert!(
        !diagnostics.is_empty(),
        "expected diagnostic for duplicate style"
    );
}

// --- Each block key tests (Bug #3) ---

fn assert_each_block(c: &Component, index: usize, expected_expr: &str, expected_key: Option<&str>) {
    if let Node::EachBlock(ref eb) = node_at(c, index) {
        assert_eq!(c.source_text(eb.expression_span), expected_expr);
        let actual_key = eb.key_span.map(|s| c.source_text(s));
        assert_eq!(actual_key, expected_key);
    } else {
        panic!("expected EachBlock at index {index}");
    }
}

#[test]
fn each_block_with_key() {
    let c = parse("{#each items as item (item.id)}content{/each}");
    assert_each_block(&c, 0, "items", Some("item.id"));
}

#[test]
fn each_block_with_index_and_key() {
    let c = parse("{#each items as item, i (item.id)}content{/each}");
    assert_each_block(&c, 0, "items", Some("item.id"));
}

// --- Deep nesting tests ---

#[test]
fn deeply_nested_blocks() {
    let c = parse("{#if a}<div>{#each items as item}{#if b}inner{/if}{/each}</div>{/if}");
    assert_node(
        &c,
        0,
        "{#if a}<div>{#each items as item}{#if b}inner{/if}{/each}</div>{/if}",
    );
}

// --- Directive tests at parser level ---

#[test]
fn class_directive_on_element() {
    let c = parse("<div class:active={isActive}>text</div>");
    assert_node(&c, 0, "<div class:active={isActive}>text</div>");
    if let Node::Element(ref el) = node_at(&c, 0) {
        assert_eq!(el.attributes.len(), 1);
        assert!(matches!(
            el.attributes[0],
            svelte_ast::Attribute::ClassDirective(_)
        ));
    } else {
        panic!("expected Element");
    }
}

#[test]
fn bind_directive_on_element() {
    let c = parse("<input bind:value={name}/>");
    assert_node(&c, 0, "<input bind:value={name}/>");
    if let Node::Element(ref el) = node_at(&c, 0) {
        assert_eq!(el.attributes.len(), 1);
        assert!(matches!(
            el.attributes[0],
            svelte_ast::Attribute::BindDirective(_)
        ));
    } else {
        panic!("expected Element");
    }
}

#[test]
fn spread_attribute_on_element() {
    let c = parse("<div {...props}>text</div>");
    if let Node::Element(ref el) = node_at(&c, 0) {
        assert_eq!(el.attributes.len(), 1);
        assert!(matches!(
            el.attributes[0],
            svelte_ast::Attribute::SpreadAttribute(_)
        ));
    } else {
        panic!("expected Element");
    }
}

// --- Error recovery tests ---

fn parse_with_diagnostics(source: &str) -> (Component, Vec<Diagnostic>) {
    Parser::new(source).parse()
}

#[test]
fn recovery_unclosed_element_with_text() {
    let (c, diags) = parse_with_diagnostics("<div><span>text");
    assert!(!diags.is_empty());
    // Auto-closed: div contains span, span contains text
    assert_eq!(c.fragment.nodes.len(), 1);
    assert!(c.store.get(c.fragment.nodes[0]).is_element());
}

#[test]
fn recovery_mismatched_close_tag() {
    let (c, diags) = parse_with_diagnostics("<div></span></div>");
    assert!(!diags.is_empty());
    // div should still be properly closed
    assert_eq!(c.fragment.nodes.len(), 1);
}

#[test]
fn recovery_unclosed_if_block() {
    let (c, diags) = parse_with_diagnostics("{#if x}hello");
    assert!(!diags.is_empty());
    // Should produce an auto-closed IfBlock
    assert_eq!(c.fragment.nodes.len(), 1);
    assert!(c.store.get(c.fragment.nodes[0]).is_if_block());
}

#[test]
fn recovery_multiple_errors() {
    let (c, diags) = parse_with_diagnostics("<div><span>");
    assert!(
        diags.len() >= 2,
        "expected multiple diagnostics, got {}",
        diags.len()
    );
    // Both unclosed elements should be auto-closed
    assert_eq!(c.fragment.nodes.len(), 1);
}

#[test]
fn recovery_empty_input() {
    let (c, diags) = parse_with_diagnostics("");
    assert!(diags.is_empty());
    assert!(c.fragment.nodes.is_empty());
}

#[test]
fn recovery_text_only() {
    let (c, diags) = parse_with_diagnostics("just text");
    assert!(diags.is_empty());
    assert_eq!(c.fragment.nodes.len(), 1);
    assert!(c.store.get(c.fragment.nodes[0]).is_text());
}

#[test]
fn recovery_close_tag_no_matching_open() {
    let (c, diags) = parse_with_diagnostics("</div>");
    assert!(!diags.is_empty());
    // Error node for the orphan close tag
    assert_eq!(c.fragment.nodes.len(), 1);
    assert!(matches!(c.store.get(c.fragment.nodes[0]), Node::Error(_)));
}

#[test]
fn dual_scripts_instance_and_module() {
    let c = parse("<script module>export let x = 1;</script><script>let y = 2;</script>");
    assert!(c.module_script.is_some());
    assert!(c.instance_script.is_some());
    let ms = c.module_script.as_ref().unwrap();
    assert_eq!(ms.context, ScriptContext::Module);
    let is_ = c.instance_script.as_ref().unwrap();
    assert_eq!(is_.context, ScriptContext::Default);
}

#[test]
fn dual_scripts_module_then_instance() {
    // Order does not matter — module goes to module_script, instance to instance_script.
    let c = parse("<script>let a = 1;</script><script module>export let b = 2;</script>");
    assert!(c.instance_script.is_some());
    assert!(c.module_script.is_some());
}

#[test]
fn recovery_duplicate_script_continues() {
    let (c, diags) = parse_with_diagnostics(
        "<script>let a = 1;</script><script>let b = 2;</script><div>ok</div>",
    );
    assert!(!diags.is_empty());
    // First instance script is kept, second is skipped, div is parsed
    assert!(c.instance_script.is_some());
    assert_eq!(c.fragment.nodes.len(), 1);
    assert!(c.store.get(c.fragment.nodes[0]).is_element());
}

#[test]
fn recovery_duplicate_module_script_continues() {
    let (c, diags) = parse_with_diagnostics(
        "<script module>let a = 1;</script><script module>let b = 2;</script><div>ok</div>",
    );
    assert!(!diags.is_empty());
    assert!(c.module_script.is_some());
    assert!(c.instance_script.is_none());
    assert_eq!(c.fragment.nodes.len(), 1);
}

#[test]
fn recovery_unclosed_each_block() {
    let (c, diags) = parse_with_diagnostics("{#each items as item}content");
    assert!(!diags.is_empty());
    assert_eq!(c.fragment.nodes.len(), 1);
}

fn assert_element(c: &Component, index: usize, name: &str, self_closing: bool) {
    if let Node::Element(ref el) = node_at(c, index) {
        assert_eq!(el.name, name, "expected element name '{name}'");
        assert_eq!(
            el.self_closing, self_closing,
            "expected self_closing={self_closing} for <{name}>"
        );
    } else {
        panic!("expected Element at index {index}");
    }
}

#[test]
fn void_element_without_slash() {
    let c = parse("<input>");
    assert_element(&c, 0, "input", true);
}

#[test]
fn void_element_with_slash() {
    let c = parse("<input />");
    assert_element(&c, 0, "input", true);
}

#[test]
fn void_element_with_attributes() {
    let c = parse(r#"<input type="text">"#);
    assert_element(&c, 0, "input", true);
}

#[test]
fn void_element_br() {
    let c = parse("<br>");
    assert_element(&c, 0, "br", true);
}

#[test]
fn void_element_img() {
    let c = parse(r#"<img src="x.png">"#);
    assert_element(&c, 0, "img", true);
}

#[test]
fn void_element_closing_tag_error() {
    let (_, diags) = parse_with_diagnostics("</input>");
    assert!(!diags.is_empty());
    assert!(diags
        .iter()
        .any(|d| d.kind == svelte_diagnostics::DiagnosticKind::VoidElementInvalidContent));
}

#[test]
fn void_element_multiple() {
    let c = parse("<input><br><hr>");
    assert_eq!(c.fragment.nodes.len(), 3);
    assert_element(&c, 0, "input", true);
    assert_element(&c, 1, "br", true);
    assert_element(&c, 2, "hr", true);
}

// --- <svelte:options> tests ---

fn assert_options_runes(c: &Component, expected: bool) {
    let opts = c.options.as_ref().expect("expected svelte:options");
    assert_eq!(opts.runes, Some(expected), "expected runes={expected}");
}

fn assert_options_namespace(c: &Component, expected: svelte_ast::Namespace) {
    let opts = c.options.as_ref().expect("expected svelte:options");
    assert_eq!(
        opts.namespace,
        Some(expected),
        "expected namespace={expected:?}"
    );
}

fn assert_options_css(c: &Component, expected: svelte_ast::CssMode) {
    let opts = c.options.as_ref().expect("expected svelte:options");
    assert_eq!(opts.css, Some(expected), "expected css={expected:?}");
}

fn assert_options_custom_element_tag(c: &Component, expected_tag: &str) {
    let opts = c.options.as_ref().expect("expected svelte:options");
    match &opts.custom_element {
        Some(svelte_ast::CustomElementConfig::Tag(tag)) => {
            assert_eq!(tag, expected_tag, "expected customElement tag");
        }
        _ => panic!("expected CustomElementConfig::Tag"),
    }
}

fn assert_options_custom_element_expression(c: &Component) {
    let opts = c.options.as_ref().expect("expected svelte:options");
    assert!(
        matches!(
            &opts.custom_element,
            Some(svelte_ast::CustomElementConfig::Expression(_))
        ),
        "expected CustomElementConfig::Expression"
    );
}

fn assert_no_options(c: &Component) {
    assert!(c.options.is_none(), "expected no svelte:options");
}

#[test]
fn svelte_options_runes_true() {
    let c = parse("<svelte:options runes={true} />");
    assert_options_runes(&c, true);
    assert!(
        c.fragment.is_empty(),
        "svelte:options should be removed from fragment"
    );
}

#[test]
fn svelte_options_runes_false() {
    let c = parse("<svelte:options runes={false} />");
    assert_options_runes(&c, false);
}

#[test]
fn svelte_options_namespace_svg() {
    let c = parse(r#"<svelte:options namespace="svg" />"#);
    assert_options_namespace(&c, svelte_ast::Namespace::Svg);
}

#[test]
fn svelte_options_namespace_mathml() {
    let c = parse(r#"<svelte:options namespace="mathml" />"#);
    assert_options_namespace(&c, svelte_ast::Namespace::Mathml);
}

#[test]
fn svelte_options_namespace_html() {
    let c = parse(r#"<svelte:options namespace="html" />"#);
    assert_options_namespace(&c, svelte_ast::Namespace::Html);
}

#[test]
fn svelte_options_css_injected() {
    let c = parse(r#"<svelte:options css="injected" />"#);
    assert_options_css(&c, svelte_ast::CssMode::Injected);
}

#[test]
fn svelte_options_custom_element_string() {
    let c = parse(r#"<svelte:options customElement="my-element" />"#);
    assert_options_custom_element_tag(&c, "my-element");
}

#[test]
fn svelte_options_custom_element_object() {
    let c = parse(r#"<svelte:options customElement={{ tag: "my-element", shadow: "open" }} />"#);
    assert_options_custom_element_expression(&c);
}

#[test]
fn svelte_options_preserve_whitespace() {
    let c = parse("<svelte:options preserveWhitespace />");
    let opts = c.options.as_ref().expect("expected svelte:options");
    assert_eq!(opts.preserve_whitespace, Some(true));
}

#[test]
fn svelte_options_immutable() {
    let c = parse("<svelte:options immutable={true} />");
    let opts = c.options.as_ref().expect("expected svelte:options");
    assert_eq!(opts.immutable, Some(true));
}

#[test]
fn svelte_options_accessors() {
    let c = parse("<svelte:options accessors={true} />");
    let opts = c.options.as_ref().expect("expected svelte:options");
    assert_eq!(opts.accessors, Some(true));
}

#[test]
fn svelte_options_multiple_attributes() {
    let c = parse(r#"<svelte:options runes={true} namespace="svg" />"#);
    assert_options_runes(&c, true);
    assert_options_namespace(&c, svelte_ast::Namespace::Svg);
}

#[test]
fn svelte_options_with_content() {
    let c = parse("<svelte:options runes={true} />\n<p>Hello</p>");
    assert_options_runes(&c, true);
    // <p> should be in the fragment, svelte:options should not
    assert_eq!(c.fragment.nodes.len(), 2); // newline text + <p>
}

#[test]
fn svelte_options_removed_from_fragment() {
    let c = parse("<svelte:options runes={true} />");
    assert!(c.options.is_some());
    assert!(
        c.fragment.is_empty(),
        "svelte:options must be removed from fragment"
    );
}

#[test]
fn no_svelte_options() {
    let c = parse("<p>Hello</p>");
    assert_no_options(&c);
}

#[test]
fn svelte_options_unknown_attribute_diagnostic() {
    let (_, diags) = parse_with_diagnostics(r#"<svelte:options foo="bar" />"#);
    assert!(!diags.is_empty());
    assert!(diags.iter().any(|d| matches!(
        &d.kind,
        svelte_diagnostics::DiagnosticKind::SvelteOptionsUnknownAttribute(name) if name == "foo"
    )));
}

#[test]
fn svelte_options_invalid_namespace_diagnostic() {
    let (_, diags) = parse_with_diagnostics(r#"<svelte:options namespace="invalid" />"#);
    assert!(!diags.is_empty());
    assert!(diags.iter().any(|d| matches!(
        &d.kind,
        svelte_diagnostics::DiagnosticKind::SvelteOptionsInvalidAttributeValue(_)
    )));
}

#[test]
fn svelte_options_invalid_css_diagnostic() {
    let (_, diags) = parse_with_diagnostics(r#"<svelte:options css="external" />"#);
    assert!(!diags.is_empty());
}

#[test]
fn svelte_options_no_children_diagnostic() {
    let (_, diags) =
        parse_with_diagnostics("<svelte:options runes={true}><p>child</p></svelte:options>");
    assert!(!diags.is_empty());
    assert!(diags.iter().any(|d| matches!(
        &d.kind,
        svelte_diagnostics::DiagnosticKind::SvelteOptionsNoChildren
    )));
}

#[test]
fn svelte_options_invalid_custom_element_tag_diagnostic() {
    let (_, diags) = parse_with_diagnostics(r#"<svelte:options customElement="NoHyphen" />"#);
    assert!(!diags.is_empty());
}

#[test]
fn svelte_options_reserved_tag_name_diagnostic() {
    let (_, diags) = parse_with_diagnostics(r#"<svelte:options customElement="font-face" />"#);
    assert!(!diags.is_empty());
    assert!(diags.iter().any(|d| matches!(
        &d.kind,
        svelte_diagnostics::DiagnosticKind::SvelteOptionsReservedTagName
    )));
}

#[test]
fn svelte_options_custom_element_null_compat() {
    // null is backwards compat from Svelte 4 — should not error
    let c = parse("<svelte:options customElement={null} />");
    let opts = c.options.as_ref().expect("expected svelte:options");
    assert!(opts.custom_element.is_none());
}

#[test]
fn svelte_options_namespace_svg_uri() {
    let c = parse(r#"<svelte:options namespace="http://www.w3.org/2000/svg" />"#);
    assert_options_namespace(&c, svelte_ast::Namespace::Svg);
}

#[test]
fn svelte_options_deprecated_tag_diagnostic() {
    let (_, diags) = parse_with_diagnostics(r#"<svelte:options tag="my-element" />"#);
    assert!(!diags.is_empty());
    assert!(diags.iter().any(|d| matches!(
        &d.kind,
        svelte_diagnostics::DiagnosticKind::SvelteOptionsDeprecatedTag
    )));
    // Should be a warning, not an error
    assert!(diags
        .iter()
        .any(|d| d.severity == svelte_diagnostics::Severity::Warning));
}

#[test]
fn svelte_options_invalid_tag_not_stored() {
    let (c, diags) = parse_with_diagnostics(r#"<svelte:options customElement="NoHyphen" />"#);
    assert!(!diags.is_empty());
    let opts = c.options.as_ref().expect("expected svelte:options");
    assert!(
        opts.custom_element.is_none(),
        "invalid tag should not be stored"
    );
}

#[test]
fn svelte_options_preserves_attributes() {
    let c = parse(r#"<svelte:options runes={true} namespace="svg" />"#);
    let opts = c.options.as_ref().expect("expected svelte:options");
    assert_eq!(opts.attributes.len(), 2);
}

// --- DebugTag tests ---

fn assert_debug_tag(c: &Component, fragment: &Fragment, index: usize, expected_ids: &[&str]) {
    if let Node::DebugTag(ref dt) = frag_node_at(c, fragment, index) {
        let actual: Vec<&str> = dt.identifiers.iter().map(|s| c.source_text(*s)).collect();
        assert_eq!(actual, expected_ids);
    } else {
        panic!("expected DebugTag at index {index}");
    }
}

#[test]
fn debug_tag_empty() {
    let c = parse("{@debug}");
    assert_debug_tag(&c, &c.fragment, 0, &[]);
}

#[test]
fn debug_tag_single() {
    let c = parse("{@debug x}");
    assert_debug_tag(&c, &c.fragment, 0, &["x"]);
}

#[test]
fn debug_tag_multiple() {
    let c = parse("{@debug x, y, z}");
    assert_debug_tag(&c, &c.fragment, 0, &["x", "y", "z"]);
}

#[test]
fn debug_tag_member_expression_error() {
    let (_, diagnostics) = Parser::new("{@debug x.y}").parse();
    assert!(
        !diagnostics.is_empty(),
        "expected diagnostic for member expression in debug tag"
    );
}

#[test]
fn debug_tag_call_expression_error() {
    let (_, diagnostics) = Parser::new("{@debug fn()}").parse();
    assert!(
        !diagnostics.is_empty(),
        "expected diagnostic for call expression in debug tag"
    );
}

// --- AwaitBlock diagnostic tests ---

#[test]
fn await_duplicate_then_clause() {
    let (_, diags) = parse_with_diagnostics("{#await p}{:then a}text{:then b}more{/await}");
    assert!(
        diags
            .iter()
            .any(|d| d.kind.code() == "block_duplicate_clause"),
        "expected block_duplicate_clause, got: {diags:?}"
    );
}

#[test]
fn await_duplicate_catch_clause() {
    let (_, diags) = parse_with_diagnostics("{#await p}{:catch e}err{:catch e2}err2{/await}");
    assert!(
        diags
            .iter()
            .any(|d| d.kind.code() == "block_duplicate_clause"),
        "expected block_duplicate_clause, got: {diags:?}"
    );
}

#[test]
fn await_valid_then_catch_no_duplicate() {
    let (_, diags) = parse_with_diagnostics("{#await p}{:then a}ok{:catch e}err{/await}");
    assert!(
        !diags
            .iter()
            .any(|d| d.kind.code() == "block_duplicate_clause"),
        "unexpected block_duplicate_clause: {diags:?}"
    );
}

#[test]
fn await_duplicate_then_after_catch_then() {
    // {:then}{:catch}{:then} — the third clause triggers the then_children.is_some() branch
    let (_, diags) =
        parse_with_diagnostics("{#await p}{:then a}t{:catch e}err{:then b}more{/await}");
    assert!(
        diags
            .iter()
            .any(|d| d.kind.code() == "block_duplicate_clause"),
        "expected block_duplicate_clause, got: {diags:?}"
    );
}

#[test]
fn await_catch_before_then_no_panic() {
    // {:catch} before {:then} with no prior {:then} must not panic
    let (_, diags) = parse_with_diagnostics("{#await p}{:catch e}err{:then a}ok{/await}");
    assert!(
        !diags
            .iter()
            .any(|d| d.kind.code() == "block_duplicate_clause"),
        "unexpected block_duplicate_clause for out-of-order catch/then: {diags:?}"
    );
}

// ---------------------------------------------------------------------------
// JS parsing tests (moved from svelte_types)
// ---------------------------------------------------------------------------

mod js_parse_tests {
    use oxc_allocator::Allocator;

    #[test]
    fn parse_script_basic() {
        let alloc = Allocator::default();
        let source = "let count = $state(0); const name = 'test';";
        let arena_source = alloc.alloc_str(source);
        let program =
            crate::parse_js::parse_script_with_alloc(&alloc, arena_source, 0, false).unwrap();
        // Script parses without error; detailed ScriptInfo extraction tested in svelte_analyze
        assert!(!program.body.is_empty());
    }

    #[test]
    fn parse_script_exports() {
        let alloc = Allocator::default();
        let source = "export const PI = 3.14; export function greet(name) { return name; }";
        let arena_source = alloc.alloc_str(source);
        let program =
            crate::parse_js::parse_script_with_alloc(&alloc, arena_source, 0, false).unwrap();
        assert!(!program.body.is_empty());
    }
}

// ---------------------------------------------------------------------------
// attribute_duplicate
// ---------------------------------------------------------------------------

fn parse_with_diags(source: &str) -> Vec<svelte_diagnostics::Diagnostic> {
    Parser::new(source).parse().1
}

#[test]
fn attribute_duplicate_same_name() {
    let diags = parse_with_diags(r#"<div foo="a" foo="b"></div>"#);
    assert!(
        diags.iter().any(|d| d.kind.code() == "attribute_duplicate"),
        "expected attribute_duplicate, got {diags:?}"
    );
}

#[test]
fn attribute_duplicate_bind_and_attr() {
    // BindDirective and HTMLAttribute share the "attr" key space.
    let diags = parse_with_diags(r#"<input bind:value={x} value="y" />"#);
    assert!(
        diags.iter().any(|d| d.kind.code() == "attribute_duplicate"),
        "expected attribute_duplicate, got {diags:?}"
    );
}

#[test]
fn attribute_duplicate_this_excluded() {
    // Two `this` attributes must NOT fire attribute_duplicate (excluded per reference).
    let diags = parse_with_diags(r#"<svelte:element this="div" this="span"></svelte:element>"#);
    assert!(
        !diags.iter().any(|d| d.kind.code() == "attribute_duplicate"),
        "unexpected attribute_duplicate"
    );
}

#[test]
fn attribute_duplicate_style_directive() {
    let diags = parse_with_diags(r#"<div style:color="red" style:color="blue"></div>"#);
    assert!(
        diags.iter().any(|d| d.kind.code() == "attribute_duplicate"),
        "expected attribute_duplicate, got {diags:?}"
    );
}
