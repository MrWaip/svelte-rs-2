import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<select><option>Two</option></select>`), App[$.FILENAME], [[
	1,
	0,
	[[2, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var select = root();
	var option = $.child(select);
	option.value = option.__value = "b";
	$.reset(select);
	$.append($$anchor, select);
	return $.pop($$exports);
}
