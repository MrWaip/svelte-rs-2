import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	const $$slots = $.sanitize_slots($$props);
	let x = $.prop($$props, "x", 8);
	$.next();
	var text = $.text();
	text.nodeValue = `${$.untrack(() => $$slots.default) ?? ""}${$$slots ?? ""}`;
	$.append($$anchor, text);
}
