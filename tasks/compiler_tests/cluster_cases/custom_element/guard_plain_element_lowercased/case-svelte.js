import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let obj = $.prop($$props, "obj", 8);
	var div = root();
	$.template_effect(() => $.set_attribute(div, "camelcase", obj()));
	$.append($$anchor, div);
}
