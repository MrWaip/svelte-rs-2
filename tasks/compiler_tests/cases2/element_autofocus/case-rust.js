import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor) {
	let enabled = true;
	var input = root();
	$.set_attribute(input, "autofocus", enabled);
	$.append($$anchor, input);
}
