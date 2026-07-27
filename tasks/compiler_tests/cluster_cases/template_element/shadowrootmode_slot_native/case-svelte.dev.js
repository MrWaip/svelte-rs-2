import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<template shadowrootmode="open"><slot></slot></template>`), App[$.FILENAME], [[
	1,
	0,
	[[1, 32]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var template = root();
	$.append($$anchor, template);
	return $.pop($$exports);
}
