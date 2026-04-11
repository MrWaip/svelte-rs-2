import * as $ from "svelte/internal/client";
var root = $.from_svg(`<rect></rect>`);
export default function App($$anchor) {
	let value = 1;
	let disabled = false;
	var rect = root();
	$.set_value(rect, value);
	rect.disabled = disabled;
	$.append($$anchor, rect);
}
