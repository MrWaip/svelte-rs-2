import * as $ from "svelte/internal/client";
var root = $.template(`<input> <input> <input type="checkbox"> <input type="checkbox"> <input> <input>`, 1);
export default function App($$anchor) {
	let value = $.state("");
	let checked = false;
	let group = $.state(undefined);
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	var input_1 = $.sibling(input, 2);
	$.remove_input_defaults(input_1);
	var input_2 = $.sibling(input_1, 2);
	$.remove_input_defaults(input_2);
	var input_3 = $.sibling(input_2, 2);
	$.remove_input_defaults(input_3);
	var input_4 = $.sibling(input_3, 2);
	$.remove_input_defaults(input_4);
	var input_5 = $.sibling(input_4, 2);
	$.remove_input_defaults(input_5);
	$.bind_value(input, () => $.get(value), ($$value) => $.set(value, $$value));
	$.bind_value(input_1, () => $.get(value), ($$value) => $.set(value, $$value));
	$.bind_checked(input_2, () => checked, ($$value) => checked = $$value);
	$.bind_checked(input_3, () => checked, ($$value) => checked = $$value);
	$.binding_group(input_4, () => $.get(group), ($$value) => $.set(group, $$value));
	$.binding_group(input_5, () => $.get(group), ($$value) => $.set(group, $$value));
	$.append($$anchor, fragment);
}
