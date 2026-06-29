import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
const s = ($$anchor, $$arg0) => {
	let a = $.derived_safe_equal(() => $.fallback($$arg0?.().a, 10));
	let b = $.derived_safe_equal(() => $.fallback($$arg0?.().b, 20));
	var button = root_1();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}`));
	$.append($$anchor, button);
};
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let v = {};
	s($$anchor, () => v);
}
