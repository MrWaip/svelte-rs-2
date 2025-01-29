import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	$.next();
	var text = $.text("some_text");
	$.append($$anchor, text);
}
