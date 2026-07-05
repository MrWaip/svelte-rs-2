import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor) {
	let x = $.state(0);
	var input = root();
	$.remove_input_defaults(input);
	$.bind_value(input, () => $.get(x), (v) => $.set(x, v, true));
	$.append($$anchor, input);
}
