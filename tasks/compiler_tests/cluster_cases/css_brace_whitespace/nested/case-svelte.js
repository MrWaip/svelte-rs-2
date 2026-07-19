import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="a svelte-1n8hfqt"><span class="b svelte-1n8hfqt">s</span></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
