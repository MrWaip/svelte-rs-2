import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="a svelte-xfktj3" data-state="open"></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
