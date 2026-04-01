import * as $ from "svelte/internal/client";
var root = $.from_html(`<textarea></textarea>`);
export default function App($$anchor) {
	let value = "hello";
	var textarea = root();
	textarea.textContent = "hello";
	$.append($$anchor, textarea);
}
