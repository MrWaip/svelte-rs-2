import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	let tmp = {}, a = $.prop($$props, "a", 24, () => $.fallback(tmp.p, () => ({}), true).a);
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, a()));
	$.append($$anchor, button);
}
