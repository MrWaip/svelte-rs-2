import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="card svelte-1u8u4ji"><h2 class="title svelte-1u8u4ji">inside</h2></div> <section class="panel svelte-1u8u4ji"><h3 class="label svelte-1u8u4ji">implicit</h3></section> <h2 class="title">outside</h2> <h3 class="label">outside implicit</h3>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(6);
	$.append($$anchor, fragment);
}
