import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>hi</div>`);
export default function App($$anchor, $$props) {
	let cond = $.prop($$props, "cond", 3, false);
	var div = root();
	let classes;
	$.template_effect(() => classes = $.set_class(div, 1, "header", null, classes, { slot: cond() }));
	$.append($$anchor, div);
}
