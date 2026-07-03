import * as $ from "svelte/internal/client";
const s = ($$anchor, a = $.noop, $$arg1) => {
	let b = $.derived_safe_equal(() => $.fallback($$arg1?.(), 2));
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${$.get(b) ?? ""}`));
	$.append($$anchor, button);
};
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let v = 1;
	s($$anchor, () => v);
}
