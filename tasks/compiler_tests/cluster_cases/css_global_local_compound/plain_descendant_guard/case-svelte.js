import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="svelte-1xdp3nc"><span class="x svelte-1xdp3nc">s</span></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
