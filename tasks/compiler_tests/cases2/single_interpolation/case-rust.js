import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	$.next();
	var text = $.text();
	$.set_text(text, some_varaible);
	$.append($$anchor, text);
}
