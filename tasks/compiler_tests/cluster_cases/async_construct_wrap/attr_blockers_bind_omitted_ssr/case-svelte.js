import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	var a;
	var $$promises = $.run([() => Promise.resolve(), () => a = 0]);
	var div = root();
	$.run_after_blockers([$$promises[1]], () => {
		$.bind_element_size(div, "clientWidth", ($$value) => a = $$value);
	});
	$.append($$anchor, div);
}
