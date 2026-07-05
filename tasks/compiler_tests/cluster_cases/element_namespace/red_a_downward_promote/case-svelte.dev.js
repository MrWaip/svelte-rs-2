App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_svg(`<a><text>Hello</text></a>`), App[$.FILENAME], [[
	1,
	0,
	[[1, 3]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var a = root();
	$.append($$anchor, a);
	return $.pop($$exports);
}
