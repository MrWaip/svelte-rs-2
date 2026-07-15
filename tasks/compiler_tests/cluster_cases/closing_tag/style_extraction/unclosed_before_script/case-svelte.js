import * as $ from "svelte/internal/client";
var root = $.from_html(`<div><span>x</span></div>`);
export default function App($$anchor) {
	let a = 1;
	var div = root();
	$.append($$anchor, div);
}
