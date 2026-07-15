import * as $ from "svelte/internal/client";
var root = $.from_html(`<table><tbody><tr><td>a</td></tr><tr><td>b</td></tr></tbody></table>`);
export default function App($$anchor) {
	var table = root();
	$.append($$anchor, table);
}
