import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>content</div>`);
export default function App($$anchor) {
	function handler() {
		console.log("got pointer capture");
	}
	var div = root();
	$.event("gotpointercapture", div, handler);
	$.append($$anchor, div);
}
