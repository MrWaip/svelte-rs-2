import * as $ from "svelte/internal/client";
var root = $.from_html(`<div><style>.x {
			color: red;
		}</style></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
