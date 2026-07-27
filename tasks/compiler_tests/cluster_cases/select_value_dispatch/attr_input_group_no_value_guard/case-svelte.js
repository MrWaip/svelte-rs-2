import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor) {
	const binding_group = [];
	let group = $.state($.proxy([]));
	var input = root();
	$.remove_input_defaults(input);
	$.bind_group(binding_group, [], input, () => $.get(group), ($$value) => $.set(group, $$value));
	$.append($$anchor, input);
}
