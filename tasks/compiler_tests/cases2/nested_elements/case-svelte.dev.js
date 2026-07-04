App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div><div><div><div></div></div></div></div>`), App[$.FILENAME], [[
	1,
	0,
	[[
		2,
		2,
		[[
			3,
			4,
			[[4, 6]]
		]]
	]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.append($$anchor, div);
	return $.pop($$exports);
}
