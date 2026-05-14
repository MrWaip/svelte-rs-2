import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let count = 0;
	let val = $.prop($$props, "val", 3, count);
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, val()));
	$.append($$anchor, p);
}
