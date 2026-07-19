App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_svg(`<svg><clipPath id="c"></clipPath><linearGradient id="g"></linearGradient></svg>`), App[$.FILENAME], [[
	1,
	0,
	[[1, 5], [1, 33]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var svg = root();
	$.append($$anchor, svg);
	return $.pop($$exports);
}
