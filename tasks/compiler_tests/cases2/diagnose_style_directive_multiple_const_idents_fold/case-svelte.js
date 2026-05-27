import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	const c = "red";
	const w = "bold";
	const s = "16px";
	var div = root();
	$.set_style(div, "", {}, {
		color: c,
		"font-weight": w,
		"font-size": s
	});
	$.append($$anchor, div);
}
