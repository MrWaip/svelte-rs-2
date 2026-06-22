import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	let c = "0, 0, 0";
	var div = root();
	$.set_style(div, "", {}, { color: "rgb(0, 0, 0)" });
	$.append($$anchor, div);
}
