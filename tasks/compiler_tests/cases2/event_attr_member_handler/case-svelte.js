import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>Click</button>`);
export default function App($$anchor) {
	let obj = { method() {
		console.log("clicked");
	} };
	var button = root();
	$.delegated("click", button, function(...$$args) {
		obj.method?.apply(this, $$args);
	});
	$.append($$anchor, button);
}
$.delegate(["click"]);
