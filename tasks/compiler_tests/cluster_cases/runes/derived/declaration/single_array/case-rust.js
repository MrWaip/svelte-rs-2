import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let x = $.proxy([1, 2]);
	let $$array = $.derived(() => $.to_array(x, 2)), a = $.derived(() => $.get($$array)[0]), b = $.derived(() => $.get($$array)[1]);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}`));
	$.append($$anchor, button);
}
