import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option></option></select>`);
export default function App($$anchor) {
	let value = 1;
	var select = root();
	var option = $.child(select);
	option.textContent = "1";
	$.reset(select);
	$.append($$anchor, select);
}
