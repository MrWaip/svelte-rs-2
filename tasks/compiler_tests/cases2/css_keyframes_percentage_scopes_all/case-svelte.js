import * as $ from "svelte/internal/client";
var root = $.from_html(`<p class="svelte-a7w5kd">plain</p> <button class="svelte-a7w5kd">btn</button>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(2);
	$.append($$anchor, fragment);
}
