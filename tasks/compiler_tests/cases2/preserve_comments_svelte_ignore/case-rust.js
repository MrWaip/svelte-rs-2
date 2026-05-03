import * as $ from "svelte/internal/client";
var root = $.from_html(`<input autofocus=""/>`);
export default function App($$anchor) {
	var input = root();
	$.append($$anchor, input);
}
