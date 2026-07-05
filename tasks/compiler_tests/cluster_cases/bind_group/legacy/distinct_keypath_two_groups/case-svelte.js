import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="checkbox"/>`);
var root_1 = $.from_html(`<input type="radio"/> <input type="radio"/> <!>`, 1);
export default function App($$anchor) {
	const binding_group = [];
	const binding_group_1 = [];
	let data = $.mutable_source({
		a: 1,
		b: []
	});
	let items = ["x", "y"];
	var fragment = root_1();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	input.value = input.__value = 1;
	var input_1 = $.sibling(input, 2);
	$.remove_input_defaults(input_1);
	input_1.value = input_1.__value = 2;
	var node = $.sibling(input_1, 2);
	$.each(node, 1, () => items, $.index, ($$anchor, item) => {
		var input_2 = root();
		$.remove_input_defaults(input_2);
		var input_2_value;
		$.template_effect(() => {
			if (input_2_value !== (input_2_value = $.get(item))) {
				input_2.value = (input_2.__value = $.get(item)) ?? "";
			}
		});
		$.bind_group(binding_group_1, [], input_2, () => {
			$.get(item);
			return $.get(data).b;
		}, ($$value) => $.mutate(data, $.get(data).b = $$value));
		$.append($$anchor, input_2);
	});
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
