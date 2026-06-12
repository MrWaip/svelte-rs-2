import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { createBubbler } from "svelte/legacy";
var root = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let checked = $.prop($$props, "checked", 12, false);
	const bubbler = createBubbler();
	$.init();
	var input = root();
	var event_handler = $.derived(() => bubbler("change"));
	$.remove_input_defaults(input);
	$.bind_checked(input, checked);
	$.event("change", input, function(...$$args) {
		$.get(event_handler)?.apply(this, $$args);
	});
	$.append($$anchor, input);
	$.pop();
}
