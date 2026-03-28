import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor) {
	let value = 0;
	var p = root();
	p.textContent = "0";
	$.append($$anchor, p);
}
