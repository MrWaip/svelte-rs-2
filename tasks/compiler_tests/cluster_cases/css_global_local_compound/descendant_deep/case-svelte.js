import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="svelte-50uvp3"><p class="x svelte-50uvp3">p</p></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
