import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<textarea>abc</textarea>`);
export default function App($$anchor) {
	var textarea = root();
	$.append($$anchor, textarea);
}
