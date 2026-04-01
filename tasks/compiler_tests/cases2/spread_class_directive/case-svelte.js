import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	let props = $.proxy({
		id: "a",
		class: "from-spread"
	});
	let active = true;
	var div = root();
	$.attribute_effect(div, () => ({
		...props,
		[$.CLASS]: { active }
	}));
	$.append($$anchor, div);
}
