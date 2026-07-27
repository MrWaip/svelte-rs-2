import * as $ from "svelte/internal/client";
var root = $.from_html(`<a href="/">home</a>`);
export default function App($$anchor) {
	var a = root();
	$.append($$anchor, a);
}
