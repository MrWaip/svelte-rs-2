import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let x = $.prop($$props, "a-b", 3, 1);
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, x()));
	$.append($$anchor, text);
}
