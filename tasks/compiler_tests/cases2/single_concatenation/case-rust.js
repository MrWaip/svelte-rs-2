import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	$.next();
	var text = $.text();
	$.set_text(text, `some text ${some_varaible ?? ""} after text`);
	$.append($$anchor, text);
}
