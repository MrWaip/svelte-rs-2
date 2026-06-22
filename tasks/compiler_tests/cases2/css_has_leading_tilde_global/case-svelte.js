import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="a svelte-ij4o60">x</div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
