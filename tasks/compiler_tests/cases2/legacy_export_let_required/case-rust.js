import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let foo = $.prop($$props, "foo", 8);
	let baz = $.prop($$props, "baz", 8, undefined);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${foo() ?? ""}${baz() ?? ""}`));
	$.append($$anchor, p);
}
