import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input data-x="value"/>`);
export default function App($$anchor, $$props) {
	let value = $.prop($$props, "value", 8);
	var input = root();
	$.append($$anchor, input);
}
