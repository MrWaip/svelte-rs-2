import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="svelte-1k0loyk"><p class="svelte-1k0loyk"><span class="y">y</span></p></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
