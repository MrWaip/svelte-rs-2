import * as $ from "svelte/internal/client";
var root = $.from_svg(`<svg><g></g><!></svg>`);
export default function App($$anchor) {
	let content = "<circle cx='5' cy='5' r='5'></circle>";
	var svg = root();
	var node = $.sibling($.child(svg));
	$.html(node, () => content);
	$.reset(svg);
	$.append($$anchor, svg);
}
