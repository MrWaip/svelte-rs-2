import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor, $$props) {
	let a = $.prop($$props, "a", 8);
	let b = $.prop($$props, "b", 8);
	var input = root();
	$.template_effect(() => $.set_attribute(input, "data-x", `${a() ?? ""}${b() ?? ""}`));
	$.append($$anchor, input);
}
