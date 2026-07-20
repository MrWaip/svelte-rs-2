import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="row svelte-9ma9ug">x</div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
