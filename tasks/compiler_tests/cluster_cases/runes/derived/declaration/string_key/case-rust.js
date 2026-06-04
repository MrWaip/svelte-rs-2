import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let src = $.proxy({
		"a-b": 1,
		"c d": 2
	});
	let ab = $.derived(() => src["a-b"]), cd = $.derived(() => src["c d"]);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(ab) ?? ""}${$.get(cd) ?? ""}`));
	$.append($$anchor, button);
}
