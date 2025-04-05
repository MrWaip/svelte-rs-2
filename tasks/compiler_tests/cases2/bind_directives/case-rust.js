import * as $ from "svelte/internal/client";
var root = $.template(`<input> <input> <input>`, 1);
export default function App($$anchor) {
	let value = $.state("");
	let name = "";
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	var input_1 = $.sibling(input, 2);
	$.remove_input_defaults(input_1);
	var input_2 = $.sibling(input_1, 2);
	$.remove_input_defaults(input_2);
	$.bind_value(input, () => $.get(value), ($$value) => $.set(value, $$value));
	$.bind_value(input_1, () => name, ($$value) => name = $$value);
	$.bind_value(input_2, () => name, ($$value) => name = $$value);
	$.append($$anchor, fragment);
}
