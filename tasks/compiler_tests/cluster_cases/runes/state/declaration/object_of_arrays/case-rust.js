import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let tmp = {
		p: [1, 2],
		q: [3, 4]
	}, $$array = $.derived(() => $.to_array(tmp.p, 2)), a = $.proxy($.get($$array)[0]), b = $.proxy($.get($$array)[1]), $$array_1 = $.derived(() => $.to_array(tmp.q, 2)), c = $.proxy($.get($$array_1)[0]), d = $.proxy($.get($$array_1)[1]);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a ?? ""}${b ?? ""}${c ?? ""}${d ?? ""}`));
	$.append($$anchor, button);
}
