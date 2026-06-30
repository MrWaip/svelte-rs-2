import * as $ from "svelte/internal/client";
var root = $.from_svg(`<svg><circle r="5"></circle></svg>`);
export default function App($$anchor) {
	var svg = root();
	$.append($$anchor, svg);
}
