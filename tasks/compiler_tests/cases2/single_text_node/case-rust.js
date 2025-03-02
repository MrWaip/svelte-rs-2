import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	$.next();
	var text = $.text("some long text line");
	$.append($$anchor, text);
}
