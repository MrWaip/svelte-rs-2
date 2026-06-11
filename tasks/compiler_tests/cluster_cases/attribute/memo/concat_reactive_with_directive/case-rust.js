import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let status = $.prop($$props, "status", 8, "neutral");
	let disabled = $.prop($$props, "disabled", 8, false);
	var div = root();
	let classes;
	$.template_effect(() => classes = $.set_class(div, 1, `slider ${(status() || "") ?? ""}`, null, classes, { disabled: disabled() }));
	$.append($$anchor, div);
}
