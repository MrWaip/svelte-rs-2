import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="radio"/>`);
export default function App($$anchor) {
	const binding_group = [];
	let group = $.state($.proxy([]));
	var input = root();
	$.remove_input_defaults(input);
	input.value = input.__value = "x";
	$.bind_group(binding_group, [], input, () => $.get(group), ($$value) => $.set(group, $$value));
	$.append($$anchor, input);
}
