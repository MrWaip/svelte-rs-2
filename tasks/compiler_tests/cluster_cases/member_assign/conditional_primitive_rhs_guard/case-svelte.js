import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>go</button>`);
export default function App($$anchor) {
	let result = {};
	let data = {};
	function fill(keys) {
		keys.forEach((key) => result[key] = data[key] ? true : false);
	}
	var button = root();
	$.delegated("click", button, () => fill([]));
	$.append($$anchor, button);
}
$.delegate(["click"]);
