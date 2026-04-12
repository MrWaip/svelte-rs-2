import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button></button>`);
export default function App($$anchor) {
	var count = 0;
	function increment() {
		count += 1;
	}
	var button = root();
	button.textContent = `clicks: ${count ?? ""}`;
	$.delegated("click", button, increment);
	$.append($$anchor, button);
}
$.delegate(["click"]);
