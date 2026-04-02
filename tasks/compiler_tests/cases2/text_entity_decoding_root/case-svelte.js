import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let name = "Tom";
	$.next();
	var text = $.text();
	text.nodeValue = "& Tom <";
	$.append($$anchor, text);
}
