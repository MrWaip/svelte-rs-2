import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="radio"/> <input type="radio"/>`, 1), App[$.FILENAME], [[6, 0], [7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const binding_group = [];
	let data = $.tag($.mutable_source({ a: 1 }), "data");
	var $$exports = { ...$.legacy_api() };
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
