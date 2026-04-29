import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let count = $.prop($$props, "count", 12, 0);
	count();
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, ($.deep_read_state(count()), $.untrack(() => $.update_prop(count)))));
	$.append($$anchor, text);
}
