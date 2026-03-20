import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor) {
	let value = $.state("");
	function action(node) {}
	var input = root();
	$.remove_input_defaults(input);
	$.action(input, ($$node) => action?.($$node));
	$.effect(() => $.bind_value(input, () => $.get(value), ($$value) => $.set(value, $$value)));
	$.append($$anchor, input);
}
