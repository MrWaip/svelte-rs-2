import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor) {
	const r = /ab/;
	var p = root();
	p.textContent = "/ab/ object x/ab/";
	$.append($$anchor, p);
}
