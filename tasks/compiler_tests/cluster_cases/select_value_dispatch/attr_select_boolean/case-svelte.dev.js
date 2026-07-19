App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<select><option>a</option><option>b</option></select>`), App[$.FILENAME], [[
	1,
	0,
	[[2, 1], [3, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var select = root();
	select.value = select.__value = true;
	$.append($$anchor, select);
	return $.pop($$exports);
}
