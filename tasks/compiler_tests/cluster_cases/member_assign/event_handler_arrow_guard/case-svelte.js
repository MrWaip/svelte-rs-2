import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>go</button>`);
export default function App($$anchor) {
	let obj = $.proxy({ x: null });
	let src = $.proxy({});
	var button = root();
	$.delegated("click", button, () => obj.x = src);
	$.append($$anchor, button);
}
$.delegate(["click"]);
