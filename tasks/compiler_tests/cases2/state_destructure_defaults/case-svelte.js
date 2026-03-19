import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let tmp = {}, a = $.state($.proxy($.fallback(tmp.a, 10))), b = $.proxy($.fallback(tmp.b, 20));
	$.set(a, $.get(a) + 1);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""} ${b ?? ""}`));
	$.append($$anchor, p);
}
