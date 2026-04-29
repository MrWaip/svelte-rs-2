import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let count = $.prop($$props, "count", 12, 0);
	count(count() - 7);
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, count()));
	$.append($$anchor, text);
}
