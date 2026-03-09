import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	$.next();
	var text = $.text();
	text.nodeValue = some_varaible;
	$.append($$anchor, text);
}
