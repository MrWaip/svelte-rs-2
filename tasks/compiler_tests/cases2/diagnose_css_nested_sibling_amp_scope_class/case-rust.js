import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="a svelte-152x0vu"></div> <div class="b svelte-152x0vu">x</div>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(2);
	$.append($$anchor, fragment);
}
