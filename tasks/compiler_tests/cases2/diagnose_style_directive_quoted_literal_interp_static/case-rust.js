import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	var div = root();
	$.set_style(div, "", {}, {
		width: "18px",
		height: "18px"
	});
	$.append($$anchor, div);
}
