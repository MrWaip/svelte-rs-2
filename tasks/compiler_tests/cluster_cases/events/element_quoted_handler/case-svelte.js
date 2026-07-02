import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>x</button>`);
export default function App($$anchor) {
	var button = root();
	$.delegated("click", button, () => console.log("x"));
	$.append($$anchor, button);
}
$.delegate(["click"]);
