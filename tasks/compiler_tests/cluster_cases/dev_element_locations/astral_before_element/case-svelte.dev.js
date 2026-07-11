App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>😀 <b>bold</b></div>`), App[$.FILENAME], [[
	1,
	0,
	[[1, 8]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.append($$anchor, div);
	return $.pop($$exports);
}
