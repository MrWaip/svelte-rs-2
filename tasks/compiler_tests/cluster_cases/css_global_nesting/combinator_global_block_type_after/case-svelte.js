import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="svelte-493t75"><p><span class="y">y</span></p></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
