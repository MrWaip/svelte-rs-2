import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span class="x svelte-uuzkzy">s</span>`);
export default function App($$anchor) {
	var span = root();
	$.append($$anchor, span);
}
