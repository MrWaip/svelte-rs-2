import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor) {
	var input = root();
	$.remove_input_defaults(input);
	$.set_value(input, false);
	$.append($$anchor, input);
}
