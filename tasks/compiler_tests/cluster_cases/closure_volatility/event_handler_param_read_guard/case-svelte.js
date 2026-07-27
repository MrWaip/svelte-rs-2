import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>go</button>`);
export default function App($$anchor) {
	var button = root();
	$.delegated("click", button, (e) => e.target);
	$.append($$anchor, button);
}
$.delegate(["click"]);
