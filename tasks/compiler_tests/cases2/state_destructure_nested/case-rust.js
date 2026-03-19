import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let tmp = { a: { b: 1 } }, b = $.proxy(tmp.a.b);
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, b));
	$.append($$anchor, p);
}
