import * as $ from "svelte/internal/client";
var root = $.from_html(`<h1></h1>`);
export default function App($$anchor) {
	let s = 0;
	const x = $.derived(() => s + 1);
	var h1 = root();
	h1.textContent = $.get(x);
	$.append($$anchor, h1);
}
