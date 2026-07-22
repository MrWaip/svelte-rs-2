import * as $ from "svelte/internal/client";
var root = $.from_html(`<div><p dir="rtl">static parent, no runtime reset</p></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
