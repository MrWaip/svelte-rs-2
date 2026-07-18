import * as $ from "svelte/internal/client";
var root = $.from_svg(`<svg><clipPath id="c"></clipPath><linearGradient id="g"></linearGradient></svg>`);
export default function App($$anchor) {
	var svg = root();
	$.append($$anchor, svg);
}
