import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="known svelte-5xgcxy"><span class="a svelte-5xgcxy">a</span></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
