import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>go</button>`);
export default function App($$anchor) {
	let cache = {};
	function set(item) {
		cache[item.id] = item;
	}
	var button = root();
	$.delegated("click", button, () => set({}));
	$.append($$anchor, button);
}
$.delegate(["click"]);
