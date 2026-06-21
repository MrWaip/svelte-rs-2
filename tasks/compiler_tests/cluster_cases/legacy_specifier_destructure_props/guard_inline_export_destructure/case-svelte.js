import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let tmp = {
		a: 1,
		b: 2
	}, a = $.prop($$props, "a", 24, () => tmp.a), b = $.prop($$props, "b", 24, () => tmp.b);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${b() ?? ""}`));
	$.append($$anchor, p);
}
