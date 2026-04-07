import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let count = 1;
	var $$exports = { count };
	var p = root();
	p.textContent = count;
	$.append($$anchor, p);
	return $.pop($$exports);
}
