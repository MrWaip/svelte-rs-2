import * as $ from "svelte/internal/client";
var root = $.from_svg(`<svg><line></line><rect></rect></svg>`);
export default function App($$anchor) {
	let w = 100;
	let h = 100;
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
}
