import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	let enabled = true;
	var div = root();
	$.autofocus(div, enabled);
	$.append($$anchor, div);
}
