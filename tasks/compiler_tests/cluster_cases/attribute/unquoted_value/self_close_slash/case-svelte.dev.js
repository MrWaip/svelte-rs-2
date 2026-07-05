import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_svg(`<svg><circle cx="50" cy="50" r="50"></circle></svg>`), App[$.FILENAME], [[
	1,
	0,
	[[2, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var svg = root();
	$.append($$anchor, svg);
	return $.pop($$exports);
}
