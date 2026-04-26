import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let obj = $.prop($$props, "obj", 24, () => ({ a: 1 }));
	obj(obj().a = 99, true);
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, obj().a));
	$.append($$anchor, p);
}
