import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>click</button>`);
export default function App($$anchor) {
	let handler = () => {};
	var button = root();
	$.delegated("click", button, handler);
	$.append($$anchor, button);
}
$.delegate(["click"]);
