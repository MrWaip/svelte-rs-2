import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="radio"/> <input type="radio"/>`, 1);
export default function App($$anchor) {
	const binding_group = [];
	let data = $.mutable_source({ a: 1 });
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	input.value = input.__value = 1;
	var input_1 = $.sibling(input, 2);
	$.remove_input_defaults(input_1);
	input_1.value = input_1.__value = 2;
	$.bind_group(binding_group, [], input, () => {
		1;
		return $.get(data).a;
	}, ($$value) => $.mutate(data, $.get(data).a = $$value));
	$.bind_group(binding_group, [], input_1, () => {
		2;
		return $.get(data).a;
	}, ($$value) => $.mutate(data, $.get(data).a = $$value));
	$.append($$anchor, fragment);
}
