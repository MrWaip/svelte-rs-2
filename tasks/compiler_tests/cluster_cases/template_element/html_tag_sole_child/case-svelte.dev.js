App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<template></template>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var template = root();
	$.hydrate_template(template);
	$.html(template, () => $$props.s, true);
	$.reset(template);
	$.append($$anchor, template);
	return $.pop($$exports);
}
