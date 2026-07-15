import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor) {
	let v = void 0;
	var input = root();
	input.defaultValue = "x";
	$.set_value(input, v);
	$.append($$anchor, input);
}
