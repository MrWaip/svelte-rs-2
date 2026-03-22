import * as $ from "svelte/internal/client";
var root = $.from_html(`<div><p></p></div>`);
export default function App($$anchor) {
	let x = 1;
	var div = root();
	var p = $.child(div);
	p.textContent = "Value: 1";
	$.reset(div);
	$.append($$anchor, div);
}
