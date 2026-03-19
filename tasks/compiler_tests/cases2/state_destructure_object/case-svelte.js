import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let tmp = {
		a: 1,
		b: 2
	}, a = $.state($.proxy(tmp.a)), b = $.proxy(tmp.b);
	$.set(a, $.get(a) + 1);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""} ${b ?? ""}`));
	$.append($$anchor, p);
}
