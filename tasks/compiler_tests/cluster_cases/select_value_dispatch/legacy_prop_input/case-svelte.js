import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor, $$props) {
	let name = $.prop($$props, "name", 12);
	var input = root();
	$.remove_input_defaults(input);
	$.bind_value(input, name);
	$.append($$anchor, input);
}
