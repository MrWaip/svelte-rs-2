import * as $ from "svelte/internal/client";
var root = $.from_html(`<button></button>`);
export default function App($$anchor) {
	function increment() {
		shared++;
	}
	var button = root();
	button.textContent = doubled;
	$.delegated("click", button, increment);
	$.append($$anchor, button);
}
$.delegate(["click"]);
