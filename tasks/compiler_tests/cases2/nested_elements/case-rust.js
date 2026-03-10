import * as $ from "svelte/internal/client";
var root = $.from_html(`<div><div><div><div></div></div></div></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
