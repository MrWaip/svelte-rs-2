import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>go</button>`);
export default function App($$anchor) {
	let obj = $.proxy({ a: 1 });
	function report() {
		console.log(obj);
	}
	var button = root();
	$.delegated("click", button, report);
	$.append($$anchor, button);
}
$.delegate(["click"]);
