import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<template id="t1"><div>foo</div></template>`);
export default function App($$anchor) {
	var template = root();
	$.append($$anchor, template);
}
