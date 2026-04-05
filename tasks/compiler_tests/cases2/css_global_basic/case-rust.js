import * as $ from "svelte/internal/client";
var root = $.from_html(`<p class="svelte-r1lfc1">content</p> <p class="active svelte-r1lfc1">active</p> <h2>heading</h2> <h2 class="featured">featured</h2>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(6);
	$.append($$anchor, fragment);
}
