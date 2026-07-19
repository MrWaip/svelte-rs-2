import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>go</button>`);
export default function App($$anchor) {
	function log() {
		console.log($.effect_tracking());
	}
	var button = root();
	$.delegated("click", button, log);
	$.append($$anchor, button);
}
$.delegate(["click"]);
