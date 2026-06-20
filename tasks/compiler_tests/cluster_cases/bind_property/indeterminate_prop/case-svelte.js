import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor, $$props) {
	let indeterminate = $.prop($$props, "indeterminate", 12);
	var input = root();
	$.bind_property("indeterminate", "change", input, indeterminate, indeterminate);
	$.append($$anchor, input);
}
