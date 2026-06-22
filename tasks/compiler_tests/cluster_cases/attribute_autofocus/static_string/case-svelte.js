import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor) {
	var input = root();
	$.autofocus(input, "autofocus");
	$.append($$anchor, input);
}
