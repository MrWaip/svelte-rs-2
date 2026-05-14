import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor, $$props) {
	var input = root();
	input.defaultValue = "3";
	$.template_effect(() => $.set_value(input, $$props.x));
	$.append($$anchor, input);
}
