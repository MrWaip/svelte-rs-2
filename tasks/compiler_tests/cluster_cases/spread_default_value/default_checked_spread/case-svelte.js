import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor) {
	let spread = {};
	let c = void 0;
	var input = root();
	$.attribute_effect(input, () => ({
		type: "checkbox",
		defaultChecked: true,
		checked: c,
		...spread
	}));
	$.append($$anchor, input);
}
