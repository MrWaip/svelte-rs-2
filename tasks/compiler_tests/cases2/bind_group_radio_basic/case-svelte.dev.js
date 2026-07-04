App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="radio"/> <input type="radio"/> <input type="radio"/>`, 1), App[$.FILENAME], [
	[5, 0],
	[6, 0],
	[7, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const binding_group = [];
	let group = $.tag($.state($.proxy([])), "group");
	var $$exports = { ...$.legacy_api() };
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
	$.bind_group(binding_group, [], input, function get() {
		return $.get(group);
	}, function set($$value) {
		$.set(group, $$value);
	});
	$.bind_group(binding_group, [], input_1, function get() {
		return $.get(group);
	}, function set($$value) {
		$.set(group, $$value);
	});
	$.bind_group(binding_group, [], input_2, function get() {
		return $.get(group);
	}, function set($$value) {
		$.set(group, $$value);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
