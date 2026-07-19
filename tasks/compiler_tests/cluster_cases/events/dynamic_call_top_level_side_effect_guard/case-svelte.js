import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>go</button>`);
export default function App($$anchor) {
	let n = $.state(0);
	function makeHandler() {
		return () => $.update(n);
	}
	var button = root();
	var event_handler = $.derived(makeHandler);
	$.delegated("click", button, function(...$$args) {
		$.get(event_handler)?.apply(this, $$args);
	});
	$.append($$anchor, button);
}
$.delegate(["click"]);
