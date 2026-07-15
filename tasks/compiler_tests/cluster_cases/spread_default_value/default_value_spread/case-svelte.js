import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor) {
	let spread = {};
	let v = void 0;
	var input = root();
	$.attribute_effect(input, () => ({
		defaultValue: "x",
		value: v,
		...spread
	}));
	$.append($$anchor, input);
}
