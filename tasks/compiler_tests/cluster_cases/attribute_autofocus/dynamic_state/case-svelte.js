import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor) {
	let enabled = true;
	var input = root();
	$.autofocus(input, enabled);
	$.append($$anchor, input);
}
