import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor) {
	let checked = $.state(false);
	var input = root();
	$.remove_input_defaults(input);
	input.value = input.__value = "x";
	$.bind_checked(input, () => $.get(checked), ($$value) => $.set(checked, $$value));
	$.append($$anchor, input);
}
