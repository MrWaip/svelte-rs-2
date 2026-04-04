import * as $ from "svelte/internal/client";
export const VERSION = "1.0.0";
var root = $.from_html(`<p>Static content</p>`);
export default function App($$anchor) {
	var p = root();
	$.append($$anchor, p);
}
