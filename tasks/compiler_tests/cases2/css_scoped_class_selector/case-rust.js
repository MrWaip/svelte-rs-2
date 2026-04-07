import * as $ from "svelte/internal/client";
var root = $.from_html(`<span class="badge">b</span>`);
export default function App($$anchor) {
	var span = root();
	$.append($$anchor, span);
}
