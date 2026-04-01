import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let coords = $.proxy({
		x: 0,
		y: 0
	});
	let x = $.derived(() => coords.x), y = $.derived(() => coords.y);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(x) ?? ""},${$.get(y) ?? ""}`));
	$.append($$anchor, p);
}
