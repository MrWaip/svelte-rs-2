import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	let offsetX = $.prop($$props, "offsetX", 8, "");
	let paddingX = $.prop($$props, "paddingX", 24, offsetX);
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, paddingX()));
	$.append($$anchor, div);
}
