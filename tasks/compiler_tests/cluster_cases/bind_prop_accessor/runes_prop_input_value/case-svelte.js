import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor, $$props) {
	let value = $.prop($$props, "value", 7, "x");
	var input = root();
	$.remove_input_defaults(input);
	$.bind_value(input, value);
	$.append($$anchor, input);
}
