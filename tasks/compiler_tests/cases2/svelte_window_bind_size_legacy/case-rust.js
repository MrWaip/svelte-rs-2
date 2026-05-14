import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor) {
	let w = $.mutable_source(0);
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, $.get(w)));
	$.bind_window_size("innerWidth", ($$value) => $.set(w, $$value));
	$.append($$anchor, div);
}
