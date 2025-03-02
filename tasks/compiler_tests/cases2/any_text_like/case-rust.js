import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	$.next();
	var text = $.text();
	text.nodeValue = `some text ${some_varaible ?? ""} after text`;
	$.append($$anchor, text);
}
