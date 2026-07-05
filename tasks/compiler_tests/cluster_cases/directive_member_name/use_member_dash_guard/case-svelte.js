import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	const fn = (node, options) => ({});
	let a = { b: { "c-d": fn } };
	let directive = $.derived(() => a);
	var div = root();
	$.action(div, ($$node) => $.get(directive).b["c-d"]?.($$node));
	$.append($$anchor, div);
}
