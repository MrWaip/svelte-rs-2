App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div><span></span></div>`), App[$.FILENAME], [[
	1,
	0,
	[[1, 5]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var span = $.child(div);
	{
		const x = 1;
	}
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
