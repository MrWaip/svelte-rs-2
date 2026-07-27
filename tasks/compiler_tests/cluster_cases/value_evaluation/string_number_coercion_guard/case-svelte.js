import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor) {
	let d = 2;
	var p = root();
	p.textContent = "3 3 0 5 ab true";
	$.append($$anchor, p);
}
