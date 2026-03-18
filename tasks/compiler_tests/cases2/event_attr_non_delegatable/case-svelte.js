import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>content</div>`);
export default function App($$anchor) {
	function handler() {
		console.log("scrolled");
	}
	var div = root();
	$.event("scroll", div, handler);
	$.append($$anchor, div);
}
