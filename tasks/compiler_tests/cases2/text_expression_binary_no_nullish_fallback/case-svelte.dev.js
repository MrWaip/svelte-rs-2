import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let a = $.prop($$props, "a", 8, 1);
	let b = $.prop($$props, "b", 8, 2);
	var $$exports = { ...$.legacy_api() };
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, `${a() ?? ""} ${a() + b()}`));
	$.append($$anchor, text);
	return $.pop($$exports);
}
