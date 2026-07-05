import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>Click</button>`);
export default function App($$anchor) {
	let count = $.state(0);
	function getHandler() {
		return () => $.update(count);
	}
	var button = root();
	var event_handler = $.derived(getHandler);
	$.delegated("click", button, function(...$$args) {
		$.get(event_handler)?.apply(this, $$args);
	});
	$.append($$anchor, button);
}
$.delegate(["click"]);
