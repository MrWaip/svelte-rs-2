import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let foo = $.prop($$props, "foo", 12);
	var $$exports = { ...$.legacy_api() };
	$.next();
	var text = $.text();
	$.event("click", $.document.body, function click() {
		return foo(!foo());
	});
	$.template_effect(() => $.set_text(text, foo()));
	$.append($$anchor, text);
	return $.pop($$exports);
}
