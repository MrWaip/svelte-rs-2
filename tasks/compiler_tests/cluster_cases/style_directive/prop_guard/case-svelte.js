import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let col = $.prop($$props, "col", 8);
	var div = root();
	let styles;
	$.template_effect(() => styles = $.set_style(div, "", styles, { color: col() }));
	$.append($$anchor, div);
}
