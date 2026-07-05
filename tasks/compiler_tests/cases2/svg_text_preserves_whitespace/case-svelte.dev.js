App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_svg(`<svg><text></text></svg>`), App[$.FILENAME], [[
	5,
	0,
	[[6, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let label = "hello";
	var $$exports = { ...$.legacy_api() };
	var svg = root();
	var text = $.child(svg);
	$.set_attribute(text, "x", 10);
	$.set_attribute(text, "y", 20);
	text.textContent = "hello";
	$.reset(svg);
	$.append($$anchor, svg);
	return $.pop($$exports);
}
