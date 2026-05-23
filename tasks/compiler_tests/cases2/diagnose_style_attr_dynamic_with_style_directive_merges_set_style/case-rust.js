import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let position = $.prop($$props, "position", 3, "static"), pb = $.prop($$props, "pb", 3, "");
	var div = root();
	let styles;
	$.template_effect(() => styles = $.set_style(div, `position: ${position() ?? ""}`, styles, { "--x": pb() }));
	$.append($$anchor, div);
}
