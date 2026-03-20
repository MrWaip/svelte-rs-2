import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor) {
	let value = "hello";
	var p = root();
	p.textContent = "hello";
	$.append($$anchor, p);
}
