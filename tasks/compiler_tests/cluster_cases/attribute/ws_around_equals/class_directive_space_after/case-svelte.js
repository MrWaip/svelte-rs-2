import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let value = $.prop($$props, "value", 8);
	var div = root();
	let classes;
	$.template_effect(() => classes = $.set_class(div, 1, "", null, classes, { x: value() }));
	$.append($$anchor, div);
}
