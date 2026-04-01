import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>Touch</div>`);
export default function App($$anchor) {
	function handleMove() {
		console.log("move");
	}
	var div = root();
	$.event("touchmove", div, handleMove, void 0, false);
	$.append($$anchor, div);
}
