import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import data from "./dep.js";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const doubled = $.mutable_source();
	let count = 0;
	$.legacy_pre_effect(() => {}, () => {
		$.set(doubled, { value: count });
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, ($.get(doubled), $.untrack(() => $.get(doubled).value))));
	$.append($$anchor, text);
	return $.pop($$exports);
}
