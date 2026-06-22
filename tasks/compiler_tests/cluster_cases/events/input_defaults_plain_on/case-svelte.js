import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor, $$props) {
	let checked = $.prop($$props, "checked", 12, false);
	function onChange() {}
	var input = root();
	$.remove_input_defaults(input);
	$.bind_checked(input, checked);
	$.event("change", input, onChange);
	$.append($$anchor, input);
}
