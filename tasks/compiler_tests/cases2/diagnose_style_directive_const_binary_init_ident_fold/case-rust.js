import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	const w = "12" + "px";
	var div = root();
	$.set_style(div, "", {}, { width: w });
	$.append($$anchor, div);
}
