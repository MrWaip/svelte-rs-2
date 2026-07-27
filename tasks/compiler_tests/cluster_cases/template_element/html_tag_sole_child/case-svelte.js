import * as $ from "svelte/internal/client";
var root = $.from_html(`<template></template>`);
export default function App($$anchor, $$props) {
	var template = root();
	$.hydrate_template(template);
	$.html(template, () => $$props.s, true);
	$.reset(template);
	$.append($$anchor, template);
}
