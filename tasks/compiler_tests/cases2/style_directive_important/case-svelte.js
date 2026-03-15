import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>Important</div>`);
export default function App($$anchor) {
	let color = "red";
	let bg = "blue";
	var div = root();
	let styles;
	$.template_effect(() => styles = $.set_style(div, "", styles, [{ color }, { "background-color": bg }]));
	$.append($$anchor, div);
}
