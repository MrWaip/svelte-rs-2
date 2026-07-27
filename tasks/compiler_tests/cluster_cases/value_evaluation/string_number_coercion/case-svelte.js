import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor) {
	var p = root();
	p.textContent = "NaN NaN NaN NaN NaN -1 NaN false false";
	$.append($$anchor, p);
}
