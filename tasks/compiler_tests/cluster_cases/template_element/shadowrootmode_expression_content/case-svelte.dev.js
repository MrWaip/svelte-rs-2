App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<template shadowrootmode="open"><p> </p></template>`), App[$.FILENAME], [[
	5,
	0,
	[[5, 32]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var template = root();
	$.hydrate_template(template);
	var p = $.child(template.content);
	var text = $.child(p, true);
	$.reset(p);
	$.reset(template);
	$.template_effect(() => $.set_text(text, $$props.x));
	$.append($$anchor, template);
	return $.pop($$exports);
}
