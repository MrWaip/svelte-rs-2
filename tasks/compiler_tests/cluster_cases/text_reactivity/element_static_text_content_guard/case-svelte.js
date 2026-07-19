import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor) {
	let label = "hi";
	var p = root();
	p.textContent = "hi";
	$.append($$anchor, p);
}
