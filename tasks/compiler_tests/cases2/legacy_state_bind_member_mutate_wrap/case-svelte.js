import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor) {
	let obj = $.mutable_source({ x: 1 });
	var input = root();
	$.remove_input_defaults(input);
	$.bind_value(input, () => $.get(obj).x, ($$value) => $.mutate(obj, $.get(obj).x = $$value));
	$.append($$anchor, input);
}
