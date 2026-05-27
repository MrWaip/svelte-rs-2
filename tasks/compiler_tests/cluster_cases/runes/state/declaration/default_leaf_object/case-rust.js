import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let tmp = {}, a = $.proxy($.fallback(tmp.a, 10)), b = $.proxy($.fallback(tmp.b, 20));
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a ?? ""}${b ?? ""}`));
	$.append($$anchor, button);
}
