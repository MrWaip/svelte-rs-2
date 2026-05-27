import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let tmp = {}, a = $.proxy($.fallback(tmp.p, {}).a);
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, a));
	$.append($$anchor, button);
}
