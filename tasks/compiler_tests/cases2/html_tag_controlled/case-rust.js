import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	let content = "<b>hello</b>";
	var div = root();
	$.html(div, () => content, true);
	$.reset(div);
	$.append($$anchor, div);
}
