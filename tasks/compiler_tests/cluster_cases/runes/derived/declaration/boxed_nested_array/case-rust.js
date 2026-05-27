import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let x = $.proxy([[1, 2], [3, 4]]);
	let $$d = $.derived(() => x.slice()), $$array = $.derived(() => $.to_array($.get($$d), 2)), $$array_1 = $.derived(() => $.to_array($.get($$array)[0], 2)), a = $.derived(() => $.get($$array_1)[0]), b = $.derived(() => $.get($$array_1)[1]), $$array_2 = $.derived(() => $.to_array($.get($$array)[1], 2)), c = $.derived(() => $.get($$array_2)[0]), d = $.derived(() => $.get($$array_2)[1]);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}${$.get(c) ?? ""}${$.get(d) ?? ""}`));
	$.append($$anchor, button);
}
