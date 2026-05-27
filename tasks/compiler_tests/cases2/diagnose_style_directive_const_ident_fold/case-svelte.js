import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	const transform = "translateY(1px)";
	var div = root();
	$.set_style(div, "", {}, { transform });
	$.append($$anchor, div);
}
