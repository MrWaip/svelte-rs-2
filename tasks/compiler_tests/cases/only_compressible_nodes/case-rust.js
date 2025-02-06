import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let name = undefined;
	$.next();
	var text = $.text();
	text.textContent = `text + ${name ?? ""} + text`;
	$.append($$anchor, text);
}
