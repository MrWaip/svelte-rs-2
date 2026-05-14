import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor) {
	const value = "x";
	var input = root();
	$.remove_input_defaults(input);
	$.set_value(input, value);
	$.append($$anchor, input);
}
