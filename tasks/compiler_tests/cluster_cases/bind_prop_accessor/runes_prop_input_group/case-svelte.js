import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="radio"/>`);
export default function App($$anchor, $$props) {
	const binding_group = [];
	let value = $.prop($$props, "value", 7, "a");
	var input = root();
	$.remove_input_defaults(input);
	input.value = input.__value = "a";
	$.bind_group(binding_group, [], input, value, value);
	$.append($$anchor, input);
}
