import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>go</button>`);
export default function App($$anchor, $$props) {
	function report() {
		console.error("Error: " + $$props.message);
	}
	var button = root();
	$.delegated("click", button, report);
	$.append($$anchor, button);
}
$.delegate(["click"]);
