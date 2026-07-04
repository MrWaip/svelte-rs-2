import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor) {
	let num = 42;
	var p = root();
	p.textContent = "42";
	$.append($$anchor, p);
}
