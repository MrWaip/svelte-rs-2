import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="checkbox" checked=""/>`);
export default function App($$anchor) {
	var input = root();
	$.remove_input_defaults(input);
	$.append($$anchor, input);
}
