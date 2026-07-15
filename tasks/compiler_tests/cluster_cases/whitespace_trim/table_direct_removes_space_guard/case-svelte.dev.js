App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<table><tbody><tr><td>a</td></tr><tr><td>b</td></tr></tbody></table>`), App[$.FILENAME], [[
	1,
	0,
	[[
		1,
		7,
		[[
			2,
			1,
			[[2, 5]]
		], [
			3,
			1,
			[[3, 5]]
		]]
	]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var table = root();
	$.append($$anchor, table);
	return $.pop($$exports);
}
