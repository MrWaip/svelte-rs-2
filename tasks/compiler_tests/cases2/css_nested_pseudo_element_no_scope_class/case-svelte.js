import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="container svelte-f7z729"><p>x</p></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
