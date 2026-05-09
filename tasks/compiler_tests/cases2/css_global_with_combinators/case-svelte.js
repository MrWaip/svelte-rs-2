import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="cell-block svelte-awt0xm"><div class="cell">a</div> <div class="cell"><div class="content">b</div></div></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
