import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let tmp = [1, 2], $$array = $.derived(() => $.to_array(tmp, 2)), a = $.proxy($.get($$array)[0]), b = $.proxy($.get($$array)[1]);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a ?? ""}${b ?? ""}`));
	$.append($$anchor, button);
}
