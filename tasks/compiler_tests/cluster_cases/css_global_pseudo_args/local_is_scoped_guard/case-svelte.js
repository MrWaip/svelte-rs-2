import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="svelte-mo6qgo"><span class="a svelte-mo6qgo">s</span></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
