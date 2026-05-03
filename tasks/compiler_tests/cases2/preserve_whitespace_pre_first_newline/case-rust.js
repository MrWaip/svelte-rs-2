import * as $ from "svelte/internal/client";
var root = $.from_html(`<pre></pre>`);
export default function App($$anchor) {
	let value = "hi";
	var pre = root();
	pre.textContent = "\nhi";
	$.append($$anchor, pre);
}
