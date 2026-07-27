import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor) {
	let a = "x";
	let g = $.state(void 0);
	var input = root();
	$.remove_input_defaults(input);
	input.value = input.__value = a;
	$.bind_checked(input, () => $.get(g), ($$value) => $.set(g, $$value));
	$.append($$anchor, input);
}
