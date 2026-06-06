import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let zero = $.prop($$props, 0, 3, 1);
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, zero()));
	$.append($$anchor, text);
}
