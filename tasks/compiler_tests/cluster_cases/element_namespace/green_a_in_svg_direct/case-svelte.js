import * as $ from "svelte/internal/client";
var root = $.from_svg(`<svg><a><text>Hello</text></a></svg>`);
export default function App($$anchor) {
	var svg = root();
	$.append($$anchor, svg);
}
