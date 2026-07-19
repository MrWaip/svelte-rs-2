import * as $ from "svelte/internal/client";
var root = $.from_html(`<div><span></span></div>`);
export default function App($$anchor) {
	var div = root();
	{
		const x = 5;
		var span = $.child(div);
		span.textContent = "5";
		$.reset(div);
	}
	$.append($$anchor, div);
}
