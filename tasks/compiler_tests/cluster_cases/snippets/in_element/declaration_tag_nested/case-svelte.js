import * as $ from "svelte/internal/client";
var root = $.from_html(`<div><span></span></div>`);
export default function App($$anchor) {
	var div = root();
	var span = $.child(div);
	{
		const x = 1;
	}
	$.reset(div);
	$.append($$anchor, div);
}
