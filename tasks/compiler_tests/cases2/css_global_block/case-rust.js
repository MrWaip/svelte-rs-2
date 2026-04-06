import * as $ from "svelte/internal/client";
var root = $.from_html(`<p class="svelte-1xnp9hn">scoped content</p> <h2>global heading</h2>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(2);
	$.append($$anchor, fragment);
}
