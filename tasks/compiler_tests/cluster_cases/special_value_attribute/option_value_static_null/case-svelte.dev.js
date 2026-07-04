import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<select><option disabled="">Select an option</option></select>`), App[$.FILENAME], [[
	5,
	0,
	[[6, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let foo = $.prop($$props, "foo", 12);
	var $$exports = { ...$.legacy_api() };
	var select = root();
	var option = $.child(select);
	option.value = (option.__value = null) ?? "";
	$.reset(select);
	$.bind_select_value(select, function get() {
		return foo();
	}, function set($$value) {
		foo($$value);
	});
	$.append($$anchor, select);
	return $.pop($$exports);
}
