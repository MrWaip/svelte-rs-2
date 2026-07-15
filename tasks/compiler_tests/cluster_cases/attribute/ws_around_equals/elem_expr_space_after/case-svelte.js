import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor, $$props) {
	let value = $.prop($$props, "value", 8);
	var input = root();
	$.template_effect(() => $.set_attribute(input, "data-x", value()));
	$.append($$anchor, input);
}
