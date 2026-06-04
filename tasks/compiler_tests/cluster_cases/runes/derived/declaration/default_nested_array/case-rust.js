import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let x = $.proxy([[1, 2], 3]);
	let $$array = $.derived(() => $.to_array(x, 2)), $$array_1 = $.derived(() => $.to_array($.fallback($.get($$array)[0], () => [9, 9], true), 2)), a = $.derived(() => $.get($$array_1)[0]), b = $.derived(() => $.get($$array_1)[1]), c = $.derived(() => $.get($$array)[1]);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}${$.get(c) ?? ""}`));
	$.append($$anchor, button);
}
