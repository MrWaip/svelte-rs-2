import * as $ from "svelte/internal/client";
var root = $.from_html(`<template shadowrootmode="open"><p> </p></template>`);
export default function App($$anchor, $$props) {
	var template = root();
	$.hydrate_template(template);
	var p = $.child(template.content);
	var text = $.child(p, true);
	$.reset(p);
	$.reset(template);
	$.template_effect(() => $.set_text(text, $$props.x));
	$.append($$anchor, template);
}
