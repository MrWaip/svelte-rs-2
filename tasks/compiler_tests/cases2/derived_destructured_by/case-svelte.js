import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let data = $.proxy({
		a: 1,
		b: 2
	});
	let $$d = $.derived(() => data), a = $.derived(() => $.get($$d).a), b = $.derived(() => $.get($$d).b);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""},${$.get(b) ?? ""}`));
	$.append($$anchor, p);
}
