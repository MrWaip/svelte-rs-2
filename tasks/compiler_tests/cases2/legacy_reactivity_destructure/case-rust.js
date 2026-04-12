import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button></button>`);
export default function App($$anchor) {
	let point = {
		left: 1,
		right: 2
	};
	let { left, right } = point;
	function swap() {
		[left, right] = [right, left];
	}
	var button = root();
	button.textContent = `${left ?? ""}:${right ?? ""}`;
	$.delegated("click", button, swap);
	$.append($$anchor, button);
}
$.delegate(["click"]);
