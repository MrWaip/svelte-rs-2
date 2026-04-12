import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let foo = 1;
	var $$exports = { foo };
	var p = root();
	p.textContent = foo;
	$.append($$anchor, p);
	return $.pop($$exports);
}
