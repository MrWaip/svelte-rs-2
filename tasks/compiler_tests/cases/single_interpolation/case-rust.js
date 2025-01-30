import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let name = undefined;
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, name));
	$.append($$anchor, text);
}
