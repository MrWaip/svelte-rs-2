import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	const k = "z";
	let tmp = { z: 1 }, v = $.proxy(tmp[k]);
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, v));
	$.append($$anchor, button);
}
