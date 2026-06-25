import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor) {
	let s = "x";
	var p = root();
	p.textContent = "v x";
	$.append($$anchor, p);
}
