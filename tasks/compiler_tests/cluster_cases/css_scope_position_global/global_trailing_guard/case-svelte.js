import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span class="x svelte-1yr508m">s</span>`);
export default function App($$anchor) {
	var span = root();
	$.append($$anchor, span);
}
