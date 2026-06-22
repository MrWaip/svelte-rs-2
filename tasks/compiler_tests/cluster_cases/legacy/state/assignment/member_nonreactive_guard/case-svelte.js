import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button></button>`);
export default function App($$anchor) {
	let count = 0;
	function go() {
		let o = { a: 0 };
		[o.a] = [1];
		return o.a;
	}
	var button = root();
	button.textContent = "0";
	$.event("click", button, go);
	$.append($$anchor, button);
}
