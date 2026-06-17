import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	let fullscreen = $.prop($$props, "fullscreen", 12);
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, fullscreen()));
	$.bind_property("fullscreenElement", "fullscreenchange", $.document, fullscreen);
	$.append($$anchor, div);
}
