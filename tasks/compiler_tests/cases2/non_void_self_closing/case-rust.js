import * as $ from "svelte/internal/client";
var root = $.from_html(`<textarea></textarea>`);
export default function App($$anchor) {
	var textarea = root();
	$.append($$anchor, textarea);
}
