import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
const s = ($$anchor, $$arg0) => {
	var $$array = $.derived(() => $.to_array($$arg0?.(), 2));
	let a = $.derived_safe_equal(() => $.fallback($.get($$array)[0], 10));
	let b = $.derived_safe_equal(() => $.fallback($.get($$array)[1], 20));
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}`));
	$.append($$anchor, button);
};
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let v = [1];
	s($$anchor, () => v);
}
