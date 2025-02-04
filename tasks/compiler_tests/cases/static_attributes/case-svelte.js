import * as $ from "svelte/internal/client";
var root = $.template(`<input>`);
export default function App($$anchor) {
	const title = "world";
	var input = root();
	$.set_attribute(input, "title", title);
	$.set_attribute(input, "invisible", false);
	$.set_attribute(input, "concatenation", `__${title ?? ""}__`);
	$.append($$anchor, input);
}
