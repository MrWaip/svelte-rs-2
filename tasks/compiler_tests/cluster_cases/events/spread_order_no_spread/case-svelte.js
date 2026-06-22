import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { createBubbler } from "svelte/legacy";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let title = $.prop($$props, "title", 8, "");
	const bubbler = createBubbler();
	$.init();
	var div = root();
	var event_handler = $.derived(() => bubbler("click"));
	$.template_effect(() => $.set_attribute(div, "title", title()));
	$.event("click", div, function(...$$args) {
		$.get(event_handler)?.apply(this, $$args);
	});
	$.append($$anchor, div);
	$.pop();
}
