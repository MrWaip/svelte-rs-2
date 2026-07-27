import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor) {
	let obj = $.proxy({ x: 0 });
	let value = 0;
	var input = root();
	$.remove_input_defaults(input);
	$.bind_value(input, () => value, (v) => obj.x = { n: v });
	$.append($$anchor, input);
}
