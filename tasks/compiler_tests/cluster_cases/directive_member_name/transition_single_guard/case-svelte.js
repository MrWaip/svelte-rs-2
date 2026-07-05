import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	const fade = (node, options) => ({});
	var div = root();
	$.transition(3, div, () => fade);
	$.append($$anchor, div);
}
