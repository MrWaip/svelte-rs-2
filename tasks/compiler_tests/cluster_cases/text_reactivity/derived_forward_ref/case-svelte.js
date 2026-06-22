import * as $ from "svelte/internal/client";
var root = $.from_html(`<h1></h1>`);
export default function App($$anchor) {
	const a = $.derived(() => $.get(b));
	const b = $.derived(() => 5);
	var h1 = root();
	h1.textContent = $.get(a);
	$.append($$anchor, h1);
}
