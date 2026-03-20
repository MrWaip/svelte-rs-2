import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor) {
	let value = $.state("");
	var input = root();
	$.remove_input_defaults(input);
	$.bind_value(input, () => $.get(value), (v) => $.set(value, v, true));
	$.append($$anchor, input);
}
