import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let width = $.prop($$props, "width", 3, "10px");
	var div = root();
	let styles;
	$.template_effect(() => styles = $.set_style(div, "color: red", styles, { width: width() }));
	$.append($$anchor, div);
}
