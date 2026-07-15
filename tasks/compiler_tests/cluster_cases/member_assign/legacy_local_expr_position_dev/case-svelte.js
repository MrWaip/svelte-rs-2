import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>go</button>`);
export default function App($$anchor) {
	let cache = {};
	function fill(items) {
		items.forEach((item) => cache[item.id] = item);
	}
	var button = root();
	$.event("click", button, () => fill([]));
	$.append($$anchor, button);
}
