import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor) {
	let items = $.proxy([]);
	let total = $.derived(() => {
		return items.filter(Boolean).length;
	});
	var p = root();
	p.textContent = $.get(total);
	$.append($$anchor, p);
}
