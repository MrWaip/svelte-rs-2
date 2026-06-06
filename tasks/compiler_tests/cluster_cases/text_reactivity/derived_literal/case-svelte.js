import * as $ from "svelte/internal/client";
var root = $.from_html(`<h1></h1>`);
export default function App($$anchor) {
	const x = $.derived(() => 5);
	var h1 = root();
	h1.textContent = $.get(x);
	$.append($$anchor, h1);
}
