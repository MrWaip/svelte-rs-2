import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	$.next();
	var text = $.text();
	text.nodeValue = `A${x ?? ""}B`;
	$.append($$anchor, text);
}
