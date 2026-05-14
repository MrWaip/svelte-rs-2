import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="box svelte-13p96nx"></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
