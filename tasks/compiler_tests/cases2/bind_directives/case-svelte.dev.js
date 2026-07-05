App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/> <input/> <input/>`, 1), App[$.FILENAME], [
	[6, 0],
	[8, 0],
	[19, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = $.tag($.state(""), "value");
	let name = "";
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	var input_1 = $.sibling(input, 2);
	$.remove_input_defaults(input_1);
	var input_2 = $.sibling(input_1, 2);
	$.remove_input_defaults(input_2);
	$.bind_value(input, function get() {
		return $.get(value);
	}, function set($$value) {
		$.set(value, $$value);
	});
	$.bind_value(input_1, function get() {
		return name;
	}, function set($$value) {
		name = $$value;
	});
	$.bind_value(input_2, function get() {
		return name;
	}, function set($$value) {
		name = $$value;
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
