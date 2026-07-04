App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div class="cell-block svelte-awt0xm"><div class="cell">a</div> <div class="cell"><div class="content">b</div></div></div>`), App[$.FILENAME], [[
	1,
	0,
	[[2, 1], [
		3,
		1,
		[[3, 19]]
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
