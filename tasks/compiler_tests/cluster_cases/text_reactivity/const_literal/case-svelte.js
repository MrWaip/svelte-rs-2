import * as $ from "svelte/internal/client";
var root = $.from_html(`<h1></h1>`);
export default function App($$anchor) {
	const x = "lit";
	var h1 = root();
	h1.textContent = "lit";
	$.append($$anchor, h1);
}
