App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div class="benefit svelte-1vvtp4y"><span class="svelte-1vvtp4y">icon</span> <span class="svelte-1vvtp4y">text</span></div>`), App[$.FILENAME], [[
	1,
	0,
	[[2, 4], [3, 4]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.append($$anchor, div);
	return $.pop($$exports);
}
