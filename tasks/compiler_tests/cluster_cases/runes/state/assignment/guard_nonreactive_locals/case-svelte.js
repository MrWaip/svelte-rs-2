import * as $ from "svelte/internal/client";
var root = $.from_html(`<button></button>`);
export default function App($$anchor) {
	let count = 0;
	function update() {
		let x, y;
		[x, y] = [1, 2];
		return x + y;
	}
	var button = root();
	button.textContent = "0";
	$.delegated("click", button, update);
	$.append($$anchor, button);
}
$.delegate(["click"]);
