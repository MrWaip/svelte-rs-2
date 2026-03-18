import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>Touch</button>`);
export default function App($$anchor) {
	function handler() {
		console.log("touch");
	}
	var button = root();
	$.delegated("touchstart", button, handler, void 0, true);
	$.append($$anchor, button);
}
$.delegate(["touchstart"]);
