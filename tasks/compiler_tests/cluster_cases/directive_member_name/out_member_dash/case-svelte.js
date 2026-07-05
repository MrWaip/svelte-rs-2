import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	const fn = (node, options) => ({});
	let a = { b: { "c-d": fn } };
	let directive = $.derived(() => a);
	var div = root();
	$.transition(2, div, () => $.get(directive).b["c-d"]);
	$.append($$anchor, div);
}
