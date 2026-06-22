import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { createBubbler } from "svelte/legacy";
var root = $.from_html(`<input type="text"/>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const bubbler = createBubbler();
	$.init();
	var input = root();
	var event_handler = $.derived(() => bubbler("click"));
	$.event("click", input, function(...$$args) {
		$.get(event_handler)?.apply(this, $$args);
	});
	$.append($$anchor, input);
	$.pop();
}
