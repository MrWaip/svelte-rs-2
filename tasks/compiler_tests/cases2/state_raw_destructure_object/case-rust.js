import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let tmp = {
		items: [],
		count: 0
	}, items = tmp.items, count = tmp.count;
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, count));
	$.append($$anchor, p);
}
