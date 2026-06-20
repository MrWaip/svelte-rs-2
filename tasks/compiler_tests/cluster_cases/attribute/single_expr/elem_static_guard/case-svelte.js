import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input data-x="text"/>`);
export default function App($$anchor) {
	var input = root();
	$.append($$anchor, input);
}
