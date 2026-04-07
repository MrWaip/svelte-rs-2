import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>fallback html</div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
