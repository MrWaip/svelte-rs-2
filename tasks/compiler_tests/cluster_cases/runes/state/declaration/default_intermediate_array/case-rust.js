import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let tmp = [[1, 2], 3], $$array = $.derived(() => $.to_array(tmp, 2)), $$array_1 = $.derived(() => $.to_array($.fallback($.get($$array)[0], [8, 9]), 2)), a = $.proxy($.get($$array_1)[0]), b = $.proxy($.get($$array_1)[1]), c = $.proxy($.get($$array)[1]);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a ?? ""}${b ?? ""}${c ?? ""}`));
	$.append($$anchor, button);
}
