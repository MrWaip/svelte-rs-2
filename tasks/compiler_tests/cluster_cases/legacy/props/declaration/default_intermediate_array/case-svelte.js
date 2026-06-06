import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	let tmp = [[1, 2], 3], $$array = $.derived(() => $.to_array(tmp, 2)), $$array_1 = $.derived(() => $.to_array($.fallback($.get($$array)[0], () => [8, 9], true), 2)), a = $.prop($$props, "a", 24, () => $.get($$array_1)[0]), b = $.prop($$props, "b", 24, () => $.get($$array_1)[1]), c = $.prop($$props, "c", 24, () => $.get($$array)[1]);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${b() ?? ""}${c() ?? ""}`));
	$.append($$anchor, button);
}
