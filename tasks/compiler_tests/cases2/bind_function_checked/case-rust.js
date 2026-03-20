import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor) {
	let checked = $.state(false);
	var input = root();
	$.remove_input_defaults(input);
	$.bind_checked(input, () => $.get(checked), (v) => $.set(checked, v, true));
	$.append($$anchor, input);
}
