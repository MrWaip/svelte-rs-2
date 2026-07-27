import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	var ref;
	var $$promises = $.run([() => Promise.resolve(), () => ref = $.state(null)]);
	var div = root();
	$.run_after_blockers([$$promises[1]], () => {
		$.bind_this(div, ($$value) => $.set(ref, $$value), () => $.get(ref));
	});
	$.append($$anchor, div);
}
