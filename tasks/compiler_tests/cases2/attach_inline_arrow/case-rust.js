import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>content</div>`);
export default function App($$anchor) {
	let message = "hello";
	var div = root();
	$.attach(div, () => (el) => {
		el.textContent = message;
	});
	$.append($$anchor, div);
}
