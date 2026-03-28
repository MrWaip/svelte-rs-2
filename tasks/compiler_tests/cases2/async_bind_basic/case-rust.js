import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor) {
	let x = 1;
	var data, value;
	var $$promises = $.run([async () => data = await fetch("/api"), () => value = $.state($.proxy(data.text))]);
	var input = root();
	$.remove_input_defaults(input);
	$.run_after_blockers([$$promises[1]], () => {
		$.bind_value(input, () => $.get(value), ($$value) => $.set(value, $$value));
	});
	$.append($$anchor, input);
}
