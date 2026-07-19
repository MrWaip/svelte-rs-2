import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>go</button>`);
export default function App($$anchor) {
	function setup(id) {
		const entry = globalThis.__cache[id] ??= {};
		return entry;
	}
	var button = root();
	$.delegated("click", button, () => setup(1));
	$.append($$anchor, button);
}
$.delegate(["click"]);
