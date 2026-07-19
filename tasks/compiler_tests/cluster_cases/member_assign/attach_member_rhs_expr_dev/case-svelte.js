import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	var div = root();
	$.attach(div, () => (node) => node.textContent = node.nodeName);
	$.append($$anchor, div);
}
