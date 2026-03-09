import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let title = $.state(10);
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, $.set(title, 30)));
	$.append($$anchor, text);
}
