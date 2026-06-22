import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	let alpha = 1;
	var div = root();
	$.set_style(div, "", {}, { "--css-variable": "rgba(0, 0, 0, 1)" });
	$.append($$anchor, div);
}
