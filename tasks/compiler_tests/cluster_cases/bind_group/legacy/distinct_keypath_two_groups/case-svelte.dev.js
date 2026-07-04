import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="checkbox"/>`), App[$.FILENAME], [[10, 1]]);
var root_1 = $.add_locations($.from_html(`<input type="radio"/> <input type="radio"/> <!>`, 1), App[$.FILENAME], [[7, 0], [8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const binding_group = [];
	const binding_group_1 = [];
	let data = $.tag($.mutable_source({
		a: 1,
		b: []
	}), "data");
	let items = ["x", "y"];
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	input.value = input.__value = 1;
	var input_1 = $.sibling(input, 2);
	$.remove_input_defaults(input_1);
	input_1.value = input_1.__value = 2;
	var node = $.sibling(input_1, 2);
	$.add_svelte_meta(() => $.each(node, 1, () => items, $.index, ($$anchor, item) => {
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
		}, function set($$value) {
			$.mutate(data, $.get(data).b = $$value);
		});
		$.append($$anchor, input_2);
	}), "each", App, 9, 0);
	$.bind_group(binding_group, [], input, () => {
		1;
		return $.get(data).a;
	}, function set($$value) {
		$.mutate(data, $.get(data).a = $$value);
	});
	$.bind_group(binding_group, [], input_1, () => {
		2;
		return $.get(data).a;
	}, function set($$value) {
		$.mutate(data, $.get(data).a = $$value);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
