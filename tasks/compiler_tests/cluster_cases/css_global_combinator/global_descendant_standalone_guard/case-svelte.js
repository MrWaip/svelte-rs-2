import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="a svelte-1fgvoxa">x</div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
