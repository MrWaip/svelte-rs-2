import * as $ from "svelte/internal/client";
var root = $.from_html(`<h1> </h1>`);
export default function App($$anchor) {
	let s = $.state(0);
	function inc() {
		$.update(s);
	}
	const x = $.derived(() => $.get(s) + 1);
	var h1 = root();
	var text = $.child(h1, true);
	$.reset(h1);
	$.template_effect(() => $.set_text(text, $.get(x)));
	$.append($$anchor, h1);
}
