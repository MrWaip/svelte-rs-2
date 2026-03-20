import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let a = 1;
	let b = 2;
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(($0) => $.set_text(text, $0), [() => $.eager(() => a + b)]);
	$.append($$anchor, p);
}
