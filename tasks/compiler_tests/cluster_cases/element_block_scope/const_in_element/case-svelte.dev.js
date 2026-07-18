App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div><span></span></div>`), App[$.FILENAME], [[
	1,
	0,
	[[3, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	{
		const x = 5;
		var span = $.child(div);
		span.textContent = "5";
		$.reset(div);
	}
	$.append($$anchor, div);
	return $.pop($$exports);
}
