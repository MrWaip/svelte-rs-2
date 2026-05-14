import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	const H = 8;
	let w = $.state(0);
	setTimeout(() => {
		$.set(w, 10);
	});
	var div = root();
	let styles;
	$.template_effect(() => styles = $.set_style(div, "", styles, {
		height: "8px",
		width: `${$.get(w) ?? ""}px`
	}));
	$.append($$anchor, div);
}
