import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let c = $.prop($$props, "c", 8);
	let s = $.prop($$props, "s", 8);
	let attributes = $.prop($$props, "attributes", 24, () => ({}));
	var div = root();
	$.attribute_effect(div, () => ({
		class: c(),
		style: s(),
		...attributes()
	}));
	$.append($$anchor, div);
}
