import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div class="x"><input value="x"/></div>`), App[$.FILENAME], [[
	1,
	0,
	[[1, 15]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var input = $.child(div);
	$.set_default_value(input, "y");
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
