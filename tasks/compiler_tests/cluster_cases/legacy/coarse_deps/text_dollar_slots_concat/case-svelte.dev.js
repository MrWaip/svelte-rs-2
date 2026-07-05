import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	const $$slots = $.sanitize_slots($$props);
	$.push($$props, false, App);
	let x = $.prop($$props, "x", 8);
	var $$exports = { ...$.legacy_api() };
	$.next();
	var text = $.text();
	text.nodeValue = `${$.untrack(() => $$slots.default) ?? ""}${$$slots ?? ""}`;
	$.append($$anchor, text);
	return $.pop($$exports);
}
