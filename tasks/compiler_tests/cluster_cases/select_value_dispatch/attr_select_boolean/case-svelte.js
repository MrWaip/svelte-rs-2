import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option>a</option><option>b</option></select>`);
export default function App($$anchor) {
	var select = root();
	select.value = select.__value = true;
	$.append($$anchor, select);
}
