import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>go</button>`);
export default function App($$anchor) {
	let cache = $.proxy({});
	async function go() {
		const value = cache.value ??= await get_value();
	}
	async function get_value() {
		return 42;
	}
	var button = root();
	$.delegated("click", button, go);
	$.append($$anchor, button);
}
$.delegate(["click"]);
