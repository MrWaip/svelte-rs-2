import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="a svelte-1o5tamq"><div class="b svelte-1o5tamq"></div></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
