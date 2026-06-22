import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { createBubbler } from "svelte/legacy";
var root = $.from_html(`<textarea></textarea>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let value = $.prop($$props, "value", 12, "");
	const bubbler = createBubbler();
	$.init();
	var textarea = root();
	var event_handler = $.derived(() => bubbler("input"));
	$.remove_textarea_child(textarea);
	$.bind_value(textarea, value);
	$.event("input", textarea, function(...$$args) {
		$.get(event_handler)?.apply(this, $$args);
	});
	$.append($$anchor, textarea);
	$.pop();
}
