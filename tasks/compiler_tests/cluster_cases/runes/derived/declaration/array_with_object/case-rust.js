import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let x = $.proxy([{ a: 1 }, [2, 3]]);
	let $$array = $.derived(() => $.to_array(x, 2)), a = $.derived(() => $.get($$array)[0].a), $$array_1 = $.derived(() => $.to_array($.get($$array)[1], 2)), b = $.derived(() => $.get($$array_1)[0]), c = $.derived(() => $.get($$array_1)[1]);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}${$.get(c) ?? ""}`));
	$.append($$anchor, button);
}
