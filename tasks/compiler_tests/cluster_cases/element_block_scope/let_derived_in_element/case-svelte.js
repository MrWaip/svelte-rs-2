import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	var div = root();
	{
		let dt = $.derived(() => 1 + 2);
		div.textContent = typeof $.get(dt);
	}
	$.append($$anchor, div);
}
