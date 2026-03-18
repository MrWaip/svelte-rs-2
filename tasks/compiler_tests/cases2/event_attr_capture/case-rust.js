import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>Click</button>`);
export default function App($$anchor) {
	function handler() {
		console.log("click capture");
	}
	var button = root();
	$.event("click", button, handler, true);
	$.append($$anchor, button);
}
