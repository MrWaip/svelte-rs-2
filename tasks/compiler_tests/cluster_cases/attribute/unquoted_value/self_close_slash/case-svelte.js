import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_svg(`<svg><circle cx="50" cy="50" r="50"></circle></svg>`);
export default function App($$anchor) {
	var svg = root();
	$.append($$anchor, svg);
}
