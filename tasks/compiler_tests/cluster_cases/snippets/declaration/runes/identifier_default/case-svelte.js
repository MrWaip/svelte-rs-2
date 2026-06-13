import * as $ from "svelte/internal/client";
const s = ($$anchor, $$arg0) => {
	let x = $.derived_safe_equal(() => $.fallback($$arg0?.(), true));
	var button = root_1();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(x)));
	$.append($$anchor, button);
};
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let v = false;
	s($$anchor, () => v);
}
