import * as $ from "svelte/internal/client";
var root = $.from_html(`<p class="svelte-11581yn">content</p> <section class="svelte-11581yn"><span class="inner">inside</span></section>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(2);
	$.append($$anchor, fragment);
}
