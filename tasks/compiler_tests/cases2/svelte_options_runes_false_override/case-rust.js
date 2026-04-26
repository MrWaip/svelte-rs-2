import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor, $$props) {
	let count = 1;
	var p = root();
	p.textContent = count;
	$.append($$anchor, p);
}
