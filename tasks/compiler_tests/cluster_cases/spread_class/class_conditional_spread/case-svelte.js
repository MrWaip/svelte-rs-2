import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let foo = $.prop($$props, "foo", 8);
	let a = $.prop($$props, "a", 8);
	let b = $.prop($$props, "b", 8);
	let attributes = $.prop($$props, "attributes", 24, () => ({}));
	var div = root();
	$.attribute_effect(div, () => ({
		class: foo() ? a() : b(),
		...attributes()
	}));
	$.append($$anchor, div);
}
