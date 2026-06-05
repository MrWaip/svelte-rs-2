import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	let tmp = [1], $$array = $.derived(() => $.to_array(tmp, 2)), a = $.prop($$props, "a", 24, () => $.fallback($.get($$array)[0], 10)), b = $.prop($$props, "b", 24, () => $.fallback($.get($$array)[1], 20));
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${b() ?? ""}`));
	$.append($$anchor, button);
}
