import * as $ from "svelte/internal/client";
var root = $.from_html(`<span data-x="1"></span>`);
export default function App($$anchor) {
	var span = root();
	$.append($$anchor, span);
}
