import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let title = $.prop($$props, "title", 8, undefined);
	var div = root();
	$.template_effect(() => $.set_attribute(div, "title", title()));
	$.append($$anchor, div);
}
