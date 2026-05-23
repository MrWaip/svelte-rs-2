import * as $ from "svelte/internal/client";
var root = $.from_html(`<h3>Hello<br/> </h3>`);
export default function App($$anchor) {
	let name = "x";
	var h3 = root();
	var text = $.sibling($.child(h3), 2, true);
	text.nodeValue = "x";
	$.reset(h3);
	$.append($$anchor, h3);
}
