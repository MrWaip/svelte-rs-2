import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<select multiple=""><option>a</option><option>b</option></select>`), App[$.FILENAME], [[
	5,
	0,
	[[6, 1], [7, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let selected = $.prop($$props, "selected", 12);
	var $$exports = { ...$.legacy_api() };
	var select = root();
	$.bind_select_value(select, function get() {
		return selected();
	}, function set($$value) {
		selected($$value);
	});
	$.append($$anchor, select);
	return $.pop($$exports);
}
