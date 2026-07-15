import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let on = $.prop($$props, "on", 8);
	let attributes = $.prop($$props, "attributes", 24, () => ({}));
	var div = root();
	$.attribute_effect(div, () => ({
		class: { active: on() },
		...attributes()
	}));
	$.append($$anchor, div);
}
