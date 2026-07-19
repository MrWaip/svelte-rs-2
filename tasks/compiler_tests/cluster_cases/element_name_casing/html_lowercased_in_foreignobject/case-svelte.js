import * as $ from "svelte/internal/client";
var root = $.from_svg(`<svg><foreignObject><div>x</div></foreignObject></svg>`);
export default function App($$anchor) {
	var svg = root();
	$.append($$anchor, svg);
}
