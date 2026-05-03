import * as $ from "svelte/internal/client";
var root = $.from_html(`<div><!-- hello world --> <span>after</span></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
