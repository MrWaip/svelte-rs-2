import * as $ from "svelte/internal/client";
var root = $.from_html(`<keygen/> <command/> <br/>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(4);
	$.append($$anchor, fragment);
}
