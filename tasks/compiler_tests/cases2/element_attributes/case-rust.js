import * as $ from "svelte/internal/client";
var root = $.from_html(`<div title="hockey" visible=""></div>`);
export default function App($$anchor) {
	var div = root();
	$.set_attribute(div, "expression", name);
	$.set_attribute(div, "description", description);
	$.set_attribute(div, "index", `number: ${idx ?? ""}`);
	$.append($$anchor, div);
}
