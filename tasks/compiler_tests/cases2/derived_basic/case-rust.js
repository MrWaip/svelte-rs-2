import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor) {
	let count = 0;
	let doubled = $.derived(() => count * 2);
	var p = root();
	p.textContent = $.get(doubled);
	$.append($$anchor, p);
}
