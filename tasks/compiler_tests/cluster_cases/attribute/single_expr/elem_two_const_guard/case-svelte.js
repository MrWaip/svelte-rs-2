import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor) {
	var input = root();
	$.set_attribute(input, "data-x", "falsetrue");
	$.append($$anchor, input);
}
