import * as $ from "svelte/internal/client";
var root = $.template(`<input> <input> <input> <input defaultValue="123">`, 1);
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
	let attributes;
	var input_3 = $.sibling(input_2, 2);
	$.template_effect(() => attributes = $.set_attributes(input_2, attributes, { ...other }));
	$.append($$anchor, fragment);
}
