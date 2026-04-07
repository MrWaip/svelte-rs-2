import * as $ from "svelte/internal/client";
var root = $.from_html(`<span class="badge svelte-9pvq8r">b</span>`);
export default function App($$anchor) {
	var span = root();
	$.append($$anchor, span);
}
