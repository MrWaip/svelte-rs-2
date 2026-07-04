App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/> <input/> <input type="checkbox"/> <input type="checkbox"/> <input/> <input/>`, 1), App[$.FILENAME], [
	[8, 0],
	[10, 0],
	[12, 0],
	[14, 0],
	[16, 0],
	[18, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const binding_group = [];
	let value = $.tag($.state(""), "value");
	let checked = false;
	let group = $.tag($.state(void 0), "group");
	var $$exports = { ...$.legacy_api() };
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
	$.bind_value(input, function get() {
		return $.get(value);
	}, function set($$value) {
		$.set(value, $$value);
	});
	$.bind_value(input_1, function get() {
		return $.get(value);
	}, function set($$value) {
		$.set(value, $$value);
	});
	$.bind_checked(input_2, function get() {
		return checked;
	}, function set($$value) {
		checked = $$value;
	});
	$.bind_checked(input_3, function get() {
		return checked;
	}, function set($$value) {
		checked = $$value;
	});
	$.bind_group(binding_group, [], input_4, function get() {
		return $.get(group);
	}, function set($$value) {
		$.set(group, $$value);
	});
	$.bind_group(binding_group, [], input_5, function get() {
		return $.get(group);
	}, function set($$value) {
		$.set(group, $$value);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
