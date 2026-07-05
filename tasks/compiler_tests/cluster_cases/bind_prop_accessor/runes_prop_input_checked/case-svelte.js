import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor, $$props) {
	let checked = $.prop($$props, "checked", 7, false);
	var input = root();
	$.remove_input_defaults(input);
	$.bind_checked(input, checked);
	$.append($$anchor, input);
}
