import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option>x</option></select>`);
export default function App($$anchor) {
	var a;
	var $$promises = $.run([() => Promise.resolve(), () => a = "a"]);
	var select = root();
	$.run_after_blockers([$$promises[1]], () => {
		$.bind_select_value(select, () => a, ($$value) => a = $$value);
	});
	$.append($$anchor, select);
}
