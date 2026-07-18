import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div id="123" class="svelte-1fu8qh9"></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
