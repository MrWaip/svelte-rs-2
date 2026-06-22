import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	let dpr = $.prop($$props, "dpr", 12);
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, dpr()));
	$.bind_property("devicePixelRatio", "resize", $.window, dpr);
	$.append($$anchor, div);
}
