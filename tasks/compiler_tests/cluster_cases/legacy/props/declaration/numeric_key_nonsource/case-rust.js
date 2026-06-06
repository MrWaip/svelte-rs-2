import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, $$props["0"]));
	$.append($$anchor, text);
}
