import * as $ from "svelte/internal/client";
export const svelte4space = "svelte4space";
var root = $.from_html(`<div>hi</div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
