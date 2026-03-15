import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>Click me</button>`);
export default function App($$anchor) {
	function handleClick() {
		console.log("clicked");
	}
	var button = root();
	$.event("click", button, handleClick);
	$.append($$anchor, button);
}
