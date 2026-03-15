import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>Styled</div>`);
export default function App($$anchor) {
	let color = "red";
	let fontSize = "16px";
	let bg = "blue";
	const staticVal = "bold";
	var div = root();
	let styles;
	$.template_effect(() => styles = $.set_style(div, "", styles, {
		color,
		"font-size": fontSize,
		"background-color": bg,
		"font-weight": staticVal
	}));
	$.append($$anchor, div);
}
