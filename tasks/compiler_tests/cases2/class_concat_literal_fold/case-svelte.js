import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>content</div>`);
export default function App($$anchor) {
	var div = root();
	$.set_class(div, 1, "1231 1231");
	$.append($$anchor, div);
}
