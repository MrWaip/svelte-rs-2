import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="used svelte-1n36she">used</div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
