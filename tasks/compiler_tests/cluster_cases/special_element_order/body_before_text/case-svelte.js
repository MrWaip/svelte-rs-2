import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let foo = $.prop($$props, "foo", 12);
	$.next();
	var text = $.text();
	$.event("click", $.document.body, () => foo(!foo()));
	$.template_effect(() => $.set_text(text, foo()));
	$.append($$anchor, text);
}
