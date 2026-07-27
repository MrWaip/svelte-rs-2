import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<template><p><!></p></template>`), App[$.FILENAME], [[
	1,
	0,
	[[1, 10]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var template = root();
	$.hydrate_template(template);
	var p = $.child(template.content);
	var node = $.child(p);
	$.slot(node, $$props, "default", {}, null);
	$.reset(p);
	$.reset(template);
	$.append($$anchor, template);
	return $.pop($$exports);
}
