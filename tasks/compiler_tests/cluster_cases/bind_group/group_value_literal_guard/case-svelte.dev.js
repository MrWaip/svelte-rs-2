import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="radio"/> <input type="radio"/>`, 1), App[$.FILENAME], [[5, 0], [6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const binding_group = [];
	let foo = $.prop($$props, "foo", 12);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	input.value = input.__value = false;
	var input_1 = $.sibling(input, 2);
	$.remove_input_defaults(input_1);
	input_1.value = input_1.__value = true;
	$.bind_group(binding_group, [], input, () => {
		false;
		return foo();
	}, function set($$value) {
		foo($$value);
	});
	$.bind_group(binding_group, [], input_1, () => {
		true;
		return foo();
	}, function set($$value) {
		foo($$value);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
