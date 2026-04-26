import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor, $$props) {
	let foo;
	var p = root();
	p.textContent = foo;
	$.append($$anchor, p);
}
