import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	const op = .5;
	var div = root();
	$.set_style(div, "", {}, { opacity: op });
	$.append($$anchor, div);
}
