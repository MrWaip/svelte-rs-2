import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="inset cell svelte-170v22j"></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
