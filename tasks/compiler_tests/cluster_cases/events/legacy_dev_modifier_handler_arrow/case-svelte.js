import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>x</button>`);
export default function App($$anchor) {
	let n = 0;
	var button = root();
	$.event("click", button, $.preventDefault((e) => n++));
	$.append($$anchor, button);
}
