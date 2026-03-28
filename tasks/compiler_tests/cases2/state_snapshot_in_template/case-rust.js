import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let items = $.proxy([
		1,
		2,
		3
	]);
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(($0) => $.set_text(text, $0), [() => JSON.stringify($state.snapshot(items))]);
	$.append($$anchor, p);
}
