import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	$.next();
	var text = $.text("hello");
	$.append($$anchor, text);
}
