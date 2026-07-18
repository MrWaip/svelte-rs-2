import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="svelte-u7wz8i"><p class="x svelte-u7wz8i">p</p></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
