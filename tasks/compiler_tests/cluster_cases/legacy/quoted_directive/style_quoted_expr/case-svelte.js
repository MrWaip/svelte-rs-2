import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	let c = "red";
	var div = root();
	$.set_style(div, "", {}, { color: c });
	$.append($$anchor, div);
}
