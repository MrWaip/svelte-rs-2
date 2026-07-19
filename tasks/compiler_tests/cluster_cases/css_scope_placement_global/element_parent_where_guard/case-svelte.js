import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="svelte-1rdk5m"><span class="a svelte-1rdk5m">a</span></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
