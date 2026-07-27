import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	var div = root();
	$.set_attribute(div, "title", { m(x) {
		return x;
	} });
	$.append($$anchor, div);
}
