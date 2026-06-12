import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { createBubbler } from "svelte/legacy";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let rest = $.prop($$props, "rest", 24, () => ({}));
	const bubbler = createBubbler();
	$.init();
	var div = root();
	var event_handler = $.derived(() => bubbler("click"));
	var event_handler_1 = $.derived(() => bubbler("change"));
	$.attribute_effect(div, () => ({ ...rest() }));
	$.event("click", div, function(...$$args) {
		$.get(event_handler)?.apply(this, $$args);
	});
	$.event("change", div, function(...$$args) {
		$.get(event_handler_1)?.apply(this, $$args);
	});
	$.append($$anchor, div);
	$.pop();
}
