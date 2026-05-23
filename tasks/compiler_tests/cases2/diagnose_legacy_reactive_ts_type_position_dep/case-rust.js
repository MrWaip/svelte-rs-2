import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import data from "./dep.js";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const doubled = $.mutable_source();
	let count = 0;
	$.legacy_pre_effect(() => {}, () => {
		$.set(doubled, { value: count });
	});
	$.legacy_pre_effect_reset();
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, ($.get(doubled), $.untrack(() => $.get(doubled).value))));
	$.append($$anchor, text);
	$.pop();
}
