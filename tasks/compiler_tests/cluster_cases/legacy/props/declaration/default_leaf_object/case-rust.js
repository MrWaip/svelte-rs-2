import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	let tmp = {}, a = $.prop($$props, "a", 24, () => $.fallback(tmp.a, 10)), b = $.prop($$props, "b", 24, () => $.fallback(tmp.b, 20));
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${b() ?? ""}`));
	$.append($$anchor, button);
}
