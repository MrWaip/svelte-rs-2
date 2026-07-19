import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div class="svelte-1k0loyk"><p class="svelte-1k0loyk"><span class="y">y</span></p></div>`), App[$.FILENAME], [[
	1,
	0,
	[[
		1,
		5,
		[[1, 8]]
	]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.append($$anchor, div);
	return $.pop($$exports);
}
