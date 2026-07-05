import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let obj = $.tag($.mutable_source({ x: 0 }), "obj");
	var $$exports = { ...$.legacy_api() };
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, ($.get(obj), $.untrack(() => $.mutate(obj, $.get(obj).x++)))));
	$.append($$anchor, text);
	return $.pop($$exports);
}
