import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="box-inner svelte-1ti3mv8"><div data-margin-right="auto">a</div></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
