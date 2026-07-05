import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let obj = $.proxy({ a: 1 });
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(($0) => $.set_text(text, $0), [() => JSON.stringify($.snapshot(obj))]);
	$.append($$anchor, p);
}
