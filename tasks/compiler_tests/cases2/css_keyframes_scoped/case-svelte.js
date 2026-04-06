import * as $ from "svelte/internal/client";
var root = $.from_html(`<p class="svelte-13q9pxc">animated</p> <div class="svelte-13q9pxc">also animated</div>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(2);
	$.append($$anchor, fragment);
}
