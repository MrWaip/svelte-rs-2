import * as $ from "svelte/internal/client";
var root = $.from_html(`<input value="a"/>`);
export default function App($$anchor) {
	var input = root();
	$.append($$anchor, input);
}
