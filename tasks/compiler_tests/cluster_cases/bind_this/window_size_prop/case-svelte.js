import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	let width = $.prop($$props, "width", 12);
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, width()));
	$.bind_window_size("innerWidth", width);
	$.append($$anchor, div);
}
