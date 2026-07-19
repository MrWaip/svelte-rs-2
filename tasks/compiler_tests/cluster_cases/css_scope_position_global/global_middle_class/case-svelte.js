import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span class="x svelte-so7yai">s</span>`);
export default function App($$anchor) {
	var span = root();
	$.append($$anchor, span);
}
