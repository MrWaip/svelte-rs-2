import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="radio"/> <input type="radio"/> <input type="radio"/>`, 1);
export default function App($$anchor) {
	const binding_group = [];
	let group = $.state($.proxy([]));
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	input.value = input.__value = "a";
	var input_1 = $.sibling(input, 2);
	$.remove_input_defaults(input_1);
	input_1.value = input_1.__value = "b";
	var input_2 = $.sibling(input_1, 2);
	$.remove_input_defaults(input_2);
	input_2.value = input_2.__value = "c";
	$.bind_group(binding_group, [], input, () => $.get(group), ($$value) => $.set(group, $$value));
	$.bind_group(binding_group, [], input_1, () => $.get(group), ($$value) => $.set(group, $$value));
	$.bind_group(binding_group, [], input_2, () => $.get(group), ($$value) => $.set(group, $$value));
	$.append($$anchor, fragment);
}
