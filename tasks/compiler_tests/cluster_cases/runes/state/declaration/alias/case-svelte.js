import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let tmp = {
		a: 1,
		b: 2
	}, x = $.proxy(tmp.a), y = $.proxy(tmp.b);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${x ?? ""}${y ?? ""}`));
	$.append($$anchor, button);
}
