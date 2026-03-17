import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>Hello world</p>`);
export default function App($$anchor) {
	var p = root();
	$.append($$anchor, p);
}
