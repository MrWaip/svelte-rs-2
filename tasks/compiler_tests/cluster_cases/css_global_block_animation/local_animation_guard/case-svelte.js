import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="x svelte-1l46h2h">d</div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
