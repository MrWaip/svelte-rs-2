App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let label = $.prop($$props, "label", 19, () => `untitled`);
	var $$exports = { ...$.legacy_api() };
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, label()));
	$.append($$anchor, text);
	return $.pop($$exports);
}
