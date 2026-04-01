import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor) {
	let name = "Tom";
	var p = root();
	p.textContent = "& Tom <";
	$.append($$anchor, p);
}
