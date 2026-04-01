import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>Touch</div>`);
export default function App($$anchor) {
	function handleMove() {
		console.log("move");
	}
	var div = root();
	$.event("touchmove", div, handleMove, false, false);
	$.append($$anchor, div);
}
