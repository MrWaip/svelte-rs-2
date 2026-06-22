import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>x</button>`);
export default function App($$anchor) {
	let n = 0;
	function handler() {
		n++;
	}
	var button = root();
	$.event("click", button, handler);
	$.append($$anchor, button);
}
