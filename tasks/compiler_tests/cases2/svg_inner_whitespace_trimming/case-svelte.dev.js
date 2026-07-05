App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_svg(`<svg><line></line><rect></rect></svg>`), App[$.FILENAME], [[
	6,
	0,
	[[7, 1], [8, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let w = 100;
	let h = 100;
	var $$exports = { ...$.legacy_api() };
	var svg = root();
	var line = $.child(svg);
	$.set_attribute(line, "x1", 0);
	$.set_attribute(line, "y1", 0);
	$.set_attribute(line, "x2", w);
	$.set_attribute(line, "y2", h);
	var rect = $.sibling(line);
	$.set_attribute(rect, "width", w);
	$.set_attribute(rect, "height", h);
	$.reset(svg);
	$.append($$anchor, svg);
	return $.pop($$exports);
}
