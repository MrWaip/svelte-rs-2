import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>hello</div>`);
export default function App($$anchor) {
	var data, handler;
	var $$promises = $.run([async () => data = await fetch("/api"), () => handler = $.proxy(data.handler)]);
	var div = root();
	$.run_after_blockers([$$promises[1]], () => {
		$.attach(div, () => handler);
	});
	$.append($$anchor, div);
}
