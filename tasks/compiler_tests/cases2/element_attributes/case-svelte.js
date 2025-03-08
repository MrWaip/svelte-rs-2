import * as $ from "svelte/internal/client";
var root = $.template(`<div title="hockey" visible=""></div>`);
export default function App($$anchor) {
	var div = root();
	$.set_attribute(div, "expression", name);
	$.set_attribute(div, "description", description);
	$.append($$anchor, div);
}
