import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let a = $.prop($$props, "a", 8, 1);
	let b = $.prop($$props, "b", 24, a);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${b() ?? ""}`));
	$.append($$anchor, p);
}
