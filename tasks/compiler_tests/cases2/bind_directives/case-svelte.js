import * as $ from "svelte/internal/client";
var root = $.template(`<input> <input>`, 1);
export default function App($$anchor) {
	let value = $.state("");
	let name = "";
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	var input_1 = $.sibling(input, 2);
	$.remove_input_defaults(input_1);
	$.bind_value(input, () => $.get(value), ($$value) => $.set(value, $$value));
	$.bind_value(input_1, () => name, ($$value) => name = $$value);
	$.append($$anchor, fragment);
}
