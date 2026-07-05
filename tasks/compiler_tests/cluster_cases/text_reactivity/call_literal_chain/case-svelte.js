import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	var p = root();
	p.textContent = `v ${"name".toUpperCase().toLowerCase() ?? ""}`;
	$.append($$anchor, p);
	$.pop();
}
