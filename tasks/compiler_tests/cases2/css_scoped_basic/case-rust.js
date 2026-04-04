import * as $ from "svelte/internal/client";
var root = $.from_html(`<p class="svelte-1a7i8ec">styled</p>`);
export default function App($$anchor) {
	var p = root();
	$.append($$anchor, p);
}
