import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	var div = root();
	$.attach(div, () => (node) => node.style.opacity = .5);
	$.append($$anchor, div);
}
