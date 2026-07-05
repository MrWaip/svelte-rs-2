import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let obj = $.tag($.mutable_source({ x: 1 }), "obj");
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.remove_input_defaults(input);
	$.bind_value(input, function get() {
		return $.get(obj).x;
	}, function set($$value) {
		$.mutate(obj, $.get(obj).x = $$value);
	});
	$.append($$anchor, input);
	return $.pop($$exports);
}
