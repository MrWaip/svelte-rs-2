import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const total = 42;
	var $$exports = { count: total };
	var p = root();
	p.textContent = "42";
	$.append($$anchor, p);
	$.bind_prop($$props, "count", total);
	return $.pop($$exports);
}
