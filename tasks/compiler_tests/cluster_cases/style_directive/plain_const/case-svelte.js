import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	let col = "red";
	var div = root();
	$.set_style(div, "", {}, { color: col });
	$.append($$anchor, div);
}
