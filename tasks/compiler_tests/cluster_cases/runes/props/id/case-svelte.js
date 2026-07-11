import * as $ from "svelte/internal/client";
var root = $.from_html(`<h1> </h1>`);
export default function App($$anchor) {
	const id = $.props_id();
	var h1 = root();
	var text = $.child(h1, true);
	$.reset(h1);
	$.template_effect(() => $.set_text(text, id));
	$.append($$anchor, h1);
}
