import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let myClass = $.prop($$props, "myClass", 8);
	let flag = $.prop($$props, "flag", 8);
	let attributes = $.prop($$props, "attributes", 24, () => ({}));
	var div = root();
	$.attribute_effect(div, () => ({
		class: myClass(),
		...attributes(),
		[$.CLASS]: { on: flag() }
	}));
	$.append($$anchor, div);
}
