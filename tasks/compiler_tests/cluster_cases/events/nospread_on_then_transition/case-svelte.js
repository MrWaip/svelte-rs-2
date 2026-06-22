import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { createBubbler } from "svelte/legacy";
import { slide } from "svelte/transition";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const bubbler = createBubbler();
	$.init();
	var div = root();
	var event_handler = $.derived(() => bubbler("click"));
	$.event("click", div, function(...$$args) {
		$.get(event_handler)?.apply(this, $$args);
	});
	$.transition(3, div, () => slide);
	$.append($$anchor, div);
	$.pop();
}
