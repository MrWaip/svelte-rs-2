import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	let scrollY = $.prop($$props, "scrollY", 12);
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, scrollY()));
	$.bind_window_scroll("y", scrollY);
	$.append($$anchor, div);
}
