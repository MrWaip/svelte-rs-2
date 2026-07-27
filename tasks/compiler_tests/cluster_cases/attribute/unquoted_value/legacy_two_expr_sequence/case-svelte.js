import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let a = $.prop($$props, "a", 8);
	let b = $.prop($$props, "b", 8);
	var div = root();
	$.template_effect(() => $.set_attribute(div, "foo", `${a() ?? ""}${b() ?? ""}`));
	$.append($$anchor, div);
}
