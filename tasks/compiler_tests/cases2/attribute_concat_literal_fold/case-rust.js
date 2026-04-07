import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	var div = root();
	$.set_attribute(div, "data-count", "value: 1231");
	$.append($$anchor, div);
}
