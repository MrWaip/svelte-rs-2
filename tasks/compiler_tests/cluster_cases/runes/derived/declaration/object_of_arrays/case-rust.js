import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let x = $.proxy({
		p: [1, 2],
		q: [3, 4]
	});
	let $$array = $.derived(() => $.to_array(x.p, 2)), $$array_1 = $.derived(() => $.to_array(x.q, 2)), a = $.derived(() => $.get($$array)[0]), b = $.derived(() => $.get($$array)[1]), c = $.derived(() => $.get($$array_1)[0]), d = $.derived(() => $.get($$array_1)[1]);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}${$.get(c) ?? ""}${$.get(d) ?? ""}`));
	$.append($$anchor, button);
}
