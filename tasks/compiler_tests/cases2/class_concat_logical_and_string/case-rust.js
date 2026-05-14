import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let x = $.prop($$props, "x", 3, "foo");
	const cond = x() === "foo";
	var div = root();
	$.template_effect(() => $.set_class(div, 1, `a ${cond && "b"}`));
	$.append($$anchor, div);
}
