import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let count = $.prop($$props, "count", 12, 0);
	count();
	var $$exports = { ...$.legacy_api() };
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, ($.deep_read_state(count()), $.untrack(() => $.update_prop(count)))));
	$.append($$anchor, text);
	return $.pop($$exports);
}
