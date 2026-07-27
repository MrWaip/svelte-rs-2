import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<template><p><!></p></template>`);
export default function App($$anchor, $$props) {
	var template = root();
	$.hydrate_template(template);
	var p = $.child(template.content);
	var node = $.child(p);
	$.slot(node, $$props, "default", {}, null);
	$.reset(p);
	$.reset(template);
	$.append($$anchor, template);
}
