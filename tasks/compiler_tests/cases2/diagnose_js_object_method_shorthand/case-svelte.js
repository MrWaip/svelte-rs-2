import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	const obj = { run(x) {
		return x + 1;
	} };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(($0) => $.set_text(text, $0), [() => obj.run(1)]);
	$.append($$anchor, p);
}
