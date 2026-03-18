import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>Click</button>`);
export default function App($$anchor) {
	function getHandler() {
		return () => console.log("clicked");
	}
	var button = root();
	var event_handler = $.derived(getHandler);
	$.delegated("click", button, function(...$$args) {
		$.get(event_handler)?.apply(this, $$args);
	});
	$.append($$anchor, button);
}
$.delegate(["click"]);
