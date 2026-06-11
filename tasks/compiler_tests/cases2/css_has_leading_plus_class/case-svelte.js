import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="a svelte-frih9y">x</div> <div class="b svelte-frih9y">y</div>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(2);
	$.append($$anchor, fragment);
}
