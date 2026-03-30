import * as $ from "svelte/internal/client";
var root = $.from_svg(`<svg><text></text></svg>`);
export default function App($$anchor) {
	let label = "hello";
	var svg = root();
	var text = $.child(svg);
	$.set_attribute(text, "x", 10);
	$.set_attribute(text, "y", 20);
	text.textContent = "hello";
	$.reset(svg);
	$.append($$anchor, svg);
}
