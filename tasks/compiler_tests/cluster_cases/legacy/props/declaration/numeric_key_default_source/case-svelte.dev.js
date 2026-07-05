App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let zero = $.prop($$props, 0, 3, 1);
	var $$exports = { ...$.legacy_api() };
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, zero()));
	$.append($$anchor, text);
	return $.pop($$exports);
}
