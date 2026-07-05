import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div><input/></div>`), App[$.FILENAME], [[
	5,
	0,
	[[5, 5]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let foo = $.prop($$props, "foo", 12);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var input = $.child(div);
	$.remove_input_defaults(input);
	$.reset(div);
	$.bind_value(input, function get() {
		return foo();
	}, function set($$value) {
		foo($$value);
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
