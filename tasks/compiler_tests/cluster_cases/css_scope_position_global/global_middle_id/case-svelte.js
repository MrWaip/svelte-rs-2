import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span id="i" class="svelte-277zqs">s</span>`);
export default function App($$anchor) {
	var span = root();
	$.append($$anchor, span);
}
