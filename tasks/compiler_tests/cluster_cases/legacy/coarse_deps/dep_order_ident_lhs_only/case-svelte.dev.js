import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { foo } from "lib";
var root = $.add_locations($.from_html(`<input/> <input/>`, 1), App[$.FILENAME], [[9, 0], [10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let total = $.tag($.mutable_source(0), "total");
	let c = $.tag($.mutable_source(0), "c");
	let d = $.tag($.mutable_source(0), "d");
	$.legacy_pre_effect(() => ($.get(c), foo, $.get(d)), () => {
		$.set(total, $.get(c) + foo($.get(d)));
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
		return $.get(d);
	}, function set($$value) {
		$.set(d, $$value);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
