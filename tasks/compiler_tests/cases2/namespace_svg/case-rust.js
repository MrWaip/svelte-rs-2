import * as $ from "svelte/internal/client";
var root = $.from_svg(`<rect width="100" height="100"></rect>`);
export default function App($$anchor) {
	var rect = root();
	$.append($$anchor, rect);
}
