import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor) {
	let obj = { w: 0 };
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, obj.w));
	$.bind_window_size("innerWidth", ($$value) => obj.w = $$value);
	$.append($$anchor, div);
}
