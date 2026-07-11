import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>ascii <b>bold</b></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
