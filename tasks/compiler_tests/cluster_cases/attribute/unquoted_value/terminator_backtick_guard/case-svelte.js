import * as $ from "svelte/internal/client";
var root = $.from_html(`<div foo="a" \`b=""></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
