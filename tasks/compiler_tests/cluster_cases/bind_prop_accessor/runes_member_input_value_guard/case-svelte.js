import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor) {
	let obj = $.proxy({ v: "x" });
	var input = root();
	$.remove_input_defaults(input);
	$.bind_value(input, () => obj.v, ($$value) => obj.v = $$value);
	$.append($$anchor, input);
}
