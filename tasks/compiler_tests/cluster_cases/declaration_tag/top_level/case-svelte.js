import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor) {
	const greeting = "hi";
	var p = root();
	p.textContent = "hi";
	$.append($$anchor, p);
}
