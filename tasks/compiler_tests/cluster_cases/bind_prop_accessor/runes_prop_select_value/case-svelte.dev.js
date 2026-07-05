App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<select><option>a</option><option>b</option></select>`), App[$.FILENAME], [[
	5,
	0,
	[[6, 1], [7, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = $.prop($$props, "value", 7, "a");
	var $$exports = { ...$.legacy_api() };
	var select = root();
	$.bind_select_value(select, function get() {
		return value();
	}, function set($$value) {
		value($$value);
	});
	$.append($$anchor, select);
	return $.pop($$exports);
}
