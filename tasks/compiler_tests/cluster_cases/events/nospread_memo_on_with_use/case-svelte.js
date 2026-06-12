import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { createBubbler } from "svelte/legacy";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const bubbler = createBubbler();
	function action(node) {}
	$.init();
	var div = root();
	var event_handler = $.derived(() => bubbler("click"));
	$.action(div, ($$node) => action?.($$node));
	$.effect(() => $.event("click", div, function(...$$args) {
		$.get(event_handler)?.apply(this, $$args);
	}));
	$.append($$anchor, div);
	$.pop();
}
