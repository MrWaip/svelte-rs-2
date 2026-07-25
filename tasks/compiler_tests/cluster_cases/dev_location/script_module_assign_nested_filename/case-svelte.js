import * as $ from "svelte/internal/client";
const cache = {};
export function fill(items) {
	items.forEach((item) => cache[item.id] = item);
}
var root = $.from_html(`<button>go</button>`);
export default function App($$anchor) {
	var button = root();
	$.delegated("click", button, () => fill([]));
	$.append($$anchor, button);
}
$.delegate(["click"]);
