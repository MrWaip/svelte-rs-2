import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let obj = $.mutable_source({ x: 0 });
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, ($.get(obj), $.untrack(() => $.mutate(obj, $.get(obj).x++)))));
	$.append($$anchor, text);
}
