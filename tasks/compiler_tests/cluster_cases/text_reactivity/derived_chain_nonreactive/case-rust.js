import * as $ from "svelte/internal/client";
var root = $.from_html(`<h1> </h1>`);
export default function App($$anchor) {
	function load() {
		return 1;
	}
	const a = $.derived(load);
	const x = $.derived(() => $.get(a) + 1);
	var h1 = root();
	var text = $.child(h1, true);
	$.reset(h1);
	$.template_effect(() => $.set_text(text, $.get(x)));
	$.append($$anchor, h1);
}
