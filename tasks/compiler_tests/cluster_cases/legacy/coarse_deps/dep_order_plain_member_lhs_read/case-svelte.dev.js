import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { foo } from "lib";
var root = $.add_locations($.from_html(`<input/> <input/>`, 1), App[$.FILENAME], [[8, 0], [9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let obj = $.tag($.mutable_source({}), "obj");
	let c = $.tag($.mutable_source(""), "c");
	$.legacy_pre_effect(() => ($.get(obj), $.get(c), foo), () => {
		$.mutate(obj, $.get(obj).purpose = ($.get(c) ? $.get(c) : "") + foo($.get(obj).type));
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	var input_1 = $.sibling(input, 2);
	$.remove_input_defaults(input_1);
	$.bind_value(input, function get() {
		return $.get(c);
	}, function set($$value) {
		$.set(c, $$value);
	});
	$.bind_value(input_1, function get() {
		return $.get(obj).x;
	}, function set($$value) {
		$.mutate(obj, $.get(obj).x = $$value);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
