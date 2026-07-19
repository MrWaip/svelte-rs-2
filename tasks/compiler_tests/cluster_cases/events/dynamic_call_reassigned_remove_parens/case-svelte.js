import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>x</button>`);
export default function App($$anchor) {
	let makeHandler = $.mutable_source(null);
	$.set(makeHandler, () => () => console.log("x"));
	var button = root();
	var event_handler = $.derived(() => $.get(makeHandler)());
	$.event("click", button, function(...$$args) {
		$.get(event_handler)?.apply(this, $$args);
	});
	$.append($$anchor, button);
}
