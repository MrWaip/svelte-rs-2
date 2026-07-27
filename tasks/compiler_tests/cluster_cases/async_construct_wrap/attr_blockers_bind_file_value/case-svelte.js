import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="file"/>`);
export default function App($$anchor) {
	var a;
	var $$promises = $.run([() => Promise.resolve(), () => a = "a"]);
	var input = root();
	$.remove_input_defaults(input);
	$.run_after_blockers([$$promises[1]], () => {
		$.bind_value(input, () => a, ($$value) => a = $$value);
	});
	$.append($$anchor, input);
}
