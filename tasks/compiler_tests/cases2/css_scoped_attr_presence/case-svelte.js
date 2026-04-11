import * as $ from "svelte/internal/client";
var root = $.from_html(`<div data-role="banner" class="svelte-1oq68bp">a</div> <span>b</span>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(2);
	$.append($$anchor, fragment);
}
